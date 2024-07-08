// Experiment file
const { Buffer } = require("buffer");
const { secp256k1 } = require("ethereum-cryptography/secp256k1.js");
const { toHex } = require("ethereum-cryptography/utils.js");
const { sha256 } = require("ethereum-cryptography/sha256.js");

const generateKeyPair = () => {
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

  console.log("sig", signature);
  console.log("sigHex", sigHex);
  console.log("sigRecovered", secp256k1.Signature.fromCompact(sigHex));
  console.log("originalPublicKey", toHex(originalPublicKey));
  console.log("publicKey", publicKey.toHex());

  return publicKey;
};

const recoverPublicKeyFromSignature = (signature, msg) => {
  const msgBytes = Buffer.from(msg);
  const hash = sha256(msgBytes);
  const msgHashHex = Buffer.from(hash).toString("hex");

  return signature.recoverPublicKey(msgHashHex).toHex();
};

module.exports = {
  generateKeyPair,
  getPublicKeyFromSignature,
  recoverPublicKeyFromSignature,
};
