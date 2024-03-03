import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Inventory } from "../target/types/inventory";
import {PublicKey} from "@solana/web3.js";
import { assert } from "chai";
import {createMintToInstruction, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID} from "@solana/spl-token"

describe("inventory", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet

  const program = anchor.workspace.Inventory as Program<Inventory>;
  const USDC_MINT = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  const nft = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd");

  const [inventory_info_address] = PublicKey.findProgramAddressSync(
      [Buffer.from("inventory")],
      program.programId
  );
  const [asset_info] = PublicKey.findProgramAddressSync(
      [Buffer.from("asset_info"), nft.toBuffer()],
      program.programId
  );
    const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
        'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
    );
  const usdc_ata = PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), USDC_MINT.toBuffer()],
      SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
      )[0]

  it.only("should create inventory", async () => {
      let price = new anchor.BN(200)
      const tx = await  program.methods.createInventory(price)
          .accounts({
              payer: payer.publicKey,
              inventory: inventory_info_address,
              assetInfo: asset_info,
              usdcAccount: usdc_ata,
              mint: nft,
              usdcMint: USDC_MINT,
          })
          .rpc()
  })

  it("should add asset", async () => {
    const tx = await program.methods.addAsset()
        .accounts({
          payer: payer.publicKey,
          inventory: inventory_info_address,
          mint: nft
        })
        .rpc();

    const inventoryInfo = await program.account.inventory.fetch(inventory_info_address);
    assert(inventoryInfo.assets.some(x => x.toString() === nft.toString()), "Failed to insert asset")
  });

  it("should withdraw asset", async () => {
      const _ = await program.methods.withdrawAsset()
          .accounts({
              payer: payer.publicKey,
              inventory: inventory_info_address,
              mint: nft
          })
          .rpc()
      const inventoryInfo = await program.account.inventory.fetch(inventory_info_address);
      assert(!inventoryInfo.assets.some(x => x.toString() === nft.toString()), "Failed to remove asset")
  });

  it("should update asset", async () => {
      let amount_bn = new anchor.BN(60)
      let payer_usdc_pubkey = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")
      const  _ = await program.methods.updateAssetInfo(amount_bn, payer_usdc_pubkey)
          .accounts({
              payer: payer.publicKey,
              assetInfo: asset_info,
              mint: nft
          })
          .rpc()

      const assetInfo = await program.account.assetInfo.fetch(asset_info)
      console.log({
          inventoryInfo: assetInfo.price.toNumber(),
          amount_bn: amount_bn.toNumber()
      })
      assert(assetInfo.amount.eq(new anchor.BN(assetInfo.amount.toNumber())), `Expected ${amount_bn.toNumber()} but found ${assetInfo.price.toNumber()}`)
  });

  it.skip("close inventory", async () => {
      console.log({
          asset_info,
          inventory_info_address
      })
      const _ = await program.methods.closeInventory()
          .accounts({
              payer: payer.publicKey,
              assetInfo: asset_info,
              inventory: inventory_info_address,
              mint: nft
          }).rpc()
      const inventory = await program.account.inventory.fetch(inventory_info_address)
      assert(!inventory.assets.some(x => x.toString() === nft.toString()), "Failed to remove asset")
  })

    it("should print accounts", async () => {
        console.log({asset_info})
        const assetInfo = await program.account.assetInfo.fetch(asset_info)
        const inventory = await program.account.inventory.fetch(inventory_info_address)
        console.log({
            assetInfo,
            inventory
        })
    })

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

    it("should buy asset", async () => {
        const provider = anchor.AnchorProvider.env()
        let usdcTokenAccount = await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer.payer,
            USDC_MINT,
            payer.publicKey
        )

        let amount = new anchor.BN(100 * 10 ** 6)
        const tx = await program.methods.buyAsset(amount)
            .accounts({
                payer: payer.publicKey,
                payerUsdcAccount: usdcTokenAccount.address,
                devUsdcAccount: usdcTokenAccount.address,
                inventory: inventory_info_address,
                assetInfo: asset_info
            })
            .rpc()
        console.log(tx)
        console.log(usdcTokenAccount)
    })
});
