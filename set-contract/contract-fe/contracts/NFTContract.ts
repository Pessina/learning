import { ethers } from "ethers";

class NFTContract {
  private static CONTRACT_ADDRESS =
    "0xb2e8760AcA2b7E6ab375DD3a4F25fa603870ecE4"; // BAYC contract address
  private provider: ethers.BrowserProvider;
  private contract: ethers.Contract;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
    const minimalABI = [
      "function balanceOf(address owner) view returns (uint256)",
      "function ownerOf(uint256 tokenId) view returns (address)",
      "function tokenURI(uint256 tokenId) view returns (string)",
      "function transferFrom(address from, address to, uint256 tokenId)",
    ];
    this.contract = new ethers.Contract(
      NFTContract.CONTRACT_ADDRESS,
      minimalABI,
      provider
    );
  }

  async balanceOf(ownerAddress: string): Promise<number> {
    try {
      const balance = await this.contract.balanceOf(ownerAddress);
      return Number(balance);
    } catch (error) {
      console.error("Error getting balance:", error);
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

  async transferFrom(from: string, to: string, tokenId: number): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).transferFrom(from, to, tokenId);
      await tx.wait();
      console.log(`Token ${tokenId} transferred from ${from} to ${to}`);
    } catch (error) {
      console.error("Error transferring token:", error);
      throw error;
    }
  }
}

export { NFTContract };
