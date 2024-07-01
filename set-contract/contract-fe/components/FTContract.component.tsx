import { useState } from "react";
import { ethers } from "ethers";
import { FTContract } from "../contracts/FTContract";

export const FTContractComponent = () => {
  const [address, setAddress] = useState("");
  const [amount, setAmount] = useState("");
  const [spender, setSpender] = useState("");
  const [from, setFrom] = useState("");
  const [to, setTo] = useState("");
  const [result, setResult] = useState("");

  const initializeContract = async () => {
    if (typeof window.ethereum === "undefined") {
      throw new Error("MetaMask is not installed");
    }
    await window.ethereum.request({ method: "eth_requestAccounts" });
    const provider = new ethers.BrowserProvider(
      window.ethereum as ethers.Eip1193Provider
    );
    return new FTContract(provider);
  };

  const handleGetBalance = async () => {
    try {
      const contract = await initializeContract();
      const balance = await contract.getBalance(address);
      setResult(`Balance: ${balance} USDT`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleTransfer = async () => {
    try {
      const contract = await initializeContract();
      await contract.transfer(to, amount);
      setResult("Transfer successful. Check console for details.");
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleMint = async () => {
    try {
      const contract = await initializeContract();
      await contract.mint(address, amount);
      setResult(`Successfully minted ${amount} USDT to ${address}`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleApprove = async () => {
    try {
      const contract = await initializeContract();
      await contract.approve(spender, amount);
      setResult("Approval successful. Check console for details.");
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleTransferFrom = async () => {
    try {
      const contract = await initializeContract();
      await contract.transferFrom(from, to, amount);
      setResult("TransferFrom successful. Check console for details.");
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleTotalSupply = async () => {
    try {
      const contract = await initializeContract();
      const supply = await contract.totalSupply();
      setResult(`Total Supply: ${supply} USDT`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleAllowance = async () => {
    try {
      const contract = await initializeContract();
      const allowanceAmount = await contract.allowance(from, spender);
      setResult(`Allowance: ${allowanceAmount} USDT`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  return (
    <div className="container mx-auto p-8 bg-gray-100 rounded-lg shadow-md">
      <h1 className="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">
        FT Contract Interaction
      </h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
        <input
          type="text"
          placeholder="Address"
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          className="border rounded-md p-2 w-full text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          className="border rounded-md p-2 w-full text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Spender"
          value={spender}
          onChange={(e) => setSpender(e.target.value)}
          className="border rounded-md p-2 w-full text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="From"
          value={from}
          onChange={(e) => setFrom(e.target.value)}
          className="border rounded-md p-2 w-full text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="To"
          value={to}
          onChange={(e) => setTo(e.target.value)}
          className="border rounded-md p-2 w-full text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div className="flex flex-wrap gap-4 mb-6">
        <button
          onClick={handleGetBalance}
          className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
        >
          Get Balance
        </button>
        <button
          onClick={handleTransfer}
          className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50"
        >
          Transfer
        </button>
        <button
          onClick={handleMint}
          className="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50"
        >
          Mint
        </button>
        <button
          onClick={handleApprove}
          className="bg-yellow-600 text-white px-4 py-2 rounded-md hover:bg-yellow-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-opacity-50"
        >
          Approve
        </button>
        <button
          onClick={handleTransferFrom}
          className="bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50"
        >
          Transfer From
        </button>
        <button
          onClick={handleTotalSupply}
          className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-50"
        >
          Total Supply
        </button>
        <button
          onClick={handleAllowance}
          className="bg-pink-600 text-white px-4 py-2 rounded-md hover:bg-pink-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-pink-500 focus:ring-opacity-50"
        >
          Allowance
        </button>
      </div>
      {result && (
        <div className="mt-6 p-4 bg-white rounded-md shadow-sm text-gray-800 border-l-4 border-blue-500">
          {result}
        </div>
      )}
    </div>
  );
};
