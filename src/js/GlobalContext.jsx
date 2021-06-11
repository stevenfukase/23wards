import React, {
  createContext, useReducer, useContext,
} from 'react';
import PropTypes from 'prop-types';
import wards from '../wards.json';

const GlobalContext = createContext();

const randomIntFromInterval = (min, max) => Math.floor(Math.random() * (max - min + 1) + min);

const generateReducer = (state, action) => {
  switch (action.type) {
    case 'GENERATE': {
      const randInt = randomIntFromInterval(0, 22);
      return wards[randInt];
    }
    default: {
      throw new Error(`Unhandled action type: ${action.type}`);
    }
  }
};

export function GlobalProvider({ children }) {
  const [state, dispatch] = useReducer(generateReducer, null);
  const value = { state, dispatch };
  return (
    <GlobalContext.Provider value={value}>
      {children}
    </GlobalContext.Provider>
  );
}

export function useWard() {
  const context = useContext(GlobalContext);
  if (context === undefined) {
    throw new Error('useWards must be used within a GlobalProvider');
  }
  return context;
}

GlobalProvider.propTypes = {
  children: PropTypes.node.isRequired,
};
