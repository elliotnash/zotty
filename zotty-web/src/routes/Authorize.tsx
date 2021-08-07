import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import Cookies from "universal-cookie";
import axios, { AxiosResponse } from "axios";
import { DiscordUser, AccessTokenResponse } from "../types";
import {BACKEND_URL} from "..";

const cookies = new Cookies();
interface AuthorizeProps extends RouteComponentProps {}
interface AuthorizeStates{}
class Authorize extends React.Component<AuthorizeProps, AuthorizeStates> {
  render() {
    return (
      <div></div>
    );
  }
  componentDidMount() {
    // parse url for auth token
    let auth_params = new URLSearchParams(window.location.search);
    let auth_code = auth_params.get("code");
    let dc_state = auth_params.get("state");
    // get cookie state var and redirect var
    let cookie_state = cookies.get("state");
    // delete cookies
    cookies.remove("state", {path: "/", sameSite: "lax"});
    cookies.remove("redirect_path", {path: "/", sameSite: "lax"});
    if (dc_state !== cookie_state) {
      // state not equal, redirect to login
      console.log(`Invalid state: state is ${cookie_state} but returned ${dc_state}`);
      this.props.history.push("/login");
      return;
    }
    let redirectUrl = new URL(window.location.origin);
    redirectUrl.pathname = window.location.pathname;
    let loginUrl = new URL(BACKEND_URL);
    loginUrl.pathname = "/api/login";
    axios.post(loginUrl.toString(), {
      code: auth_code,
      redirect_uri: redirectUrl.toString()
    }).then((response: AxiosResponse<AccessTokenResponse>) => {
      // set cookies with token data
      cookies.set("access_token", response.data.access_token, {
        path: "/", sameSite: "lax", maxAge: response.data.expires_in-1000
      });
      cookies.set("refresh_token", response.data.refresh_token, {
        path: "/", sameSite: "lax", maxAge: 2147483647
      });
      // set auth header for all axios
      axios.defaults.headers.common['Authorization'] = `Bearer ${response.data.access_token}`
      // attempt to fetch user
      let meUrl = new URL(BACKEND_URL);
      meUrl.pathname = "/api/users/@me";
      axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
        // authentication complete, close oauth window or redirect
        window.opener?.authorize(response.data);
        window.close();
      });

    }).catch((err) => {
      // if invalid code, redirect to login page again
      console.log(err.response.data);
      window.close();
    })
  }
}
export default withRouter(Authorize);