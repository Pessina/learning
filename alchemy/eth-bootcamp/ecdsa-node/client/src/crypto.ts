import { Buffer } from "buffer";
import canonicalize from "canonicalize";
import { keccak256 } from "ethereum-cryptography/keccak";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import {
  bytesToHex,
  concatBytes,
  hexToBytes,
} from "ethereum-cryptography/utils";
import { ethers } from "ethers";

const serialize = (data: Record<string, any>): string => {
  let serialized = canonicalize(data);

  if (!serialized) {
    throw new Error("Failed to serialize data");
  }

  return serialized;
};

export const walletSign = async (
  data: Record<string, any>,
  metamask: ethers.Eip1193Provider
) => {
  const serialized = serialize(data);

  const provider = new ethers.BrowserProvider(metamask);
  const signer = await provider.getSigner();
  const signature = await signer.signMessage(serialized);

  const publicKey = recoverPublicKeyFromSignature(getRSV(signature), data);
  console.log({ address: getETHAddress(publicKey) });

  return signature;
};

export const recoverPublicKeyFromSignature = (
  { r, s, v }: RSV,
  data: Record<string, any>
) => {
  const serialized = serialize(data);
  const encoder = new TextEncoder();

  const eip191Standard = concatBytes(
    new Uint8Array([0x19]),
    new Uint8Array([0x45]),
    encoder.encode(`thereum Signed Message:\n${serialized.length}`),
    encoder.encode(serialized)
  );
  const hash = keccak256(eip191Standard);

  const signature = new secp256k1.Signature(
    BigInt(r),
    BigInt(s)
  ).addRecoveryBit(v - 27);
  return signature.recoverPublicKey(hash).toRawBytes(false);
};

type RSV = {
  r: string;
  s: string;
  v: number;
};

const getRSV = (signature: string): RSV => {
  const cleanSignature = signature.startsWith("0x")
    ? signature.slice(2)
    : signature;

  if (cleanSignature.length !== 130) {
    throw new Error(
      "Invalid signature length. Expected 130 characters after removing 0x."
    );
  }

  return {
    r: "0x" + cleanSignature.slice(0, 64),
    s: "0x" + cleanSignature.slice(64, 128),
    v: parseInt(cleanSignature.slice(128), 16),
  };
};

function getETHAddress(publicKey: Uint8Array): string {
  const hash = keccak256(publicKey.slice(1));
  const address = hash.slice(-20);

  return `0x${bytesToHex(address)}`;
}

// export const localSign = (data: Record<string, any>, privateKey: string) => {
//   const hashHex = serializeAndHash(data);

//   const signature = secp256k1.sign(hashHex, privateKey);
//   const recovery = signature.recovery.toString(16).padStart(2, "0");

//   recoverPublicKeyFromSignature(`${signature.toCompactHex}${recovery}`, data);

//   return {
//     r: signature.r.toString(16).padStart(2, "0"),
//     s: signature.s.toString(16).padStart(2, "0"),
//     recovery: signature.recovery,
//   };
// };
