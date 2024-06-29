import { ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

class CallerRegistryContract {
  private static CONTRACT_ADDRESS =
    "0x2fa5f72e70771ec5b238b4E4EAFfd6F21bF6adf5";
  private provider: ethers.BrowserProvider;
  private contract: ethers.Contract;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
    const minimalABI = [
      "function viewCallerData(string) view returns (string)",
      "function setCallerData(string, string)",
    ];
    this.contract = new ethers.Contract(
      CallerRegistryContract.CONTRACT_ADDRESS,
      minimalABI,
      provider
    );
  }

  async callContractWithDataField(
    functionSignature: string,
    params: string[]
  ): Promise<void> {
    const signer = await this.provider.getSigner();
    const iface = new ethers.Interface([`function ${functionSignature}`]);

    const data = iface.encodeFunctionData(
      functionSignature.split("(")[0],
      params
    );

    try {
      const tx = await signer.sendTransaction({
        to: CallerRegistryContract.CONTRACT_ADDRESS,
        data: data,
      });
      await tx.wait();
      console.log(`${functionSignature} called successfully using data field`);
    } catch (error) {
      console.error(
        `Error calling ${functionSignature} using data field:`,
        error
      );
    }
  }

  async viewCallerDataWithDataField(key: string): Promise<void> {
    const functionSignature = "viewCallerData(string)";

    const iface = new ethers.Interface([`function ${functionSignature}`]);

    const data = iface.encodeFunctionData("viewCallerData", [key]);

    try {
      const result = await this.provider.call({
        to: CallerRegistryContract.CONTRACT_ADDRESS,
        data: data,
      });

      const decodedResult = ethers.AbiCoder.defaultAbiCoder().decode(
        ["string"],
        result
      );
      console.log(`Caller data for key "${key}": ${decodedResult[0]}`);
    } catch (error) {
      console.error("Error viewing caller data using data field:", error);
    }
  }

  async setCallerData(key: string, value: string): Promise<void> {
    await this.callContractWithDataField("setCallerData(string,string)", [
      key,
      value,
    ]);
  }

  async viewCallerData(key: string): Promise<void> {
    await this.viewCallerDataWithDataField(key);
  }
}

export { CallerRegistryContract };
