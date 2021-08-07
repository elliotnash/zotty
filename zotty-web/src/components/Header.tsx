import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './Header.sass';
import { login } from "../utils/login";
import { DiscordUser } from "../types";

interface HeaderProps extends RouteComponentProps {
  user: DiscordUser | undefined
};
interface HeaderStates{};
class Header extends React.Component<HeaderProps, HeaderStates> {
  componentDidUpdate(prevProps: HeaderProps, prevState: HeaderStates) {
    if (this.props.user !== prevProps.user) {
      console.log("OHHH DADDDY YOU UPDATED MY USER VAR SO HARDDD");
    }
  }
  render() {
    return (
      <div>
        <div id="header-div">
          <div id="login-btn" className="btn" onClick={login}>LOG IN</div>
        </div>
        {/*render all child components bellow*/}
        {this.props.children}
      </div>
    );
  };
}
export default withRouter(Header);