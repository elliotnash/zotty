import Cookies from "universal-cookie";
import axios from "axios";
import { nanoid } from 'nanoid'
import {BACKEND_URL} from "..";

interface OAuthInfo{
  api_url: string,
  client_id: string
}

const cookies = new Cookies();
export function login() {
  // create oauth window, need to do it now else calling window.open 
  // from other context gets blocked by browser popup blocker
  // we'll set content later
  const width = 500;
  const height = 800;
  const left = window.screenX + (window.outerWidth - width) / 2;
  const top = window.screenY + (window.outerHeight - height) / 2.5;

  const windowFeatures = `toolbar=0,scrollbars=0,status=0,resizable=0,location=0,menuBar=0,width=${width},height=${height},top=${top},left=${left}`;
  let oauth_window = window.open(
      "",
      "Login",
      windowFeatures
  );
  // ping api, recieve oauth info
  let pingUrl = new URL(BACKEND_URL);
  pingUrl.pathname = "/api/ping";
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
    // set oauth window url
    oauth_window?.location.replace(oauthUrl.toString());
  });
}
