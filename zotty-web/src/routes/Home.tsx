import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import "./Home.sass"
import Header from "../components/Header";
import { DiscordUser } from "../types";

interface HomeProps extends RouteComponentProps {
  user: DiscordUser | undefined
}
interface HomeStates{}
class Home extends React.Component<HomeProps, HomeStates> {
  render() {
    return (
      <Header user={this.props.user}>
        <br></br>
        <span id="about-span" className="text">THIS IS A GOD WEBPAGE</span>
      </Header>
    );
  };
}
export default withRouter(Home);