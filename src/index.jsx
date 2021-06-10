import React from 'react';
import ReactDOM from 'react-dom';
import './styles/index.css';
import App from './js/App';
import GlobalContext from './js/GlobalContext';

ReactDOM.render(

  <React.StrictMode>
    <GlobalContext>
      <App />
    </GlobalContext>
  </React.StrictMode>,

  document.getElementById('root'),
);
