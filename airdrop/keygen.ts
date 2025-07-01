// import { Keypair } from "@solana/web3.js"

// let kp = Keypair.generate()

// console.log("you've Generated a new Soalana Wallet: ", kp.publicKey.toBase58())

// console.log(`${kp.secretKey}`)

import bs58 from 'bs58';
import fs from 'fs';

const base58Key = "" // add any base58 key to convert into Unit8Array 

// Decode base58 string into a Uint8Array
const secretKey = bs58.decode(base58Key);

// Save it as a JSON array to Turbin3-wallet.json
fs.writeFileSync('Turbin3-wallet.json', JSON.stringify(Array.from(secretKey)));

console.log('✅ Wallet saved as Turbin3-wallet.json');