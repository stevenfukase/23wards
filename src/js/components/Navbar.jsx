import React, { useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faBars, faTimes } from '@fortawesome/free-solid-svg-icons';
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
        className="flex fixed w-full items-center justify-between px-6 h-16 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-white border-b border-gray-200 dark:border-gray-400"
      >

        <button
          type="button"
          className="text-2xl focus:outline-none"
          onClick={() => setIsOpenSidebar(true)}
        >
          <FontAwesomeIcon icon={faBars} />
        </button>
        <div
          className="hidden md:block md:flex md:justify-between md:bg-transparent"
        >
          <p className="text-gray-400">By STEVEN FUKASE</p>
        </div>
      </nav>
      <aside
        className={`transform top-0 left-0 w-64 bg-white text-gray-700 fixed h-full overflow-auto ease-in-out transition-all duration-300 px-6 py-4 border-r border-gray-200 z-30 ${isOpenSidebar ? 'translate-x-0' : '-translate-x-full'}`}
      >
        <button
          type="button"
          className="text-3xl focus:outline-none"
          onClick={() => setIsOpenSidebar(false)}
        >
          <FontAwesomeIcon icon={faTimes} />
        </button>
        <div>
          <h2 className="mt-2 text-2xl font-bold">Settings</h2>
          <h3 className="mt-2 text-lg font-bold">Exclude:</h3>

          {wards.map((ward) => (
            <div key={ward.id}>
              <label
                htmlFor={ward.ward}
                className="inline-flex items-center"
              >
                <input
                  type="checkbox"
                  className="mr-1 h-4 w-4"
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
