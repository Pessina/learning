import { accounts } from "./constants/accounts.json";
import express, { Request, Response } from "express";
import cors from "cors";
import { recoverAddressFromSignMessage } from "./crypto";

const app = express();
const port = 3042;

app.use(cors());
app.use(express.json());

interface Balances {
  [key: string]: number;
}

const balances: Balances = {};

app.get("/balance/:address", (req: Request, res: Response) => {
  const { address } = req.params;
  const balance = getBalance(address) ?? 0;

  res.send({ balance });
});

interface SendRequestBody {
  signature: string;
  data: {
    amount: number;
    recipient: string;
  };
}

app.post("/send", (req: Request<{}, {}, SendRequestBody>, res: Response) => {
  const { signature, data } = req.body;
  const { amount, recipient } = data;

  const sender = recoverAddressFromSignMessage(signature, data);

  if (balances[sender] < amount) {
    res.status(400).send({ message: "Not enough funds!" });
  } else {
    setBalance(sender, (getBalance(sender) ?? 0) - amount);
    setBalance(recipient, (getBalance(recipient) ?? 0) + amount);
    res.send({ balance: getBalance(sender) ?? 0 });
  }
});

interface CreateAccountRequestBody {
  signature: string;
  data: {
    balance: number;
  };
}

app.post(
  "/create-account",
  (req: Request<{}, {}, CreateAccountRequestBody>, res: Response) => {
    const { signature, data } = req.body;
    const address = recoverAddressFromSignMessage(signature, data);

    if (getBalance(address) !== undefined) {
      return res.status(400).send({ message: "Account already exists" });
    }

    setBalance(address, data.balance);
    return res.send({ address });
  }
);

app.listen(port, () => {
  console.log(`Listening on port ${port}!`);
});

const setBalance = (address: string, balance: number): void => {
  balances[address.toLowerCase()] = balance;
};

const getBalance = (address: string): number | undefined => {
  return balances[address.toLowerCase()];
};
