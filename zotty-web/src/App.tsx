import React from "react";
import {
  BrowserRouter as Router,
  Route,
  RouteComponentProps
} from "react-router-dom";
import { login } from "./utils/login";
import Home from "./routes/Home";
import Authorize from "./routes/Authorize";

class Login extends React.Component {
  render() {
    return (
      <div>
        <span>LOGIN</span>
        <br/>
        <button onClick={login}>login</button>
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
        <Route path="/authorize" exact component={Authorize}/>
        <Route path="/" exact component={Home}/>
      </Router>
    );
  }
}
