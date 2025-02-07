import React from 'react';
import StartButton from './StartButton';
import TaskbarItems from './TaskbarItems';
import SystemTray from './SystemTray';

const Taskbar: React.FC = () => {
  return (
    <div className="fixed bottom-0 left-0 right-0 bg-[#c0c0c0] border-t border-[#dfdfdf] shadow-win98 z-50 h-[30px] flex items-center">
      <StartButton />
      <TaskbarItems />
      <SystemTray />
    </div>
  );
};

export default Taskbar;
