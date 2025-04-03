import "dotenv/config"; // Loads environment variables from a .env file
import {
  getKeypairFromEnvironment, // Helper to get a keypair from env variables
  getExplorerLink, // Helper to generate Solana Explorer links
} from "@solana-developers/helpers";
import {
  Connection, // Class to connect to Solana network
  clusterApiUrl, // Function to get cluster endpoint (devnet, mainnet, etc.)
  PublicKey, // Class to handle Solana public keys
  Transaction, // Class to create transactions
  sendAndConfirmTransaction,
  SendTransactionError, // Function to send and confirm transactions
} from "@solana/web3.js";
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata"; // Metaplex function to create metadata instruction

// Get the user's keypair from the SECRET_KEY in .env file
const user = getKeypairFromEnvironment("SECRET_KEY");

// Create a connection to Solana devnet
const connection = new Connection(clusterApiUrl("devnet"));

// Log the user's public key for reference
console.log(`Public Key: ${user.publicKey.toBase58()}`);

// Define the Token Metadata Program ID
// This is the fixed address of the Metaplex Token Metadata Program on Solana
const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// Define the token mint account public key
// This is the address of the token you want to create metadata for
const tokenMintAccount = new PublicKey(
  "NWNG73Tg2Nisa94DPCbHJ4taAYgMybyj3dZBw52Xa7b"
);

// Call the function to create metadata
createMetaData();

// Async function to create token metadata
async function createMetaData() {
  // Define the metadata structure for the token
  const metadataData = {
    name: "Solana Pakistan", // Name of the token
    symbol: "SP", // Symbol of the token
    uri: "https://purple-adjacent-mule-439.mypinata.cloud/ipfs/bafkreidujlq3iafvt2xm2swjwwmnffrugn6slbqo42ouz72bl43xewl6o4", // URI to off-chain metadata (e.g., JSON on IPFS)
    sellerFeeBasisPoints: 0, // Royalty fee (0 means no royalties)
    creators: null, // List of creators (null means no creators specified)
    collection: null, // Collection details (null means no collection)
    uses: null, // Usage details (null means no specific usage)
  };

  // Calculate the Program Derived Address (PDA) for the metadata account
  // PDA is a unique address derived from seeds and program ID
  const metadataPDAAndBump = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"), // Seed 1: "metadata" string
      TOKEN_METADATA_PROGRAM_ID.toBuffer(), // Seed 2: Token Metadata Program ID
      tokenMintAccount.toBuffer(), // Seed 3: Token mint address
    ],
    TOKEN_METADATA_PROGRAM_ID // Program ID used to derive the PDA
  );
  const metadataPDA = metadataPDAAndBump[0]; // Extract the PDA from the result

  // Log the metadata PDA for debugging
  console.log(metadataPDA.toBase58());

  // Create a new transaction object
  const transaction = new Transaction();

  // Create the instruction to create a metadata account
  const createMetadataAccountInstruction =
    createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataPDA, // Address where metadata will be stored
        mint: tokenMintAccount, // Token mint address
        mintAuthority: user.publicKey, // Authority that controls the mint
        payer: user.publicKey, // Account paying for the transaction
        updateAuthority: user.publicKey, // Authority that can update metadata
      },
      {
        createMetadataAccountArgsV3: {
          collectionDetails: null, // No collection details
          data: metadataData, // The metadata to store
          isMutable: true, // Whether the metadata can be updated later
        },
      }
    );

  // Add the instruction to the transaction
  transaction.add(createMetadataAccountInstruction);

  try {
    // Send the transaction to the network and wait for confirmation
    const transactionSignature = await sendAndConfirmTransaction(
      connection, // Network connection
      transaction, // Transaction to send
      [user] // Signers (user's keypair)
    );

    // Generate a link to view the transaction on Solana Explorer
    const transactionLink = getExplorerLink(
      "transaction", // Type of link
      transactionSignature, // Transaction ID
      "devnet" // Network
    );

    // Log the confirmed transaction link
    console.log(`Transaction confirmed:`, transactionLink);

    // Generate a link to view the token mint on Solana Explorer
    const tokenMintLink = getExplorerLink(
      "address", // Type of link
      tokenMintAccount.toString(), // Token mint address
      "devnet" // Network
    );

    // Log the token mint link
    console.log(`Token Mint Link:`, tokenMintLink);
  } catch (error) {
    // Handle errors during transaction sending
    if (error instanceof SendTransactionError) {
      // If it's a transaction error, log detailed logs
      console.log(await error.getLogs(connection));
    }
    throw error; // Re-throw the error for further handling if needed
  }
}
