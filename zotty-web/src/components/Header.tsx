import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './Header.sass';
import { login } from "../utils/login";
import { DiscordUser } from "../types";
import { getAvatarUrl } from "../utils/discord";

interface HeaderProps extends RouteComponentProps {
  user: DiscordUser | undefined
};
interface HeaderStates{
  avatarUrl: string | undefined
};
class Header extends React.Component<HeaderProps, HeaderStates> {
  constructor(props: HeaderProps){
    super(props);
    this.state = {avatarUrl: undefined};
  }
  componentDidUpdate(prevProps: HeaderProps, prevState: HeaderStates) {
    if (this.props.user !== prevProps.user) {
      if (this.props.user) {
        // we just logged in
        console.log("got avatar url")
        let avatarUrl = getAvatarUrl(this.props.user, 64);
        console.log(avatarUrl);
        this.setState({avatarUrl});
      }
    }
  }
  render() {
    return (
      <div>
        <div id="header-div">
          <div id="login-btn" className="btn" onClick={login}>
            <span id="login-text">LOG IN</span>
            <img id="header-avatar" src={this.state.avatarUrl} alt="" width="32" height="32"/>
          </div>
        </div>
        {/*render all child components bellow*/}
        {this.props.children}
      </div>
    );
  };
}
export default withRouter(Header);