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
  avatarUrl: string | undefined,
  avatar: boolean
};
class Header extends React.Component<HeaderProps, HeaderStates> {
  constructor(props: HeaderProps){
    super(props);
    this.state = {
      avatarUrl: undefined,
      avatar: false
    };
  }
  componentDidUpdate(prevProps: HeaderProps, prevState: HeaderStates) {
    if (this.props.user !== prevProps.user) {
      if (this.props.user) {
        // we just logged in
        console.log("got avatar url")
        let avatarUrl = getAvatarUrl(this.props.user, 64);
        console.log(avatarUrl);
        this.setState({avatarUrl});
        // set toAvatar state to 1 to start animation
        this.setState({avatar: true});
      }
    }
  }
  render() {
    return (
      <React.Fragment>
        <div id="header-div">
          <div id="login-btn-container">
            <div id="login-btn" className="btn" data-avatar={this.state.avatar} onClick={login}>
              <span id="login-text" data-avatar={this.state.avatar}>LOG IN</span>
              <img id="header-avatar" data-avatar={this.state.avatar} onClick={this.avatarClick}
                src={this.state.avatarUrl} alt=""/>
            </div>
          </div>
        </div>
        {/*render all child components bellow*/}
        {this.props.children}
      </React.Fragment>
    );
  };
  avatarClick(event: React.MouseEvent) {
    event.stopPropagation()
    console.log("AVATAR CLICKKKKKEEEEDDD");
  }
}
export default withRouter(Header);