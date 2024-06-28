import { ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

const CONTRACT_ADDRESS = "0x2fa5f72e70771ec5b238b4E4EAFfd6F21bF6adf5";

async function connectToProvider(): Promise<ethers.JsonRpcProvider> {
  return new ethers.JsonRpcProvider(
    `https://sepolia.infura.io/v3/${process.env.INFURA_PROJECT_ID}`
  );
}

async function createContractInstance(
  provider: ethers.JsonRpcProvider
): Promise<ethers.Contract> {
  const minimalABI = [
    "function viewCallerData(string) view returns (string)",
    "function setCallerData(string, string)",
  ];
  return new ethers.Contract(CONTRACT_ADDRESS, minimalABI, provider);
}

async function viewCallerData(
  contract: ethers.Contract,
  key: string
): Promise<void> {
  try {
    const value = await contract.viewCallerData(key);
    console.log(`Caller data for key "${key}": ${value}`);
  } catch (error) {
    console.error("Error viewing caller data:", error);
  }
}

async function setCallerData(
  contract: ethers.Contract,
  signer: ethers.Wallet,
  key: string,
  value: string
): Promise<void> {
  try {
    const tx = await (
      contract.connect(signer) as ethers.Contract
    ).setCallerData(key, value);
    await tx.wait();
    console.log("Caller data set successfully");
  } catch (error) {
    console.error("Error setting caller data:", error);
  }
}

async function callContractWithDataField(
  signer: ethers.Wallet,
  functionSignature: string,
  params: string[]
): Promise<void> {
  const iface = new ethers.Interface([`function ${functionSignature}`]);

  const data = iface.encodeFunctionData(
    functionSignature.split("(")[0],
    params
  );

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

function decodeData(
  data: string,
  functionSignature: string
): { functionName: string; parameters: any[] } {
  const abi = [`function ${functionSignature}`];

  const iface = new ethers.Interface(abi);

  const func = iface.getFunction(functionSignature.split("(")[0]);

  if (!func) {
    throw new Error("Invalid function signature");
  }

  const decodedData = iface.parseTransaction({ data });

  if (!decodedData) {
    throw new Error("Failed to decode data");
  }

  return {
    functionName: func.name,
    parameters: decodedData.args,
  };
}

async function viewCallerDataWithDataField(
  provider: ethers.JsonRpcProvider,
  key: string
): Promise<void> {
  const functionSignature = "viewCallerData(string)";

  const iface = new ethers.Interface([`function ${functionSignature}`]);

  const data = iface.encodeFunctionData("viewCallerData", [key]);

  const decodedData = decodeData(data, functionSignature);
  console.log("Decoded Data:");
  console.log("Function:", decodedData.functionName);
  console.log("Parameters:", decodedData.parameters);

  try {
    const result = await provider.call({
      to: CONTRACT_ADDRESS,
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

async function interactWithCallerRegistry(): Promise<void> {
  const provider = await connectToProvider();
  const contract = await createContractInstance(provider);
  const signer = new ethers.Wallet(process.env.PRIVATE_KEY as string, provider);

  const key = "felipe";
  const value = "friday 28";

  // await viewCallerData(contract, key);
  // await setCallerData(contract, signer, key, value);

  await callContractWithDataField(signer, "setCallerData(string,string)", [
    key,
    value,
  ]);
  await viewCallerDataWithDataField(provider, key);
}

interactWithCallerRegistry();
