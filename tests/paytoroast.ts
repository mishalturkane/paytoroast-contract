
import { Paytoroast } from "../target/types/paytoroast";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import {
    createMint,
    createAccount,
    mintTo,
    getAccount,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

describe("roast-escrow", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.RoastEscrow as Program<RoastEscrow>;
    const payer = provider.wallet;

    // Helper function to create a new keypair
    const createKeypair = (): Keypair => {
        return Keypair.generate();
    };

    // Helper function to get the current slot
    const getSlot = async (): Promise<number> => {
        return await provider.connection.getSlot();
    };

    let mint: PublicKey;
    let roaster: Keypair;
    let receiver: Keypair;
    let roasterTokenAccount: PublicKey;
    let escrowPda: PublicKey;
    let escrowPdaTokenAccount: PublicKey;
    let roastEscrowAccount: Keypair;
    let seedId: number;
    let bump: number;


    before(async () => {
        // Create a new token mint
        mint = await createMint(
            provider.connection,
            payer,
            payer.publicKey,
            null,
            0,
            TOKEN_PROGRAM_ID
        );

        // Generate keypairs for the roaster and receiver
        roaster = createKeypair();
        receiver = createKeypair();

        // Airdrop SOL to the roaster and receiver
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(roaster.publicKey, 1000000000), // 1 SOL
            "confirmed"
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(receiver.publicKey, 1000000000), // 1 SOL
            "confirmed"
        );

        // Derive the escrow PDA and associated token account
        seedId = Date.now(); // Use a unique seed
        [escrowPda, bump] = await PublicKey.findProgramAddress(
            [Buffer.from("escrow"), roaster.publicKey.toBuffer(), receiver.publicKey.toBuffer(), new anchor.BN(seedId).toArray('le', 8)],
            program.programId
        );

        escrowPdaTokenAccount = await getAssociatedTokenAddress(
            mint,
            escrowPda,
            true,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // Create the roaster's token account
        roasterTokenAccount = await getAssociatedTokenAddress(
            mint,
            roaster.publicKey,
            false,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        await createAccount(
            provider.connection,
            payer,
            mint,
            roaster.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // Mint tokens to the roaster's token account
        await mintTo(
            provider.connection,
            payer,
            mint,
            roasterTokenAccount,
            payer,
            1000 // 10 tokens
        );
        roastEscrowAccount = createKeypair();

    });

    it("Should initialize the roast escrow", async () => {
        const roastMessage = "This is a roast message.";
        const amount = new anchor.BN(100); // 1 token

        const tx = await program.methods.initializeRoast(roastMessage, amount, mint, new anchor.BN(seedId))
            .accounts({
                roaster: roaster.publicKey,
                roastEscrow: roastEscrowAccount.publicKey,
                escrowPda,
                escrowPdaTokenAccount,
                mint,
                receiver: receiver.publicKey,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            })
            .signers([roaster, roastEscrowAccount])
            .rpc();

        const account = await program.account.roastEscrow.fetch(roastEscrowAccount.publicKey);

        assert.ok(account.roaster.equals(roaster.publicKey));
        assert.ok(account.receiver.equals(receiver.publicKey));
        assert.ok(account.roastMessage === roastMessage);
        assert.ok(account.amount.eq(amount));
        assert.ok(account.mint.equals(mint));
        assert.ok(account.escrowPda.equals(escrowPda));
        assert.ok(account.status === "Pending");
        assert.ok(account.seedId.eq(new anchor.BN(seedId)));
        assert.ok(account.bump > 0);
    });

    it("Should deposit tokens into the escrow", async () => {
        const amount = new anchor.BN(100); // 100 tokens

        const preRoasterTokenBalance = await getAccount(provider.connection, roasterTokenAccount);
        const preEscrowTokenBalance = await getAccount(provider.connection, escrowPdaTokenAccount);

        const tx = await program.methods.depositInEscrow(amount)
            .accounts({
                roaster: roaster.publicKey,
                roastEscrow: roastEscrowAccount.publicKey,
                escrowPda,
                roasterTokenAccount,
                escrowPdaTokenAccount,
                mint,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([roaster])
            .rpc();

        const postRoasterTokenBalance = await getAccount(provider.connection, roasterTokenAccount);
        const postEscrowTokenBalance = await getAccount(provider.connection, escrowPdaTokenAccount);

        assert.ok(postRoasterTokenBalance.amount.eq(preRoasterTokenBalance.amount.sub(amount)));
        assert.ok(postEscrowTokenBalance.amount.eq(preEscrowTokenBalance.amount.add(amount)));
    });

    it("Should allow the receiver to accept the roast", async () => {
        const tx = await program.methods.receiverResponds(true)
            .accounts({
                receiver: receiver.publicKey,
                roastEscrow: roastEscrowAccount.publicKey,
            })
            .signers([receiver])
            .rpc();

        const account = await program.account.roastEscrow.fetch(roastEscrowAccount.publicKey);
        assert.ok(account.status === "Accepted");
    });

    it("Should transfer funds to the receiver when accepted", async () => {
        // Create receiver token account
        const receiverTokenAccount = await getAssociatedTokenAddress(
            mint,
            receiver.publicKey,
            false,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        await createAccount(
            provider.connection,
            payer,
            mint,
            receiver.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        const preReceiverTokenBalance = await getAccount(provider.connection, receiverTokenAccount);
        const preEscrowTokenBalance = await getAccount(provider.connection, escrowPdaTokenAccount);
        const roastEscrowAccountData = await program.account.roastEscrow.fetch(roastEscrowAccount.publicKey);

        const tx = await program.methods.executeTransfer()
            .accounts({
                roastEscrow: roastEscrowAccount.publicKey,
                escrowPda,
                escrowPdaTokenAccount,
                roasterTokenAccount: roasterTokenAccount,
                receiverTokenAccount: receiverTokenAccount,
                mint,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .signers([])
            .rpc();

        const postReceiverTokenBalance = await getAccount(provider.connection, receiverTokenAccount);
        const postEscrowTokenBalance = await getAccount(provider.connection, escrowPdaTokenAccount);

        assert.ok(postReceiverTokenBalance.amount.eq(preReceiverTokenBalance.amount.add(roastEscrowAccountData.amount)));
        assert.ok(postEscrowTokenBalance.amount.eq(new anchor.BN(0))); // Escrow account should be closed
    });

    it("Should allow the receiver to reject the roast", async () => {
        //re-initialize the scenario
        const roastMessage = "This is a roast message.";
        const amount = new anchor.BN(100);
        seedId = Date.now() + 1;
        [escrowPda, bump] = await PublicKey.findProgramAddress(
            [Buffer.from("escrow"), roaster.publicKey.toBuffer(), receiver.publicKey.toBuffer(), new anchor.BN(seedId).toArray('le', 8)],
            program.programId
        );
        escrowPdaTokenAccount = await getAssociatedTokenAddress(
            mint,
            escrowPda,
            true,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        roastEscrowAccount = createKeypair();

        await program.methods.initializeRoast(roastMessage, amount, mint, new anchor.BN(seedId))
            .accounts({
                roaster: roaster.publicKey,
                roastEscrow: roastEscrowAccount.publicKey,
                escrowPda,
                escrowPdaTokenAccount,
                mint,
                receiver: receiver.publicKey,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            })
            .signers([roaster, roastEscrowAccount])
            .rpc();

        await mintTo(
            provider.connection,
            payer,
            mint,
            roasterTokenAccount,
            payer,
            1000 // 10 tokens
        );

        await program.methods.depositInEscrow(amount)
            .accounts({
                roaster: roaster.publicKey,
                roastEscrow: roastEscrowAccount.publicKey,
                escrowPda,
                roasterTokenAccount,
                escrowPdaTokenAccount,
                mint,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([roaster])
            .rpc();


        const tx = await program.methods.receiverResponds(false)
            .accounts({
                receiver: receiver.publicKey,
                roastEscrow: roastEscrowAccount.publicKey,
            })
            .signers([receiver])
            .rpc();

        const account = await program.account.roastEscrow.fetch(roastEscrowAccount.publicKey);
        assert.ok(account.status === "Rejected");
    });

    it("Should refund funds to the roaster when rejected", async () => {
        const preRoasterTokenBalance = await getAccount(provider.connection, roasterTokenAccount);
        const preEscrowTokenBalance = await getAccount(provider.connection, escrowPdaTokenAccount);
        const roastEscrowAccountData = await program.account.roastEscrow.fetch(roastEscrowAccount.publicKey);


        const tx = await program.methods.executeTransfer()
            .accounts({
                roastEscrow: roastEscrowAccount.publicKey,
                escrowPda,
                escrowPdaTokenAccount,
                roasterTokenAccount: roasterTokenAccount,
                receiverTokenAccount: await getAssociatedTokenAddress(mint, receiver.publicKey), //need a receiver acc
                mint,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .signers([])
            .rpc();

        const postRoasterTokenBalance = await getAccount(provider.connection, roasterTokenAccount);
        const postEscrowTokenBalance = await getAccount(provider.connection, escrowPdaTokenAccount);

        assert.ok(postRoasterTokenBalance.amount.eq(preRoasterTokenBalance.amount.add(roastEscrowAccountData.amount)));
        assert.ok(postEscrowTokenBalance.amount.eq(new anchor.BN(0)));
    });

    it("Should close the roast escrow account", async () => {
        const preRoasterBalance = await provider.connection.getBalance(roaster.publicKey);
        const roastEscrowAccountData = await program.account.roastEscrow.fetch(roastEscrowAccount.publicKey);


        const tx = await program.methods.closeRoastEscrow()
            .accounts({
                roastEscrow: roastEscrowAccount.publicKey,
                receiver: receiver.publicKey, // or roaster.publicKey depending on status
                roaster: roaster.publicKey,
            })
            .signers([roaster])
            .rpc();

        const postRoasterBalance = await provider.connection.getBalance(roaster.publicKey);

        // Check if the roast escrow account has been closed (account info is null)
        const accountInfo = await provider.connection.getAccountInfo(roastEscrowAccount.publicKey);
        assert.isNull(accountInfo);

        // Verify that the roaster received the rent back (approximately)
        assert.isTrue(postRoasterBalance > preRoasterBalance);
    });
});
