import axios from "axios";
import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import * as request from "../utils/request";
import { PartialGuild } from "../utils/request";
import "./Home.sass";

interface ServersProps extends RouteComponentProps {}
interface ServersStates{
  guilds: PartialGuild[] | undefined
}
class Servers extends React.Component<ServersProps, ServersStates> {
  constructor(props: ServersProps){
    super(props);
    this.state = {guilds: undefined};
    console.log("servers constructer called ");
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
  componentDidMount(): void {
    if (axios.defaults.headers['common']['Authorization']) {
      // then we're already authorized
      this.fetchServers();
    }
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
      <React.Fragment>
        <br></br>
        <span id="about-span" className="text">This is where we show your servers</span>
        <br></br>
        <span id="servers" className="text">{JSON.stringify(this.state.guilds)}</span>
      </React.Fragment>
    );
  }
}
export default withRouter(Servers);