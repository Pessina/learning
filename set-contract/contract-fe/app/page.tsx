"use client";

import { useState } from "react";
import { ethers } from "ethers";
import {
  connectToProvider,
  createContractInstance,
  callContractWithDataField,
  viewCallerDataWithDataField,
} from "../utils/contract";

export default function Home() {
  const [key, setKey] = useState("");
  const [value, setValue] = useState("");
  const [result, setResult] = useState("");

  const handleInteract = async () => {
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const signer = await provider.getSigner();

      await callContractWithDataField(signer, "setCallerData(string,string)", [
        key,
        value,
      ]);

      setResult("Interaction successful. Check console for details.");
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleViewData = async () => {
    try {
      if (!window.ethereum) {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      await viewCallerDataWithDataField(provider, key);
      setResult("View operation successful. Check console for details.");
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  return (
    <div className="container mx-auto p-4 bg-gray-100">
      <h1 className="text-2xl font-bold mb-4 text-gray-800">
        Caller Registry Interaction
      </h1>
      <div className="mb-4">
        <input
          type="text"
          placeholder="Key"
          value={key}
          onChange={(e) => setKey(e.target.value)}
          className="border p-2 mr-2 text-gray-800"
        />
        <input
          type="text"
          placeholder="Value"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          className="border p-2 mr-2 text-gray-800"
        />
        <button
          onClick={handleInteract}
          className="bg-blue-600 text-white p-2 rounded hover:bg-blue-700 mr-2"
        >
          Interact with Contract
        </button>
        <button
          onClick={handleViewData}
          className="bg-green-600 text-white p-2 rounded hover:bg-green-700"
        >
          View Data
        </button>
      </div>
      {result && (
        <div className="mt-4 p-2 bg-white rounded text-gray-800">{result}</div>
      )}
    </div>
  );
}
