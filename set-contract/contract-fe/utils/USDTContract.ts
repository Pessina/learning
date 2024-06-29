import { ethers } from "ethers";

class USDTContract {
  private static CONTRACT_ADDRESS =
    "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06"; // Sepolia USDT contract address
  private provider: ethers.BrowserProvider;
  private contract: ethers.Contract;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
    const minimalABI = [
      "function balanceOf(address account) view returns (uint256)",
      "function transfer(address to, uint256 amount) returns (bool)",
      "function _mint(address receiver, uint256 amount) returns (bool)",
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

  async transfer(to: string, amount: string): Promise<boolean> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 6);
      const tx = await (
        this.contract.connect(signer) as ethers.Contract
      ).transfer(to, amountWei);
      await tx.wait();
      console.log("Transfer successful");
      return true;
    } catch (error) {
      console.error("Error transferring USDT:", error);
      throw error;
    }
  }

  async mint(receiver: string, amount: string): Promise<boolean> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 6);
      const tx = await (this.contract.connect(signer) as ethers.Contract)._mint(
        receiver,
        amountWei
      );
      await tx.wait();
      console.log(`Successfully minted ${amount} USDT to ${receiver}`);
      return true;
    } catch (error) {
      console.error("Error minting USDT:", error);
      throw error;
    }
  }
}

export { USDTContract };
