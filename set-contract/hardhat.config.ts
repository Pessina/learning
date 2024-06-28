import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import dotenv from "dotenv";

dotenv.config();

const config: HardhatUserConfig = {
  solidity: "0.8.24",
  networks: {
    sepolia: {
      url: `https://sepolia.infura.io/v3/${
        process.env.INFURA_PROJECT_ID ?? ""
      }`,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
    },
  },
};

export default config;
