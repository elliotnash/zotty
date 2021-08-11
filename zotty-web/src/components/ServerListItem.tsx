import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
//import './ServerListItem.sass';
import type { PartialGuild } from "../utils/request";

interface ServerListItemProps extends RouteComponentProps {
  guild: PartialGuild
}
interface ServerListItemStates{}
class ServerListItem extends React.Component<ServerListItemProps, ServerListItemStates> {

  constructor(props: ServerListItemProps) {
    super(props);
  }

  render() {
    return (
      <div>
        {this.props.guild.name}
      </div>
    );
  }
}
export default withRouter(ServerListItem);
