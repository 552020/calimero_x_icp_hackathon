import React from 'react';

const SystemTray: React.FC = () => {
  const currentTime = new Date().toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
  });

  return (
    <div className="flex items-center px-2 py-1 shadow-win98-inset bg-[#c0c0c0] min-w-[70px] h-full">
      <div className="flex items-center gap-2">
        {/* Add your system tray icons here */}
        <img src="/icons/volume.png" alt="Volume" className="w-4 h-4" />
        <span className="text-sm">{currentTime}</span>
      </div>
    </div>
  );
};

export default SystemTray;
