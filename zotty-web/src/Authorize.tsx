import React from "react";
import { RouteComponentProps } from "react-router-dom";
import axios from "axios";
import {BACKEND_URL} from ".";

interface OAuthInfo{
  api_url: string,
  client_id: string
}
interface AuthorizeProps extends RouteComponentProps {
}
interface AuthorizeStates{
}
export default class Login extends React.Component<AuthorizeProps, AuthorizeStates> {
  render() {
    return (
      <div></div>
    );
  }
  componentDidMount() {
    // parse url for auth token
    let auth_params = new URLSearchParams(window.location.search);
    let auth_code = auth_params.get("code");
  }
}
