import React from "react";
import {
  BrowserRouter as Router,
  Route
} from "react-router-dom";
import type { DiscordUser } from "./utils/request";
import { authorize, cookieLogin, newLogin } from "./utils/auth";
import Home from "./routes/Home";
import Servers from "./routes/Servers";
import Authorize from "./routes/Authorize";
import Cookies from "universal-cookie";
import Header from "./components/Header";
import axios from "axios";
import EventEmitter from "eventemitter3";

class Login extends React.Component {
  render() {
    return (
      <React.Fragment>
        <span>LOGIN</span>
        <br/>
        <button onClick={newLogin}>login</button>
      </React.Fragment>
    );
  }
}

declare global {
  interface Window {
    emiter: EventEmitter
  }
}
interface AppProps{}
interface AppStates{
  user: DiscordUser | undefined
}
export default class App extends React.Component<AppProps, AppStates> {

  cookies = new Cookies();

  constructor(props: AppProps){
    super(props);
    // add emiter to window and listen to login and logout
    window.emiter = new EventEmitter();
    // add user state
    this.state = {
      user: this.cookies.get("user")
    };
  }

  componentDidMount(): void {
    window.emiter.on('login', this.login.bind(this));
    window.emiter.on('logout', this.logout.bind(this));
    window.emiter.on('authorize', this.authorize.bind(this));
    // on page load try to log in using cookies
    cookieLogin().then(() => {
      console.log("page loaded, firing loaded event");
      window.emiter.emit('loaded');
    });
  }

  login(user: DiscordUser): void {
    console.log(`LOGIN FUCKTION CALLED WITH DATA: ${JSON.stringify(user)}`);
    this.setState({ user });
    console.log("axios default (in app)");
    console.log(axios.defaults.headers?.common);
  }
  logout(): void {
    console.log("LOGOUT FUCKTION CALLED");
    this.setState({ user: undefined });
  }
  authorize(code: string, state: string): void {
    authorize(code, state);
  }

  render(): React.ReactNode {
    return (
      <Router>
        <div style={{display: "flex", flexDirection: "column", height: "100vh"}} >
          <Header user={this.state.user}>
            <Route path="/authorize" exact>
              <Authorize/>
            </Route>
            <Route path="/" exact>
              <Home/>
            </Route>
            <Route path="/login" exact>
              <Login/>
            </Route>
            <Route path="/servers" exact>
              <Servers/>
            </Route>
          </Header>
        </div>
      </Router>
    );
  }
}
