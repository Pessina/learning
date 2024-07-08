import { accounts } from "./constants/accounts.json";
import express, { Request, Response } from "express";
import cors from "cors";
import { recoverPublicKeyFromSignature } from "./crypto";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import { toHex } from "ethereum-cryptography/utils";

const app = express();
const port = 3042;

app.use(cors());
app.use(express.json());

interface Balances {
  [key: string]: number;
}

const balances: Balances = {};
for (let i = 0; i < accounts.length; i++) {
  balances[accounts[i].publicKey] = accounts[i].balance;
}

app.get("/balance/:address", (req: Request, res: Response) => {
  const { address } = req.params;
  const balance = balances[address] || 0;
  res.send({ balance });
});

interface SendRequestBody {
  signature: {
    compactRS: string;
    recovery: number;
  };
  msg: string;
}

app.post("/send", (req: Request<{}, {}, SendRequestBody>, res: Response) => {
  const { signature, msg } = req.body;
  const { recipient, amount } = JSON.parse(msg) as {
    recipient: string;
    amount: number;
  };
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

function setInitialBalance(address: string): void {
  if (!balances[address]) {
    balances[address] = 0;
  }
}
