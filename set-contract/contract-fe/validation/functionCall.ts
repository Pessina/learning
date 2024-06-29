import { ethers } from "ethers";

// ERC20 Interface
const erc20Interface = new ethers.Interface([
  "function transfer(address, uint256) returns (bool)",
  "function approve(address, uint256) returns (bool)",
  "function transferFrom(address, address, uint256) returns (bool)",
]);

// ERC721 Interface
const erc721Interface = new ethers.Interface([
  "function safeTransferFrom(address, address, uint256, bytes) payable",
  "function safeTransferFrom(address, address, uint256) payable",
  "function transferFrom(address, address, uint256) payable",
  "function approve(address, uint256) payable",
  "function setApprovalForAll(address, bool)",
]);

// ERC1155 Interface
const erc1155Interface = new ethers.Interface([
  "function safeTransferFrom(address, address, uint256, uint256, bytes)",
  "function safeBatchTransferFrom(address, address, uint256[], uint256[], bytes)",
  "function setApprovalForAll(address, bool)",
]);

// ERC777 Interface
const erc777Interface = new ethers.Interface([
  "function authorizeOperator(address)",
  "function revokeOperator(address)",
  "function send(address,uint256,bytes)",
  "function operatorSend(address,address,uint256,bytes,bytes)",
  "function burn(uint256,bytes)",
  "function operatorBurn(address,uint256,bytes,bytes)",
]);

export function getUserFriendlyDescription(tx: {
  data: string;
  to: string;
  value?: string;
}): string {
  if (tx.data === "0x" && tx.value) {
    return `You are sending ${ethers.formatEther(tx.value)} ETH to ${tx.to}`;
  }

  if (tx.to === null || tx.to === undefined || tx.to === "") {
    return "You are deploying a new contract. Please verify the contract code carefully.";
  }

  const interfaces = [
    { name: "ERC20", interface: erc20Interface },
    { name: "ERC721", interface: erc721Interface },
    { name: "ERC1155", interface: erc1155Interface },
    { name: "ERC777", interface: erc777Interface },
  ];

  for (const { name, interface: iface } of interfaces) {
    try {
      const decoded = iface.parseTransaction({ data: tx.data });
      if (decoded) {
        switch (`${name}:${decoded.name}`) {
          case "ERC20:transfer":
            return `You are transferring ${ethers.formatUnits(
              decoded.args[1],
              6
            )} tokens to ${
              decoded.args[0]
            }. If the recipient address is incorrect, your tokens could be lost.`;
          case "ERC20:approve":
            return `You are approving ${
              decoded.args[0]
            } to spend ${ethers.formatUnits(
              decoded.args[1],
              6
            )} of your tokens. If this address is compromised or malicious, they can transfer your tokens without further consent.`;
          case "ERC20:transferFrom":
            return `You are allowing the transfer of ${ethers.formatUnits(
              decoded.args[2],
              6
            )} tokens from ${decoded.args[0]} to ${
              decoded.args[1]
            }. Ensure you trust the contract initiating this transfer.`;
          case "ERC721:safeTransferFrom":
            if (decoded.args.length === 3) {
              return `You are safely transferring NFT #${decoded.args[2]} from ${decoded.args[0]} to ${decoded.args[1]}. If the recipient address is incorrect, your NFT could be permanently lost.`;
            } else {
              return `You are safely transferring NFT #${decoded.args[2]} from ${decoded.args[0]} to ${decoded.args[1]} with additional data. If the recipient address is incorrect, your NFT could be permanently lost.`;
            }
          case "ERC721:transferFrom":
            return `You are transferring NFT #${decoded.args[2]} from ${decoded.args[0]} to ${decoded.args[1]}. If the recipient address is incorrect, your NFT could be permanently lost.`;
          case "ERC721:approve":
            return `You are approving ${decoded.args[0]} to transfer your NFT #${decoded.args[1]}. This address will be able to transfer this specific NFT on your behalf.`;
          case "ERC721:setApprovalForAll":
            return `You are ${decoded.args[1] ? "approving" : "revoking"} ${
              decoded.args[0]
            } to manage ALL your NFTs. If approved, this address will have full control over all your NFTs in this collection.`;
          case "ERC1155:safeTransferFrom":
            return `You are transferring ${decoded.args[3]} of token ID ${decoded.args[2]} from ${decoded.args[0]} to ${decoded.args[1]}. If the recipient address is incorrect, your tokens could be permanently lost.`;
          case "ERC1155:safeBatchTransferFrom":
            return `You are batch transferring multiple token IDs from ${decoded.args[0]} to ${decoded.args[1]}. This operation affects multiple assets simultaneously.`;
          case "ERC1155:setApprovalForAll":
            return `You are ${decoded.args[1] ? "approving" : "revoking"} ${
              decoded.args[0]
            } to manage ALL your tokens. If approved, this address will have full control over all your tokens in this collection.`;
          case "ERC777:send":
            return `You are sending ${ethers.formatUnits(
              decoded.args[1],
              18
            )} tokens to ${
              decoded.args[0]
            }. If the recipient address is incorrect, your tokens could be lost.`;
          case "ERC777:burn":
            return `You are burning ${ethers.formatUnits(
              decoded.args[0],
              18
            )} tokens. This operation is irreversible and will permanently remove these tokens from circulation.`;
          case "ERC777:authorizeOperator":
            return `You are authorizing ${decoded.args[0]} as an operator for your tokens. This address will be able to transfer and burn your tokens on your behalf.`;
          case "ERC777:revokeOperator":
            return `You are revoking ${decoded.args[0]} as an operator for your tokens. This may impact any automated operations you have set up with this operator.`;
          case "ERC777:operatorSend":
            return `An operator is sending ${ethers.formatUnits(
              decoded.args[2],
              18
            )} tokens from ${decoded.args[0]} to ${
              decoded.args[1]
            }. Ensure you trust this operator and the transaction details.`;
          case "ERC777:operatorBurn":
            return `An operator is burning ${ethers.formatUnits(
              decoded.args[1],
              18
            )} tokens from ${
              decoded.args[0]
            }. This operation is irreversible and will permanently remove these tokens from circulation.`;
          default:
            return `You are interacting with a ${name} contract. Verify all details carefully.`;
        }
      }
    } catch (error) {
      // If parsing fails, continue to the next interface
      console.error(`Error parsing ${name} interface:`, error);
    }
  }

  return "CAUTION: You are performing an unknown operation. Please verify all transaction details carefully before proceeding.";
}
