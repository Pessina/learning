import { ExternalProvider } from "@ethersproject/providers";

declare global {
  interface Window {
    ethereum?: MetaMaskInpageProvider;
  }
}

export {};
