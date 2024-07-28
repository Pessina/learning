import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const EmitWinnerModule = buildModule("EmitWinnerModule", (m) => {
  const emitWinner = m.contract("EmitWinner", []);
  return { emitWinner };
});

export default EmitWinnerModule;