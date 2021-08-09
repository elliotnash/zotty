import React, { createRef } from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './Header.sass';
import UserMenu from "./UserMenu";
import { newLogin } from "../utils/auth";
import { DiscordUser } from "../types";
import { getAvatarUrl } from "../utils/discord";

interface HeaderProps extends RouteComponentProps {
  user: DiscordUser | undefined
}
interface HeaderStates{
  avatar: boolean,
  menuOpen: boolean
}
class Header extends React.Component<HeaderProps, HeaderStates> {
  constructor(props: HeaderProps){
    super(props);
    this.state = {
      avatar: !!this.props.user,
      menuOpen: false
    };
    this.avatarClick = this.avatarClick.bind(this);
  }
  componentDidUpdate(prevProps: HeaderProps): void {
    if (this.props.user !== prevProps.user) {
      if (this.props.user) {
        // we just logged in
        // set toAvatar state to 1 to start animation
        this.setState({avatar: true});
      } else {
        // we just logged out
        console.log("header recieved logout");
        this.setState({avatar: false});
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
            <div id="login-btn-container">
              <div id="login-btn" className="btn" data-avatar={this.state.avatar} onClick={newLogin}>
                <span id="login-text" data-avatar={this.state.avatar}>LOG IN</span>
                <img id="header-avatar" data-avatar={this.state.avatar} onClick={this.avatarClick}
                  src={getAvatarUrl(this.props.user, 64)} alt="" ref={this.avatarRef}/>
              </div>
              <UserMenu isOpen={this.state.menuOpen} setIsOpen={(menuOpen) => {this.setState({menuOpen});}} openRef={this.avatarRef}/>
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