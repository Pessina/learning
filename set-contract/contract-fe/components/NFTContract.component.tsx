import { useState } from "react";
import { ethers } from "ethers";
import { NFTContract } from "../contracts/NFTContract";

export const NFTContractComponent = () => {
  const [address, setAddress] = useState("");
  const [tokenId, setTokenId] = useState("");
  const [result, setResult] = useState("");
  const [uri, setUri] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [fromAddress, setFromAddress] = useState("");
  const [toAddress, setToAddress] = useState("");
  const [amount, setAmount] = useState("");
  const [data, setData] = useState("");
  const [operator, setOperator] = useState("");
  const [approved, setApproved] = useState(false);
  const [ids, setIds] = useState("");
  const [amounts, setAmounts] = useState("");

  const handleAction = async (action: () => Promise<void>) => {
    setIsLoading(true);
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new NFTContract(provider);

      await action();
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleGetBalance = () =>
    handleAction(async () => {
      const balance = await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).balanceOf(address, tokenId ? Number(tokenId) : undefined);
      setResult(`Balance: ${balance} tokens`);
    });

  const handleBalanceOfBatch = () =>
    handleAction(async () => {
      const accounts = address.split(",");
      const tokenIds = ids.split(",").map(Number);
      const balances = await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).balanceOfBatch(accounts, tokenIds);
      setResult(`Batch Balances: ${balances.join(", ")}`);
    });

  const handleGetOwner = () =>
    handleAction(async () => {
      const owner = await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).ownerOf(Number(tokenId));
      setResult(`Owner of token ${tokenId}: ${owner}`);
    });

  const handleGetTokenURI = () =>
    handleAction(async () => {
      const tokenURI = await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).tokenURI(Number(tokenId));
      setResult(`Token URI for ${tokenId}: ${tokenURI}`);
    });

  const handleSafeTransferFrom = () =>
    handleAction(async () => {
      await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).safeTransferFrom(
        fromAddress,
        toAddress,
        Number(tokenId),
        amount ? Number(amount) : undefined,
        data
      );
      setResult(
        `Token ${tokenId} safely transferred from ${fromAddress} to ${toAddress}`
      );
    });

  const handleTransferFrom = () =>
    handleAction(async () => {
      await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).transferFrom(fromAddress, toAddress, Number(tokenId));
      setResult(
        `Token ${tokenId} transferred from ${fromAddress} to ${toAddress}`
      );
    });

  const handleApprove = () =>
    handleAction(async () => {
      await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).approve(toAddress, Number(tokenId));
      setResult(`Approved ${toAddress} to manage token ${tokenId}`);
    });

  const handleSetApprovalForAll = () =>
    handleAction(async () => {
      await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).setApprovalForAll(operator, approved);
      setResult(
        `Set approval for all: operator ${operator}, approved ${approved}`
      );
    });

  const handleGetApproved = () =>
    handleAction(async () => {
      const approvedAddress = await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).getApproved(Number(tokenId));
      setResult(`Approved address for token ${tokenId}: ${approvedAddress}`);
    });

  const handleIsApprovedForAll = () =>
    handleAction(async () => {
      const isApproved = await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).isApprovedForAll(address, operator);
      setResult(
        `Is ${operator} approved for all of ${address}'s tokens: ${isApproved}`
      );
    });

  const handleSafeBatchTransferFrom = () =>
    handleAction(async () => {
      const tokenIds = ids.split(",").map(Number);
      const tokenAmounts = amounts.split(",").map(Number);
      await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).safeBatchTransferFrom(
        fromAddress,
        toAddress,
        tokenIds,
        tokenAmounts,
        data
      );
      setResult(`Batch transfer from ${fromAddress} to ${toAddress} completed`);
    });

  const handleMint = () =>
    handleAction(async () => {
      await new NFTContract(
        new ethers.BrowserProvider(window.ethereum as ethers.Eip1193Provider)
      ).mint(toAddress, Number(tokenId), uri);
      setResult(`Token ${tokenId} minted to ${toAddress} with URI ${uri}`);
    });

  return (
    <div className="container mx-auto p-6 bg-gray-100 rounded-lg shadow-lg">
      <h1 className="text-3xl font-bold mb-6 text-gray-800">
        NFT Contract Interaction
      </h1>
      <div className="mb-6 space-y-4">
        <input
          type="text"
          placeholder="Address"
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Token ID"
          value={tokenId}
          onChange={(e) => setTokenId(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="URI"
          value={uri}
          onChange={(e) => setUri(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="From Address"
          value={fromAddress}
          onChange={(e) => setFromAddress(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="To Address"
          value={toAddress}
          onChange={(e) => setToAddress(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Data"
          value={data}
          onChange={(e) => setData(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Operator"
          value={operator}
          onChange={(e) => setOperator(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <div className="flex items-center">
          <input
            type="checkbox"
            checked={approved}
            onChange={(e) => setApproved(e.target.checked)}
            className="mr-2 h-5 w-5 text-blue-600"
          />
          <label className="text-gray-700">Approved</label>
        </div>
        <input
          type="text"
          placeholder="IDs (comma-separated)"
          value={ids}
          onChange={(e) => setIds(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Amounts (comma-separated)"
          value={amounts}
          onChange={(e) => setAmounts(e.target.value)}
          className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div className="grid grid-cols-3 gap-4 mb-6">
        <button
          onClick={handleGetBalance}
          disabled={isLoading}
          className="bg-blue-600 text-white p-3 rounded-md hover:bg-blue-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          Get Balance
        </button>
        <button
          onClick={handleBalanceOfBatch}
          disabled={isLoading}
          className="bg-blue-600 text-white p-3 rounded-md hover:bg-blue-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          Balance Of Batch
        </button>
        <button
          onClick={handleGetOwner}
          disabled={isLoading}
          className="bg-green-600 text-white p-3 rounded-md hover:bg-green-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-green-500"
        >
          Get Owner
        </button>
        <button
          onClick={handleGetTokenURI}
          disabled={isLoading}
          className="bg-purple-600 text-white p-3 rounded-md hover:bg-purple-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-purple-500"
        >
          Get Token URI
        </button>
        <button
          onClick={handleSafeTransferFrom}
          disabled={isLoading}
          className="bg-yellow-600 text-white p-3 rounded-md hover:bg-yellow-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-yellow-500"
        >
          Safe Transfer From
        </button>
        <button
          onClick={handleTransferFrom}
          disabled={isLoading}
          className="bg-red-600 text-white p-3 rounded-md hover:bg-red-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-red-500"
        >
          Transfer From
        </button>
        <button
          onClick={handleApprove}
          disabled={isLoading}
          className="bg-indigo-600 text-white p-3 rounded-md hover:bg-indigo-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500"
        >
          Approve
        </button>
        <button
          onClick={handleSetApprovalForAll}
          disabled={isLoading}
          className="bg-pink-600 text-white p-3 rounded-md hover:bg-pink-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-pink-500"
        >
          Set Approval For All
        </button>
        <button
          onClick={handleGetApproved}
          disabled={isLoading}
          className="bg-teal-600 text-white p-3 rounded-md hover:bg-teal-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-teal-500"
        >
          Get Approved
        </button>
        <button
          onClick={handleIsApprovedForAll}
          disabled={isLoading}
          className="bg-gray-600 text-white p-3 rounded-md hover:bg-gray-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          Is Approved For All
        </button>
        <button
          onClick={handleSafeBatchTransferFrom}
          disabled={isLoading}
          className="bg-orange-600 text-white p-3 rounded-md hover:bg-orange-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-orange-500"
        >
          Safe Batch Transfer From
        </button>
        <button
          onClick={handleMint}
          disabled={isLoading}
          className="bg-green-600 text-white p-3 rounded-md hover:bg-green-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-green-500"
        >
          Mint
        </button>
      </div>
      {isLoading && <div className="text-center text-gray-600">Loading...</div>}
      {result && (
        <div className="mt-6 p-4 bg-white rounded-md shadow text-gray-800 break-words">
          {result}
        </div>
      )}
    </div>
  );
};
