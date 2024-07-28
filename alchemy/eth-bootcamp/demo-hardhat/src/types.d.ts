import { ethers } from 'ethers';

declare global {
  namespace NodeJS {
    interface Global {
      ethers: typeof ethers;
    }
  }
}
