import React, { useReducer } from "react";

import { Board } from "./Board";
// import { Info } from "./Info";

const reducer = (state, action) => {
  switch (action.type) {
    case "PLAY":
      if (state.board[action.payload] !== 0) return state;
      const newBoard = state.board.slice();
      newBoard[action.payload] = state.player;
      return {
        ...state,
        player: state.player === 1 ? 2 : 1,
        board: newBoard
      };
    default:
      return state;
  }
};

const initialState = {
  board: new Array(19 * 19).fill(0),
  player: 1
};

export const Gomoku = () => {
  // TEMPORARY
  const [state, dispatch] = useReducer(reducer, initialState);

  // useEffect to get initial data
  // return GameSelection

  return (
    <div>
      <Board
        board={state.board}
        player={state.player}
        onClick={payload => dispatch({ type: "PLAY", payload })}
      />
      {/* <Info /> */}
    </div>
  );
};
