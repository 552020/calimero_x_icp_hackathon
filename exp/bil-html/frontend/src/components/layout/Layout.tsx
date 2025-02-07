import React from 'react';
import Taskbar from './Taskbar';
import BackgroundCorner from './BackgroundCorner';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => (
  <div className="min-h-screen bg-secondary">
    <Taskbar />
    <main className="p-4">{children}</main>
    <BackgroundCorner />
  </div>
);

export default Layout;
