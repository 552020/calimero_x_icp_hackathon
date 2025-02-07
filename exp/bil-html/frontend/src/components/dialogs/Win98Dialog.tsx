import React from 'react';

interface Win98DialogProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
}

const Win98Dialog: React.FC<Win98DialogProps> = ({
  isOpen,
  onClose,
  title,
  children,
}) => {
  if (!isOpen) return null;

  return (
    <dialog className="fixed inset-0 bg-transparent p-0 m-0 h-full w-full backdrop:bg-black/50">
      <div className="min-h-full w-full p-6 flex items-center justify-center">
        <div className="window shadow-win98 bg-[#c0c0c0] border-2 border-[#dfdfdf]">
          <div className="flex items-center justify-between bg-[#000080] text-white px-2 py-1">
            <span>{title}</span>
            <button
              onClick={onClose}
              className="text-white hover:bg-red-600 px-2"
            >
              ×
            </button>
          </div>
          <div className="p-4">{children}</div>
        </div>
      </div>
    </dialog>
  );
};

export default Win98Dialog;
