import React from 'react';
import ReactDOM from 'react-dom';
import './index.sass';
import App from './App';

export const BACKEND_URL = get_backend_url();
function get_backend_url(): string {
  if (window.location.port === "3000")
    return "http://localhost:8000";
  else
    return window.location.origin;
}

ReactDOM.render(
  <React.StrictMode>
    <App/>
  </React.StrictMode>,
  document.getElementById('root')
);
