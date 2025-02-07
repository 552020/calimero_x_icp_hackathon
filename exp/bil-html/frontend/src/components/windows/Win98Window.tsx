// components/windows/Win98Window.jsx

import React from 'react';
import WindowTitleBar from './WindowTitleBar';
// import WindowResizeHandles from './WindowResizeHandles';
import WindowResizeHandles from './windowResizeHandles';

const Win98Window = ({
  title,
  icon,
  children,
  onClose,
}: {
  title: string;
  icon: string;
  children: React.ReactNode;
  onClose: () => void;
}) => (
  <div className="window shadow-win98 bg-[#c0c0c0] border-2 border-[#dfdfdf] shadow-[inset_-1px_-1px_#0a0a0a,inset_1px_1px_#fff]">
    <WindowTitleBar title={title} icon={icon} onClose={onClose} />
    <div className="window-content p-4">{children}</div>
    <WindowResizeHandles />
  </div>
);

export default Win98Window;
