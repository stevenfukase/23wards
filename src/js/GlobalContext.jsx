import React, { useState, createContext } from 'react';
import PropTypes from 'prop-types';

const GlobalContext = createContext();

export default function GlobalProvider({ children }) {
  const [test] = useState(true);

  return (
    <GlobalContext.Provider value={test}>
      {children}
    </GlobalContext.Provider>
  );
}

GlobalProvider.propTypes = {
  children: PropTypes.node.isRequired,
};
