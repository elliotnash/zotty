import React from "react";
import {
  BrowserRouter as Router,
  Route,
  RouteComponentProps
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

declare global {
  interface Window { authorize: {():void};}
}
interface AuthorizeProps{}
interface AuthorizeStates{}
export default class App extends React.Component<AuthorizeProps, AuthorizeStates> {

  constructor(props: AuthorizeProps){
    super(props);
    window.authorize = this.authorize;
}

  authorize() {
    console.log("AUTHORIZE FUCKTION CALLED");
  }

  render() {
    return (
      <Router>
        <div>
          <Route path="/" exact component={Index}/>
          <Route path="/authorize" exact component={Authorize}/>
        </div>
      </Router>
    );
  }
}
