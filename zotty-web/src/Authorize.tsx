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
    let redirectUrl = new URL(window.location.origin);
    redirectUrl.pathname = window.location.pathname;
    let loginUrl = new URL(BACKEND_URL);
    loginUrl.pathname = "/api/login";
    axios.post(loginUrl.toString(), {
      code: auth_code,
      redirect_uri: redirectUrl.toString()
    }).then((response) => {
       console.log(response.data);
    }).catch((err) => {
      // if invalid code, redirect to login page again
      console.log(err.response.data);
      this.props.history.push("/login");
    })
  }
}
