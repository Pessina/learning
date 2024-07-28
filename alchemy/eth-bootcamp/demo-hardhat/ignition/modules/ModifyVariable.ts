import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const ModifyValueModule = buildModule("ModifyValueModule", (m) => {
  const initialValue = m.getParameter("initialValue", 1);

  const modifyValue = m.contract("ModifyValue", [initialValue]);
  return { modifyValue };
});

export default ModifyValueModule;