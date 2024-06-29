import { ethers } from "ethers";
import { getUserFriendlyDescription } from "@/validation/functionCall";

class NFTContract {
  private static CONTRACT_ADDRESS =
    "0x3D72C76702EFBC59e656b4dc91794FbBDb50457d"; // BAYC contract address
  private provider: ethers.BrowserProvider;
  private contract: ethers.Contract;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
    const minimalABI = [
      // ERC721
      "function balanceOf(address owner) view returns (uint256)",
      "function ownerOf(uint256 tokenId) view returns (address)",
      "function safeTransferFrom(address from, address to, uint256 tokenId)",
      "function safeTransferFrom(address from, address to, uint256 tokenId, bytes data)",
      "function transferFrom(address from, address to, uint256 tokenId)",
      "function approve(address to, uint256 tokenId)",
      "function setApprovalForAll(address operator, bool approved)",
      "function getApproved(uint256 tokenId) view returns (address)",
      "function isApprovedForAll(address owner, address operator) view returns (bool)",
      // ERC1155
      "function balanceOf(address account, uint256 id) view returns (uint256)",
      "function balanceOfBatch(address[] calldata accounts, uint256[] calldata ids) view returns (uint256[] memory)",
      "function setApprovalForAll(address operator, bool approved)",
      "function isApprovedForAll(address account, address operator) view returns (bool)",
      "function safeTransferFrom(address from, address to, uint256 id, uint256 amount, bytes calldata data)",
      "function safeBatchTransferFrom(address from, address to, uint256[] calldata ids, uint256[] calldata amounts, bytes calldata data)",
      // Additional functions
      "function tokenURI(uint256 tokenId) view returns (string)",
      "function mint(address to, uint256 tokenId, string memory uri)",
    ];
    this.contract = new ethers.Contract(
      NFTContract.CONTRACT_ADDRESS,
      minimalABI,
      provider
    );
  }

  async balanceOf(ownerAddress: string, tokenId?: number): Promise<number> {
    try {
      if (tokenId !== undefined) {
        // ERC1155
        const balance = await this.contract.balanceOf(ownerAddress, tokenId);
        return Number(balance);
      } else {
        // ERC721
        const balance = await this.contract.balanceOf(ownerAddress);
        return Number(balance);
      }
    } catch (error) {
      console.error("Error getting balance:", error);
      throw error;
    }
  }

  async balanceOfBatch(accounts: string[], ids: number[]): Promise<number[]> {
    try {
      const balances = await this.contract.balanceOfBatch(accounts, ids);
      return balances.map(Number);
    } catch (error) {
      console.error("Error getting batch balances:", error);
      throw error;
    }
  }

  async ownerOf(tokenId: number): Promise<string> {
    try {
      return await this.contract.ownerOf(tokenId);
    } catch (error) {
      console.error("Error getting owner:", error);
      throw error;
    }
  }

  async tokenURI(tokenId: number): Promise<string> {
    try {
      return await this.contract.tokenURI(tokenId);
    } catch (error) {
      console.error("Error getting token URI:", error);
      throw error;
    }
  }

  async safeTransferFrom(
    from: string,
    to: string,
    tokenId: number,
    amount?: number,
    data?: string
  ): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      let tx;
      if (amount !== undefined) {
        // ERC1155
        tx = await (
          this.contract.connect(signer) as ethers.Contract
        ).safeTransferFrom(from, to, tokenId, amount, data || "0x");
      } else {
        // ERC721
        tx = await (
          this.contract.connect(signer) as ethers.Contract
        ).safeTransferFrom(from, to, tokenId, data || "0x");
      }
      await tx.wait();
      console.log(getUserFriendlyDescription(tx));
    } catch (error) {
      console.error("Error safely transferring token:", error);
      throw error;
    }
  }

  async transferFrom(from: string, to: string, tokenId: number): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).transferFrom(from, to, tokenId);
      await tx.wait();
      console.log(getUserFriendlyDescription(tx));
    } catch (error) {
      console.error("Error transferring token:", error);
      throw error;
    }
  }

  async approve(to: string, tokenId: number): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).approve(to, tokenId);
      await tx.wait();
      console.log(getUserFriendlyDescription(tx));
    } catch (error) {
      console.error("Error approving token:", error);
      throw error;
    }
  }

  async setApprovalForAll(operator: string, approved: boolean): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).setApprovalForAll(operator, approved);
      await tx.wait();
      console.log(getUserFriendlyDescription(tx));
    } catch (error) {
      console.error("Error setting approval for all:", error);
      throw error;
    }
  }

  async getApproved(tokenId: number): Promise<string> {
    try {
      return await this.contract.getApproved(tokenId);
    } catch (error) {
      console.error("Error getting approved address:", error);
      throw error;
    }
  }

  async isApprovedForAll(owner: string, operator: string): Promise<boolean> {
    try {
      return await this.contract.isApprovedForAll(owner, operator);
    } catch (error) {
      console.error("Error checking approval for all:", error);
      throw error;
    }
  }

  async safeBatchTransferFrom(
    from: string,
    to: string,
    ids: number[],
    amounts: number[],
    data: string
  ): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).safeBatchTransferFrom(from, to, ids, amounts, data);
      await tx.wait();
      console.log(getUserFriendlyDescription(tx));
    } catch (error) {
      console.error("Error batch transferring tokens:", error);
      throw error;
    }
  }

  async mint(to: string, tokenId: number, uri: string): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const tx = await (this.contract.connect(signer) as ethers.Contract).mint(
        to,
        tokenId,
        uri
      );
      await tx.wait();
      console.log(`Token ${tokenId} minted to ${to} with URI ${uri}`);
    } catch (error) {
      console.error("Error minting token:", error);
      throw error;
    }
  }
}

export { NFTContract };
