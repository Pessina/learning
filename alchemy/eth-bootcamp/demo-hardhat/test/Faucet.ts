import { ethers } from "hardhat";
import { Faucet, FaucetInterface } from "../typechain-types/Faucet";

const { loadFixture } = require('@nomicfoundation/hardhat-network-helpers');
const { expect } = require('chai');


describe("Faucet", () => {
  const deployContractAndSetVariable = async () => {
    const Faucet = await ethers.getContractFactory("Faucet");
    const faucet = await Faucet.deploy({ value: ethers.parseEther("1") }) as Faucet;

    const [owner] = await ethers.getSigners();
    console.log("Signer 1 address: :", owner.address);

    return { faucet, owner };
  }

  it("Should deploy and set the owner correctly", async () => {
    const {faucet, owner} = await deployContractAndSetVariable();

    expect(await faucet.owner()).to.equal(owner.address);
  })

  it("Should revert if non-owner call the withdrawAll", async () => {
    const {faucet, owner} = await deployContractAndSetVariable();
    
    const caller = (await ethers.getSigners())[1];

    await expect(faucet.connect(caller).withdrawAll())
    .to
    .be
    .revertedWith('Only owner can call this function');
  })

  it("Should not revert if non-owner call the withdrawAll", async () => {
    const {faucet, owner} = await deployContractAndSetVariable();
    
    await expect(faucet.connect(owner).withdrawAll())
    .not
    .to
    .be
    .reverted;
  })

  it("Should revert on withdraw more than .1 ETH", async () => {
    const {faucet, owner} = await deployContractAndSetVariable();
    
    await expect(faucet.withdraw(ethers.parseEther('0.11')))
    .to
    .be
    .revertedWith("Can't withdraw more than .1 ETH");
  })

  it("Should not revert on withdraw less than or equal to .1 ETH", async () => {
    const {faucet, owner} = await loadFixture(deployContractAndSetVariable);

    const secondAccount = (await ethers.getSigners())[1];

    await expect(faucet.connect(secondAccount).withdraw(ethers.parseEther('0.05')))
      .not
      .to
      .be
      .reverted;
  })

})