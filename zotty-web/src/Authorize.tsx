import React from "react";
import { RouteComponentProps } from "react-router-dom";
import Cookies from "universal-cookie";
import axios, { AxiosResponse } from "axios";
import {BACKEND_URL} from ".";

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
interface AuthorizeProps extends RouteComponentProps {
  cookies: Cookies
}
interface AuthorizeStates{
}
export default class Login extends React.Component<AuthorizeProps, AuthorizeStates> {

  constructor(props: AuthorizeProps) {
    props.cookies = new Cookies;
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
    let cookie_state = this.props.cookies.get("state");
    let redirect_path = this.props.cookies.get("redirect_path");
    // delete cookies
    this.props.cookies.remove("state");
    this.props.cookies.remove("redirect_path");
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
      this.props.cookies.set("access_token", response.data.access_token, {
        sameSite: "lax", maxAge: response.data.expires_in-1000
      });
      this.props.cookies.set("refresh_token", response.data.refresh_token, {
        sameSite: "lax", maxAge: 2147483647
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
        this.props.cookies.set("user", response.data, {
          sameSite: "lax"
        });
        // authentication complete, redirect to redirect path
        this.props.history.push(redirect_path);
      });

    }).catch((err) => {
      // if invalid code, redirect to login page again
      console.log(err.response.data);
      this.props.history.push("/login");
    })
  }
}
