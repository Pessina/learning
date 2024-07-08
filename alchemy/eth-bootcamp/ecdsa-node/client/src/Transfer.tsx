import React, { useState, ChangeEvent, FormEvent } from "react";
import server from "./server";
import { walletSign } from "./crypto";

const Transfer: React.FC = () => {
  const [sendAmount, setSendAmount] = useState<string>("");
  const [recipient, setRecipient] = useState<string>("");

  const setValue =
    (setter: React.Dispatch<React.SetStateAction<string>>) =>
    (evt: ChangeEvent<HTMLInputElement>) =>
      setter(evt.target.value);

  async function transfer(evt: FormEvent<HTMLFormElement>) {
    evt.preventDefault();

    if (!window.ethereum) {
      throw new Error("MetaMask is required to transfer");
    }

    const data = {
      amount: parseInt(sendAmount),
      recipient,
    };

    const signature = await walletSign(data, window.ethereum);

    try {
      const {
        data: { balance },
      } = await server.post<{ balance: number }>(`send`, {
        signature,
        data,
      });
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
};

export default Transfer;
