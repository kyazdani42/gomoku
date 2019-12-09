import React, { useEffect, useState } from 'react';

import { Board } from './Board';
import { GameSelection } from './GameSelection';
// import { Info } from "./Info";

export const Gomoku = () => {
  const [state, setState] = useState(null);
  const [error, setError] = useState(null);
  const [positions, setPositions] = useState(null);
  const [initParam, setInitParam] = useState(null);

  useEffect(() => {
    if (initParam) handleInit(initParam, setState, setError);
  }, [initParam]);

  useEffect(() => {
    if (positions) handlePlay(positions, setState, setError);
  }, [positions]);

  if (!state) return <GameSelection setInitParam={setInitParam} />;

  return (
    <div style={{ paddingTop: '50px' }}>
      <Board
        winner={state.winner}
        board={state.board.flat()}
        player={state.player}
        onClick={payload => {
          const newBoard = JSON.parse(JSON.stringify(state.board));
          const newLine = Math.floor(payload / state.board.length);
          const newCol = payload % state.board.length;
          if (newBoard[newLine][newCol] === 0) {
            newBoard[newLine][newCol] = state.player;
            setState({ ...state, board: newBoard });
            setPositions({
              line: newLine,
              col: newCol
            });
          }
        }}
      />
      {/* <Info /> */}
      <button onClick={() => playIa(setState, setError)}>
        Play Ia
      </button>
    </div>
  );
};

const handleInit = async (initParam, setState, setError) => {
  const { ia, size } = initParam;
  const res = await fetch(getInitUrl({ ia, size }));
  const { ok, headers } = res;
  if (ok && headers.get('Content-Type') === 'application/json') {
    setState(await res.json());
  } else {
    setError();
  }
};

const URL = 'http://localhost:3001';

const getInitUrl = ({ ia, size }) => `${URL}/init?ia=${ia}&size=${size}`;

const handlePlay = async (positions, setState, setError) => {
  const { line, col } = positions;
  const res = await fetch(getPlayUrl({ line, col }));
  const { ok, headers } = res;
  if (ok && headers.get('Content-Type') === 'application/json') {
    setState(await res.json());
  } else {
    setError();
  }
};

const getPlayUrl = ({ line, col }) => `${URL}/play?line=${line}&col=${col}`;

const getPlayIa = () => `${URL}/play_ia`;

const playIa = async (setState, setError) => {
  const res = await fetch(getPlayIa());
  const { ok, headers } = res;
  if (ok && headers.get('Content-Type') === 'application/json') {
    setState(await res.json());
  } else {
    setError();
  }
}
