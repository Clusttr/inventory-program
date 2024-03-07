import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Inventory} from "../target/types/inventory";
import {PublicKey} from "@solana/web3.js";
import {assert} from "chai";
import {getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID} from "@solana/spl-token"

describe("inventory", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet

  const program = anchor.workspace.Inventory as Program<Inventory>;
  const USDC_MINT = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  const nft = new PublicKey("Fnd3WMEGywcTjp3hdBnAepfJjcMJ2N1RwPpGqoV8Qsmp");

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

    async function get_usdc_ata(keypair: anchor.web3.Keypair) {
        const provider = anchor.AnchorProvider.env()
        return await getOrCreateAssociatedTokenAccount(
            provider.connection,
            keypair,
            USDC_MINT,
            payer.publicKey
        )
    }


    it("should initialize program", async () => {
        const tx = await program.methods.initialize()
            .accounts({
                payer: payer.publicKey,
                inventory: inventory_info_address
            })
            .rpc()
    } )

  it("should create inventory", async () => {
      let price = new anchor.BN(200)
      const usdc_ata_ = (await get_usdc_ata(payer.payer)).address
      const tx = await  program.methods.createInventory(price)
          .accounts({
              payer: payer.publicKey,
              inventory: inventory_info_address,
              assetInfo: asset_info,
              usdcAccount: usdc_ata_,
              mint: nft,
              usdcMint: USDC_MINT,
          })
          .rpc()
      console.log({usdc_ata})
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

    it("should buy asset", async () => {
        const provider = anchor.AnchorProvider.env()
        const payerUsdcAccount = (await get_usdc_ata(payer.payer)).address

        let amount = new anchor.BN(1)
        const tx = await program.methods.buyAsset(amount)
            .accounts({
                payer: payer.publicKey,
                payerUsdcAccount,
                devUsdcAccount: payerUsdcAccount,
                inventory: inventory_info_address,
                assetInfo: asset_info,
                usdcMint: USDC_MINT,
                mint: nft
            })
            .rpc()
        console.log(tx)
    })

    it.only("should print accounts", async () => {
        console.log({asset_info})
        const assetInfo = await program.account.assetInfo.fetch(asset_info)
        const inventory = await program.account.inventory.fetch(inventory_info_address)
        console.log({
            assetInfo,
            inventory
        })
    })

  it("close inventory", async () => {
      const _ = await program.methods.closeInventory()
          .accounts({
              payer: payer.publicKey,
              inventory: inventory_info_address,
              assetInfo: asset_info,
              mint: nft
          }).rpc()
      const inventory = await program.account.inventory.fetch(inventory_info_address)
      assert(!inventory.assets.some(x => x.toString() === nft.toString()), "Failed to remove asset")
  })

});
