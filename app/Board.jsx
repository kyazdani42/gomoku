import React, { useState } from "react";

export const Board = ({ onClick, board, player }) => {
  const lines = new Array(Math.sqrt(board.length)).fill(0);
  return (
    <div style={boardStyle}>
      <Cleanup />
      {lines.map((_, line) => (
        <div style={lineStyle} key={`line-${line}`}>
          {board
            .slice(line * lines.length, (line + 1) * lines.length)
            .map((block, index) => (
              <Block
                block={block}
                onClick={onClick}
                player={player}
                index={line * lines.length + index}
                key={`${line}${index}`}
              />
            ))}
        </div>
      ))}
    </div>
  );
};

const Cleanup = () => (
  <React.Fragment>
    <div
      style={{
        width: "34px",
        position: "absolute",
        left: 0,
        top: 0,
        bottom: 0,
        zIndex: 1000,
        background: "#c48e54"
      }}
    />
    <div
      style={{
        height: "34px",
        position: "absolute",
        left: 0,
        top: 0,
        right: 0,
        zIndex: 1000,
        background: "#c48e54"
      }}
    />
    <div
      style={{
        height: "34px",
        position: "absolute",
        left: 0,
        bottom: 0,
        right: 0,
        zIndex: 1000,
        background: "#c48e54"
      }}
    />
    <div
      style={{
        width: "34px",
        position: "absolute",
        right: 0,
        top: 0,
        bottom: 0,
        zIndex: 1000,
        background: "#c48e54"
      }}
    />
  </React.Fragment>
);

const Block = ({ index, block, onClick, player }) => {
  const [hover, setHover] = useState(false);
  return (
    <div
      style={blockStyle}
      onClick={() => {
        onClick(index);
        setHover(false);
      }}
      onMouseOver={() => setHover(block === 0)}
      onMouseOut={() => setHover(false)}
    >
      {cross}
      <WhiteStone hover={hover && player === 1} occupied={block === 1} />
      <BlackStone hover={hover && player === 2} occupied={block === 2} />
    </div>
  );
};

const cross = (
  <svg height="40px" width="40px" style={{ position: "absolute" }}>
    <path stroke="#000000" strokeWidth="2" d="M0,20H40M20,0V40" />
  </svg>
);

const WhiteStone = ({ hover, occupied }) => (
  <svg
    style={{
      overflow: "hidden",
      position: "absolute",
      opacity: occupied ? 1 : hover ? 0.6 : 0,
      zIndex: 2000
    }}
    height="40px"
    width="40px"
  >
    <defs>
      <radialGradient id="1r_0.75_0.75__fff-_A0A0A0" fx="0.75" fy="0.75">
        <stop offset="0%" stopColor="#ffffff"></stop>
        <stop offset="100%" stopColor="#a0a0a0" stopOpacity="1"></stop>
      </radialGradient>
    </defs>
    <circle
      cx="20"
      cy="20"
      r="12"
      fill="#ffffff"
      stroke="#000"
      strokeWidth="0"
    ></circle>
    <circle
      style={{ fillOpacity: 1, opacity: 1, strokeOpacity: 0.3 }}
      cx="20"
      cy="20"
      r="14"
      fill="url(#1r_0.75_0.75__fff-_A0A0A0)"
      stroke="#000"
      fillOpacity="1"
      opacity="1"
      strokeOpacity="0.3"
      strokeWidth="1.1"
    ></circle>
  </svg>
);

const BlackStone = ({ occupied, hover }) => (
  <svg
    style={{
      overflow: "hidden",
      position: "absolute",
      opacity: occupied ? 1 : hover ? 0.6 : 0,
      zIndex: 2000
    }}
    height="40px"
    width="40px"
  >
    <defs>
      <radialGradient id="1r_0.75_0.75__A0A0A0-_000" fx="0.75" fy="0.75">
        <stop offset="0%" stopColor="#a0a0a0"></stop>
        <stop offset="100%" stopColor="#000000" stopOpacity="0.9"></stop>
      </radialGradient>
    </defs>
    <circle
      cx="20"
      cy="20"
      r="14"
      fill="#ffffff"
      stroke="#000"
      strokeWidth="0"
    ></circle>
    <circle
      style={{ fillOpacity: 1, opacity: 1, strokeOpacity: 0.3 }}
      cx="20"
      cy="20"
      r="14"
      fill="url(#1r_0.75_0.75__A0A0A0-_000)"
      stroke="#000"
      fillOpacity="1"
      opacity="1"
      strokeOpacity="0.3"
      strokeWidth="1.2"
    ></circle>
  </svg>
);

const blockStyle = {
  width: "40px",
  height: "40px",
  cursor: "pointer"
};

const lineStyle = {
  display: "flex",
  maxWidth: "fit-content"
};

const boardStyle = {
  backgroundColor: "#c48e54",
  position: "relative",
  maxWidth: "fit-content",
  padding: "15px",
  margin: "auto",
  boxShadow: "0 5px 10px rgba(0,0,0,0.20), 0 7px 20px rgba(0,0,0,0.15)",
  border: "6px outset #c7977a"
};
