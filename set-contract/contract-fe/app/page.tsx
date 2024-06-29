"use client";

import { SetContractComponent } from "../components/SetContract.component";
import { USDTContractComponent } from "../components/USDTContract.component";
import { BoredApeContractComponent } from "../components/BoredApeContract.component";

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-r from-blue-100 to-purple-100">
      <div className="container mx-auto p-8">
        <h1 className="text-4xl font-bold mb-8 text-center text-gray-800 border-b-2 border-gray-300 pb-4">
          Smart Contract Interaction
        </h1>
        <div className="flex flex-col gap-4">
          <SetContractComponent />
          <USDTContractComponent />
          <BoredApeContractComponent />
        </div>
      </div>
    </div>
  );
}