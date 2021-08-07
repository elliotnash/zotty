import React from "react";
import {
  BrowserRouter as Router,
  Route
} from "react-router-dom";
import { DiscordUser } from "./types";
import { login } from "./utils/login";
import Header from "./components/Header";
import Home from "./routes/Home";
import Authorize from "./routes/Authorize";

class Login extends React.Component {
  render() {
    return (
      <React.Fragment>
        <span>LOGIN</span>
        <br/>
        <button onClick={login}>login</button>
      </React.Fragment>
    );
  };
}

declare global {
  interface Window { authorize: {(user: DiscordUser):void};}
}
interface AppProps{}
interface AppStates{
  user: DiscordUser | undefined
}
export default class App extends React.Component<AppProps, AppStates> {

  headerRef: React.Ref<typeof Header>;

  constructor(props: AppProps){
    super(props);
    // create header ref
    this.headerRef = React.createRef();
    // set authorize attribute in window
    // we need to use bind so function still has access
    // to setState when called from other contexts
    window.authorize = this.authorize.bind(this);
    // add user state
    this.state = {
      user: undefined
    };
  }

  authorize(user: DiscordUser) {
    console.log("AUTHORIZE FUCKTION CALLED");
    this.setState({ user });
  }

  render() {
    return (
      <Router>
        <Route path="/authorize" exact>
          <Authorize/>
        </Route>
        <Route path="/" exact>
          <Home user={this.state.user}/>
        </Route>
        <Route path="/login" exact>
          <Login/>
        </Route>
      </Router>
    );
  }
}
