import { useState } from "react";
import { ethers } from "ethers";
import { USDTContract } from "../contracts/USDTContract";

export const USDTContractComponent = () => {
  const [address, setAddress] = useState("");
  const [amount, setAmount] = useState("");
  const [result, setResult] = useState("");

  const handleGetBalance = async () => {
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new USDTContract(provider);

      const balance = await contract.getBalance(address);
      setResult(`Balance: ${balance} USDT`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleTransfer = async () => {
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new USDTContract(provider);

      await contract.transfer(address, amount);
      setResult("Transfer successful. Check console for details.");
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleMint = async () => {
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new USDTContract(provider);

      await contract.mint(address, amount);
      setResult(`Successfully minted ${amount} USDT to ${address}`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  return (
    <div className="container mx-auto p-4 bg-gray-100">
      <h1 className="text-2xl font-bold mb-4 text-gray-800">
        USDT Contract Interaction
      </h1>
      <div className="mb-4">
        <input
          type="text"
          placeholder="Address"
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          className="border p-2 mr-2 text-gray-800"
        />
        <input
          type="text"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          className="border p-2 mr-2 text-gray-800"
        />
        <button
          onClick={handleGetBalance}
          className="bg-blue-600 text-white p-2 rounded hover:bg-blue-700 mr-2"
        >
          Get Balance
        </button>
        <button
          onClick={handleTransfer}
          className="bg-green-600 text-white p-2 rounded hover:bg-green-700 mr-2"
        >
          Transfer
        </button>
        <button
          onClick={handleMint}
          className="bg-purple-600 text-white p-2 rounded hover:bg-purple-700"
        >
          Mint
        </button>
      </div>
      {result && (
        <div className="mt-4 p-2 bg-white rounded text-gray-800">{result}</div>
      )}
    </div>
  );
};