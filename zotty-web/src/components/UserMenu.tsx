import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import './UserMenu.sass';

interface MenuProps extends RouteComponentProps {};
interface MenuStates{};
class UserMenu extends React.Component<MenuProps, MenuStates> {
  render() {
    return (
      <div/>
    );
  }
}
export default withRouter(UserMenu);