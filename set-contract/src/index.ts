import { ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

// Constants
const CONTRACT_ADDRESS: string = "0xA131f5E638013686B409A71356d8551Db4f3AE05";
const CALLER_ADDRESS: string = "0x4174678c78fEaFd778c1ff319D5D326701449b25";

async function connectToProvider(): Promise<ethers.JsonRpcProvider> {
  return new ethers.JsonRpcProvider(
    `https://sepolia.infura.io/v3/${process.env.INFURA_PROJECT_ID}`
  );
}

async function createContractInstance(
  provider: ethers.JsonRpcProvider
): Promise<ethers.Contract> {
  // For this example, we'll use a minimal ABI with just the functions we need
  const minimalABI: string[] = [
    "function viewCallerStatus(address) view returns (bool)",
    "function setCallerStatus(bool)",
  ];
  return new ethers.Contract(CONTRACT_ADDRESS, minimalABI, provider);
}

async function viewCallerStatus(
  contract: ethers.Contract,
  callerAddress: string
): Promise<void> {
  try {
    const status: boolean = await contract.viewCallerStatus(callerAddress);
    console.log(`Caller status for ${callerAddress}: ${status}`);
  } catch (error) {
    console.error("Error viewing caller status:", error);
  }
}

async function setCallerStatus(
  contract: ethers.Contract,
  provider: ethers.JsonRpcProvider,
  status: boolean
): Promise<void> {
  const signer: ethers.Wallet = new ethers.Wallet(
    process.env.PRIVATE_KEY as string,
    provider
  );
  const contractWithSigner = contract.connect(signer) as ethers.Contract;
  try {
    const tx = await contractWithSigner.setCallerStatus(status);
    await tx.wait();
    console.log("Caller status set successfully");
  } catch (error) {
    console.error("Error setting caller status:", error);
  }
}

async function setCallerStatusWithDataField(
  provider: ethers.JsonRpcProvider,
  status: boolean
): Promise<void> {
  const signer: ethers.Wallet = new ethers.Wallet(
    process.env.PRIVATE_KEY as string,
    provider
  );
  const functionSignature: string = "setCallerStatus(bool)";
  const functionSelector: string = ethers.id(functionSignature).slice(0, 10);
  const encodedStatus: string = ethers.AbiCoder.defaultAbiCoder().encode(
    ["bool"],
    [status]
  );
  const data: string = functionSelector + encodedStatus.slice(2);

  try {
    const tx: ethers.TransactionResponse = await signer.sendTransaction({
      to: CONTRACT_ADDRESS,
      data: data,
    });
    await tx.wait();
    console.log("Caller status set successfully using data field");
  } catch (error) {
    console.error("Error setting caller status using data field:", error);
  }
}

async function viewCallerStatusWithDataField(
  provider: ethers.JsonRpcProvider,
  callerAddress: string
): Promise<void> {
  const functionSignature: string = "viewCallerStatus(address)";
  const functionSelector: string = ethers.id(functionSignature).slice(0, 10);
  const encodedAddress: string = ethers.AbiCoder.defaultAbiCoder().encode(
    ["address"],
    [callerAddress]
  );
  const data: string = functionSelector + encodedAddress.slice(2);

  try {
    const result: string = await provider.call({
      to: CONTRACT_ADDRESS,
      data: data,
    });
    const decodedResult = ethers.AbiCoder.defaultAbiCoder().decode(
      ["bool"],
      result
    );
    const callerStatus: boolean = decodedResult[0] as boolean;
    console.log(`Caller status for ${callerAddress}: ${callerStatus}`);
  } catch (error) {
    console.error("Error viewing caller status using data field:", error);
  }
}

async function interactWithCallerRegistry(): Promise<void> {
  const provider: ethers.JsonRpcProvider = await connectToProvider();
  const contract: ethers.Contract = await createContractInstance(provider);

  await viewCallerStatus(contract, CALLER_ADDRESS);
  await setCallerStatus(contract, provider, true);

  // Using data field
  await setCallerStatusWithDataField(provider, false);
  await viewCallerStatusWithDataField(provider, CALLER_ADDRESS);
}

interactWithCallerRegistry();
