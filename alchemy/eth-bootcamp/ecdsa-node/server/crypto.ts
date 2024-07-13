import { secp256k1 } from "ethereum-cryptography/secp256k1";
import { keccak256 } from "ethereum-cryptography/keccak";
import { bytesToHex, concatBytes } from "ethereum-cryptography/utils";
import canonicalize from "canonicalize";

interface RSV {
  r: string;
  s: string;
  v: number;
}

interface KeyPair {
  privateKey: Uint8Array;
  publicKey: Uint8Array;
}

const serialize = (data: Record<string, any>): string => {
  let serialized = canonicalize(data);

  if (!serialized) {
    throw new Error("Failed to serialize data");
  }

  return serialized;
};

const generateKeyPair = (): KeyPair => {
  const privateKey = secp256k1.utils.randomPrivateKey();
  const publicKey = secp256k1.getPublicKey(privateKey);

  return { privateKey, publicKey };
};

export const recoverAddressFromSignMessage = (
  signature: string,
  data: Record<string, any>
) => {
  const { r, s, v } = getRSV(signature);
  const serialized = serialize(data);
  const encoder = new TextEncoder();

  const eip191Standard = concatBytes(
    new Uint8Array([0x19]),
    new Uint8Array([0x45]),
    encoder.encode(`thereum Signed Message:\n${serialized.length}`),
    encoder.encode(serialized)
  );
  const hash = keccak256(eip191Standard);

  const secpSignature = new secp256k1.Signature(
    BigInt(r),
    BigInt(s)
  ).addRecoveryBit(v - 27);

  const publicKey = secpSignature.recoverPublicKey(hash).toRawBytes(false);

  return getETHAddress(publicKey);
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
