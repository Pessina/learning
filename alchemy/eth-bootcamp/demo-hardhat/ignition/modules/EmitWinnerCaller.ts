import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const EmitWinnerCallerModule = buildModule("EmitWinnerCallerModule", (m) => {
  const emitWinnerCaller = m.contract("EmitWinnerCaller", []);
  return { emitWinnerCaller };
});

export default EmitWinnerCallerModule;