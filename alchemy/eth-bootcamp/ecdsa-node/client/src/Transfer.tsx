import { Buffer } from "buffer";
import { sha256 } from "ethereum-cryptography/sha256";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import React, { useState, ChangeEvent, FormEvent } from "react";
import server from "./server";

interface TransferProps {
  address: string;
  setBalance: (balance: number) => void;
  privateKey: string;
}

function Transfer({ address, setBalance, privateKey }: TransferProps) {
  const [sendAmount, setSendAmount] = useState<string>("");
  const [recipient, setRecipient] = useState<string>("");

  const setValue =
    (setter: React.Dispatch<React.SetStateAction<string>>) =>
    (evt: ChangeEvent<HTMLInputElement>) =>
      setter(evt.target.value);

  async function transfer(evt: FormEvent<HTMLFormElement>) {
    evt.preventDefault();

    const msg = JSON.stringify({
      amount: parseInt(sendAmount),
      recipient,
    });

    const msgBytes = Buffer.from(msg);
    const msgHashed = sha256(msgBytes);
    const msgHashedHex = Buffer.from(msgHashed).toString("hex");

    const signature = secp256k1.sign(msgHashedHex, privateKey);

    try {
      const {
        data: { balance },
      } = await server.post<{ balance: number }>(`send`, {
        signature: {
          compactRS: signature.toCompactHex(),
          recovery: signature.recovery,
        },
        msg,
      });
      setBalance(balance);
    } catch (ex) {
      if (ex instanceof Error) {
        alert(ex.message);
      } else {
        alert("An unknown error occurred");
      }
    }
  }

  return (
    <form className="container transfer" onSubmit={transfer}>
      <h1>Send Transaction</h1>

      <label>
        Send Amount
        <input
          placeholder="1, 2, 3..."
          value={sendAmount}
          onChange={setValue(setSendAmount)}
        />
      </label>

      <label>
        Recipient
        <input
          placeholder="Type an address, for example: 0x2"
          value={recipient}
          onChange={setValue(setRecipient)}
        />
      </label>

      <input type="submit" className="button" value="Transfer" />
    </form>
  );
}

export default Transfer;
