import * as anchor from "@coral-xyz/anchor";
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
  addEthereumMessagePrefix,
  compressedPublicKeyToEthAddress,
  parseEthereumSignature,
  validateEthereumAddress,
} from "../utils/ethereum";
import {
  SOLANA_MAX_COMPUTE_UNITS,
  SOLANA_PRE_COMPILED_ERRORS,
} from "../utils/constants";

const SECP256K1_PROGRAM_ID = new PublicKey(
  "KeccakSecp256k11111111111111111111111111111"
);

// Constants from the documentation
const SIGNATURE_OFFSETS_SERIALIZED_SIZE = 11;
const DATA_START = SIGNATURE_OFFSETS_SERIALIZED_SIZE + 1; // 12
const SIGNATURE_SERIALIZED_SIZE = 64;
const HASHED_PUBKEY_SERIALIZED_SIZE = 20;

/**
 * Creates a secp256k1 verification instruction for Ethereum signatures
 */
function createSecp256k1VerificationInstruction(
  signature: Buffer,
  recoveryId: number,
  ethAddressBytes: Buffer,
  messageBytes: Buffer
): TransactionInstruction {
  // Calculate total instruction data size
  const messageOffset =
    DATA_START + HASHED_PUBKEY_SERIALIZED_SIZE + SIGNATURE_SERIALIZED_SIZE + 1;
  const messageSize = messageBytes.length;
  const instructionDataSize = messageOffset + messageSize;
  const instructionData = Buffer.alloc(instructionDataSize);

  // Number of signatures (always 1 in this case)
  instructionData.writeUInt8(1, 0);

  // Define offsets for instruction data
  const ethAddressOffset = DATA_START; // 12
  const signatureOffset = DATA_START + HASHED_PUBKEY_SERIALIZED_SIZE; // 32
  const recoveryIdOffset =
    DATA_START + HASHED_PUBKEY_SERIALIZED_SIZE + SIGNATURE_SERIALIZED_SIZE; // 96

  // Serialize SecpSignatureOffsets
  const offsetsBuffer = Buffer.alloc(SIGNATURE_OFFSETS_SERIALIZED_SIZE);
  offsetsBuffer.writeUInt16LE(signatureOffset, 0);
  offsetsBuffer.writeUInt8(0, 2); // signature_instruction_index
  offsetsBuffer.writeUInt16LE(ethAddressOffset, 3);
  offsetsBuffer.writeUInt8(0, 5); // eth_address_instruction_index
  offsetsBuffer.writeUInt16LE(messageOffset, 6);
  offsetsBuffer.writeUInt16LE(messageSize, 8);
  offsetsBuffer.writeUInt8(0, 10); // message_instruction_index
  offsetsBuffer.copy(instructionData, 1);

  // Write data into instruction
  ethAddressBytes.copy(instructionData, ethAddressOffset);
  signature.copy(instructionData, signatureOffset);
  instructionData.writeUInt8(recoveryId, recoveryIdOffset);
  messageBytes.copy(instructionData, messageOffset);

  return new TransactionInstruction({
    keys: [],
    programId: SECP256K1_PROGRAM_ID,
    data: instructionData,
  });
}

/**
 * Prepares Ethereum data for verification
 */
function prepareEthereumData(
  ethData: { signature: string; message: string },
  ethAddress: string
) {
  // Parse the signature using utility function
  const { signature, recoveryId } = parseEthereumSignature(ethData.signature);

  // Add Ethereum message prefix
  const messageWithPrefix = addEthereumMessagePrefix(ethData.message);
  const messageBytes = Buffer.from(messageWithPrefix, "utf8");

  // Validate Ethereum address
  const ethAddressBytes = validateEthereumAddress(ethAddress);

  return {
    signature,
    recoveryId,
    messageBytes,
    ethAddressBytes,
    ethDataArgs: {
      signature: ethData.signature,
      message: messageWithPrefix,
    },
  };
}

