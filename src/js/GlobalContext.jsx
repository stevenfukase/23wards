import React, {
  createContext, useReducer, useContext,
} from 'react';
import PropTypes from 'prop-types';
import wards from '../wards.json';

const GlobalContext = createContext();

const initialState = wards;

const excludePlaceReducer = (state, action) => {
  switch (action.type) {
    case 'removeItem': {
      return state.filter((id) => id !== action.payload);
    }
    case 'addItem': {
      return [...state];
    }
    default: {
      throw new Error(`Unhandled action type: ${action.type}`);
    }
  }
};

export function GlobalProvider({ children }) {
  const [state, dispatch] = useReducer(excludePlaceReducer, initialState);
  const value = { state, dispatch };
  return (
    <GlobalContext.Provider value={value}>
      {children}
    </GlobalContext.Provider>
  );
}

export function useWards() {
  const context = useContext(GlobalContext);
  if (context === undefined) {
    throw new Error('useWards must be used within a GlobalProvider');
  }
  return context;
}

GlobalProvider.propTypes = {
  children: PropTypes.node.isRequired,
};
