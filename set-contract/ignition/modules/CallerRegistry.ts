import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const CallerRegistryModule = buildModule("CallerRegistryModule", (m) => {
  const callerRegistry = m.contract("CallerRegistry");

  return { callerRegistry };
});

export default CallerRegistryModule;
