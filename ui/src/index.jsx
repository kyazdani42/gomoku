import React from "react";
import ReactDOM from "react-dom";
import bg from "./background.jpg";

import { Gomoku } from "./Gomoku";

const element = document.getElementById("root");

const App = () => (
  <div
    style={{
      width: "100vw",
      height: "100vh",
      background: `url(${bg})`,
      backgroundSize: "cover"
    }}
  >
    <Gomoku />
  </div>
);

ReactDOM.render(<App />, element);
