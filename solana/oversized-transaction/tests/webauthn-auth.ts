import * as anchor from "@coral-xyz/anchor";
import { createHash } from "crypto";
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
const SIGNATURE_OFFSETS_SERIALIZED_SIZE = 14; // 7 fields * 2 bytes each
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
    HEADER_SIZE + signature.length + publicKey.length + message.length
  );

  data.writeUInt8(1, 0);
  data.writeUInt8(0, 1);

  const signatureOffset = HEADER_SIZE;
  const publicKeyOffset = signatureOffset + signature.length;
  const messageDataOffset = publicKeyOffset + publicKey.length;
  const messageDataSize = message.length;

  data.writeUInt16LE(signatureOffset, DATA_START);
  data.writeUInt16LE(INSTRUCTION_INDEX_NOT_USED, DATA_START + 2);
  data.writeUInt16LE(publicKeyOffset, DATA_START + 4);
  data.writeUInt16LE(INSTRUCTION_INDEX_NOT_USED, DATA_START + 6);
  data.writeUInt16LE(messageDataOffset, DATA_START + 8);
  data.writeUInt16LE(messageDataSize, DATA_START + 10);
  data.writeUInt16LE(INSTRUCTION_INDEX_NOT_USED, DATA_START + 12);

  Buffer.from(signature).copy(data, signatureOffset);
  Buffer.from(publicKey).copy(data, publicKeyOffset);
  Buffer.from(message).copy(data, messageDataOffset);

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
  const clientDataHash = createHash("sha256")
    .update(Buffer.from(webauthnData.clientData, "utf-8"))
    .digest();

  const authenticatorDataBuffer = Buffer.from(
    webauthnData.authenticatorData.slice(2),
    "hex"
  );
  const clientDataHashBuffer = Buffer.from(clientDataHash);

  const message = Buffer.concat([
    authenticatorDataBuffer,
    clientDataHashBuffer,
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
 * Function to analyze signatures and compare them for debugging
 */
function analyzeSignature(signature: string): { r: Buffer; s: Buffer } {
  const sigBuffer = Buffer.from(signature.slice(2), "hex");
  const r = sigBuffer.slice(0, 32);
  const s = sigBuffer.slice(32, 64);

  return { r, s };
}

/**
 * Function to normalize an ECDSA signature by ensuring s is in the lower range
 * When s is in the upper range, we replace it with N - s as per ECDSA standard
 */
function normalizeSignature(signature: string): string {
  const { r, s } = analyzeSignature(signature);

  // N is the order of the secp256r1 curve
  const N = Buffer.from(
    "FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551",
    "hex"
  );
  const halfN = Buffer.from(
    "7FFFFFFF800000007FFFFFFFFFFFFFFFDE73D56D538CF427D9CE5617E3192A8",
    "hex"
  );

  // Check if s is in the upper range
  if (Buffer.compare(s, halfN) > 0) {
    // Calculate N - s
    const sBigInt = BigInt("0x" + s.toString("hex"));
    const nBigInt = BigInt("0x" + N.toString("hex"));
    const newSBigInt = nBigInt - sBigInt;

    const newSHex = newSBigInt.toString(16).padStart(64, "0");
    const newS = Buffer.from(newSHex, "hex");

    // Create new signature with same r but normalized s
    const normalizedSig = Buffer.concat([r, newS]);
    return "0x" + normalizedSig.toString("hex");
  }

  // If s is already in the lower range, return the original signature
  return signature;
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

    if (addVerificationInstruction) {
      // Normalize the signature to ensure s is in the lower range
      const normalizedSignature = normalizeSignature(webauthnData.signature);

      const signatureBytes = Buffer.from(normalizedSignature.slice(2), "hex");

      const publicKeyBytes = Buffer.from(publicKey.slice(2), "hex");

      const verificationInstruction = createSecp256r1VerificationInstruction(
        signatureBytes,
        publicKeyBytes,
        message
      );
      instructions.push(verificationInstruction);
    }

    const computeBudgetInstruction = ComputeBudgetProgram.setComputeUnitLimit({
      units: SOLANA_MAX_COMPUTE_UNITS,
    });
    instructions.unshift(computeBudgetInstruction);

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
    console.error("Transaction error occurred:", error);

    if (error.transactionLogs) {
      console.error("Transaction logs:", error.transactionLogs);
    }

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

  const TEST_INPUTS = {
    SET_1: {
      COMPRESSED_PUBLIC_KEY:
        "0x020a34c8cfba4e32bdd93427cd30ddfe256adc6b101257282768f0f42af231b07f",
      INPUTS: [
        {
          CLIENT_DATA:
            '{"type":"webauthn.get","challenge":"arbU40dsETYiCh-EVZnhYr6LCwGXYEdMuT9TkPYIfkM","origin":"http://localhost:3000","crossOrigin":false}',
          AUTHENTICATOR_DATA:
            "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
          SIGNATURE:
            "0x54cdfd0dd30cf52e65345db8e151c0533c5fd51e894da5d93e6ad894643908032bacf1600319e6f22c3702c726c8734452f589ff1058f4cd53e5fc5d48876349",
        },
        {
          CLIENT_DATA:
            '{"type":"webauthn.get","challenge":"yJbTAOV9R_J-GVGsRDzquIWSjE4IervE7zjI5BX8UG4","origin":"http://localhost:3000","crossOrigin":false}',
          AUTHENTICATOR_DATA:
            "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
          SIGNATURE:
            "0x8549305067d642d5b7fd18f31b116752bf746cc283b0cc356beba35994e766893bce4c1c146cf0e247a865ded8e18db824bac0eef4e82acd4be8aafd17c203a0",
        },
        {
          CLIENT_DATA:
            '{"type":"webauthn.get","challenge":"s1BkcpB2a4MNysBrTKU3ruFP6N6AW5Nnf66UsQUebLQ","origin":"http://localhost:3000","crossOrigin":false,"other_keys_can_be_added_here":"do not compare clientDataJSON against a template. See https://goo.gl/yabPex"}',
          AUTHENTICATOR_DATA:
            "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
          SIGNATURE:
            "0xdbad2925d6958b4d17bb71bec492a680bc919db69e1577ff62ffe748f131220951e1aab36feae93baa75d7afaea9ad6730de031eeba309e0dfe9e5472199440d",
        },
      ],
    },
    SET_2: {
      COMPRESSED_PUBLIC_KEY:
        "0x031a08c5e977ab0a71d1ac3e5b8c435a431afb4c6d641b00a8b91496c5b085e6a3",
      INPUTS: [
        {
          CLIENT_DATA:
            '{"type":"webauthn.get","challenge":"DrZECYyV1n-dEUgWnwHu9_vun9jvTs6R_fIkDcwhwgA","origin":"http://localhost:3000","crossOrigin":false,"other_keys_can_be_added_here":"do not compare clientDataJSON against a template. See https://goo.gl/yabPex"}',
          AUTHENTICATOR_DATA:
            "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
          SIGNATURE:
            "0xd007d9c2850714db0addabe30b05fc3d1605cd9c4d848c794fd11fca184012c27262d46e06caff500847d800219fa16400ed751017146ae2bf7ddf8267fcda5b",
        },
        {
          CLIENT_DATA:
            '{"type":"webauthn.get","challenge":"7NEINnchZ-NH77mImg2kMibaTnOKjPyFn_-lYaD8Bh8","origin":"http://localhost:3000","crossOrigin":false}',
          AUTHENTICATOR_DATA:
            "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
          SIGNATURE:
            "0x7bcaded9357738fa21bca7cc6f4f37e7a366f03498678188fcf35cea66f2dbcd28455b66e1cc23b9b73a0238ec43160b2b63b305d1f1a4ab4651099ea46036ff",
        },
        {
          CLIENT_DATA:
            '{"type":"webauthn.get","challenge":"4SzZvQR_13EYvnAvUF0Qq78E07BiBSZKKNvvMVQbpyo","origin":"http://localhost:3000","crossOrigin":false}',
          AUTHENTICATOR_DATA:
            "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
          SIGNATURE:
            "0x23c70b2fa596b1c56ffc3f43567ccf86ddf910304ac32eb2d44ad59e0e4e3441c9f2c9c57cecb348e6ed6e5b2a242a477089b010f5bc62862c91d4ee4741c4c4",
        },
      ],
    },
  };

  it.only("should validate WebAuthn signature correctly", async () => {
    const testPromises = [];

    for (const testSet of Object.values(TEST_INPUTS)) {
      const compressedPublicKey = testSet.COMPRESSED_PUBLIC_KEY;

      for (const input of testSet.INPUTS) {
        const webauthnData = {
          clientData: input.CLIENT_DATA,
          authenticatorData: input.AUTHENTICATOR_DATA,
          signature: input.SIGNATURE,
        };

        testPromises.push(
          (async () => {
            const result = await verifyWebauthnSignature(
              webauthnData,
              compressedPublicKey
            );

            assert.strictEqual(
              result.returnValue,
              1,
              `Should have a successful return value for public key ${compressedPublicKey}`
            );
          })()
        );
      }
    }

    await Promise.all(testPromises);
  });

  // it("should fail to validate WebAuthn signature with wrong public key", async () => {
  //   const computeUnitsInstruction = ComputeBudgetProgram.setComputeUnitPrice({
  //     microLamports: SOLANA_MAX_COMPUTE_UNITS,
  //   });

  //   const result = await verifyWebauthnSignature(
  //     failingWebauthnData,
  //     invalidCompressedPublicKey,
  //     {
  //       additionalInstructions: [computeUnitsInstruction],
  //     }
  //   );

  //   assert.isFalse(result.success, "Transaction should fail");
  //   if (result.error) {
  //     assert.strictEqual(
  //       result.error.transactionMessage,
  //       `Transaction simulation failed: Error processing Instruction 1: custom program error: ${SOLANA_PRE_COMPILED_ERRORS.INVALID_SIGNATURE}`,
  //       "Should fail with processing error"
  //     );
  //   } else {
  //     assert.fail("Expected an error but none was thrown");
  //   }
  // });

  // it("should fail to validate WebAuthn signature if there is no verification instruction", async () => {
  //   const result = await verifyWebauthnSignature(
  //     validWebauthnData,
  //     validCompressedPublicKey,
  //     { addVerificationInstruction: false }
  //   );

  //   assert.isFalse(result.success, "Transaction should fail");
  //   if (result.error && result.error.error) {
  //     assert.include(
  //       result.error.error.errorMessage || "",
  //       "Missing secp256r1 verification instruction",
  //       "Should fail with missing verification instruction error"
  //     );
  //   } else {
  //     assert.fail("Expected a specific error message but none was found");
  //   }
  // });
});
