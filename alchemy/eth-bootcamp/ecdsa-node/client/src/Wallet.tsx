import React, { ChangeEvent } from "react";
import server from "./server";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import { toHex } from "ethereum-cryptography/utils";

interface WalletProps {
  address: string;
  setAddress: (address: string) => void;
  balance: number;
  setBalance: (balance: number) => void;
  privateKey: string;
  setPrivateKey: (privateKey: string) => void;
}

const Wallet: React.FC<WalletProps> = ({
  address,
  setAddress,
  balance,
  setBalance,
  privateKey,
  setPrivateKey,
}) => {
  async function onChange(evt: ChangeEvent<HTMLInputElement>) {
    const privateKey = evt.target.value;

    if (privateKey.length < 64) {
      setPrivateKey(privateKey);
      return;
    }

    setPrivateKey(privateKey);
    const publicKey = toHex(secp256k1.getPublicKey(privateKey));

    setAddress(publicKey);
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
          value={privateKey}
          onChange={onChange}
        />
      </label>

      <div className="balance">Balance: {balance}</div>
    </div>
  );
};

export default Wallet;
