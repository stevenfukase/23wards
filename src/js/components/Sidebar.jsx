import React from 'react';

export default function Sidebar(props) {
  return (
    <aside
      className={`transform top-0 left-0 w-64 bg-white fixed h-full overflow-auto ease-in-out transition-all duration-300 z-30 ${props.isOpen ? 'translate-x-0' : '-translate-x-full'}`}
    >
      Sidebar
    </aside>
  );
}
