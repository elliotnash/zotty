import React from "react";
import { RouteComponentProps } from "react-router-dom";
import "./Home.sass"
import { login } from "../utils/login";

interface HomeProps extends RouteComponentProps {}
interface HomeStates{}
export default class Home extends React.Component<HomeProps, HomeStates> {
  render() {
    return (
      <div>
        <span id="about-span" className="text">THIS IS A GOD WEBPAGE</span>
      </div>
    );
  };
}