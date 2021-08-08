import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './UserMenu.sass';

interface MenuItemProps extends RouteComponentProps {};
interface MenuItemStates{};
const MenuItem = withRouter(class MenuItem extends React.Component<MenuItemProps, MenuItemStates> {
  render() {
    return (
      <div className="menu-item-div">

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
        <MenuItem/>
      </div>
    );
  }
}
export default withRouter(UserMenu);