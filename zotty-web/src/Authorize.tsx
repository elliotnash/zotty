import React from "react";
import { RouteComponentProps } from "react-router-dom";
import { withCookies, Cookies } from "react-cookie";
import axios from "axios";
import {BACKEND_URL} from ".";

interface AccessTokenResponse{
  access_token: string,
  token_type: string,
  expires_in: number,
  refresh_token: string,
  scope: string
}
interface AuthorizeProps extends RouteComponentProps {
  cookies: Cookies
}
interface AuthorizeStates{
}
class Login extends React.Component<AuthorizeProps, AuthorizeStates> {
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
      // set cookies with token data
      let token_response: AccessTokenResponse = response.data;
      this.props.cookies.set("access_token", token_response.access_token, {
        path: "/", sameSite: "lax", maxAge: token_response.expires_in-1000
      });
      this.props.cookies.set("refresh_token", token_response.refresh_token, {
        path: "/", sameSite: "lax", maxAge: 2147483647
      });
    }).catch((err) => {
      // if invalid code, redirect to login page again
      console.log(err.response.data);
      this.props.history.push("/login");
    })
  }
}
export default withCookies(Login);