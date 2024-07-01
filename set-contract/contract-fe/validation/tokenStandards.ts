import { ethers } from "ethers";

// Interface for ERC165
const erc165Interface = new ethers.Interface([
  "function supportsInterface(bytes4 interfaceId) view returns (bool)",
]);

// Interface IDs
const ERC721_INTERFACE_ID = "0x80ac58cd";
const ERC1155_INTERFACE_ID = "0xd9b67a26";

// ERC20 Interface
const erc20Interface = new ethers.Interface([
  "function totalSupply() view returns (uint256)",
  "function balanceOf(address) view returns (uint256)",
  "function transfer(address, uint256) returns (bool)",
]);

export async function identifyTokenStandard(
  contractAddress: string,
  provider: ethers.Provider
): Promise<"ERC20" | "ERC721" | "ERC1155" | "Unknown"> {
  const contract = new ethers.Contract(
    contractAddress,
    erc165Interface,
    provider
  );

  try {
    // Check for ERC721 and ERC1155
    const [isERC721, isERC1155] = await Promise.all([
      contract.supportsInterface(ERC721_INTERFACE_ID).catch(() => false),
      contract.supportsInterface(ERC1155_INTERFACE_ID).catch(() => false),
    ]);

    if (isERC721) return "ERC721";
    if (isERC1155) return "ERC1155";

    // Check for ERC20
    const erc20Contract = new ethers.Contract(
      contractAddress,
      erc20Interface,
      provider
    );

    try {
      await Promise.all([
        erc20Contract.totalSupply(),
        erc20Contract.balanceOf(contractAddress),
        erc20Contract.transfer.staticCall(ethers.ZeroAddress, 0),
      ]);
      return "ERC20";
    } catch {
      // If ERC20 check fails, it's not a standard ERC20 token
      return "Unknown";
    }
  } catch (error) {
    console.error("Error identifying token standard:", error);
    return "Unknown";
  }
}

export async function fetchTokenDecimals(
  contractAddress: string,
  provider: ethers.Provider
): Promise<number> {
  const decimalsInterface = new ethers.Interface([
    "function decimals() view returns (uint8)",
  ]);

  const contract = new ethers.Contract(
    contractAddress,
    decimalsInterface,
    provider
  );

  try {
    const decimals = await contract.decimals();
    return decimals;
  } catch (error) {
    console.error("Error fetching token decimals:", error);
    return 18; // Default to 18 if unable to fetch decimals
  }
}
