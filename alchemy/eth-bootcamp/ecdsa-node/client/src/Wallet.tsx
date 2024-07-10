import React, { ChangeEvent, useState } from "react";
import server from "./server";

interface WalletProps {
  balance: number;
  setBalance: (balance: number) => void;
}

const Wallet: React.FC<WalletProps> = ({ balance, setBalance }) => {
  const [publicKey, setPublicKey] = useState("");

  async function onChange(evt: ChangeEvent<HTMLInputElement>) {
    const publicKey = evt.target.value;

    if (publicKey) {
      const {
        data: { balance },
      } = await server.get<{ balance: number }>(`balance/${publicKey}`);
      setBalance(balance);
    } else {
      setBalance(0);
    }
  }

  return (
    <div className="container wallet">
      <h1>Your Wallet</h1>

      <label>
        Wallet Address
        <input
          placeholder="Type an address, for example: 0x1"
          value={publicKey}
          onChange={onChange}
        />
      </label>

      <div className="balance">Balance: {balance}</div>
    </div>
  );
};

export default Wallet;
