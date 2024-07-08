import { Buffer } from "buffer";
import { sha256 } from "ethereum-cryptography/sha256";
import { secp256k1 } from "ethereum-cryptography/secp256k1";
import { useState } from "react";
import server from "./server";

function Transfer({ address, setBalance, privateKey }) {
  const [sendAmount, setSendAmount] = useState("");
  const [recipient, setRecipient] = useState("");

  const setValue = (setter) => (evt) => setter(evt.target.value);

  async function transfer(evt) {
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
      } = await server.post(`send`, {
        signature: {
          compactRS: signature.toCompactHex(),
          recovery: signature.recovery,
        },
        msg,
      });
      setBalance(balance);
    } catch (ex) {
      alert(ex.response.data.message);
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
        ></input>
      </label>

      <label>
        Recipient
        <input
          placeholder="Type an address, for example: 0x2"
          value={recipient}
          onChange={setValue(setRecipient)}
        ></input>
      </label>

      <input type="submit" className="button" value="Transfer" />
    </form>
  );
}

export default Transfer;
