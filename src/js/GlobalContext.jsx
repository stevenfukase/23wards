import React, {
  useState, createContext, useReducer, useContext,
} from 'react';
import PropTypes from 'prop-types';
import wards from '../wards.json';

const GlobalContext = createContext();

export function GlobalProvider({ children }) {
  const [wardsArray] = useState(wards);

  const wardReducer = (state, action) => {
    switch (action.type) {
      case 'ADD_WARD': {
        const randInt = randomIntFromInterval(0, wardsArray.length - 1);
        return wardsArray[randInt];
      }
      case 'REMOVE_WARD': {
        const randInt = randomIntFromInterval(0, wardsArray.length - 1);
        return wardsArray[randInt];
      }
      default: {
        throw new Error(`Unhandled action type: ${action.type}`);
      }
    }
  };

  const [state, dispatch] = useReducer(wardReducer, null);
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
