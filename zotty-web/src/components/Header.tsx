import React, { createRef } from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './Header.sass';
import UserMenu from "./UserMenu";
import { newLogin } from "../utils/auth";
import { DiscordUser } from "../utils/request";
import { getAvatarUrl } from "../utils/discord";

interface HeaderProps extends RouteComponentProps {
  user: DiscordUser | undefined
}
interface HeaderStates{
  avatarUrl: string,
  menuOpen: boolean
}
class Header extends React.Component<HeaderProps, HeaderStates> {
  constructor(props: HeaderProps){
    super(props);
    this.state = {
      avatarUrl: this.props.user ? getAvatarUrl(this.props.user, 64) : "",
      menuOpen: false
    };
    this.avatarClick = this.avatarClick.bind(this);
  }
  componentDidUpdate(prevProps: HeaderProps): void {
    if (this.props.user !== prevProps.user) {
      if (this.props.user) {
        // we just logged in
        // set toAvatar state to 1 to start animation
        this.setState({avatarUrl: getAvatarUrl(this.props.user, 64)});
      } else {
        // we just logged out
        console.log("header recieved logout");
      }
    }
  }
  avatarRef = createRef<HTMLImageElement>();
  render(): React.ReactNode {
    // array is list of routes to not render header in
    if (["/authorize", "/login"].includes(this.props.location.pathname)) {
      return this.props.children;
    }
    else {
      return (
        <React.Fragment>
          <div id="header-div">
            <span id="title-span">ZOTTY</span>
            <div id="login-container-container" className="container-container">
              <div id="login-btn-container" className="btn-container" data-hidden={!!this.props.user}>
                <div id="login-btn" className="btn btn-animation" data-avatar={!!this.props.user} onClick={newLogin}>
                  <span id="login-text" data-avatar={!!this.props.user}>LOG IN</span>
                </div>
              </div>
              <div id="avatar-btn-container" className="btn-container" data-hidden={!this.props.user}>
                <UserMenu isOpen={this.state.menuOpen} setIsOpen={(menuOpen) => {this.setState({menuOpen});}} openRef={this.avatarRef}/>
                <div className="avatar-container" onClick={this.avatarClick}>
                  <img id="header-avatar" data-avatar={!!this.props.user}
                    src={this.state.avatarUrl} alt="" ref={this.avatarRef}/>
                  <span id="username-span" data-avatar={!!this.props.user}>{this.props.user?.username}</span>
                </div>
              </div>
            </div>
          </div>
          {/*render all child components bellow*/}
          {this.props.children}
        </React.Fragment>
      );
    }
  }
  avatarClick(event: React.MouseEvent) {
    // prevent divs underneath from being clicked
    event.stopPropagation();
    // toggle menu
    this.setState({menuOpen: !this.state.menuOpen});
  }
}
export default withRouter(Header);