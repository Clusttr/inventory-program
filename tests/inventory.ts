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

  it("should create inventory", async () => {
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
    console.log({inventoryInfo})
    // assert(inventoryInfo.assets.includes(nft), "Failed to insert asset")
  });
});
