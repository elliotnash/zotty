import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import Cookies from "universal-cookie";
import axios, { AxiosResponse } from "axios";
import { DiscordUser, AccessTokenResponse } from "../types";
import {BACKEND_URL} from "..";
import { setTokenResponseData } from "../utils/auth";

const cookies = new Cookies();
interface AuthorizeProps extends RouteComponentProps {}
interface AuthorizeStates{}
class Authorize extends React.Component<AuthorizeProps, AuthorizeStates> {
  render() {
    return null;
  }
  componentDidMount() {
    // parse url for auth token
    const authParams = new URLSearchParams(window.location.search);
    const authCode = authParams.get("code");
    const dcState = authParams.get("state");
    // get cookie state var and redirect var
    const cookieState = cookies.get("state");
    // delete cookies
    cookies.remove("state", {path: "/", sameSite: "lax"});
    cookies.remove("redirect_path", {path: "/", sameSite: "lax"});
    if (dcState !== cookieState) {
      // state not equal, redirect to login
      console.log(`Invalid state: state is ${cookieState} but returned ${dcState}`);
      this.props.history.push("/login");
      return;
    }
    const redirectUrl = new URL(window.location.origin);
    redirectUrl.pathname = window.location.pathname;
    const loginUrl = new URL(BACKEND_URL);
    loginUrl.pathname = "/api/login";
    axios.post(loginUrl.toString(), {
      code: authCode,
      redirect_uri: redirectUrl.toString()
    }).then((response: AxiosResponse<AccessTokenResponse>) => {
      // set cookies and auth header
      setTokenResponseData(response.data);
      // attempt to fetch user
      const meUrl = new URL(BACKEND_URL);
      meUrl.pathname = "/api/users/@me";
      axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
        // set user cookie
        cookies.set("user", response.data, {
          path: "/", sameSite: "lax", maxAge: 2147483647
        });
        // authentication complete, close oauth window or redirect
        window.opener?.login(response.data);
        window.close();
      });

    }).catch((err) => {
      // if invalid code, redirect to login page again
      console.log(err.response.data);
      window.close();
    });
  }
}
export default withRouter(Authorize);