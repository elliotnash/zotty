import axios from "axios";
import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import * as request from "../utils/request";
import type { PartialGuild } from "../utils/request";
import ServerListItem from "../components/ServerListItem";
import "./Servers.sass";

interface ServersProps extends RouteComponentProps {}
interface ServersStates{
  guilds: PartialGuild[] | undefined
}
class Servers extends React.Component<ServersProps, ServersStates> {
  constructor(props: ServersProps){
    super(props);
    this.state = {guilds: undefined};
    console.log("servers constructer called ");
  }
  componentDidMount(): void {
    console.log("Servers component mounted");
    console.log(axios.defaults.headers['common']['Authorization']);
    if (axios.defaults.headers['common']['Authorization']) {
      // then we're already authorized
      this.fetchServers();
    }
    // on page load, we should fetch servers if authorized, otherwise redirect
    window.emiter.on('loaded', () => {
      if (axios.defaults.headers['common']['Authorization']) {
        this.fetchServers();
      } else {
        this.props.history.push("/login");
      }
    });
    // on logout we need to redirect, this page is for authed users only
    window.emiter.on('logout', () => {
      this.props.history.push("/");
    });
  }
  fetchServers(): void {
    console.log(axios.defaults.headers?.common);
    request.guilds().then((guilds) => {
      this.setState({guilds});
      console.log(guilds);
    });
  }
  render() {
    return (
      !this.state.guilds?(
        // runs while guilds not loaded
        <div className="dot-background" >
          <div className="dot-pulse" ></div>
        </div>
      ):(
        // runs while guilds loaded
        <div id="server-container" className="server-container">
          {this.state.guilds.map((guild) => {
            return <ServerListItem key={guild.id} guild={guild}/>;
          })}
        </div>
      )
    );
  }
}
export default withRouter(Servers);