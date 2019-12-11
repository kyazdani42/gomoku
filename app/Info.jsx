import React from 'react';

export const Info = ({ p1_captured, p2_captured }) => (
  <div style={{ height: '300px', width: '200px', backgroundColor: '#ffffff' }}>
    <span>
      whites captured <b>{p1_captured}</b> stones
    </span>
    <br />
    <span>
      blacks captured <b>{p2_captured}</b> stones
    </span>
  </div>
);
