import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import * as crypto from "crypto";
import * as borsh from "borsh";

// Constants to match our program's limits
const MAX_CHUNKS_PER_ACCOUNT = 5;
const MAX_CHUNK_SIZE = 900;

class TestData {
  greeting: string;
  numbers: Uint8Array;

  constructor(props: { greeting: string; numbers: Uint8Array }) {
    this.greeting = props.greeting;
    this.numbers = props.numbers;
  }

  static schema = {
    struct: {
      greeting: "string",
      numbers: { array: { type: "u8" } },
    },
  };
}

describe("oversized-transaction", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Stores and retrieves Borsh-serialized data in multiple storage accounts", async () => {
    // Use a large dataset (larger than what fits in a single account)
    const greeting =
      "Hello, Solana! This is a large test of oversized data handling with multiple chunks across multiple accounts. We'll use many different PDAs to store different parts of the data. This demonstrates our ability to handle data much larger than a single Solana account can store.";

    // Create a 200KB dataset by increasing the array size
    const numbers = new Uint8Array(
      Array(200000) // Increase to ~200K elements for a dataset >100KB
        .fill(0)
        .map((_, i) => i % 256)
    );

    const originalData = new TestData({ greeting, numbers });
    const serializedData = borsh.serialize(
      TestData.schema,
      originalData
    ) as Buffer;

    console.log("Total serialized data size:", serializedData.length, "bytes");

    // Generate a unique ID for this dataset
    const dataId = Array.from(
      anchor.web3.Keypair.generate().secretKey.slice(0, 32)
    );
    const dataHash = Array.from(
      crypto.createHash("sha256").update(serializedData).digest()
    );

    // Split data into appropriately sized chunks
    const chunks: Buffer[] = [];
    for (let i = 0; i < serializedData.length; i += MAX_CHUNK_SIZE) {
      chunks.push(
        serializedData.slice(
          i,
          Math.min(i + MAX_CHUNK_SIZE, serializedData.length)
        )
      );
    }
    console.log(
      `Split data into ${chunks.length} chunks of max ${MAX_CHUNK_SIZE} bytes each`
    );

    // Group chunks by storage account (MAX_CHUNKS_PER_ACCOUNT chunks per account)
    const accountGroups: Buffer[][] = [];
    for (let i = 0; i < chunks.length; i += MAX_CHUNKS_PER_ACCOUNT) {
      accountGroups.push(chunks.slice(i, i + MAX_CHUNKS_PER_ACCOUNT));
    }
    console.log(`Using ${accountGroups.length} storage accounts`);

    // Store each group of chunks in a separate storage account
    for (let groupIndex = 0; groupIndex < accountGroups.length; groupIndex++) {
      const groupChunks = accountGroups[groupIndex];

      // Create a unique data ID for each group, derived from the main data ID
      const groupDataId = Array.from(
        crypto
          .createHash("sha256")
          .update(Buffer.from(dataId))
          .update(Buffer.from([groupIndex]))
          .digest()
          .slice(0, 32)
      );

      const [storagePda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("unified_storage"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(groupDataId),
        ],
        program.programId
      );

      console.log(
        `Storing ${groupChunks.length} chunks in account #${groupIndex + 1}`
      );

      for (let i = 0; i < groupChunks.length; i++) {
        try {
          const tx = await program.methods
            .storeChunk(
              groupDataId,
              i,
              groupChunks.length,
              dataHash, // Same hash for integrity verification
              Buffer.from(groupChunks[i])
            )
            .accounts({
              payer: provider.wallet.publicKey,
              // unified_storage is derived by Anchor
            })
            .rpc();

          console.log(
            `Stored chunk ${i + 1}/${groupChunks.length} in account #${
              groupIndex + 1
            }, size: ${groupChunks[i].length} bytes, tx: ${tx.substring(
              0,
              10
            )}...`
          );
        } catch (error) {
          console.error(
            `Error storing chunk ${i} in account #${groupIndex + 1}:`,
            error
          );
          throw error;
        }
      }

      // Verify metadata for this group
      const metadata = await program.methods
        .getDataMetadata()
        .accounts({
          unifiedStorage: storagePda,
          payer: provider.wallet.publicKey,
        })
        .view();

      assert.equal(
        metadata.chunksStored,
        groupChunks.length,
        `Not all chunks were stored in account #${groupIndex + 1}`
      );
    }

    // Retrieve and reassemble the data
    let reassembledData = Buffer.alloc(0);

    for (let groupIndex = 0; groupIndex < accountGroups.length; groupIndex++) {
      const groupChunks = accountGroups[groupIndex];

      // Get the storage PDA for this group
      const groupDataId = Array.from(
        crypto
          .createHash("sha256")
          .update(Buffer.from(dataId))
          .update(Buffer.from([groupIndex]))
          .digest()
          .slice(0, 32)
      );

      const [storagePda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("unified_storage"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(groupDataId),
        ],
        program.programId
      );

      // Retrieve all chunks from this account
      for (let i = 0; i < groupChunks.length; i++) {
        try {
          const chunkData = await program.methods
            .retrieveChunk(i)
            .accounts({
              unifiedStorage: storagePda,
              payer: provider.wallet.publicKey,
            })
            .view();

          console.log(
            `Retrieved chunk ${i + 1}/${groupChunks.length} from account #${
              groupIndex + 1
            }, size: ${chunkData.length} bytes`
          );

          reassembledData = Buffer.concat([
            reassembledData,
            Buffer.from(chunkData),
          ]);
        } catch (error) {
          console.error(
            `Error retrieving chunk ${i} from account #${groupIndex + 1}:`,
            error
          );
          throw error;
        }
      }

      // Close the storage account to reclaim rent
      try {
        const tx = await program.methods
          .closeStorage()
          .accounts({
            unifiedStorage: storagePda,
            payer: provider.wallet.publicKey,
          })
          .rpc();

        console.log(
          `Closed storage account #${groupIndex + 1}, tx: ${tx.substring(
            0,
            10
          )}...`
        );
      } catch (error) {
        console.error(
          `Error closing storage account #${groupIndex + 1}:`,
          error
        );
      }
    }

    // Verify integrity of reassembled data
    const reassembledHash = Array.from(
      crypto.createHash("sha256").update(reassembledData).digest()
    );

    const hashesMatch =
      JSON.stringify(reassembledHash) === JSON.stringify(dataHash);

    console.log(
      "Full data integrity verification (hash comparison):",
      hashesMatch
    );
    assert.isTrue(hashesMatch, "Data hash verification failed");

    // Deserialize and verify data
    const deserializedData = borsh.deserialize(
      TestData.schema,
      reassembledData
    ) as TestData;

    console.log("Deserialized data:", {
      greeting: deserializedData.greeting,
      numbersPreview:
        Array.from(deserializedData.numbers.slice(0, 10)).join(",") + "...",
    });

    assert.equal(deserializedData.greeting, greeting);
    assert.equal(deserializedData.numbers.length, numbers.length);
    assert.deepEqual(Array.from(deserializedData.numbers), Array.from(numbers));

    console.log(
      `Total chunks: ${chunks.length} across ${accountGroups.length} accounts`
    );
    console.log(`Original data size: ${serializedData.length} bytes`);
    console.log(`Reassembled data size: ${reassembledData.length} bytes`);
  });

  it("Tests storage with small data in single account", async () => {
    // Create a small test dataset that fits entirely in one account
    const smallData = new TestData({
      greeting: "Small data test",
      numbers: new Uint8Array(
        Array(50)
          .fill(0)
          .map((_, i) => i % 256)
      ),
    });

    const smallDataSerialized = borsh.serialize(
      TestData.schema,
      smallData
    ) as Buffer;

    const smallDataId = Array.from(
      anchor.web3.Keypair.generate().secretKey.slice(0, 32)
    );

    const smallDataHash = Array.from(
      crypto.createHash("sha256").update(smallDataSerialized).digest()
    );

    console.log(
      `Storing small data (${smallDataSerialized.length} bytes) in a single account`
    );

    // Split small data into chunks (should fit within MAX_CHUNKS_PER_ACCOUNT)
    const smallChunks: Buffer[] = [];
    for (let i = 0; i < smallDataSerialized.length; i += MAX_CHUNK_SIZE) {
      smallChunks.push(
        smallDataSerialized.slice(
          i,
          Math.min(i + MAX_CHUNK_SIZE, smallDataSerialized.length)
        )
      );
    }

    const [smallStoragePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("unified_storage"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(smallDataId),
      ],
      program.programId
    );

    // Store small data chunks
    for (let i = 0; i < smallChunks.length; i++) {
      const tx = await program.methods
        .storeChunk(
          smallDataId,
          i,
          smallChunks.length,
          smallDataHash,
          Buffer.from(smallChunks[i])
        )
        .accounts({
          payer: provider.wallet.publicKey,
        })
        .rpc();

      console.log(
        `Stored small chunk ${i + 1}/${smallChunks.length}, size: ${
          smallChunks[i].length
        } bytes, tx: ${tx.substring(0, 10)}...`
      );
    }

    // Retrieve and verify small data
    let reassembledSmallData = Buffer.alloc(0);

    for (let i = 0; i < smallChunks.length; i++) {
      const chunkData = await program.methods
        .retrieveChunk(i)
        .accounts({
          unifiedStorage: smallStoragePda,
          payer: provider.wallet.publicKey,
        })
        .view();

      reassembledSmallData = Buffer.concat([
        reassembledSmallData,
        Buffer.from(chunkData),
      ]);
    }

    const smallDataIntegrityMatch =
      JSON.stringify(
        Array.from(
          crypto.createHash("sha256").update(reassembledSmallData).digest()
        )
      ) === JSON.stringify(smallDataHash);

    console.log("Small data integrity verification:", smallDataIntegrityMatch);
    assert.isTrue(smallDataIntegrityMatch, "Small data integrity failed");

    // Clean up
    await program.methods
      .closeStorage()
      .accounts({
        unifiedStorage: smallStoragePda,
        payer: provider.wallet.publicKey,
      })
      .rpc();

    console.log(
      "Test completed - successfully stored and retrieved data using multiple accounts"
    );
  });
});
