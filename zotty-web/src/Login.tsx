import React from "react";
import { RouteComponentProps } from "react-router-dom";
import axios from "axios";
import {BACKEND_URL} from ".";

interface OAuthInfo{
  api_url: string,
  client_id: string
}
interface LoginProps extends RouteComponentProps {
}
interface LoginStates{
}
export default class Login extends React.Component<LoginProps, LoginStates> {
  render() {
    return (
      <div></div>
    );
  }
  componentDidMount() {
    // ping api, recieve oauth info
    let pingUrl = new URL(BACKEND_URL);
    pingUrl.pathname = "/api/ping";
    console.log(pingUrl);

    axios.get(
      pingUrl.toString()
    ).then((response) => {
      let oauthInfo = response.data as OAuthInfo;
      // construct redirect url
      let redirectUrl = new URL(window.location.origin);
      redirectUrl.pathname = "/authorize";
      // construct discord oauth url
      let oauthUrl = new URL(oauthInfo.api_url);
      oauthUrl.pathname = "/oauth2/authorize";
      oauthUrl.search = new URLSearchParams({
        client_id: oauthInfo.client_id,
        redirect_uri: redirectUrl.toString(),
        response_type: "code",
        scope: "identify guilds",
        prompt: "consent"
      }).toString();
      window.location.replace(oauthUrl.toString());
    });
  }
}
