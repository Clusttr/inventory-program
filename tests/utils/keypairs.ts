import fs from "fs";

const keypair1234 = new Uint8Array(JSON.parse(fs.readFileSync("/Users/matthewchukwuemeka/.config/solana/id.json").toString()))


export { keypair1234 }