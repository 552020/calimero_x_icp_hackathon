import React from 'react';

interface TaskbarItem {
  id: string;
  title: string;
  icon?: string;
  isActive: boolean;
}

interface TaskbarItemsProps {
  items?: TaskbarItem[];
}

const TaskbarItems: React.FC<TaskbarItemsProps> = ({ items = [] }) => {
  return (
    <div className="flex-1 flex items-center px-1 gap-1">
      {items.map((item) => (
        <button
          key={item.id}
          className={`
            flex items-center gap-1 px-2 py-1 min-w-[120px] h-[22px] text-left
            ${
              item.isActive
                ? 'shadow-win98-inset bg-[#c0c0c0]'
                : 'shadow-win98 hover:shadow-win98-inset'
            }
          `}
        >
          {item.icon && <img src={item.icon} alt="" className="w-4 h-4" />}
          <span className="truncate text-sm">{item.title}</span>
        </button>
      ))}
    </div>
  );
};

export default TaskbarItems;
