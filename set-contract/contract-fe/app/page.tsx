"use client";

import { useState, useEffect } from "react";
import { ethers } from "ethers";
import { SetContractComponent } from "../components/SetContract.component";
import { FTContractComponent } from "../components/FTContract.component";
import { NFTContractComponent } from "../components/NFTContract.component";

export default function Home() {
  const [provider, setProvider] = useState<ethers.BrowserProvider | null>(null);

  // Initialize provider when the component mounts
  useEffect(() => {
    const initializeProvider = async () => {
      if (typeof window.ethereum !== "undefined") {
        const provider = new ethers.BrowserProvider(window.ethereum);
        setProvider(provider);
      } else {
        console.error("MetaMask is not installed");
      }
    };

    initializeProvider();
  }, []);
  // Add a method to switch accounts
  const switchAccount = async (): Promise<void> => {
    if (!provider) {
      console.error("Provider not initialized");
      return;
    }

    try {
      // Request the user to choose an account
      await window.ethereum.request({
        method: "wallet_requestPermissions",
        params: [{ eth_accounts: {} }],
      });

      // Get the selected account
      const accounts = await window.ethereum.request({
        method: "eth_requestAccounts",
      });

      if (accounts.length > 0) {
        const selectedAccount = accounts[0];
        console.log("Switched to account:", selectedAccount);
        // You might want to update the UI or perform other actions with the new account
      } else {
        console.log("No account selected");
      }
    } catch (error) {
      console.error("Error switching account:", error);
    }
  };
  return (
    <div className="min-h-screen bg-gradient-to-r from-blue-100 to-purple-100">
      <div className="container mx-auto p-8">
        <h1 className="text-4xl font-bold mb-8 text-center text-gray-800 border-b-2 border-gray-300 pb-4">
          Smart Contract Interaction
        </h1>
        <button
          onClick={switchAccount}
          className="mb-4 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
        >
          Switch Account
        </button>
        <div className="flex flex-col gap-4">
          <SetContractComponent />
          <FTContractComponent />
          <NFTContractComponent />
        </div>
      </div>
    </div>
  );
}
