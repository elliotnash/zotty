import React from "react";
import {
  BrowserRouter as Router,
  Route
} from "react-router-dom";
import { DiscordUser } from "./utils/request";
import { cookieLogin, newLogin } from "./utils/auth";
import Home from "./routes/Home";
import Servers from "./routes/Servers";
import Authorize from "./routes/Authorize";
import Cookies from "universal-cookie";
import Header from "./components/Header";

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
    login: (user: DiscordUser)=>void,
    logout: ()=>void;
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
    // set login and logout attributes in window
    // we need to use bind so function still has access
    // to setState when called from other contexts
    window.login = this.login.bind(this);
    window.logout = this.logout.bind(this);
    // add user state
    this.state = {
      user: this.cookies.get("user")
    };
  }

  componentDidMount(): void {
    // on page load try to log in using cookies
    cookieLogin();
  }

  login(user: DiscordUser): void {
    console.log(`LOGIN FUCKTION CALLED WITH DATA: ${JSON.stringify(user)}`);
    this.setState({ user });
  }
  logout(): void {
    console.log("LOGOUT FUCKTION CALLED");
    this.setState({ user: undefined });
  }

  render(): React.ReactNode {
    return (
      <Router>
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
      </Router>
    );
  }
}
