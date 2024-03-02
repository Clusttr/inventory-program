import {generateSigner} from "@metaplex-foundation/umi";
import {umi} from "../utils";
import {createFungible, createFungibleAsset} from "@metaplex-foundation/mpl-token-metadata";


const create = generateSigner(umi)

async function createToken() {
    console.log("##Creating Token##")
    //const tx = await createFungibleAsset(umi, {create, ""})
}