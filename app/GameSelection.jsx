import React, { useState } from 'react';

export const GameSelection = ({ setInitParam }) => {
  const [ia, setIa] = useState(0);
  const [size, setSize] = useState(19);
  return (
    <div>
      <form onSubmit={handleSubmit(setInitParam)(ia, size)}>
        <input
          type="checkbox"
          checked={ia !== 0}
          onChange={() => setIa(ia === 0 ? 1 : 0)}
        />
        <select value={size} onChange={e => setSize(e.target.value)}>
          {[19, 20, 21, 22, 23, 24, 25].map(v => (
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

const handleSubmit = setInitParam => (ia, size) => e => {
  e.preventDefault();
  setInitParam({ ia: 1, size });
};
