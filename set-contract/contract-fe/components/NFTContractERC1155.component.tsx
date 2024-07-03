import { useState } from "react";
import { ethers } from "ethers";
import { NFTContractERC1155 } from "../contracts/NFTContractERC1155";

export const NFTContractERC1155Component = () => {
  const [inputs, setInputs] = useState({
    address: "",
    id: "",
    fromAddress: "",
    toAddress: "",
    amount: "",
    data: "",
    operator: "",
    approved: false,
    ids: "",
    amounts: "",
    uri: "",
  });
  const [result, setResult] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setInputs((prev) => ({
      ...prev,
      [name]: type === "checkbox" ? checked : value,
    }));
  };

  const handleAction = async (
    action: (contract: NFTContractERC1155) => Promise<void>
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
      const contract = new NFTContractERC1155(provider);

      await action(contract);
    } catch (error: any) {
      setResult(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const renderInputField = (
    label: string,
    name: string,
    type: string = "text"
  ) => (
    <div className="mb-4">
      <label className="block text-gray-700 text-sm font-bold mb-2">
        {label}
      </label>
      <input
        type={type}
        name={name}
        value={
          type === "checkbox"
            ? undefined
            : (inputs[name as keyof typeof inputs] as string)
        }
        onChange={handleInputChange}
        checked={
          type === "checkbox"
            ? (inputs[name as keyof typeof inputs] as boolean)
            : undefined
        }
        className="w-full border p-3 rounded-md text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
  );

  const renderActionButton = (
    label: string,
    action: (contract: NFTContractERC1155) => Promise<void>,
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
        ERC1155 NFT Contract Interaction
      </h1>
      <div className="grid grid-cols-2 gap-4 mb-6">
        {renderInputField("Address", "address")}
        {renderInputField("Token ID", "id")}
        {renderInputField("From Address", "fromAddress")}
        {renderInputField("To Address", "toAddress")}
        {renderInputField("Amount", "amount")}
        {renderInputField("Data", "data")}
        {renderInputField("Operator", "operator")}
        {renderInputField("IDs (comma-separated)", "ids")}
        {renderInputField("Amounts (comma-separated)", "amounts")}
        {renderInputField("Approved", "approved", "checkbox")}
        {renderInputField("URI", "uri")}
      </div>
      <div className="grid grid-cols-3 gap-4 mb-6">
        {renderActionButton(
          "Safe Transfer From",
          async (contract) => {
            await contract.safeTransferFrom(
              inputs.fromAddress,
              inputs.toAddress,
              Number(inputs.id),
              Number(inputs.amount),
              inputs.data
            );
            setResult(
              `${inputs.amount} of token ${inputs.id} safely transferred from ${inputs.fromAddress} to ${inputs.toAddress}`
            );
          },
          ["fromAddress", "toAddress", "id", "amount"]
        )}
        {renderActionButton(
          "Safe Batch Transfer From",
          async (contract) => {
            const ids = inputs.ids.split(",").map(Number);
            const amounts = inputs.amounts.split(",").map(Number);
            await contract.safeBatchTransferFrom(
              inputs.fromAddress,
              inputs.toAddress,
              ids,
              amounts,
              inputs.data
            );
            setResult(
              `Batch transfer of tokens from ${inputs.fromAddress} to ${inputs.toAddress} completed`
            );
          },
          ["fromAddress", "toAddress", "ids", "amounts"]
        )}
        {renderActionButton(
          "Set Approval For All",
          async (contract) => {
            await contract.setApprovalForAll(inputs.operator, inputs.approved);
            setResult(
              `Set approval for all: operator ${inputs.operator}, approved ${inputs.approved}`
            );
          },
          ["operator"]
        )}
        {renderActionButton(
          "Balance Of",
          async (contract) => {
            const balance = await contract.balanceOf(
              inputs.address,
              Number(inputs.id)
            );
            setResult(
              `Balance of ${inputs.address} for token ${inputs.id}: ${balance}`
            );
          },
          ["address", "id"]
        )}
        {renderActionButton(
          "Balance Of Batch",
          async (contract) => {
            const addresses = inputs.address.split(",");
            const ids = inputs.ids.split(",").map(Number);
            const balances = await contract.balanceOfBatch(addresses, ids);
            setResult(`Batch balances: ${balances.join(", ")}`);
          },
          ["address", "ids"]
        )}
        {renderActionButton(
          "URI",
          async (contract) => {
            const uri = await contract.uri(Number(inputs.id));
            setResult(`URI for token ${inputs.id}: ${uri}`);
          },
          ["id"]
        )}
        {renderActionButton(
          "Mint",
          async (contract) => {
            await contract.mint(
              inputs.toAddress,
              Number(inputs.id),
              Number(inputs.amount),
              inputs.data
            );
            setResult(
              `${inputs.amount} tokens of id ${inputs.id} minted to ${inputs.toAddress}`
            );
          },
          ["toAddress", "id", "amount"]
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
