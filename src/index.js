import React from 'react';
import ReactDOM from 'react-dom/client'; // ✅ Correct import for React 18
import './index.css';
import App from './App';
import { store } from './storeManager/store';
import { Provider } from 'react-redux';
import * as serviceWorker from './serviceWorker';

const root = ReactDOM.createRoot(
  document.getElementById('root')
);
root.render(
  <React.StrictMode>
    <Provider store={store}>
      <App />
    </Provider>
  </React.StrictMode>
  );

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
