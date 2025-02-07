// components/ui/Win98Button.jsx
const Win98Button = ({
  children,
  onClick,
  className,
}: {
  children: React.ReactNode;
  onClick: () => void;
  className: string;
}) => (
  <button
    onClick={onClick}
    className={`px-4 py-1 text-sm bg-secondary shadow-win98 active:shadow-win98-inset ${className}`}
  >
    {children}
  </button>
);

export default Win98Button;
