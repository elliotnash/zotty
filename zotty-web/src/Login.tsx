import React from "react";
import { RouteComponentProps } from "react-router-dom";
import axios from "axios";
import BuildUrl from "build-url";
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
  constructor(props: LoginProps) {
    super(props);
  }

  render() {
    return (
      <div></div>
    );
  }
  componentDidMount() {
    // ping api, recieve oauth info
    axios.get(
      BuildUrl(BACKEND_URL, {path: '/api/ping'})
    ).then((response) => {
      let oauthInfo = response.data as OAuthInfo;
      // construct redirect url
      let redirectUrl = BuildUrl(window.location.origin, {
        path: "/authorize"
      });
      // construct discord oauth url
      let oauthUrl = BuildUrl(oauthInfo.api_url, {
        path: "/oauth2/authorize",
        queryParams: {
          client_id: oauthInfo.client_id,
          redirect_uri: redirectUrl,
          response_type: "code",
          scope: "identify guilds",
          prompt: "consent"
        }
      })
      window.location.replace(oauthUrl);
    });
  }
}
