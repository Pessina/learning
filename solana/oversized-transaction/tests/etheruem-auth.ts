import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import { confirmTransaction, getTransactionReturnValue } from "../utils/solana";

describe("Ethereum Auth", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  const validEthPublicKey =
    "0x0304ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";

  const invalidEthPublicKey =
    "0x0314ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";

  const sampleMessage =
    '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}';
  const validSignature =
    "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d111b";

  const tamperedSignature =
    "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d121b";

  /**
   * Helper function to execute the Ethereum signature verification
   */
  async function verifyEthSignature(
    message: string,
    signature: string,
    publicKey: string
  ) {
    const ethData = { message, signature };

    const txSignature = await program.methods
      .verifyEthereumSignature(ethData, publicKey)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
      });

    await confirmTransaction(provider.connection, txSignature);

    const result = await getTransactionReturnValue<Uint8Array | null>(
      provider.connection,
      txSignature
    );

    return {
      txSignature,
      returnValue: result ? (result[0] as 0 | 1) : null,
    };
  }

  it("should validate valid Ethereum signature correctly", async () => {
    const { txSignature, returnValue } = await verifyEthSignature(
      sampleMessage,
      validSignature,
      validEthPublicKey
    );

    const txInfo = await provider.connection.getTransaction(txSignature, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });
    console.log("Transaction info:", JSON.stringify(txInfo, null, 2));

    assert.strictEqual(returnValue, 1, "Signature verification should succeed");
  });

  it("should reject signature with incorrect public key", async () => {
    const { returnValue } = await verifyEthSignature(
      sampleMessage,
      validSignature,
      invalidEthPublicKey
    );

    assert.notStrictEqual(
      returnValue,
      1,
      "Signature verification should fail with wrong public key"
    );
  });

  it("should reject tampered signature", async () => {
    const { returnValue } = await verifyEthSignature(
      sampleMessage,
      tamperedSignature,
      validEthPublicKey
    );

    assert.notStrictEqual(
      returnValue,
      1,
      "Signature verification should fail with tampered signature"
    );
  });
});
