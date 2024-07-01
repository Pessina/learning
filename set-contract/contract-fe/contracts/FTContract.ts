import { getUserFriendlyDescription } from "@/validation/functionCall";
import { ethers } from "ethers";

class FTContract {
  private static CONTRACT_ADDRESS =
    "0xF3F795f8Bde4421ff3e8D18964a39B64fA685690";
  private provider: ethers.BrowserProvider;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
  }

  async totalSupply(): Promise<string> {
    try {
      const iface = new ethers.Interface([
        "function totalSupply() view returns (uint256)",
      ]);
      const data = iface.encodeFunctionData("totalSupply");

      const result = await this.provider.call({
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      });

      const [supply] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint256"],
        result
      );
      return ethers.formatUnits(supply, 18);
    } catch (error) {
      console.error("Error getting total supply:", error);
      throw error;
    }
  }

  async getBalance(address: string): Promise<string> {
    try {
      const iface = new ethers.Interface([
        "function balanceOf(address) view returns (uint256)",
      ]);
      const data = iface.encodeFunctionData("balanceOf", [address]);

      const result = await this.provider.call({
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      });

      const [balance] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint256"],
        result
      );
      return ethers.formatUnits(balance, 18);
    } catch (error) {
      console.error("Error getting balance:", error);
      throw error;
    }
  }

  async transfer(to: string, amount: string): Promise<boolean> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 18);
      const iface = new ethers.Interface([
        "function transfer(address,uint256) returns (bool)",
      ]);
      console.log({ to, amount });
      const data = iface.encodeFunctionData("transfer", [to, amountWei]);

      const transaction = {
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log("Transfer successful");
      return true;
    } catch (error) {
      console.error("Error transferring tokens:", error);
      throw error;
    }
  }

  async approve(spender: string, amount: string): Promise<boolean> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 18);
      const iface = new ethers.Interface([
        "function approve(address,uint256) returns (bool)",
      ]);
      const data = iface.encodeFunctionData("approve", [spender, amountWei]);

      const transaction = {
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log("Approval successful");
      return true;
    } catch (error) {
      console.error("Error approving tokens:", error);
      throw error;
    }
  }

  async transferFrom(
    from: string,
    to: string,
    amount: string
  ): Promise<boolean> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 18);
      const iface = new ethers.Interface([
        "function transferFrom(address,address,uint256) returns (bool)",
      ]);
      const data = iface.encodeFunctionData("transferFrom", [
        from,
        to,
        amountWei,
      ]);

      const transaction = {
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log("TransferFrom successful");
      return true;
    } catch (error) {
      console.error("Error transferring tokens from another address:", error);
      throw error;
    }
  }

  async allowance(owner: string, spender: string): Promise<string> {
    try {
      const iface = new ethers.Interface([
        "function allowance(address,address) view returns (uint256)",
      ]);
      const data = iface.encodeFunctionData("allowance", [owner, spender]);

      const result = await this.provider.call({
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      });

      const [allowanceAmount] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint256"],
        result
      );
      return ethers.formatUnits(allowanceAmount, 18);
    } catch (error) {
      console.error("Error getting allowance:", error);
      throw error;
    }
  }

  async mint(receiver: string, amount: string): Promise<boolean> {
    try {
      const signer = await this.provider.getSigner();
      const amountWei = ethers.parseUnits(amount, 18);
      const iface = new ethers.Interface([
        "function mint(address,uint256) returns (bool)",
      ]);
      const data = iface.encodeFunctionData("mint", [receiver, amountWei]);

      const transaction = {
        to: FTContract.CONTRACT_ADDRESS,
        data: data,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`Successfully minted ${amount} tokens to ${receiver}`);
      return true;
    } catch (error) {
      console.error("Error minting tokens:", error);
      throw error;
    }
  }
}

export { FTContract };
