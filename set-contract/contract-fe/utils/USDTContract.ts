import { ethers } from "ethers";

class USDTContract {
  private static CONTRACT_ADDRESS =
    "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06"; // Sepolia USDT contract address
  private provider: ethers.BrowserProvider;
  private contract: ethers.Contract;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
    const minimalABI = [
      "function balanceOf(address) view returns (uint256)",
      "function transfer(address, uint256) returns (bool)",
      "function approve(address, uint256) returns (bool)",
      "function allowance(address, address) view returns (uint256)",
      "function totalSupply() view returns (uint256)",
      "event Transfer(address indexed from, address indexed to, uint256 value)",
      "event Approval(address indexed owner, address indexed spender, uint256 value)",
    ];
    this.contract = new ethers.Contract(
      USDTContract.CONTRACT_ADDRESS,
      minimalABI,
      provider
    );
  }

  async getBalance(address: string): Promise<string> {
    try {
      const balance = await this.contract.balanceOf(address);
      return ethers.formatUnits(balance, 6); // USDT uses 6 decimal places
    } catch (error) {
      console.error("Error getting balance:", error);
      throw error;
    }
  }

  async transfer(to: string, amount: string): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 6);
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).transfer(to, amountWei);
      await tx.wait();
      console.log("Transfer successful");
    } catch (error) {
      console.error("Error transferring USDT:", error);
      throw error;
    }
  }

  async approve(spender: string, amount: string): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 6);
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).approve(spender, amountWei);
      await tx.wait();
      console.log("Approval successful");
    } catch (error) {
      console.error("Error approving USDT:", error);
      throw error;
    }
  }

  async getAllowance(owner: string, spender: string): Promise<string> {
    try {
      const allowance = await this.contract.allowance(owner, spender);
      return ethers.formatUnits(allowance, 6);
    } catch (error) {
      console.error("Error getting allowance:", error);
      throw error;
    }
  }

  async getTotalSupply(): Promise<string> {
    try {
      const totalSupply = await this.contract.totalSupply();
      return ethers.formatUnits(totalSupply, 6);
    } catch (error) {
      console.error("Error getting total supply:", error);
      throw error;
    }
  }
}

export { USDTContract };
