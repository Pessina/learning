import React from "react";
import Wallet from "./Wallet";
import Transfer from "./Transfer";
import "./App.scss";
import { useState } from "react";

function App() {
  const [balance, setBalance] = useState(0);

  return (
    <div className="app">
      <Wallet balance={balance} setBalance={setBalance} />
      <Transfer />
    </div>
  );
}

export default App;
