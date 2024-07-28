
import { ModifyVariable } from "../typechain-types";
import {expect, assert} from "chai"
import {ethers} from "hardhat"


describe("TestModifyVariable", () => {
  it('Should modify variable to 1337', async () => {
    const ModifyVariable = await ethers.getContractFactory("ModifyVariable");
    const contract = await ModifyVariable.deploy(10) as ModifyVariable;
    await contract.modifyToLeet();

    const value = await contract.value();
    expect(value).to.equal(1337);
  })
})