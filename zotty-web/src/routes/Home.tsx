import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import "./Home.sass";

interface HomeProps extends RouteComponentProps {}
interface HomeStates{}
class Home extends React.Component<HomeProps, HomeStates> {
  render() {
    return (
      <React.Fragment>
        <br></br>
        <span id="about-span" className="text">This is where we put info about Zotty</span>
      </React.Fragment>
    );
  }
}
export default withRouter(Home);