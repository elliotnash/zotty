import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import { getGuildIconUrl } from "../utils/discord";
import './ServerListItem.sass';
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
      <div className="item-container">
        <img src={getGuildIconUrl(this.props.guild, 256)} className="server-img" ></img>
        <div className="name-background">
          <div className="name-background-animated"></div>
          <span className="server-item-text" >{this.props.guild.name}</span>
        </div>
      </div>
    );
  }
}
export default withRouter(ServerListItem);
