import React, { useState } from 'react';
// import { Win98Button } from '../ui/Win98Button';
import Win98Button from '../ui/Win98Button';
import windowsLogo from '../../assets/windows-logo.png'; // You'll need this image

interface StartMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

const StartMenu: React.FC<StartMenuProps> = ({ isOpen, onClose }) => {
  if (!isOpen) return null;

  return (
    <div className="absolute bottom-full left-0 mb-1 w-64 bg-[#c0c0c0] shadow-win98 border-2 border-[#dfdfdf]">
      {/* Add your start menu items here */}
      <div
        className="p-2 hover:bg-primary hover:text-white cursor-pointer"
        onClick={onClose}
      >
        Programs
      </div>
      <div
        className="p-2 hover:bg-primary hover:text-white cursor-pointer"
        onClick={onClose}
      >
        Settings
      </div>
      {/* Add more menu items as needed */}
    </div>
  );
};

const StartButton: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <div className="relative">
      <Win98Button
        onClick={() => setIsMenuOpen(!isMenuOpen)}
        className="flex items-center gap-2 px-2 py-1"
      >
        <img src={windowsLogo} alt="Windows" className="w-4 h-4" />
        <span>Start</span>
      </Win98Button>
      <StartMenu isOpen={isMenuOpen} onClose={() => setIsMenuOpen(false)} />
    </div>
  );
};

export default StartButton;
