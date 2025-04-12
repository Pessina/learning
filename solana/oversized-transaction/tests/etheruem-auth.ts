import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";
import { getTransactionReturnValue } from "../utils/solana";
import { publicKeyToAddress } from "viem/accounts";
import * as elliptic from "elliptic";

const SECP256K1_PROGRAM_ID = new PublicKey(
  "KeccakSecp256k11111111111111111111111111111"
);

// Constants from the documentation
const SIGNATURE_OFFSETS_SERIALIZED_SIZE = 11;
const DATA_START = SIGNATURE_OFFSETS_SERIALIZED_SIZE + 1; // 12
const SIGNATURE_SERIALIZED_SIZE = 64;
const HASHED_PUBKEY_SERIALIZED_SIZE = 20;

/**
 * Verifies an Ethereum signature by constructing a Secp256k1 instruction and calling the program.
 * @param {Object} ethData - Contains the signature (hex string) and message (string).
 * @param {string} ethAddress - Ethereum address as a hex string (e.g., "0x...").
 * @returns {Promise<{success: boolean, returnValue?: number}>} - Result of the verification.
 */
async function verifyEthSignature(
  ethData: { signature: string; message: string },
  ethAddress: string
): Promise<{ success: boolean; returnValue?: 1 | 0 }> {
  const provider = anchor.getProvider();
  const program = anchor.workspace.oversizedTransaction;

  // Parse the signature (65 bytes: 64 bytes signature + 1 byte recovery ID)
  const signatureHex = ethData.signature.slice(2); // Remove '0x'
  const signatureBytes = Buffer.from(signatureHex, "hex");
  if (signatureBytes.length !== 65) {
    throw new Error(
      `Invalid signature length: expected 65 bytes, got ${signatureBytes.length}`
    );
  }
  const signature = signatureBytes.slice(0, 64); // First 64 bytes
  const v = signatureBytes[64]; // Last byte is v
  const recoveryId = v - 27; // Convert Ethereum v (27 or 28) to recovery ID (0 or 1)
  if (recoveryId < 0 || recoveryId > 3) {
    throw new Error(`Invalid recovery ID: ${recoveryId}`);
  }

  // Convert message to bytes
  // Compute the Ethereum signed message hash with prefix
  const prefix = `\x19Ethereum Signed Message:\n${ethData.message.length}`;
  const messageWithPrefix = prefix + ethData.message;
  const messageBytes = Buffer.from(messageWithPrefix, "utf8");

  // Parse Ethereum address (20 bytes)
  const ethAddressBytes = Buffer.from(ethAddress.slice(2), "hex");
  if (ethAddressBytes.length !== 20) {
    throw new Error(
      `Invalid Ethereum address length: expected 20 bytes, got ${ethAddressBytes.length}`
    );
  }

  // Define offsets for instruction data
  const ethAddressOffset = DATA_START; // 12
  const signatureOffset = DATA_START + HASHED_PUBKEY_SERIALIZED_SIZE; // 32
  const recoveryIdOffset =
    DATA_START + HASHED_PUBKEY_SERIALIZED_SIZE + SIGNATURE_SERIALIZED_SIZE; // 96
  const messageOffset =
    DATA_START + HASHED_PUBKEY_SERIALIZED_SIZE + SIGNATURE_SERIALIZED_SIZE + 1; // 97
  const messageSize = messageBytes.length;

  // Calculate total instruction data size
  const instructionDataSize = messageOffset + messageSize;
  const instructionData = Buffer.alloc(instructionDataSize);

  // Number of signatures (always 1 in this case)
  instructionData.writeUInt8(1, 0);

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

  // Create Secp256k1 instruction
  const secpInstruction = new TransactionInstruction({
    keys: [],
    programId: SECP256K1_PROGRAM_ID,
    data: instructionData,
  });

  // Create program instruction to verify the signature
  const programInstruction = await program.methods
    .verifyEthereumSignature(ethData, ethAddress)
    .accounts({
      signer: provider.wallet.publicKey,
    })
    .instruction();

  // Build and send the transaction
  const transaction = new anchor.web3.Transaction();
  transaction.add(secpInstruction);
  transaction.add(programInstruction);

  try {
    const signature = await provider.sendAndConfirm(transaction);
    const returnValue = (await getTransactionReturnValue(
      provider.connection,
      signature
    )) as 1 | 0;
    return { success: true, returnValue };
  } catch (error) {
    console.error("Transaction failed:", error);
    return { success: false };
  }
}

describe.only("Ethereum Signature Verification", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  // Convert compressed public key to uncompressed format using noble-secp256k1
  const compressedPublicKey =
    "0x0304ab3cb2897344aa3f6ffaac94e477aeac170b9235d2416203e2a72bc9b8a7c7";

  // Use elliptic library to convert compressed public key to uncompressed format
  const ec = new elliptic.ec("secp256k1");
  const keyPair = ec.keyFromPublic(compressedPublicKey.slice(2), "hex");
  const publicKey = keyPair.getPublic().encode("hex", false);

  const validEthAddress = publicKeyToAddress(`0x${publicKey}`);

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

  it.only("should validate Ethereum signature correctly", async () => {
    const result = await verifyEthSignature(validEthData, validEthAddress);

    assert.isTrue(result.success, "Transaction should complete successfully");
    assert.strictEqual(
      result.returnValue,
      1,
      "Should have a successful return value"
    );
  });

  // it("should fail to validate Ethereum signature with wrong public key", async () => {
  //   const computeUnitsInstruction = ComputeBudgetProgram.setComputeUnitPrice({
  //     microLamports: 1_400_000,
  //   });

  //   const result = await verifyEthSignature(failingEthData, invalidEthAddress, {
  //     additionalInstructions: [computeUnitsInstruction],
  //   });

  //   assert.isFalse(result.success, "Transaction should fail");
  //   if (result.error) {
  //     assert.fail(result.error.toString(), "Error processing Instruction");
  //   } else {
  //     assert.fail("Expected an error but none was thrown");
  //   }
  // });

  // it("should fail to validate Ethereum signature if there is no verification instruction", async () => {
  //   const result = await verifyEthSignature(validEthData, validEthAddress, {
  //     addVerificationInstruction: false,
  //   });

  //   assert.isFalse(result.success, "Transaction should fail");
  //   if (result.error && result.error.error) {
  //     assert.include(
  //       result.error.error.errorMessage || "",
  //       "Missing secp256k1 verification instruction",
  //       "Should fail with missing verification instruction error"
  //     );
  //   } else {
  //     assert.fail("Expected a specific error message but none was found");
  //   }
  // });
});
