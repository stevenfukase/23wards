import React, {
  createContext, useReducer, useContext,
} from 'react';
import PropTypes from 'prop-types';
import wards from '../wards.json';

const GlobalContext = createContext();

export function GlobalProvider({ children }) {
  const wardReducer = (state, action) => {
    switch (action.type) {
      case 'REMOVE_WARD': {
        return state.filter((item) => item.id === action.payload);
      }
      case 'ADD_WARD': {
        return state.push(wards.find((item) => item.id === action.payload));
      }
      default: {
        throw new Error(`Unhandled action type: ${action.type}`);
      }
    }
  };

  const [state, dispatch] = useReducer(wardReducer, wards);
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
