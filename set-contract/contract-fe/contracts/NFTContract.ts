import { ethers } from "ethers";
import { getUserFriendlyDescription } from "@/validation/functionCall";

class NFTContract {
  private static CONTRACT_ADDRESS =
    "0x3D72C76702EFBC59e656b4dc91794FbBDb50457d"; 
  private provider: ethers.BrowserProvider;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
  }

  async safeTransferFrom(
    from: string,
    to: string,
    tokenId: number,
    data?: string
  ): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function safeTransferFrom(address from, address to, uint256 tokenId, bytes data)",
      ]);
      const callData = iface.encodeFunctionData("safeTransferFrom", [
        from,
        to,
        tokenId,
        data || "0x",
      ]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`Token ${tokenId} safely transferred from ${from} to ${to}`);
    } catch (error) {
      console.error("Error safely transferring token:", error);
      throw error;
    }
  }

  async transferFrom(from: string, to: string, tokenId: number): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function transferFrom(address from, address to, uint256 tokenId)",
      ]);
      const callData = iface.encodeFunctionData("transferFrom", [
        from,
        to,
        tokenId,
      ]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`Token ${tokenId} transferred from ${from} to ${to}`);
    } catch (error) {
      console.error("Error transferring token:", error);
      throw error;
    }
  }

  async approve(to: string, tokenId: number): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function approve(address to, uint256 tokenId)",
      ]);
      const callData = iface.encodeFunctionData("approve", [to, tokenId]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`Approval given to ${to} for token ${tokenId}`);
    } catch (error) {
      console.error("Error approving token:", error);
      throw error;
    }
  }

  async setApprovalForAll(operator: string, approved: boolean): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function setApprovalForAll(address operator, bool approved)",
      ]);
      const callData = iface.encodeFunctionData("setApprovalForAll", [
        operator,
        approved,
      ]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
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

  async safeTransferFromERC1155(
    from: string,
    to: string,
    id: number,
    amount: number,
    data: string
  ): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function safeTransferFrom(address from, address to, uint256 id, uint256 amount, bytes calldata data)",
      ]);
      const callData = iface.encodeFunctionData("safeTransferFrom", [
        from,
        to,
        id,
        amount,
        data,
      ]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
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
    data: string
  ): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function safeBatchTransferFrom(address from, address to, uint256[] calldata ids, uint256[] calldata amounts, bytes calldata data)",
      ]);
      const callData = iface.encodeFunctionData("safeBatchTransferFrom", [
        from,
        to,
        ids,
        amounts,
        data,
      ]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
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

  async tokenURI(tokenId: number): Promise<string> {
    try {
      const iface = new ethers.Interface([
        "function tokenURI(uint256 tokenId) view returns (string)",
      ]);
      const callData = iface.encodeFunctionData("tokenURI", [tokenId]);

      const result = await this.provider.call({
        to: NFTContract.CONTRACT_ADDRESS,
        data: callData,
      });

      const [uri] = ethers.AbiCoder.defaultAbiCoder().decode(
        ["string"],
        result
      );
      return uri;
    } catch (error) {
      console.error("Error getting token URI:", error);
      throw error;
    }
  }

  async mint(to: string, tokenId: number, uri: string): Promise<void> {
    try {
      const signer = await this.provider.getSigner();
      const iface = new ethers.Interface([
        "function mint(address to, uint256 tokenId, string memory uri)",
      ]);
      const callData = iface.encodeFunctionData("mint", [to, tokenId, uri]);

      const transaction = {
        to: NFTContract.CONTRACT_ADDRESS,
        data: callData,
      };

      console.log(await getUserFriendlyDescription(transaction, this.provider));

      const tx = await signer.sendTransaction(transaction);
      await tx.wait();
      console.log(`Token ${tokenId} minted to ${to} with URI ${uri}`);
    } catch (error) {
      console.error("Error minting token:", error);
      throw error;
    }
  }
}

export { NFTContract };
