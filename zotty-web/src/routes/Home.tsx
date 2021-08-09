import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import "./Home.sass";
import { DiscordUser } from "../utils/request";

interface HomeProps extends RouteComponentProps {
  user: DiscordUser | undefined
}
interface HomeStates{}
class Home extends React.Component<HomeProps, HomeStates> {
  render() {
    return (
      <React.Fragment>
        <br></br>
        <span id="about-span" className="text">THIS IS A GOD WEBPAGE</span>
      </React.Fragment>
    );
  }
}
export default withRouter(Home);