// Experiment file
import { Buffer } from "buffer";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import { toHex } from "ethereum-cryptography/utils";
import { sha256 } from "ethereum-cryptography/sha256";

interface KeyPair {
  privateKey: Uint8Array;
  publicKey: Uint8Array;
}

const generateKeyPair = (): KeyPair => {
  const privateKey = secp256k1.utils.randomPrivateKey();
  const publicKey = secp256k1.getPublicKey(privateKey);

  return { privateKey, publicKey };
};

const getPublicKeyFromSignature = () => {
  const privateKey = secp256k1.utils.randomPrivateKey();
  const originalPublicKey = secp256k1.getPublicKey(privateKey);
  const msg = "123456";
  const signature = secp256k1.sign(msg, privateKey);
  const publicKey = signature.recoverPublicKey(msg);

  const sigHex = signature.toCompactHex();

  return publicKey;
};

const recoverPublicKeyFromSignature = (
  signature: ReturnType<typeof secp256k1.sign>,
  msg: string
): string => {
  const msgBytes = Buffer.from(msg);
  const hash = sha256(msgBytes);
  const msgHashHex = Buffer.from(hash).toString("hex");

  return signature.recoverPublicKey(msgHashHex).toHex();
};

export {
  generateKeyPair,
  getPublicKeyFromSignature,
  recoverPublicKeyFromSignature,
};
