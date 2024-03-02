import {getMint} from "@solana/spl-token";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js"
import * as cluster from "cluster";

const connection = new Connection("http://127.0.0.1:8899")//clusterApiUrl('testnet'), 'confirmed')

async function mint_usdc() {
    const address = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    const mintInfo = await getMint(connection, address)
    console.log({mintInfo})
}

export { mint_usdc }