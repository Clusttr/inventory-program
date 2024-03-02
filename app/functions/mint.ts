import {PublicKey} from "@metaplex-foundation/umi";
import {mintV1, TokenStandard} from "@metaplex-foundation/mpl-token-metadata";
import {umi} from "../utils";


async function mint(mint: PublicKey, tokenOwner: PublicKey, amount: Number) {
    return mintV1(umi, {
        mint,
        authority: umi.payer,
        amount: 10000,
        tokenOwner,
        tokenStandard: TokenStandard.FungibleAsset
    })
}

export { mint }