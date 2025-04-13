import * as anchor from "@coral-xyz/anchor";
import { sha256 } from "js-sha256";
import { Program } from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import { confirmTransaction, getTransactionReturnValue } from "../utils/solana";
import {
  SOLANA_MAX_COMPUTE_UNITS,
  SOLANA_PRE_COMPILED_ERRORS,
} from "../utils/constants";

const SECP256R1_PROGRAM_ID = new PublicKey(
  "Secp256r1SigVerify1111111111111111111111111"
);

/**
 * Constants for secp256r1 verification instruction
 */
const SIGNATURE_OFFSETS_SERIALIZED_SIZE = 14;
const DATA_START = 2; // 1 byte for number of signatures + 1 byte for padding
const HEADER_SIZE = DATA_START + SIGNATURE_OFFSETS_SERIALIZED_SIZE;
const INSTRUCTION_INDEX_NOT_USED = 0xffff;

/**
 * Creates a secp256r1 verification instruction for WebAuthn signatures
 */
function createSecp256r1VerificationInstruction(
  signature: Uint8Array,
  publicKey: Uint8Array,
  message: Uint8Array
): TransactionInstruction {
  const data = Buffer.alloc(
    HEADER_SIZE + publicKey.length + signature.length + message.length
  );

  data.writeUInt8(1, 0); // Number of signatures
  data.writeUInt8(0, 1); // Padding

  const signatureOffset = HEADER_SIZE;
  const publicKeyOffset = HEADER_SIZE + signature.length;
  const messageDataOffset = HEADER_SIZE + signature.length + publicKey.length;
  const messageDataSize = message.length;

  data.writeUInt16LE(signatureOffset, 2);
  data.writeUInt16LE(INSTRUCTION_INDEX_NOT_USED, 4);
  data.writeUInt16LE(publicKeyOffset, 6);
  data.writeUInt16LE(INSTRUCTION_INDEX_NOT_USED, 8);
  data.writeUInt16LE(messageDataOffset, 10);
  data.writeUInt16LE(messageDataSize, 12);
  data.writeUInt16LE(INSTRUCTION_INDEX_NOT_USED, 14);

  data.set(signature, signatureOffset);
  data.set(publicKey, publicKeyOffset);
  data.set(message, messageDataOffset);

  return new TransactionInstruction({
    keys: [],
    programId: SECP256R1_PROGRAM_ID,
    data: data,
  });
}

/**
 * Prepares WebAuthn data for verification
 */
function prepareWebAuthnData(webauthnData: {
  signature: string;
  authenticatorData: string;
  clientData: string;
}) {
  const clientDataHash = sha256.arrayBuffer(webauthnData.clientData);
  const message = Buffer.concat([
    Buffer.from(webauthnData.authenticatorData.slice(2), "hex"),
    Buffer.from(clientDataHash),
  ]);

  return {
    message,
    webauthnDataArgs: {
      signature: webauthnData.signature,
      authenticatorData: webauthnData.authenticatorData,
      clientData: webauthnData.clientData,
    },
  };
}

/**
 * Helper function to verify WebAuthn signatures
 */
async function verifyWebauthnSignature(
  webauthnData: {
    signature: string;
    authenticatorData: string;
    clientData: string;
  },
  publicKey: string,
  options: {
    addVerificationInstruction?: boolean;
    additionalInstructions?: TransactionInstruction[];
  } = {}
) {
  const { addVerificationInstruction = true, additionalInstructions = [] } =
    options;
  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  try {
    const { message, webauthnDataArgs } = prepareWebAuthnData(webauthnData);

    const instructions = [...additionalInstructions];

    // Add verification instruction if needed
    if (addVerificationInstruction) {
      const verificationInstruction = createSecp256r1VerificationInstruction(
        Buffer.from(webauthnData.signature.slice(2), "hex"),
        Buffer.from(publicKey.slice(2), "hex"),
        message
      );
      instructions.push(verificationInstruction);
    }

    const txSignature = await program.methods
      .verifyWebauthnSignature(webauthnDataArgs, publicKey)
      .accounts({
        instructions_sysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .preInstructions(instructions)
      .rpc();

    await confirmTransaction(provider.connection, txSignature);

    const result = await getTransactionReturnValue<Uint8Array | null>(
      provider.connection,
      txSignature
    );

    return {
      txSignature,
      returnValue: result ? (result[0] as 0 | 1) : null,
      success: true,
    };
  } catch (error) {
    return {
      error,
      success: false,
      txSignature: null,
      returnValue: null,
    };
  }
}

describe("WebAuthn Authentication", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const validCompressedPublicKey =
    "0x0220fb23e028391b72c517850b3cc83ba529ef4db766098a29bf3c8d06be957878";
  const invalidCompressedPublicKey =
    "0x0220fb23e028391b72c517850b3cc83ba529ef4db766098a29bf3c8d06be957878";

  const validWebauthnData = {
    signature:
      "0xf77969b7eaeaaed4b9a5cc5636b3755259d29d1406d8e852a8ce43dc74644da11453962702ea21a9efdd4a7077e39fcd754e3d01579493cf972f0151b6672f1f",
    authenticatorData:
      "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631900000000",
    clientData:
      '{"type":"webauthn.get","challenge":"tAuyPmQcczI8CFoTekJz5iITeP80zcJ60VTC4sYz5s8","origin":"http://localhost:3000","crossOrigin":false}',
  };

  const failingWebauthnData = {
    signature:
      "0xf77969b7eaeaaed4b9a5cc5636b3755359d29d1406d8e852a8ce43dc74644da11453962702ea21a9efdd4a7077e39fcd754e3d01579493cf972f0151b6672f1f",
    authenticatorData:
      "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631900000000",
    clientData:
      '{"type":"webauthn.get","challenge":"tAuyPmQcczI8CFoTekJz5iITeP80zcJ60VTC4sYz5s8","origin":"http://localhost:3000","crossOrigin":false}',
  };

  it("should validate WebAuthn signature correctly", async () => {
    const result = await verifyWebauthnSignature(
      validWebauthnData,
      validCompressedPublicKey
    );

    assert.isTrue(result.success, "Transaction should complete successfully");
    assert.strictEqual(
      result.returnValue,
      1,
      "Should have a successful return value"
    );
  });

  it("should fail to validate WebAuthn signature with wrong public key", async () => {
    const computeUnitsInstruction = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: SOLANA_MAX_COMPUTE_UNITS,
    });

    const result = await verifyWebauthnSignature(
      failingWebauthnData,
      invalidCompressedPublicKey,
      {
        additionalInstructions: [computeUnitsInstruction],
      }
    );

    assert.isFalse(result.success, "Transaction should fail");
    if (result.error) {
      assert.strictEqual(
        result.error.transactionMessage,
        `Transaction simulation failed: Error processing Instruction 1: custom program error: ${SOLANA_PRE_COMPILED_ERRORS.INVALID_SIGNATURE}`,
        "Should fail with processing error"
      );
    } else {
      assert.fail("Expected an error but none was thrown");
    }
  });

  it("should fail to validate WebAuthn signature if there is no verification instruction", async () => {
    const result = await verifyWebauthnSignature(
      validWebauthnData,
      validCompressedPublicKey,
      { addVerificationInstruction: false }
    );

    assert.isFalse(result.success, "Transaction should fail");
    if (result.error && result.error.error) {
      assert.include(
        result.error.error.errorMessage || "",
        "Missing secp256r1 verification instruction",
        "Should fail with missing verification instruction error"
      );
    } else {
      assert.fail("Expected a specific error message but none was found");
    }
  });
});
