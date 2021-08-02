import React from "react";
import {
  BrowserRouter as Router,
  Route,
  Link
} from "react-router-dom";
import { login } from "./utils/login";
import Authorize from "./routes/Authorize";

class Index extends React.Component {
  login() { login(); };
  render() {
    return (
      <div>
        <span>HOME</span>
        <br/>
        <button onClick={this.login}>login</button>
      </div>
    );
  };
}

class Login extends React.Component {
  render() {
    return (
      <div></div>
    );
  };
  componentDidMount() { login("/") };
}

export default class App extends React.Component {
  render() {
    return (
      <Router>
        <div>
          <Route path="/" exact component={Index}/>
          <Route path="/login" exact component={Login}/>
          <Route path="/authorize" exact component={Authorize}/>
        </div>
      </Router>
    );
  }
}
