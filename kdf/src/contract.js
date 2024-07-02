import { Contract, Connection, Account } from "@near-js/accounts";
import { InMemoryKeyStore } from "@near-js/keystores";

const getMultichainContract = (account, contract) => {
  return new Contract(account, contract, {
    viewMethods: ["public_key"],
    changeMethods: ["sign"],
    useLocalViewExecution: false,
  });
};

export async function getRootPublicKey(contract, nearNetworkId) {
  const nearConnection = Connection.fromConfig({
    networkId: nearNetworkId,
    provider: {
      type: "JsonRpcProvider",
      args: {
        url: {
          testnet: "https://rpc.testnet.near.org",
          mainnet: "https://rpc.mainnet.near.org",
        }[nearNetworkId],
      },
    },
    signer: { type: "InMemorySigner", keyStore: new InMemoryKeyStore() },
  });

  const nearAccount = new Account(nearConnection, "dontcare");
  const multichainContractAcc = getMultichainContract(nearAccount, contract);

  return await multichainContractAcc.public_key();
}
