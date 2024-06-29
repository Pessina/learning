import { ethers } from "ethers";

class USDTContract {
  private static CONTRACT_ADDRESS =
    "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06"; // Sepolia USDT contract address
  private provider: ethers.BrowserProvider;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
  }

  async getBalance(address: string): Promise<string> {
    try {
      const iface = new ethers.Interface([
        "function balanceOf(address) view returns (uint256)",
      ]);
      const data = iface.encodeFunctionData("balanceOf", [address]);

      const result = await this.provider.call({
        to: USDTContract.CONTRACT_ADDRESS,
        data: data,
      });

      const [balance] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint256"],
        result
      );
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
      const iface = new ethers.Interface([
        "function transfer(address,uint256) returns (bool)",
      ]);
      const data = iface.encodeFunctionData("transfer", [to, amountWei]);

      const tx = await signer.sendTransaction({
        to: USDTContract.CONTRACT_ADDRESS,
        data: data,
      });
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
      const iface = new ethers.Interface([
        "function _mint(address,uint256) returns (bool)",
      ]);
      const data = iface.encodeFunctionData("_mint", [receiver, amountWei]);

      const tx = await signer.sendTransaction({
        to: USDTContract.CONTRACT_ADDRESS,
        data: data,
      });
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