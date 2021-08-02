import Cookies from "universal-cookie";
import axios from "axios";
import { nanoid } from 'nanoid'
import {BACKEND_URL} from ".";

interface OAuthInfo{
  api_url: string,
  client_id: string
}

const cookies = new Cookies;
export function login(redirect_path=window.location.pathname) {
  // ping api, recieve oauth info
  let pingUrl = new URL(BACKEND_URL);
  pingUrl.pathname = "/api/ping";
  console.log(pingUrl);

  axios.get(
    pingUrl.toString()
  ).then((response) => {
    let oauthInfo = response.data as OAuthInfo;
    // generate state var
    let state = nanoid();
    // set state cookie
    cookies.set("state", state, {
      path: "/", sameSite: "lax", maxAge: 2147483647
    });
    // set redirect_url cookie
    if (redirect_path)
    cookies.set("redirect_path", redirect_path, {
      path: "/", sameSite: "lax", maxAge: 2147483647
    });
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
      prompt: "consent",
      state
    }).toString();
    window.location.replace(oauthUrl.toString());
  });
}
