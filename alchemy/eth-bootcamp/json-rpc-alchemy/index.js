const { Alchemy, Network, Wallet, Utils } = require('alchemy-sdk');
const ethers = require('ethers')
require('dotenv').config();

const { TEST_API_KEY, TEST_PRIVATE_KEY } = process.env;

const settings = {
  apiKey: TEST_API_KEY,
  network: Network.ETH_SEPOLIA,
};
const alchemy = new Alchemy(settings);

let wallet = new Wallet(TEST_PRIVATE_KEY);

async function alchemyCall() {
  const nonce = await alchemy.core.getTransactionCount(
    wallet.address,
    'latest'
  );

  let transaction = {
    to: "0xC5fFedAd2701BeB8F70F4a7887A63f8E95db607a",
    value: Utils.parseEther('0.001'), // 0.001 worth of ETH being sent
    gasLimit: '21000',
    maxPriorityFeePerGas: Utils.parseUnits('5', 'gwei'),
    maxFeePerGas: Utils.parseUnits('20', 'gwei'),
    nonce: nonce,
    type: 2,
    chainId: 11155111, 
  };

  let rawTransaction = await wallet.signTransaction(transaction);
  console.log('Raw tx: ', rawTransaction);
  let tx = await alchemy.core.sendTransaction(rawTransaction);
  console.log(`https://goerli.etherscan.io/tx/${tx.hash}`);
}

const abi = [
  {"inputs":[],"name":"count","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},
  {"inputs":[],"name":"dec","outputs":[],"stateMutability":"nonpayable","type":"function"},
  {"inputs":[],"name":"get","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},
  {"inputs":[],"name":"inc","outputs":[],"stateMutability":"nonpayable","type":"function"}]

async function contractCall() {
  const provider = new ethers.AlchemyProvider('sepolia', TEST_API_KEY)
  const wallet = new ethers.Wallet(TEST_PRIVATE_KEY, provider)
  const contract = new ethers.Contract('0x5F91eCd82b662D645b15Fd7D2e20E5e5701CCB7A', abi, wallet);

  const tx = await contract.inc()
  console.log(tx)
  const count = await contract.count()
  console.log(count)
}

contractCall();