import * as anchor from "@coral-xyz/anchor";
import {assert} from "chai";
import {createMintToInstruction, getOrCreateAssociatedTokenAccount} from "@solana/spl-token";

describe.skip("", async () => {
    const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet
    const USDC_MINT = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

    it("should mint some usdc", async () => {
        const provider = anchor.AnchorProvider.env()

        assert.ok(payer.publicKey.toBase58() == provider.wallet.publicKey.toBase58())

        let usdcTokenAccount = await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer.payer,
            USDC_MINT,
            payer.publicKey
        )

        const mintTokenTx = new anchor.web3.Transaction()
        mintTokenTx.add(createMintToInstruction(
            USDC_MINT,
            usdcTokenAccount.address,
            payer.publicKey,
            1000 * 10 ** 6 //1000 usdc tokens
        ))

        await provider.sendAndConfirm(mintTokenTx)
        const newBalance = await provider.connection.getTokenAccountBalance(usdcTokenAccount.address)
        console.log({newBalance})
        assert.equal(Number(newBalance.value.uiAmount), 1000)
    })
})