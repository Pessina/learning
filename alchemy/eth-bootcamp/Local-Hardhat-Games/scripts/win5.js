// add the game address here and update the contract name if necessary
const gameAddr = "0x610178da211fef7d417bc0e6fed39f05609ad788";
const contractName = "Game5";

async function main() {
    // attach to the game
    const game = await hre.ethers.getContractAt(contractName, gameAddr);

    let tx;
    // do whatever you need to do to win the game here:
    tx = await game.giveMeAllowance(10000);
    await tx.wait()
    tx = await game.mint(10000);
    await tx.wait()
    tx = await game.win();

    // did you win? Check the transaction receipt!
    // if you did, it will be in both the logs and events array
    const receipt = await tx.wait();
    console.log(receipt);
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
