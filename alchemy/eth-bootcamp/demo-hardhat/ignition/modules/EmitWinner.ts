import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const EmitWinnerModule = buildModule("EmitWinnerModule", (m) => {
  const counter = m.contract("EmitWinner", []);
  return { counter };
});

export default EmitWinnerModule;