import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import * as crypto from "crypto";
import * as borsh from "borsh";

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

  it("Stores and retrieves Borsh-serialized data in unified storage", async () => {
    const greeting =
      "Hello, Solana! This is a test of oversized data handling with multiple chunks. The data will be split across multiple chunks in a single unified storage account.";
    const numbers = new Uint8Array(
      Array(500)
        .fill(0)
        .map((_, i) => i % 256)
    );

    const originalData = new TestData({ greeting, numbers });

    const serializedData = borsh.serialize(
      TestData.schema,
      originalData
    ) as Buffer;

    console.log("Total serialized data size:", serializedData.length, "bytes");

    const dataId = Array.from(
      anchor.web3.Keypair.generate().secretKey.slice(0, 32)
    );

    const dataHash = Array.from(
      crypto.createHash("sha256").update(serializedData).digest()
    );

    const chunkSize = 200;
    const chunks: Buffer[] = [];

    for (let i = 0; i < serializedData.length; i += chunkSize) {
      chunks.push(
        serializedData.slice(i, Math.min(i + chunkSize, serializedData.length))
      );
    }

    console.log(`Split data into ${chunks.length} chunks`);

    const [storagePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("unified_storage"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(dataId),
      ],
      program.programId
    );

    const chunkTxs: string[] = [];

    for (let i = 0; i < chunks.length; i++) {
      const chunkIndex = i;
      const totalChunks = chunks.length;
      const chunkData = chunks[i];

      try {
        const tx = await program.methods
          .storeChunk(
            dataId,
            chunkIndex,
            totalChunks,
            dataHash,
            Buffer.from(chunkData)
          )
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();

        chunkTxs.push(tx);
        console.log(
          `Stored chunk ${chunkIndex + 1}/${totalChunks}, size: ${
            chunkData.length
          } bytes, tx: ${tx.substring(0, 10)}...`
        );
      } catch (error) {
        console.error(`Error storing chunk ${chunkIndex}:`, error);
        throw error;
      }
    }

    const metadata = await program.methods
      .getDataMetadata()
      .accounts({
        unifiedStorage: storagePda,
        payer: provider.wallet.publicKey,
      })
      .view();

    console.log("Dataset metadata:", {
      totalChunks: metadata.totalChunks,
      chunksStored: metadata.chunksStored,
    });

    assert.equal(
      metadata.chunksStored,
      chunks.length,
      "Not all chunks were stored"
    );

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
          `Retrieved chunk ${i + 1}/${chunks.length}, size: ${
            chunkData.length
          } bytes`
        );

        reassembledData = Buffer.concat([
          reassembledData,
          Buffer.from(chunkData),
        ]);
      } catch (error) {
        console.error(`Error retrieving chunk ${i}:`, error);
        throw error;
      }
    }

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

    try {
      const tx = await program.methods
        .closeStorage()
        .accounts({
          unifiedStorage: storagePda,
          payer: provider.wallet.publicKey,
        })
        .rpc();

      console.log(
        `Closed unified storage account, tx: ${tx.substring(0, 10)}...`
      );
    } catch (error) {
      console.error("Error closing storage account:", error);
      throw error;
    }

    console.log(`Total chunks: ${chunks.length}`);
    console.log(`Original data size: ${serializedData.length} bytes`);
    console.log(`Reassembled data size: ${reassembledData.length} bytes`);
  });
});
