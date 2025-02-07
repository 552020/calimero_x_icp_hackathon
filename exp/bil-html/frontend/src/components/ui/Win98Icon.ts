// components/ui/Win98Icon.jsx
const Win98Icon = ({ icon, label, onClick }) => (
  <div
    className="flex flex-col items-center gap-1 p-2 cursor-pointer text-white hover:bg-primary/20"
    onClick={onClick}
  >
    <img src={icon} alt={label} className="w-8 h-8" />
    <span className="text-xs text-shadow">{label}</span>
  </div>
);
