import React from "react";
import {
  BrowserRouter as Router,
  Route
} from "react-router-dom";
import { DiscordUser } from "./types";
import { cookieLogin, newLogin } from "./utils/auth";
import Home from "./routes/Home";
import Authorize from "./routes/Authorize";
import Cookies from "universal-cookie";

class Login extends React.Component {
  render() {
    return (
      <React.Fragment>
        <span>LOGIN</span>
        <br/>
        <button onClick={newLogin}>login</button>
      </React.Fragment>
    );
  };
}

declare global {
  interface Window {
    login: (user: DiscordUser)=>void,
    logout: ()=>void;
  }
}
interface AppProps{}
interface AppStates{
  user: DiscordUser | undefined,
  loaded: boolean
}
export default class App extends React.Component<AppProps, AppStates> {

  cookies = new Cookies();

  constructor(props: AppProps){
    super(props);
    // set login and logout attributes in window
    // we need to use bind so function still has access
    // to setState when called from other contexts
    window.login = this.login.bind(this);
    window.logout = this.logout.bind(this);
    // add user state
    this.state = {
      user: undefined,
      loaded: false
    };
  }

  componentDidMount() {
    // on page load try to log in using cookies
    cookieLogin().then(() => {
      console.log("page should now be loaded");
    });
  }

  login(user: DiscordUser) {
    console.log(`LOGIN FUCKTION CALLED WITH DATA: ${JSON.stringify(user)}`);
    this.setState({ user });
  }
  logout() {
    console.log("LOGOUT FUCKTION CALLED");
    this.setState({ user: undefined });
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
