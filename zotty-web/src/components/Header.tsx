import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './Header.sass';
import { login } from "../utils/login";

interface HeaderProps extends RouteComponentProps {};
interface HeaderStates{};
class Header extends React.Component<HeaderProps, HeaderStates> {
  render() {
    return (
      <div>
        <div id="header-div">
          <button id="login-btn" className="btn">LOG IN</button>
        </div>
        {/*render all child components bellow*/}
        {this.props.children}
      </div>
    );
  };
}
export default withRouter(Header);