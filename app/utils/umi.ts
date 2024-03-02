import {createSignerFromKeypair, keypairIdentity} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {web3JsEddsa} from "@metaplex-foundation/umi-eddsa-web3js";
import {web3JsRpc} from "@metaplex-foundation/umi-rpc-web3js";
import {fetchHttp} from "@metaplex-foundation/umi-http-fetch";
import {loadKeypair} from "./wallet";
import {use} from "chai";

const keypair  = loadKeypair()
const endpoint = "http://127.0.0.1:8899"

const umi = createUmi(endpoint)
    .use(web3JsEddsa())
    .use(web3JsRpc(endpoint))
    .use(fetchHttp())

const mKeypair = umi.eddsa.createKeypairFromSecretKey(keypair.secretKey)
const keypairSigner = createSignerFromKeypair(umi, mKeypair)
umi.use(keypairIdentity(keypairSigner))

export { umi }
