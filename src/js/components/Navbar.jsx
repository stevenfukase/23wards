import React, { useState } from 'react';
import wards from '../../wards.json';
import { useWard } from '../GlobalContext';

export default function Navbar() {
  const { dispatch } = useWard();
  const [isOpenSidebar, setIsOpenSidebar] = useState(false);

  const handleCheckboxChange = (e) => {
    if (e.target.checked === true) {
      dispatch({ type: 'REMOVE_WARD', payload: e.target.value });
    }

    if (e.target.checked === false) {
      dispatch({ type: 'ADD_WARD', payload: e.target.value });
    }
  };

  return (
    <>
      <nav
        className="flex fixed w-full items-center justify-between px-6 h-16 bg-white text-gray-700 border-b border-gray-200"
      >

        <button
          type="button"
          onClick={() => setIsOpenSidebar(true)}
        >
          OPEN SIDEBAR
        </button>
        <div
          className="hidden md:block md:flex md:justify-between md:bg-transparent"
        >
          <p className="text-gray-400">By STEVEN FUKASE</p>
        </div>
      </nav>
      <aside
        className={`transform top-0 left-0 w-64 bg-white fixed h-full overflow-auto ease-in-out transition-all duration-300 p-6 border-r border-gray-200 z-30 ${isOpenSidebar ? 'translate-x-0' : '-translate-x-full'}`}
      >
        <button
          type="button"
          onClick={() => setIsOpenSidebar(false)}
        >
          Close SIDEBAR
        </button>
        <div>
          <h2>Settings</h2>
          <h3>Exclude following places</h3>

          {wards.map((ward) => (
            <div key={ward.id}>
              <label htmlFor={ward.ward}>
                <input
                  type="checkbox"
                  id={ward.ward}
                  value={ward.id}
                  onChange={handleCheckboxChange}
                />
                {ward.ward}
              </label>
            </div>
          ))}

        </div>
      </aside>
    </>
  );
}
