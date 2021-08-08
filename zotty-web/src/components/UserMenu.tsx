import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './UserMenu.sass';

interface MenuItemProps extends RouteComponentProps {
  text: string
};
interface MenuItemStates{};
const MenuItem = withRouter(class MenuItem extends React.Component<MenuItemProps, MenuItemStates> {
  render() {
    return (
      <div className="menu-item-div">
        <span className="menu-item-text">{this.props.text}</span>
      </div>
    );
  }
});

interface UserMenuProps extends RouteComponentProps {};
interface UserMenuStates{};
class UserMenu extends React.Component<UserMenuProps, UserMenuStates> {
  render() {
    return (
      <div id="menu-div">
        <MenuItem text="Servers"/>
        <MenuItem text="Log Out"/>
      </div>
    );
  }
}
export default withRouter(UserMenu);