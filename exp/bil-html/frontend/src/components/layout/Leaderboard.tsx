import React from 'react';
// import { Win98Window } from '../windows/Win98Window';
import Win98Window from '../windows/Win98Window';
const Leaderboard = () => {
  return (
    <Win98Window
      title="Leaderboard"
      icon="/icons/trophy.png"
      onClose={() => {}}
    >
      <div className="p-4">
        {/* Leaderboard content */}
        <h2 className="text-lg font-bold mb-4">Top Miners</h2>
        <div className="space-y-2">{/* Add leaderboard items here */}</div>
      </div>
    </Win98Window>
  );
};

export default Leaderboard;
