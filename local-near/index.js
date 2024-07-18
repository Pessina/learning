const { connect, keyStores, KeyPair, utils } = require("near-api-js");

const main = async () => {
  const keyStore = new keyStores.InMemoryKeyStore();
  const privateKey =
    "ed25519:bpYqxdxLzmA6cYiLYd2qFrrEmRfdUx8VxPxakmYgtiNYRg9XFG5UmkyXPQgtGFsxoQCexW5Xzn1t6NYVEkqecyD";
  const keyPair = KeyPair.fromString(privateKey);

  // Add the key pair to the key store
  await keyStore.setKey("near", "test.near", keyPair);

  const config = {
    networkId: "near",
    keyStore,
    nodeUrl: "http://localhost:24567",
    walletUrl: "http://localhost:24567",
    helperUrl: "http://localhost:24567",
  };

  const near = await connect(config);

  const newAccountPrivateKey =
    "ed25519:81NCMLUAvo1Kd9B3WfmU3vtchPNJpfeS75ACJsWNvDFwC7SbdYMAdJuTP5qBfWJWSzaB7DKutzWRLzS3JuU4mk8";
  const newAccountKeyPair = KeyPair.fromString(newAccountPrivateKey);

  const newAccountId = "felipe-sandbox.near";
  const newPublicKey = newAccountKeyPair.publicKey.toString();

  const account = await near.createAccount(newAccountId, newPublicKey);
  console.log(`Successfully created account: ${account.accountId}`);

  const balance = await account.getAccountBalance();
  console.log(`Account balance for ${newAccountId}:`, balance);

  // // Top up the newly created account
  // const amount = "1"; // Amount in NEAR to top up (adjust as needed)
  // const sender = await near.account("test.near"); // Using the account we initially set up

  // try {
  //   const result = await sender.sendMoney(
  //     newAccountId,
  //     utils.format.parseNearAmount(amount)
  //   );
  //   console.log(`Successfully topped up ${amount} NEAR to ${newAccountId}`);
  //   console.log("Transaction result:", result);

  //   // Check the new balance after top-up
  //   const newBalance = await account.getAccountBalance();
  //   console.log(`Updated account balance for ${newAccountId}:`, newBalance);
  // } catch (error) {
  //   console.error("Error topping up account:", error);
  // }
};

main().catch(console.error);
