import React from "react";
import ReactDOM from "react-dom";

import { Gomoku } from './Gomoku';

const element = document.getElementById("root");

const App = () => (
  <div>
    <Gomoku />
  </div>
);

ReactDOM.render(<App />, element);
