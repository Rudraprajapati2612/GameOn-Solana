import "dotenv/config";
import {Connection} from "@solana/web3.js"

if(!process.env.SOLANA_RPC_URL){
    throw new Error("Solana RPC Missing")
}   
export const connection = new Connection(
    process.env.SOLANA_RPC_URL , "confirmed"
)
