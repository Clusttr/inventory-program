import {PublicKey} from "@metaplex-foundation/umi";
import {mint} from "./functions/mint";
import {umi} from "./utils";
import {mint_usdc} from "./functions";


const usdcMint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" as PublicKey
const adminUSDC_ATA = umi.payer.publicKey

async function main() {
    // const tx = await mint(usdcMint, adminUSDC_ATA, 10_000)
    // console.log({tx})
    await mint_usdc()
}

main().then(() => {
    console.log("__success__")
}).catch(err => {
    console.log({err})
})