import React from "react";
import axios from "axios";
import BuildUrl from "build-url";
import {BACKEND_URL} from ".";

interface LoginProps {
}
interface LoginStates{
  data: string
}
export default class Login extends React.Component<LoginProps, LoginStates> {

  constructor(props: LoginProps) {
    super(props);
    this.state = { data : "test"};
  }

  render() {
    return (
      <div>{this.state.data}</div>
    );
  }
  componentDidMount() {
    axios.get(
      BuildUrl(BACKEND_URL, {path: '/api/ping'})
    ).then((response) => {
      this.setState({
        data: JSON.stringify(response.data),
      });
      console.log(this.state.data);
    });
  }
}
