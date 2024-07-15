import { ethers } from "ethers";
import { fetchTokenInfo } from "./tokenStandards";

// ERC20 Interface
const erc20Interface = new ethers.Interface([
  "function transfer(address, uint256)",
  "function approve(address, uint256)",
  "function transferFrom(address, address, uint256)",
]);

// ERC721 Interface
const erc721Interface = new ethers.Interface([
  "function safeTransferFrom(address, address, uint256, bytes)",
  "function safeTransferFrom(address, address, uint256)",
  "function transferFrom(address, address, uint256)",
  "function approve(address, uint256)",
  "function setApprovalForAll(address, bool)",
]);

// ERC1155 Interface
const erc1155Interface = new ethers.Interface([
  "function safeTransferFrom(address, address, uint256, uint256, bytes)",
  "function safeBatchTransferFrom(address, address, uint256[], uint256[], bytes)",
  "function setApprovalForAll(address, bool)",
]);

export async function getUserFriendlyDescription(
  tx: {
    data: string;
    to: string;
    value?: string;
  },
  provider: ethers.Provider
): Promise<string> {
  if (tx.data === "0x" && tx.value) {
    return `You are sending ${ethers.formatEther(tx.value)} ETH to ${tx.to}`;
  }

  if (tx.to === null || tx.to === undefined || tx.to === "") {
    return "You are deploying a new contract. Please verify the contract code carefully.";
  }

  const { ercStandard, decimals, symbol, name } = await fetchTokenInfo(
    tx.to,
    provider
  );

  let iface: ethers.Interface;
  switch (ercStandard) {
    case "ERC20":
      iface = erc20Interface;
      break;
    case "ERC721":
      iface = erc721Interface;
      break;
    case "ERC1155":
      iface = erc1155Interface;
      break;
    default:
      return "CAUTION: You are performing an unknown operation. Please verify all transaction details carefully before proceeding.";
  }

  try {
    const decoded = iface.parseTransaction({ data: tx.data });
    if (decoded) {
      switch (decoded.name) {
        case "transfer":
          if (ercStandard === "ERC20") {
            return `You are transferring ${ethers.formatUnits(
              decoded.args[1],
              decimals
            )} ${symbol} (${name}) tokens to ${
              decoded.args[0]
            }. Please verify the recipient address and amount carefully.`;
          } else {
            return `You are transferring token ID ${decoded.args[1]} (${name}) to ${decoded.args[0]}. Please verify the recipient address and token ID carefully.`;
          }

        case "approve":
          if (ercStandard === "ERC20") {
            return `You are approving ${
              decoded.args[0]
            } to manage up to ${ethers.formatUnits(
              decoded.args[1],
              decimals
            )} ${symbol} (${name}) tokens. This allows them to transfer this amount on your behalf.`;
          } else {
            return `You are approving ${decoded.args[0]} to manage your token ID ${decoded.args[1]} (${name}). This allows them to transfer this specific token on your behalf.`;
          }

        case "transferFrom":
          if (ercStandard === "ERC20") {
            return `You are initiating a transfer of ${ethers.formatUnits(
              decoded.args[2],
              decimals
            )} ${symbol} (${name}) tokens from ${decoded.args[0]} to ${
              decoded.args[1]
            }. Ensure you have the necessary permissions for this action.`;
          } else {
            return `You are initiating a transfer of token ID ${decoded.args[2]} (${name}) from ${decoded.args[0]} to ${decoded.args[1]}. Ensure you have the necessary permissions for this action.`;
          }

        case "safeTransferFrom":
          if (ercStandard === "ERC721") {
            return `You are safely transferring token ID ${decoded.args[2]} (${name}) from ${decoded.args[0]} to ${decoded.args[1]}. This method includes additional safety checks.`;
          } else if (ercStandard === "ERC1155") {
            return `You are safely transferring ${decoded.args[3]} of token ID ${decoded.args[2]} (${name}) from ${decoded.args[0]} to ${decoded.args[1]}. This method includes additional safety checks.`;
          }

        case "setApprovalForAll":
          return `You are ${
            decoded.args[1] ? "granting" : "revoking"
          } permission for ${
            decoded.args[0]
          } to manage ALL your ${ercStandard} (${name}) tokens in this collection. This is a powerful permission, use with caution.`;

        case "safeBatchTransferFrom":
          return `You are batch transferring multiple ${ercStandard} (${name}) tokens from ${decoded.args[0]} to ${decoded.args[1]}. This operation affects ${decoded.args[2].length} different token IDs simultaneously. Please verify all details carefully.`;

        default:
          return `You are interacting with a ${ercStandard} (${name}) contract using the ${decoded.name} function. Please verify all details carefully.`;
      }
    }
  } catch (error) {
    console.error(`Error parsing ${ercStandard} interface:`, error);
  }

  return "CAUTION: You are performing an unknown operation. Please verify all transaction details carefully before proceeding.";
}
