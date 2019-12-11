import React, { useState } from 'react';

// TODO: we might wan't to check if the board is full so we can reset the game
// But in the backend it might be better
export const Board = ({ onClick, board, player, winner, onReset }) => {
  const lines = new Array(Math.sqrt(board.length)).fill(0);
  const win = winner !== 0 ? <Winning player={winner} onReset={onReset} /> : null;
  return (
    <React.Fragment>
      {win}
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
    </React.Fragment>
  );
};

// TODO: button to reset with same properties
// TODO: button to go back to selection screen
const Winning = ({ player, onReset }) => (
  <div
    style={{
      width: '100vw',
      height: '100vh',
      position: 'fixed',
      top: 0,
      left: 0,
      backgroundColor: '#00000050',
      zIndex: 2001
    }}
  >
    <div
      style={{
        width: '100%',
        height: '100%',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'column'
      }}
    >
      <div style={{ marginBottom: '20px', width: '200px', height: '200px' }}>
        {player === 1 ? (
          <WhiteStone scale={5} occupied={true} />
        ) : (
          <BlackStone scale={5} occupied={true} />
        )}
      </div>
      <span style={{ fontSize: '9em', color: '#ffffff' }}>winner!</span>
      <button onClick={onReset}>Click me !</button>
    </div>
  </div>
);

const Cleanup = () => (
  <React.Fragment>
    <div
      style={{
        width: '34px',
        position: 'absolute',
        left: 0,
        top: 0,
        bottom: 0,
        zIndex: 1000,
        background: '#c48e54'
      }}
    />
    <div
      style={{
        height: '34px',
        position: 'absolute',
        left: 0,
        top: 0,
        right: 0,
        zIndex: 1000,
        background: '#c48e54'
      }}
    />
    <div
      style={{
        height: '34px',
        position: 'absolute',
        left: 0,
        bottom: 0,
        right: 0,
        zIndex: 1000,
        background: '#c48e54'
      }}
    />
    <div
      style={{
        width: '34px',
        position: 'absolute',
        right: 0,
        top: 0,
        bottom: 0,
        zIndex: 1000,
        background: '#c48e54'
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
      {block === 3 && <Forbidden/>}
      <WhiteStone hover={hover && player === 1} occupied={block === 1} />
      <BlackStone hover={hover && player === 2} occupied={block === 2} />
    </div>
  );
};

const cross = (
  <svg height="40px" width="40px" style={{ position: 'absolute' }}>
    <path stroke="#000000" strokeWidth="2" d="M0,20H40M20,0V40" />
  </svg>
);

const WhiteStone = ({ hover, occupied, scale = 1 }) => (
  <svg
    style={{
      overflow: 'hidden',
      position: 'absolute',
      opacity: occupied ? 1 : hover ? 0.6 : 0,
      zIndex: 2000
    }}
    transform={`scale(${scale})`}
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

const BlackStone = ({ occupied, hover, scale = 1 }) => (
  <svg
    style={{
      overflow: 'hidden',
      position: 'absolute',
      opacity: occupied ? 1 : hover ? 0.6 : 0,
      zIndex: 2000
    }}
    transform={`scale(${scale})`}
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
    />
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
    />
  </svg>
);

const Forbidden = () => (
  <svg
    style={{
      overflow: 'hidden',
      position: 'absolute',
      opacity: 1,
      zIndex: 2000
    }}
    height="40px"
    width="40px"
    viewBox="0 0 448 512"
  >
    <path fill="red"
          d="M439.15 453.06L297.17 384l141.99-69.06c7.9-3.95 11.11-13.56 7.15-21.46L432 264.85c-3.95-7.9-13.56-11.11-21.47-7.16L224 348.41 37.47 257.69c-7.9-3.95-17.51-.75-21.47 7.16L1.69 293.48c-3.95 7.9-.75 17.51 7.15 21.46L150.83 384 8.85 453.06c-7.9 3.95-11.11 13.56-7.15 21.47l14.31 28.63c3.95 7.9 13.56 11.11 21.47 7.15L224 419.59l186.53 90.72c7.9 3.95 17.51.75 21.47-7.15l14.31-28.63c3.95-7.91.74-17.52-7.16-21.47zM150 237.28l-5.48 25.87c-2.67 12.62 5.42 24.85 16.45 24.85h126.08c11.03 0 19.12-12.23 16.45-24.85l-5.5-25.87c41.78-22.41 70-62.75 70-109.28C368 57.31 303.53 0 224 0S80 57.31 80 128c0 46.53 28.22 86.87 70 109.28zM280 112c17.65 0 32 14.35 32 32s-14.35 32-32 32-32-14.35-32-32 14.35-32 32-32zm-112 0c17.65 0 32 14.35 32 32s-14.35 32-32 32-32-14.35-32-32 14.35-32 32-32z"
    />
  </svg>
)

const blockStyle = {
  width: '40px',
  height: '40px',
  cursor: 'pointer'
};

const lineStyle = {
  display: 'flex',
  maxWidth: 'max-content'
};

const boardStyle = {
  backgroundColor: '#c48e54',
  position: 'relative',
  maxWidth: 'max-content',
  padding: '15px',
  margin: 'auto',
  boxShadow: '0 5px 10px rgba(0,0,0,0.20), 0 7px 20px rgba(0,0,0,0.15)',
  border: '6px outset #c7977a'
};
