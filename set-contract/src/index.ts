import { ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

// Constants
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
  const functionSelector = ethers.id(functionSignature).slice(0, 10);
  const encodedParams = ethers.AbiCoder.defaultAbiCoder().encode(
    params.map(() => "string"),
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

async function viewCallerDataWithDataField(
  provider: ethers.JsonRpcProvider,
  key: string
): Promise<void> {
  const functionSignature = "viewCallerData(string)";
  const functionSelector = ethers.id(functionSignature).slice(0, 10);
  const encodedKey = ethers.AbiCoder.defaultAbiCoder().encode(
    ["string"],
    [key]
  );
  const data = functionSelector + encodedKey.slice(2);

  try {
    const result = await provider.call({
      to: CONTRACT_ADDRESS,
      data: data,
    });
    const [value] = ethers.AbiCoder.defaultAbiCoder().decode(
      ["string"],
      result
    );
    console.log(`Caller data for key "${key}": ${value}`);
  } catch (error) {
    console.error("Error viewing caller data using data field:", error);
  }
}

async function interactWithCallerRegistry(): Promise<void> {
  const provider = await connectToProvider();
  const contract = await createContractInstance(provider);
  const signer = new ethers.Wallet(process.env.PRIVATE_KEY as string, provider);

  const key = "exampleKey";
  const value = "exampleValue";

  await viewCallerData(contract, key);
  await setCallerData(contract, signer, key, value);

  // Using data field
  await callContractWithDataField(signer, "setCallerData(string,string)", [
    key,
    "newValue",
  ]);
  await viewCallerDataWithDataField(provider, key);
}

interactWithCallerRegistry();
