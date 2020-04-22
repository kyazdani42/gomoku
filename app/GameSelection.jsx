import React, { useState } from 'react';

export const GameSelection = ({ setInitParam }) => {
  const [ia, setIa] = useState(0);
  const [level, setLevel] = useState(1);
  return (
    <div>
      <form onSubmit={handleSubmit(setInitParam)(ia, level)}>
        <label>ia (0: disabled, 1: whites, 2: blacks)</label>
        <select value={ia} onChange={(e) => setIa(e.target.value)}>
          {[0, 1, 2].map((v) => (
            <option value={v} key={v}>
              {v}
            </option>
          ))}
        </select>
        <label>ia level</label>
        <select value={level} onChange={(e) => setLevel(e.target.value)}>
          {[1, 2, 3, 4, 5].map((v) => (
            <option value={v} key={v}>
              {v}
            </option>
          ))}
        </select>
        <input type="submit" value="start game" />
      </form>
    </div>
  );
};

const handleSubmit = (setInitParam) => (ia, level) => (e) => {
  e.preventDefault();
  setInitParam({ ia, level });
};
