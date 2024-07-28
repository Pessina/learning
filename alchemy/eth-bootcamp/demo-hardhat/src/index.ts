const ethers = require('ethers');
const dotenv = require('dotenv');

dotenv.config();

const abi = [
{
  "inputs": [],
  "name": "callAttempt",
  "outputs": [],
  "stateMutability": "nonpayable",
  "type": "function"
}
]

const callEmitWinnerCaller = async () => {
  const contractAddress = "0x0deC3b684e6616B90dE2853a8f85e451CB1Cc8eE"

  const provider = new ethers.AlchemyProvider(
    'sepolia', 
    process.env.TEST_API_KEY
  );
  const wallet = new ethers.Wallet(process.env.TEST_PRIVATE_KEY, provider)
  const contract = new ethers.Contract(contractAddress, abi, wallet)

  const tx = await contract.callAttempt();
  console.log(tx.hash);
}

callEmitWinnerCaller()