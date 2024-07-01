import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Inventory} from "../target/types/inventory";
import {PublicKey} from "@solana/web3.js";
import {assert} from "chai";
import {getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID} from "@solana/spl-token"

describe("inventory", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet

    const program = anchor.workspace.Inventory as Program<Inventory>;
    const USDC_MINT = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    const nft = new PublicKey("Fnd3WMEGywcTjp3hdBnAepfJjcMJ2N1RwPpGqoV8Qsmp");
    const provider = anchor.AnchorProvider.env()

    const [inventory_info_address] = PublicKey.findProgramAddressSync(
        [Buffer.from("inventory")],
        program.programId
    );
    const [asset_info] = PublicKey.findProgramAddressSync(
        [Buffer.from("asset_info"), nft.toBuffer()],
        program.programId
    );
    const [asset_vault] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), nft.toBuffer()],
        program.programId
    )
    const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
        'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
    );
    const usdc_ata = PublicKey.findProgramAddressSync(
        [payer.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), USDC_MINT.toBuffer()],
        SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
    )[0]

    // const [mintVault] = PublicKey.findProgramAddressSync(
    //     [Buffer.from("vault"), nft.toBuffer(), payer.publicKey.toBuffer()],
    //     program.programId
    // )

    async function get_mint_ata(keypair: anchor.web3.Keypair, mint: anchor.web3.PublicKey) {
        const provider = anchor.AnchorProvider.env()
        return await getOrCreateAssociatedTokenAccount(
            provider.connection,
            keypair,
            mint,
            payer.publicKey
        )
    }

    const priceOracle = new anchor.web3.PublicKey("2QQpxGtYLFqKZp5SSejPBMPfWUGq1K3GKN8gEtJUgf6q")

    const assetPrice = PublicKey.findProgramAddressSync(
        [nft.toBuffer()],
        priceOracle
    )[0]


    it.skip("should initialize program", async () => {
        const tx = await program.methods.initialize()
            .accounts({
                signer: payer.publicKey,
                inventory: inventory_info_address
            })
            .rpc()
    })

    it.skip("should create inventory", async () => {
        let price = new anchor.BN(10 * 10 ** 2)
        const usdc_ata = (await get_mint_ata(payer.payer, USDC_MINT)).address
        const tx = await program.methods.createInventory()
            .accounts({
                signer: payer.publicKey,
                merchantUsdcAccount: usdc_ata,
                inventory: inventory_info_address,
                assetInfo: asset_info,
                assetVault: asset_vault,
                assetMint: nft,
                usdcMint: USDC_MINT,
            })
            .rpc()
        console.log({tx})
    })

    it.skip("should add asset", async () => {
        let userAssetAccount = (await get_mint_ata(payer.payer, nft)).address
        const tx_mint = await mintTo(provider.connection, payer.payer, nft, userAssetAccount, payer.publicKey, 100)
        console.log({tx_mint})
        let amount = new anchor.BN(10)
        const tx = await program.methods.addAsset(amount)
            .accounts({
                signer: payer.publicKey,
                userAssetAccount,
                assetVault: asset_vault,
                assetMint: nft
            })
            .rpc();
        const balance = await provider.connection.getTokenAccountBalance(asset_vault)
        assert(balance.value.uiAmount === 10, `Expected 10 but found ${balance.value.uiAmount}`)
    });

    /// start local validator cmd: solana-test-validator -r --account EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v clones/usdc.json  --account Fnd3WMEGywcTjp3hdBnAepfJjcMJ2N1RwPpGqoV8Qsmp clones/lotus.json
    it.skip("should withdraw asset", async () => {
        let amount = new anchor.BN(5)
        let userAssetAccount = (await get_mint_ata(payer.payer, nft)).address
        const _ = await program.methods.withdrawAsset(amount)
            .accounts({
                signer: payer.publicKey,
                userAssetAccount,
                assetVault: asset_vault,
                assetMint: nft
            })
            .rpc()
    });

    it.skip("should buy asset", async () => {
        const payerUsdcAccount = (await get_mint_ata(payer.payer, USDC_MINT)).address
        const payerMintAccount = (await get_mint_ata(payer.payer, nft)).address

        // const tx_mint = await mintTo(provider.connection, payer.payer, USDC_MINT, payerUsdcAccount, payer.publicKey, 100000 * 10 ** 6)
        // console.log({tx_mint})

        let amount = new anchor.BN(1)
        const buy_tx = await program.methods.buyAsset(amount)
            .accounts({
                signer: payer.publicKey,
                buyerUsdcAccount: payerUsdcAccount,
                buyerAssetAccount: payerMintAccount,
                merchantUsdcAccount: payerUsdcAccount,
                assetInfo: asset_info,
                assetPrice,
                assetVault: asset_vault,
                usdcMint: USDC_MINT,
                assetMint: nft,
                priceOracle
            })
            .rpc()
        console.log({buy_tx})
    })

    it("should close inventory", async () => {
        let closeAccTx = await program.methods.closeInventory()
            .accounts({
                signer: payer.publicKey,
                inventory: inventory_info_address,
                assetVault: asset_vault,
                assetInfo: asset_info,
                assetMint: nft
            })
            .rpc()
        console.log({closeAccTx})
    })
});
