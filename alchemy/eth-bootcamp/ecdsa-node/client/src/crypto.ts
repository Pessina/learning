import { Buffer } from "buffer";
import canonicalize from "canonicalize";
import { keccak256 } from "ethereum-cryptography/keccak";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import { ethers } from "ethers";

const serialize = (data: Record<string, any>): string => {
  let serialized = canonicalize(data);

  if (!serialized) {
    throw new Error("Failed to serialize data");
  }

  return serialized;
};

// Return the signature on a hex string with RSV
export const walletSign = async (
  data: Record<string, any>,
  metamask: ethers.Eip1193Provider
): Promise<string> => {
  const serialized = serialize(data);

  const provider = new ethers.BrowserProvider(metamask);
  const signer = await provider.getSigner();
  const signature = await signer.signMessage(serialized);

  return signature;
};

// Return the signature on a hex string with RSV
export const localSign = (
  data: Record<string, any>,
  privateKey: string
): string => {
  const serialized = serialize(data);
  const hash = keccak256(Buffer.from(serialized));
  const signature = secp256k1.sign(hash, privateKey);

  return `0x${signature.toCompactHex()}${(signature.recovery + 27)
    .toString(16)
    .padStart(2, "0")}`;
};
