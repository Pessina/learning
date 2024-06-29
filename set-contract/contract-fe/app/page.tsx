"use client";

import { SetContractComponent } from "../components/SetContract.component";
import { USDTContractComponent } from "../components/USDTContract.component";
import { BoredApeContractComponent } from "../components/BoredApeContract.component";

export default function Home() {
  return (
    <div className="container mx-auto p-4">
      <h1 className="text-3xl font-bold mb-6 text-gray-800">
        Smart Contract Interaction
      </h1>
      <SetContractComponent />
      <USDTContractComponent />
      <BoredApeContractComponent />
    </div>
  );
}
