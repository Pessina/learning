const { accounts } = require("./constants/accounts.json");
const express = require("express");
const app = express();
const cors = require("cors");
const { recoverPublicKeyFromSignature } = require("./crypto");
const { secp256k1 } = require("ethereum-cryptography/secp256k1");
const { toHex } = require("ethereum-cryptography/utils");

const port = 3042;

app.use(cors());
app.use(express.json());

const balances = {};
for (let i = 0; i < accounts.length; i++) {
  balances[accounts[i].publicKey] = accounts[i].balance;
}

app.get("/balance/:address", (req, res) => {
  const { address } = req.params;
  const balance = balances[address] || 0;
  res.send({ balance });
});

app.post("/send", (req, res) => {
  const { signature, msg } = req.body;
  const { recipient, amount } = JSON.parse(msg);
  const { compactRS, recovery } = signature;

  const sender = recoverPublicKeyFromSignature(
    secp256k1.Signature.fromCompact(compactRS).addRecoveryBit(recovery),
    msg
  );

  setInitialBalance(sender);
  setInitialBalance(recipient);

  if (balances[sender] < amount) {
    res.status(400).send({ message: "Not enough funds!" });
  } else {
    balances[sender] -= amount;
    balances[recipient] += amount;
    res.send({ balance: balances[sender] });
  }
});

app.listen(port, () => {
  console.log(`Listening on port ${port}!`);
});

function setInitialBalance(address) {
  if (!balances[address]) {
    balances[address] = 0;
  }
}
