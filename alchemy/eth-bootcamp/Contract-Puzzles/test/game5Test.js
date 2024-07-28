const { loadFixture } = require('@nomicfoundation/hardhat-network-helpers');
const { assert } = require('chai');

describe('Game5', function () {
  async function deployContractAndSetVariables() {
    const Game = await ethers.getContractFactory('Game5');
    const game = await Game.deploy();

    return { game };
  }
  it('should be a winner', async function () {
    const { game } = await loadFixture(deployContractAndSetVariables);

    // good luck
    const signer = ethers.provider.getSigner(0);
    
    let count = 0;
    while(true) {
      let wallet = ethers.Wallet.createRandom().connect(ethers.provider);
      await signer.sendTransaction({
        to: wallet.address,
        value: ethers.utils.parseEther('0.1')
      })
      try {
        await game.connect(wallet).win();
        break
      } catch (error) {
        count++;
      }
    }

    // leave this assertion as-is
    assert(await game.isWon(), 'You did not win the game');
  });
});
