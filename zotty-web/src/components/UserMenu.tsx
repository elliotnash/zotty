import React, { createRef, RefObject } from "react";
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

interface UserMenuProps extends RouteComponentProps {
  isOpen: boolean,
  setIsOpen: (state: boolean)=>void,
  openRef: RefObject<HTMLElement>
};
interface UserMenuStates{};
class UserMenu extends React.Component<UserMenuProps, UserMenuStates> {

  private menuRef = createRef<HTMLDivElement>();
  constructor(props: UserMenuProps) {
    super(props);

    this.handleClickOutside = this.handleClickOutside.bind(this);
  }
  componentDidMount() {document.addEventListener('mousedown', this.handleClickOutside);}
  componentWillUnmount() {document.removeEventListener('mousedown', this.handleClickOutside);}
  handleClickOutside(event: MouseEvent) {
    if (event.target instanceof HTMLElement && this.menuRef 
    && !this.menuRef.current?.contains(event.target) && this.props.openRef
    && !this.props.openRef?.current?.contains(event.target)) {
      //clicked outside menu. we will close if open. wait till click propagates to avatar
      if (this.props.isOpen){
        setTimeout(() => {
          this.props.setIsOpen(false);
        }, 100);
      }
    }
  }

  render() {
    return (
      <div ref={this.menuRef} className="menu-div" style={this.props.isOpen ? undefined: {display: 'none'}}>
        <MenuItem text="Servers"/>
        <MenuItem text="Log Out"/>
      </div>
    );
  }
}
export default withRouter(UserMenu);