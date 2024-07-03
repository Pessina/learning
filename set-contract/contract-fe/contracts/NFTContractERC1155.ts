import { ethers } from "ethers";
import { getUserFriendlyDescription } from "@/validation/functionCall";

class NFTContractERC1155 {
  private static CONTRACT_ADDRESS_ERC1155 =
    "0x392633e8fBA6B35995cD78514DC51A85116644FF";
  private provider: ethers.BrowserProvider;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
  }

  async safeTransferFrom(
    from: string,
    to: string,
    id: number,
    amount: number,
    data?: string
  ): Promise<void> {
    try {
      if (!from || !to || id === undefined || amount === undefined) {
        throw new Error(
          "Missing required fields: from, to, id, and amount are required."
        );
      }
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function safeTransferFrom(address from, address to, uint256 id, uint256 amount, bytes calldata data)",
      ]);
      const callData = iface.encodeFunctionData("safeTransferFrom", [
        from,
        to,
        id,
        amount,
        data || "0x",
      ]);

      const transaction = {
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(
        `${amount} of token ${id} safely transferred from ${from} to ${to}`
      );
    } catch (error) {
      console.error("Error safely transferring ERC1155 token:", error);
      throw error;
    }
  }

  async safeBatchTransferFrom(
    from: string,
    to: string,
    ids: number[],
    amounts: number[],
    data?: string
  ): Promise<void> {
    try {
      if (!from || !to || !ids || !amounts) {
        throw new Error(
          "Missing required fields: from, to, ids, and amounts are required."
        );
      }
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function safeBatchTransferFrom(address from, address to, uint256[] calldata ids, uint256[] calldata amounts, bytes calldata data)",
      ]);
      const callData = iface.encodeFunctionData("safeBatchTransferFrom", [
        from,
        to,
        ids,
        amounts,
        data || "0x",
      ]);

      const transaction = {
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`Batch transfer of tokens from ${from} to ${to} completed`);
    } catch (error) {
      console.error("Error batch transferring tokens:", error);
      throw error;
    }
  }

  async setApprovalForAll(operator: string, approved: boolean): Promise<void> {
    try {
      if (!operator || approved === undefined) {
        throw new Error(
          "Missing required fields: operator and approved are required."
        );
      }
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function setApprovalForAll(address operator, bool approved)",
      ]);
      const callData = iface.encodeFunctionData("setApprovalForAll", [
        operator,
        approved,
      ]);

      const transaction = {
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(
        `Approval for all set to ${approved} for operator ${operator}`
      );
    } catch (error) {
      console.error("Error setting approval for all:", error);
      throw error;
    }
  }

  async balanceOf(account: string, id: number): Promise<number> {
    try {
      if (!account || id === undefined) {
        throw new Error(
          "Missing required fields: account and id are required."
        );
      }
      const iface = new ethers.Interface([
        "function balanceOf(address account, uint256 id) view returns (uint256)",
      ]);
      const callData = iface.encodeFunctionData("balanceOf", [account, id]);

      const result = await this.provider.call({
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      });

      const [balance] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint256"],
        result
      );
      return balance;
    } catch (error) {
      console.error("Error getting balance:", error);
      throw error;
    }
  }

  async balanceOfBatch(accounts: string[], ids: number[]): Promise<number[]> {
    try {
      if (!accounts || !ids || accounts.length !== ids.length) {
        throw new Error(
          "Missing required fields: accounts and ids are required and must have the same length."
        );
      }
      const iface = new ethers.Interface([
        "function balanceOfBatch(address[] calldata accounts, uint256[] calldata ids) view returns (uint256[] memory)",
      ]);
      const callData = iface.encodeFunctionData("balanceOfBatch", [
        accounts,
        ids,
      ]);

      const result = await this.provider.call({
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      });

      const [balances] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["uint256[]"],
        result
      );
      return balances;
    } catch (error) {
      console.error("Error getting batch balances:", error);
      throw error;
    }
  }

  async uri(id: number): Promise<string> {
    try {
      if (id === undefined) {
        throw new Error("Missing required field: id is required.");
      }
      const iface = new ethers.Interface([
        "function uri(uint256 id) view returns (string)",
      ]);
      const callData = iface.encodeFunctionData("uri", [id]);

      const result = await this.provider.call({
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      });

      const [tokenUri] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["string"],
        result
      );
      return tokenUri;
    } catch (error) {
      console.error("Error getting token URI:", error);
      throw error;
    }
  }

  async mint(
    to: string,
    id: number,
    amount: number,
    data: string = "0x"
  ): Promise<void> {
    try {
      if (!to || id === undefined || amount === undefined) {
        throw new Error(
          "Missing required fields: to, id, and amount are required."
        );
      }
      if (to === "0x0000000000000000000000000000000000000000") {
        throw new Error("_to must be non-zero.");
      }
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function mint(address _to, uint256 _id, uint256 _value, bytes calldata _data) external",
      ]);
      const callData = iface.encodeFunctionData("mint", [to, id, amount, data]);

      const transaction = {
        to: NFTContractERC1155.CONTRACT_ADDRESS_ERC1155,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`${amount} tokens of id ${id} minted to ${to}`);

      // Note: The contract will handle the balance update and event emission
      // The _doSafeTransferAcceptanceCheck is also handled by the contract if _to is a contract
    } catch (error) {
      console.error("Error minting token:", error);
      throw error;
    }
  }
}

export { NFTContractERC1155 };
