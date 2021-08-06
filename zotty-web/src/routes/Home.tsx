import React from "react";
import { RouteComponentProps } from "react-router-dom";
import "./Home.sass"
import Header from "../components/Header";

interface HomeProps extends RouteComponentProps {}
interface HomeStates{}
export default class Home extends React.Component<HomeProps, HomeStates> {
  render() {
    return (
      <Header>
        <span id="about-span" className="text">THIS IS A GOD WEBPAGE</span>
      </Header>
    );
  };
}
