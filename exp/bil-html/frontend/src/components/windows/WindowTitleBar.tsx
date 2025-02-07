// components/windows/WindowTitleBar.jsx
import React from 'react';

const WindowTitleBar = ({
  title,
  icon,
  onClose,
}: {
  title: string;
  icon: string;
  onClose: () => void;
}) => (
  <div className="flex items-center justify-between bg-primary text-primary-foreground px-2 py-1">
    <span>{title}</span>
    <button onClick={onClose}>Close</button>
  </div>
);

export default WindowTitleBar;
