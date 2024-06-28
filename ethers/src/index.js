import { ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

// Constants
const CONTRACT_ADDRESS = "0xA131f5E638013686B409A71356d8551Db4f3AE05";
const CALLER_ADDRESS = "0x4174678c78fEaFd778c1ff319D5D326701449b25";

async function connectToProvider() {
  return new ethers.JsonRpcProvider(
    `https://sepolia.infura.io/v3/${process.env.INFURA_PROJECT_ID}`
  );
}

async function createContractInstance(provider) {
  // For this example, we'll use a minimal ABI with just the functions we need
  const minimalABI = [
    "function viewCallerStatus(address) view returns (bool)",
    "function setCallerStatus(bool)",
  ];
  return new ethers.Contract(CONTRACT_ADDRESS, minimalABI, provider);
}

async function viewCallerStatus(contract, callerAddress) {
  try {
    const status = await contract.viewCallerStatus(callerAddress);
    console.log(`Caller status for ${callerAddress}: ${status}`);
  } catch (error) {
    console.error("Error viewing caller status:", error);
  }
}

async function setCallerStatus(contract, provider, status) {
  const signer = new ethers.Wallet(process.env.PRIVATE_KEY, provider);
  const contractWithSigner = contract.connect(signer);
  try {
    const tx = await contractWithSigner.setCallerStatus(status);
    await tx.wait();
    console.log("Caller status set successfully");
  } catch (error) {
    console.error("Error setting caller status:", error);
  }
}

async function setCallerStatusWithDataField(provider, status) {
  const signer = new ethers.Wallet(process.env.PRIVATE_KEY, provider);
  const functionSignature = "setCallerStatus(bool)";
  const functionSelector = ethers.id(functionSignature).slice(0, 10);
  const encodedStatus = ethers.AbiCoder.defaultAbiCoder().encode(
    ["bool"],
    [status]
  );
  const data = functionSelector + encodedStatus.slice(2);

  try {
    const tx = await signer.sendTransaction({
      to: CONTRACT_ADDRESS,
      data: data,
    });
    await tx.wait();
    console.log("Caller status set successfully using data field");
  } catch (error) {
    console.error("Error setting caller status using data field:", error);
  }
}

async function viewCallerStatusWithDataField(provider, callerAddress) {
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
    const decodedResult = ethers.AbiCoder.defaultAbiCoder().decode(
      ["bool"],
      result
    );
    console.log(`Caller status for ${callerAddress}: ${decodedResult[0]}`);
  } catch (error) {
    console.error("Error viewing caller status using data field:", error);
  }
}

async function interactWithCallerRegistry() {
  const provider = await connectToProvider();
  const contract = await createContractInstance(provider);

  await viewCallerStatus(contract, CALLER_ADDRESS);
  await setCallerStatus(contract, provider, true);

  // Using data field
  await setCallerStatusWithDataField(provider, false);
  await viewCallerStatusWithDataField(provider, CALLER_ADDRESS);
}

interactWithCallerRegistry();
