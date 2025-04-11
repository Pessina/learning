import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ComputeBudgetProgram, PublicKey } from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import * as crypto from "crypto";
import * as borsh from "borsh";

describe("Ethereum Auth", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  const compressedPublicKey =
    "0x0304ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";

  it("should validate Ethereum signature correctly", async () => {
    const ethData = {
      message:
        '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}',
      signature:
        "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d111b",
    };

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 1_400_000,
    });

    const txSignature = await program.methods
      .verifyEthereumSignature(ethData, compressedPublicKey)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .preInstructions([computeBudgetIx])
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
      });

    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
      {
        signature: txSignature,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
      "confirmed"
    );

    const txInfo = (await provider.connection.getTransaction(txSignature, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    })) as unknown as {
      meta: {
        returnData: {
          data: string[];
        };
        logMessages: string[];
      };
    };

    console.log("txInfo", JSON.stringify(txInfo, null, 2));
    console.log(
      "txInfo.meta.returnData.data[0]",
      txInfo.meta.returnData.data[0]
    );

    let returnValue = txInfo?.meta?.returnData?.data
      ? (Buffer.from(txInfo.meta.returnData.data[0], "base64")[0] as unknown as
          | 0
          | 1)
      : null;

    assert.isTrue(returnValue === 1, "Should have a return value");
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
      units: 1_400_000,
    });

    const txSignature = await program.methods
      .verifyEthereumSignature(ethData, wrongCompressedPublicKey)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .preInstructions([computeBudgetIx])
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
      });

    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
      {
        signature: txSignature,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
      "confirmed"
    );

    const txInfo = (await provider.connection.getTransaction(txSignature, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    })) as unknown as {
      meta: {
        returnData: {
          data: string[];
        };
        logMessages: string[];
      };
    };

    let returnValue = txInfo?.meta?.returnData?.data
      ? (Buffer.from(txInfo.meta.returnData.data[0], "base64")[0] as unknown as
          | 0
          | 1)
      : null;

    assert.isTrue(returnValue !== 1, "Should have a return value");
  });

  it("should fail to validate Ethereum signature with tampered message", async () => {
    const ethData = {
      message:
        '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}',
      signature:
        "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d121b",
    };

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 1_400_000,
    });

    const txSignature = await program.methods
      .verifyEthereumSignature(ethData, compressedPublicKey)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .preInstructions([computeBudgetIx])
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
      });

    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
      {
        signature: txSignature,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
      "confirmed"
    );

    const txInfo = (await provider.connection.getTransaction(txSignature, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    })) as unknown as {
      meta: {
        returnData: {
          data: string[];
        };
        logMessages: string[];
      };
    };

    let returnValue = txInfo?.meta?.returnData?.data
      ? (Buffer.from(txInfo.meta.returnData.data[0], "base64")[0] as unknown as
          | 0
          | 1)
      : null;

    assert.isTrue(returnValue !== 1, "Should have a return value");
  });
});
