import { publicKeyToAddress } from "viem/accounts";
import * as elliptic from "elliptic";

/**
 * Adds the Ethereum signed message prefix to a message
 * @param message - The original message
 * @returns The message with the Ethereum prefix
 */
export function addEthereumMessagePrefix(message: string): string {
  const prefix = `\x19Ethereum Signed Message:\n${message.length}`;
  return prefix + message;
}

/**
 * Converts a compressed public key to an Ethereum address
 * @param compressedPublicKey - The compressed public key in hex format (with 0x prefix)
 * @returns The Ethereum address with 0x prefix
 */
export function compressedPublicKeyToEthAddress(
  compressedPublicKey: string
): string {
  // Use elliptic library to convert compressed public key to uncompressed format
  const ec = new elliptic.ec("secp256k1");
  const keyPair = ec.keyFromPublic(compressedPublicKey.slice(2), "hex");
  const publicKey = keyPair.getPublic().encode("hex", false);

  // Convert to Ethereum address
  return publicKeyToAddress(`0x${publicKey}`);
}

/**
 * Parses an Ethereum signature into its components
 * @param signature - The Ethereum signature (65 bytes with 0x prefix)
 * @returns The signature components (signature bytes and recovery ID)
 */
export function parseEthereumSignature(signature: string): {
  signature: Buffer;
  recoveryId: number;
} {
  const signatureHex = signature.slice(2); // Remove '0x'
  const signatureBytes = Buffer.from(signatureHex, "hex");

  if (signatureBytes.length !== 65) {
    throw new Error(
      `Invalid signature length: expected 65 bytes, got ${signatureBytes.length}`
    );
  }

  const signaturePart = signatureBytes.slice(0, 64); // First 64 bytes
  const v = signatureBytes[64]; // Last byte is v
  const recoveryId = v - 27; // Convert Ethereum v (27 or 28) to recovery ID (0 or 1)

  if (recoveryId < 0 || recoveryId > 3) {
    throw new Error(`Invalid recovery ID: ${recoveryId}`);
  }

  return { signature: signaturePart, recoveryId };
}

/**
 * Validates an Ethereum address format
 * @param ethAddress - The Ethereum address to validate (with 0x prefix)
 * @returns The address bytes
 */
export function validateEthereumAddress(ethAddress: string): Buffer {
  const ethAddressBytes = Buffer.from(ethAddress.slice(2), "hex");

  if (ethAddressBytes.length !== 20) {
    throw new Error(
      `Invalid Ethereum address length: expected 20 bytes, got ${ethAddressBytes.length}`
    );
  }

  return ethAddressBytes;
}
