import React from "react";
import { RouteComponentProps } from "react-router-dom";
import Cookies from "universal-cookie";
import axios, { AxiosResponse } from "axios";
import {BACKEND_URL} from "..";

const cookies = new Cookies();
interface DiscordUser {
  id: number,
  username: string,
  discriminator: string,
  avatar: string
}
interface AccessTokenResponse{
  access_token: string,
  token_type: string,
  expires_in: number,
  refresh_token: string,
  scope: string
}
interface AuthorizeProps extends RouteComponentProps {}
interface AuthorizeStates{}
export default class Login extends React.Component<AuthorizeProps, AuthorizeStates> {

  constructor(props: AuthorizeProps) {
    super(props);

    this.state = {
      user: undefined
    };
  }

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
        this.setState(() => ({
          user: response.data
        }));
        cookies.set("user", response.data, {
          path: "/", sameSite: "lax"
        });
        // authentication complete, close oauth window or redirect
        window.opener?.authorize();
        window.close();
      });

    }).catch((err) => {
      // if invalid code, redirect to login page again
      console.log(err.response.data);
      window.close();
    })
  }
}
