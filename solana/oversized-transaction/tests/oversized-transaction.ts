import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ComputeBudgetProgram, PublicKey } from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import * as crypto from "crypto";
import * as borsh from "borsh";

// Constants to match our program's limits
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

  it.skip("Stores and retrieves Borsh-serialized data up to 32kb (solana single tx heap limit)", async () => {
    const greeting =
      "Hello, Solana! This is a large test of oversized data handling with multiple chunks across multiple accounts. We'll use many different PDAs to store different parts of the data. This demonstrates our ability to handle data much larger than a single Solana account can store.";

    const numbers = new Uint8Array(
      Array(24000)
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

    for (let i = 0; i < chunks.length; i += 1) {
      let tx: string;

      if (i === 0) {
        tx = await program.methods
          .initStorage(
            dataId,
            0,
            chunks.length,
            dataHash,
            Buffer.from(chunks[0])
          )
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();
      } else {
        tx = await program.methods
          .storeChunk(
            dataId,
            i,
            chunks.length,
            dataHash, // Same hash for integrity verification
            Buffer.from(chunks[i])
          )
          .accounts({
            payer: provider.wallet.publicKey,
            // unified_storage is derived by Anchor
          })
          .rpc();
      }

      console.log(
        `Stored chunk ${i + 1}/${chunks.length}, tx: ${tx.substring(0, 10)}...`
      );
    }

    const [storagePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("unified_storage"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(dataId),
      ],
      program.programId
    );

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
      chunks.length,
      `Failed to store all chunks`
    );

    console.log(`Using ${chunks.length} storage accounts`);

    // Retrieve and reassemble the data
    let reassembledData = Buffer.alloc(0);

    for (let i = 0; i < chunks.length; i++) {
      try {
        const chunkData = await program.methods
          .retrieveChunk(i)
          .accounts({
            unifiedStorage: storagePda,
            payer: provider.wallet.publicKey,
          })
          .view();

        console.log(
          `Retrieved chunk ${i + 1}/${chunks.length} from account #${
            i + 1
          }, size: ${chunkData.length} bytes`
        );

        reassembledData = Buffer.concat([
          reassembledData,
          Buffer.from(chunkData),
        ]);
      } catch (error) {
        console.error(
          `Error retrieving chunk ${i} from account #${i + 1}:`,
          error
        );
        throw error;
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

    console.log(`Total chunks: ${chunks.length}`);
    console.log(`Original data size: ${serializedData.length} bytes`);
    console.log(`Reassembled data size: ${reassembledData.length} bytes`);

    // Close the storage account to reclaim rent
    try {
      const tx = await program.methods
        .closeStorage()
        .accounts({
          unifiedStorage: storagePda,
          payer: provider.wallet.publicKey,
        })
        .rpc();

      console.log(`Closed storage account, tx: ${tx.substring(0, 10)}...`);
    } catch (error) {
      console.error(`Error closing storage account:`, error);
    }
  });

  it.skip("Tests storage with small data in single account", async () => {
    const data = new TestData({
      greeting: "Small data test",
      numbers: new Uint8Array(
        Array(50)
          .fill(0)
          .map((_, i) => i % 256)
      ),
    });

    const dataSerialized = borsh.serialize(TestData.schema, data) as Buffer;

    const dataId = Array.from(
      anchor.web3.Keypair.generate().secretKey.slice(0, 32)
    );

    const dataHash = Array.from(
      crypto.createHash("sha256").update(dataSerialized).digest()
    );

    console.log(
      `Storing small data (${dataSerialized.length} bytes) in a single account`
    );

    // Split small data into chunks (should fit within MAX_CHUNKS_PER_ACCOUNT)
    const chunks: Buffer[] = [];
    for (let i = 0; i < dataSerialized.length; i += MAX_CHUNK_SIZE) {
      chunks.push(
        dataSerialized.slice(
          i,
          Math.min(i + MAX_CHUNK_SIZE, dataSerialized.length)
        )
      );
    }

    for (let i = 0; i < chunks.length; i++) {
      let tx: string;

      if (i === 0) {
        tx = await program.methods
          .initStorage(
            dataId,
            0,
            chunks.length,
            dataHash,
            Buffer.from(chunks[0])
          )
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();
      } else {
        tx = await program.methods
          .storeChunk(
            dataId,
            i,
            chunks.length,
            dataHash,
            Buffer.from(chunks[i])
          )
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();
      }

      console.log(
        `Stored small chunk ${i + 1}/${chunks.length}, size: ${
          chunks[i].length
        } bytes, tx: ${tx.substring(0, 10)}...`
      );
    }

    const [storePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("unified_storage"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(dataId),
      ],
      program.programId
    );

    // Retrieve and verify small data
    let reassembledData = Buffer.alloc(0);

    for (let i = 0; i < chunks.length; i++) {
      const chunkData = await program.methods
        .retrieveChunk(i)
        .accounts({
          unifiedStorage: storePda,
          payer: provider.wallet.publicKey,
        })
        .view();

      reassembledData = Buffer.concat([
        reassembledData,
        Buffer.from(chunkData),
      ]);
    }

    const dataIntegrityMatch =
      JSON.stringify(
        Array.from(crypto.createHash("sha256").update(reassembledData).digest())
      ) === JSON.stringify(dataHash);

    console.log("Small data integrity verification:", dataIntegrityMatch);
    assert.isTrue(dataIntegrityMatch, "Small data integrity failed");

    await program.methods
      .closeStorage()
      .accounts({
        unifiedStorage: storePda,
        payer: provider.wallet.publicKey,
      })
      .rpc();

    console.log(
      "Test completed - successfully stored and retrieved data using multiple accounts"
    );
  });

  // Ethereum Signature Verification Tests
  function getCompressedPublicKey(): string {
    return "0x0304ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";
  }

  it.only("should validate Ethereum signature correctly", async () => {
    const compressedPublicKey = getCompressedPublicKey();

    const ethData = {
      message: `"{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}"`,
      signature:
        "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d111b",
    };

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 1_400_000,
    });

    const isValid = await program.methods
      .verifyEthereumSignature(ethData, compressedPublicKey)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .preInstructions([computeBudgetIx])
      .rpc();

    console.log("isValid", isValid);

    assert.isTrue(isValid);
  });

  it("should fail to validate Ethereum signature with wrong public key", async () => {
    const wrongCompressedPublicKey =
      "0x0314ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";

    const ethData = {
      message:
        '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}',
      signature:
        "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d111b",
    };

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 200_000,
    });

    try {
      const isValid = await program.methods
        .verifyEthereumSignature(ethData, wrongCompressedPublicKey)
        .accounts({
          payer: provider.wallet.publicKey,
        })
        .preInstructions([computeBudgetIx])
        .rpc();

      console.log(
        "Ethereum signature verification with wrong key result:",
        isValid
      );
      assert.isFalse(
        isValid,
        "Ethereum signature should fail with incorrect public key"
      );
    } catch (error) {
      console.error(
        "Error in Ethereum signature verification with wrong key:",
        error
      );
      throw error;
    }
  });

  it("should fail to validate Ethereum signature with tampered message", async () => {
    const compressedPublicKey = getCompressedPublicKey();

    // Original signature has 1b at the end, changed to 2b to simulate tampering
    const tamperedSignature =
      "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d121b";

    const ethData = {
      message:
        '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}',
      signature: tamperedSignature,
    };

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 200_000,
    });

    try {
      const isValid = await program.methods
        .verifyEthereumSignature(ethData, compressedPublicKey)
        .accounts({
          payer: provider.wallet.publicKey,
        })
        .preInstructions([computeBudgetIx])
        .view();

      console.log(
        "Ethereum signature verification with tampered signature result:",
        isValid
      );
      assert.isFalse(
        isValid,
        "Ethereum signature should fail with tampered signature"
      );
    } catch (error) {
      console.error(
        "Error in Ethereum signature verification with tampered signature:",
        error
      );
      throw error;
    }
  });

  /* Commented out since the OIDC signature verification method is not implemented
  it.only("should validate oidc signature correctly", async () => {
    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 2_000_000,
    });

    const isValid = await program.methods
      .verifyOidcSignature()
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .preInstructions([computeBudgetIx])
      .rpc();

    console.log("isValid", isValid);

    assert.isTrue(isValid, "Valid signature should verify correctly");
  });
  */
});
