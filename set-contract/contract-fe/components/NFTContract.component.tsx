import { useState } from "react";
import { ethers } from "ethers";
import { NFTContract } from "../contracts/NFTContract";

export const NFTContractComponent = () => {
  const [address, setAddress] = useState("");
  const [tokenId, setTokenId] = useState("");
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
      const contract = new NFTContract(provider);

      const balance = await contract.balanceOf(address);
      setResult(`Balance: ${balance} BAYC tokens`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleGetOwner = async () => {
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new NFTContract(provider);

      const owner = await contract.ownerOf(Number(tokenId));
      setResult(`Owner of token ${tokenId}: ${owner}`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  const handleGetTokenURI = async () => {
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new NFTContract(provider);

      const tokenURI = await contract.tokenURI(Number(tokenId));
      setResult(`Token URI for ${tokenId}: ${tokenURI}`);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    }
  };

  return (
    <div className="container mx-auto p-4 bg-gray-100">
      <h1 className="text-2xl font-bold mb-4 text-gray-800">
        NFT Contract Interaction
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
          placeholder="Token ID"
          value={tokenId}
          onChange={(e) => setTokenId(e.target.value)}
          className="border p-2 mr-2 text-gray-800"
        />
        <button
          onClick={handleGetBalance}
          className="bg-blue-600 text-white p-2 rounded hover:bg-blue-700 mr-2"
        >
          Get Balance
        </button>
        <button
          onClick={handleGetOwner}
          className="bg-green-600 text-white p-2 rounded hover:bg-green-700 mr-2"
        >
          Get Owner
        </button>
        <button
          onClick={handleGetTokenURI}
          className="bg-purple-600 text-white p-2 rounded hover:bg-purple-700"
        >
          Get Token URI
        </button>
      </div>
      {result && (
        <div className="mt-4 p-2 bg-white rounded text-gray-800">{result}</div>
      )}
    </div>
  );
};
