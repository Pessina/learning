import { fetchDerivedEVMAddress } from "./utils/kdf.js";

fetchDerivedEVMAddress({
  signerId: "felipe-near.testnet",
  path: '{"chain":60}',
  nearNetworkId: "testnet",
  multichainContractId: "v2.multichain-mpc.testnet",
}).then((address) => console.log(address));
