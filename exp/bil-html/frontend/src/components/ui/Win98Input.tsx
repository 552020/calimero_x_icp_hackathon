// components/ui/Win98Input.jsx
const Win98Input = ({ type, id, placeholder, ...props }) => (
  <input
    type={type}
    id={id}
    className="w-full bg-white px-2 py-1 text-sm shadow-win98-inset outline-none font-title"
    placeholder={placeholder}
    {...props}
  />
);
