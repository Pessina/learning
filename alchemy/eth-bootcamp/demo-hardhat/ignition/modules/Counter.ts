import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const CounterModule = buildModule("CounterModule", (m) => {
  const initialValue = m.getParameter("initialValue", 1);

  const counter = m.contract("Counter", [initialValue]);
  return { counter };
});

export default CounterModule;