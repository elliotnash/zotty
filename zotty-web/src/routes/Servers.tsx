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
  }
  componentDidMount(): void {
    console.log("axios default (in servers)");
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
      </React.Fragment>
    );
  }
}
export default withRouter(Servers);