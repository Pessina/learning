import { ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

// Constants
const CONTRACT_ADDRESS = "0xA131f5E638013686B409A71356d8551Db4f3AE05";
const CALLER_ADDRESS = "0x4174678c78fEaFd778c1ff319D5D326701449b25";

async function connectToProvider(): Promise<ethers.JsonRpcProvider> {
  return new ethers.JsonRpcProvider(
    `https://sepolia.infura.io/v3/${process.env.INFURA_PROJECT_ID}`
  );
}

async function createContractInstance(
  provider: ethers.JsonRpcProvider
): Promise<ethers.Contract> {
  const minimalABI = [
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
    const status = await contract.viewCallerStatus(callerAddress);
    console.log(`Caller status for ${callerAddress}: ${status}`);
  } catch (error) {
    console.error("Error viewing caller status:", error);
  }
}

async function setCallerStatus(
  contract: ethers.Contract,
  signer: ethers.Wallet,
  status: boolean
): Promise<void> {
  try {
    const tx = await (
      contract.connect(signer) as ethers.Contract
    ).setCallerStatus(status);
    await tx.wait();
    console.log("Caller status set successfully");
  } catch (error) {
    console.error("Error setting caller status:", error);
  }
}

async function callContractWithDataField(
  signer: ethers.Wallet,
  functionSignature: string,
  params: any[]
): Promise<void> {
  const functionSelector = ethers.id(functionSignature).slice(0, 10);
  const encodedParams = ethers.AbiCoder.defaultAbiCoder().encode(
    params.map(() => "bool"),
    params
  );
  const data = functionSelector + encodedParams.slice(2);

  try {
    const tx = await signer.sendTransaction({
      to: CONTRACT_ADDRESS,
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

async function viewCallerStatusWithDataField(
  provider: ethers.JsonRpcProvider,
  callerAddress: string
): Promise<void> {
  const functionSignature = "viewCallerStatus(address)";
  const functionSelector = ethers.id(functionSignature).slice(0, 10);
  const encodedAddress = ethers.AbiCoder.defaultAbiCoder().encode(
    ["address"],
    [callerAddress]
  );
  const data = functionSelector + encodedAddress.slice(2);

  try {
    const result = await provider.call({
      to: CONTRACT_ADDRESS,
      data: data,
    });
    const [callerStatus] = ethers.AbiCoder.defaultAbiCoder().decode(
      ["bool"],
      result
    );
    console.log(`Caller status for ${callerAddress}: ${callerStatus}`);
  } catch (error) {
    console.error("Error viewing caller status using data field:", error);
  }
}

async function interactWithCallerRegistry(): Promise<void> {
  const provider = await connectToProvider();
  const contract = await createContractInstance(provider);
  const signer = new ethers.Wallet(process.env.PRIVATE_KEY as string, provider);

  await viewCallerStatus(contract, CALLER_ADDRESS);
  await setCallerStatus(contract, signer, true);

  // Using data field
  await callContractWithDataField(signer, "setCallerStatus(bool)", [false]);
  await viewCallerStatusWithDataField(provider, CALLER_ADDRESS);
}

interactWithCallerRegistry();
