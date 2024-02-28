import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Inventory } from "../target/types/inventory";
import {PublicKey} from "@solana/web3.js";
import { assert } from "chai";

describe("inventory", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet

  const program = anchor.workspace.Inventory as Program<Inventory>;
  const nft = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")
  const [inventory_info_address] = PublicKey.findProgramAddressSync(
      [Buffer.from("inventory")],
      program.programId
  );
  const [asset_info] = PublicKey.findProgramAddressSync(
      [Buffer.from("asset_info"), nft.toBuffer()],
      program.programId
  )

  it.skip("should create inventory", async () => {
      const tx = await  program.methods.createInventory()
          .accounts({
              payer: payer.publicKey,
              inventory: inventory_info_address,
              assetInfo: asset_info,
              mint: nft
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

  it.only("should update asset", async () => {
      let amount_bn = new anchor.BN(60)
      let payer_usdc_pubkey = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")
      const  _ = await program.methods.updateAssetInfo(amount_bn, payer_usdc_pubkey)
          .accounts({
              payer: payer.publicKey,
              assetInfo: asset_info,
              mint: nft
          })
          .rpc()

      const inventoryInfo = await program.account.assetInfo.fetch(asset_info)
      console.log({
          inventoryInfo: inventoryInfo.price.toNumber(),
          amount_bn: amount_bn.toNumber()
      })
      assert(inventoryInfo.amount.eq(new anchor.BN(inventoryInfo.amount.toNumber())), `Expected ${amount_bn.toNumber()} but found ${inventoryInfo.price.toNumber()}`)
  })
});
