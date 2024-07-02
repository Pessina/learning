import { useState } from "react";
import { ethers } from "ethers";
import { FTContract } from "../contracts/FTContract";

export const FTContractComponent = () => {
  const [inputs, setInputs] = useState({
    address: "",
    amount: "",
    spender: "",
    from: "",
    to: "",
  });
  const [result, setResult] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setInputs((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleAction = async (
    action: (contract: FTContract) => Promise<void>
  ) => {
    setIsLoading(true);
    try {
      if (typeof window.ethereum === "undefined") {
        throw new Error("MetaMask is not installed");
      }

      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(
        window.ethereum as ethers.Eip1193Provider
      );
      const contract = new FTContract(provider);

      await action(contract);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const renderInputField = (label: string, name: string) => (
    <div className="mb-4">
      <label className="block text-gray-700 text-sm font-bold mb-2">
        {label}
      </label>
      <input
        type="text"
        name={name}
        value={inputs[name as keyof typeof inputs]}
        onChange={handleInputChange}
        className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
  );

  const renderActionButton = (
    label: string,
    action: (contract: FTContract) => Promise<void>,
    requiredFields: string[]
  ) => (
    <button
      onClick={() => handleAction(action)}
      disabled={
        isLoading ||
        requiredFields.some((field) => !inputs[field as keyof typeof inputs])
      }
      className="bg-blue-600 text-white p-3 rounded-md hover:bg-blue-700 transition duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      {label}
    </button>
  );

  return (
    <div className="container mx-auto p-6 bg-gray-100 rounded-lg shadow-lg">
      <h1 className="text-3xl font-bold mb-6 text-gray-800">
        FT Contract Interaction
      </h1>
      <div className="grid grid-cols-2 gap-4 mb-6">
        {renderInputField("Address", "address")}
        {renderInputField("Amount", "amount")}
        {renderInputField("Spender", "spender")}
        {renderInputField("From", "from")}
        {renderInputField("To", "to")}
      </div>
      <div className="grid grid-cols-3 gap-4 mb-6">
        {renderActionButton(
          "Get Balance",
          async (contract) => {
            const balance = await contract.getBalance(inputs.address);
            setResult(`Balance: ${balance} USDT`);
          },
          ["address"]
        )}
        {renderActionButton(
          "Transfer",
          async (contract) => {
            await contract.transfer(inputs.to, inputs.amount);
            setResult("Transfer successful. Check console for details.");
          },
          ["to", "amount"]
        )}
        {renderActionButton(
          "Mint",
          async (contract) => {
            await contract.mint(inputs.address, inputs.amount);
            setResult(
              `Successfully minted ${inputs.amount} USDT to ${inputs.address}`
            );
          },
          ["address", "amount"]
        )}
        {renderActionButton(
          "Approve",
          async (contract) => {
            await contract.approve(inputs.spender, inputs.amount);
            setResult("Approval successful. Check console for details.");
          },
          ["spender", "amount"]
        )}
        {renderActionButton(
          "Transfer From",
          async (contract) => {
            await contract.transferFrom(inputs.from, inputs.to, inputs.amount);
            setResult("TransferFrom successful. Check console for details.");
          },
          ["from", "to", "amount"]
        )}
        {renderActionButton(
          "Total Supply",
          async (contract) => {
            const supply = await contract.totalSupply();
            setResult(`Total Supply: ${supply} USDT`);
          },
          []
        )}
        {renderActionButton(
          "Allowance",
          async (contract) => {
            const allowanceAmount = await contract.allowance(
              inputs.from,
              inputs.spender
            );
            setResult(`Allowance: ${allowanceAmount} USDT`);
          },
          ["from", "spender"]
        )}
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
