import React from "react";
import {
  BrowserRouter as Router,
  Route,
  Link
} from "react-router-dom";
import Login from "./Login";
import Authorize from "./Authorize";

class Index extends React.Component {
  render() {
    return (
      <div>
        <span>HOME</span>
        <br/>
        <Link to="/login">login</Link>
      </div>
    );
  }
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
