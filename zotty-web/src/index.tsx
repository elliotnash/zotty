import React from 'react';
import ReactDOM from 'react-dom';
import './index.sass';
import App from './App';

export const BACKEND_URL = get_backend_url();
function get_backend_url(): string {
  let dev_url = "http://localhost:8000";
  if (dev_url !== "")
    return dev_url
  else
    return window.location.origin;
}

ReactDOM.render(
  <React.StrictMode>
    <App/>
  </React.StrictMode>,
  document.getElementById('root')
);