/**
 * Verifies an Ethereum signature by constructing a Secp256k1 instruction and calling the program.
 * @param {Object} ethData - Contains the signature (hex string) and message (string).
 * @param {string} ethAddress - Ethereum address as a hex string (e.g., "0x...").
 * @param {Object} options - Optional parameters for verification.
 * @returns {Promise<{success: boolean, returnValue?: number, error?: any}>} - Result of the verification.
 */
async function verifyEthSignature(
  ethData: { signature: string; message: string },
  ethAddress: string,
  options: {
    addVerificationInstruction?: boolean;
    additionalInstructions?: TransactionInstruction[];
  } = {}
): Promise<{
  success: boolean;
  returnValue?: 1 | 0;
  error?: any;
  txSignature?: string | null;
}> {
  const { addVerificationInstruction = true, additionalInstructions = [] } =
    options;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;

  try {
    const {
      signature,
      recoveryId,
      messageBytes,
      ethAddressBytes,
      ethDataArgs,
    } = prepareEthereumData(ethData, ethAddress);

    const instructions = [...additionalInstructions];

    // Add verification instruction if needed
    if (addVerificationInstruction) {
      const verificationInstruction = createSecp256k1VerificationInstruction(
        signature,
        recoveryId,
        ethAddressBytes,
        messageBytes
      );
      instructions.push(verificationInstruction);
    }

    const txSignature = await program.methods
      .verifyEthereumSignature(ethDataArgs, ethAddress)
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

describe("Ethereum Signature Verification", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  // Convert compressed public key to uncompressed format using noble-secp256k1
  const compressedPublicKey =
    "0x0304ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";

  // Get Ethereum address from compressed public key using utility function
  const validEthAddress = compressedPublicKeyToEthAddress(compressedPublicKey);

  const invalidEthAddress =
    validEthAddress.slice(0, 3) + "1" + validEthAddress.slice(4);

  const validEthData = {
    signature:
      "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d111b",
    message:
      '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}',
  };

  const failingEthData = {
    signature:
      "0x1413a2cc33c3ad9a150de47566c098c7f0a3f3236767ae80cfb3dcef1447d5ad1850f86f1161a5cc3620dcd8a0675f5e7ccf76f5772bb3af6ed6ea6e4ee05d111b",
    message:
      '{"actions":[{"Transfer":{"deposit":"10000000000000000000"}}],"nonce":"4","receiver_id":"felipe-sandbox-account.testnet"}',
  };

  it("should validate Ethereum signature correctly", async () => {
    const result = await verifyEthSignature(validEthData, validEthAddress);

    assert.isTrue(result.success, "Transaction should complete successfully");
    assert.strictEqual(
      result.returnValue,
      1,
      "Should have a successful return value"
    );
  });

  it("should fail to validate Ethereum signature with wrong public key", async () => {
    const computeUnitsInstruction = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: SOLANA_MAX_COMPUTE_UNITS,
    });

    const result = await verifyEthSignature(failingEthData, invalidEthAddress, {
      additionalInstructions: [computeUnitsInstruction],
    });

    assert.isFalse(result.success, "Transaction should fail");
    if (result.error) {
      // 0x2 is the InvalidSignature ErrorCode for the secp256k1 program
      assert.strictEqual(
        result.error.transactionMessage,
        `Transaction simulation failed: Error processing Instruction 1: custom program error: ${SOLANA_PRE_COMPILED_ERRORS.INVALID_SIGNATURE}`,
        "Should fail with processing error"
      );
    } else {
      assert.fail("Expected an error but none was thrown");
    }
  });

  it("should fail to validate Ethereum signature if there is no verification instruction", async () => {
    const result = await verifyEthSignature(validEthData, validEthAddress, {
      addVerificationInstruction: false,
    });

    assert.isFalse(result.success, "Transaction should fail");
    if (result.error && result.error.error) {
      assert.include(
        result.error.error.errorMessage || "",
        "Missing secp256k1 verification instruction",
        "Should fail with missing verification instruction error"
      );
    } else {
      assert.fail("Expected a specific error message but none was found");
    }
  });
});
