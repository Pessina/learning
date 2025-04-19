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
    // SET_2: {
    //   COMPRESSED_PUBLIC_KEY:
    //     "0x03f6c9bde7c398eaaf91f1f2f142b2dd81c8e3b3082c348d186a382d17f13b41f0",
    //   INPUTS: [
    //     {
    //       CLIENT_DATA:
    //         '{"type":"webauthn.get","challenge":"jIQjdzTBeBvOJ-kSXpx0ePSbQSx1IRNIseDVMv0Bick","origin":"http://localhost:3000","crossOrigin":false}',
    //       AUTHENTICATOR_DATA:
    //         "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
    //       SIGNATURE:
    //         "0x276f7ba8593bfcb8dd2842f7602ee80827f4bfa5b617019bd0221442fbd1feffd856691fb6dd2283b92f7c8ee76b6727765fd024d9f170a419f77e4e9adcfe4a",
    //     },
    //     {
    //       CLIENT_DATA:
    //         '{"type":"webauthn.get","challenge":"gRY3-0x9VUcQxpsswrEKZcfOof9yguPKqIXBcUP4ZNE","origin":"http://localhost:3000","crossOrigin":false}',
    //       AUTHENTICATOR_DATA:
    //         "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
    //       SIGNATURE:
    //         "0xa314e582181038e2702cc313331e451908d40a8d45f3c0c2a80f23ca8ce000409fd5729b034ff44c2e8148ee656b7694d8f21e863207cbd7ebcbd706cbc50124",
    //     },
    //     {
    //       CLIENT_DATA:
    //         '{"type":"webauthn.get","challenge":"04LMuuH5ZdbP51T4WIGJJ4aIMGSZTWHgKH1EHxBrHOo","origin":"http://localhost:3000","crossOrigin":false}',
    //       AUTHENTICATOR_DATA:
    //         "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
    //       SIGNATURE:
    //         "0x284fddd33c089c8ab6b82659efe9b755be8a871c0a7daa5010b9d98554a82f9de6e4fd100301578e5781012eeb932d7d38a2145e39e82532c6830dfbe17e01e5",
    //     },
    //   ],
    // },
  };

  it("should validate WebAuthn signature correctly", async () => {
    const testPromises = [];

    for (const testSet of Object.values(TEST_INPUTS)) {
      const compressedPublicKey = testSet.COMPRESSED_PUBLIC_KEY;

      for (const input of testSet.INPUTS) {
        testPromises.push(
          (async () => {
            const webauthnData = {
              clientData: input.CLIENT_DATA,
              authenticatorData: input.AUTHENTICATOR_DATA,
              signature: input.SIGNATURE,
            };

            const result = await verifyWebauthnSignature(
              webauthnData,
              compressedPublicKey
            );

            console.log(result.error);

            assert.isTrue(
              result.success,
              "Transaction should complete successfully"
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
