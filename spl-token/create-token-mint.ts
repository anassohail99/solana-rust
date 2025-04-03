import { createMint } from "@solana/spl-token";
import "dotenv/config";
import {
  getKeypairFromEnvironment,
  getExplorerLink,
} from "@solana-developers/helpers";
import { Connection, clusterApiUrl } from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"));

const user = getKeypairFromEnvironment("SECRET_KEY");
console.log(`Public key: ${user.publicKey.toBase58()}`);
createToken();

async function createToken() {
  const tokenMint = await createMint(connection, user, user.publicKey, null, 2);
  const link = getExplorerLink("address", tokenMint.toString(), "devnet");
  console.log(`Finished! Created token mint: ${link}`);
}
