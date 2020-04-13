import React, { useEffect, useState } from 'react';

import { Board } from './Board';
import { GameSelection } from './GameSelection';
import { Info } from './Info';

export const Gomoku = () => {
  const [state, setState] = useState(null);
  const [error, setError] = useState(null);
  const [positions, setPositions] = useState(null);
  const [initParam, setInitParam] = useState(null);

  useEffect(() => {
    if (initParam) handleInit(initParam, setState, setError);
  }, [initParam]);

  useEffect(() => {
    if (positions !== null) handlePlay(positions, setState, setError);
  }, [positions]);

  if (state === null) return <GameSelection setInitParam={setInitParam} />;

  return (
    <div style={{ paddingTop: '50px' }}>
      <Board
        winner={state.winner}
        board={state.board}
        player={state.player}
        onReset={() => setState(null)}
        onClick={(payload) => {
          let tile = state.board[payload];
          if (tile === 0 || tile === 5) {
            const newBoard = state.board.slice();
            newBoard[payload] = state.player;
            setState({ ...state, board: newBoard });
            setPositions(payload);
          }
        }}
      />
      <Info {...state} />
      <button onClick={() => playIa(setState, setError)}>Play Ia</button>
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

const handlePlay = async (index, setState, setError) => {
  const res = await fetch(getPlayUrl(index));
  const { ok, headers } = res;
  if (ok && headers.get('Content-Type') === 'application/json') {
    setState(await res.json());
  } else {
    setError();
  }
};

const getPlayUrl = (index) => `${URL}/play?index=${index}`;

const getPlayIa = () => `${URL}/play_ia`;

const playIa = async (setState, setError) => {
  const res = await fetch(getPlayIa());
  const { ok, headers } = res;
  if (ok && headers.get('Content-Type') === 'application/json') {
    setState(await res.json());
  } else {
    setError();
  }
};
