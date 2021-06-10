import React, { useState } from 'react';

export default function Navbar() {
  const [isOpenSidebar, setIsOpenSidebar] = useState(false);
  return (
    <>
      <nav
        className="flex fixed w-full items-center justify-between px-6 h-16 bg-white text-gray-700 border-b border-gray-200 z-10"
      >
        <div>
          <button
            type="button"
            onClick={() => setIsOpenSidebar(true)}
          >
            OPEN SIDEBAR
          </button>
        </div>

        <div
          className="hidden md:block md:flex md:justify-between md:bg-transparent"
        >
          SIDEBAR
        </div>
      </nav>
      <aside
        className={`transform top-0 left-0 w-64 bg-white fixed h-full overflow-auto ease-in-out transition-all duration-300 z-30 ${isOpenSidebar ? 'translate-x-0' : '-translate-x-full'}`}
      >
        <button
          type="button"
          onClick={() => setIsOpenSidebar(false)}
        >
          Close SIDEBAR
        </button>
      </aside>
    </>
  );
}
