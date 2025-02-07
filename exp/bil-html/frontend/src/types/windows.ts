// types/windows.ts
export interface Win98Window {
  id: string;
  title: string;
  icon?: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  isMaximized: boolean;
  isMinimized: boolean;
  zIndex: number;
}

export interface Win98Theme {
  colors: {
    desktop: string;
    window: string;
    primary: string;
    secondary: string;
    border: {
      light: string;
      dark: string;
      darker: string;
    };
  };
  shadows: {
    raised: string;
    sunken: string;
  };
}
