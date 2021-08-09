import Cookies from "universal-cookie";
import axios, { AxiosResponse } from "axios";
import { nanoid } from 'nanoid'
import {BACKEND_URL} from "..";
import { AccessTokenResponse, DiscordUser } from "../types";

interface OAuthInfo{
  api_url: string,
  client_id: string
}

const cookies = new Cookies();
export function newLogin() {
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

function getAccessToken() {
  return new Promise<string>((resolve, reject) => {
    let accessToken: string | undefined = cookies.get("access_token");
    if (accessToken){
      resolve(accessToken);
    } else {
      console.log("access_token missing or expired");
      refresh().then(() => {
        reject();
      });
    }
  });
}

export function cookieLogin() {
  return new Promise<void>((resolve) => {
    getAccessToken().then((accessToken) => {
      // we now have access token, set axios auth header and make req
      axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;
      // get user obj and set login state
      let meUrl = new URL(BACKEND_URL);
      meUrl.pathname = "/api/users/@me";
      axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
        // authentication complete, update main state and resolve
        window.login(response.data);
        resolve();
      }).catch((err) => {
        console.log(err)
        console.log("invalid access token, requesting new token using refresh code");
        cookies.remove("access_token", {path: "/", sameSite: "lax"});
        getAccessToken().then((accessToken) => {
          // we now have access token, set axios auth header and make req
          axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;
          axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
            // authentication complete, update main state
            window.login(response.data);
            resolve();
          });
        }).catch(() => {
          console.log("refresh_token invalid");
          resolve();
        })
      });
    }).catch(() => {
      resolve();
    });
  });
}

function refresh() {
  return new Promise<void>((resolve) => {
    let refreshToken: string | undefined = cookies.get("refresh_token");
    if (!refreshToken){
      console.log("no refresh_token cookie found");
      resolve();
      return;
    }
    let refreshUrl = new URL(BACKEND_URL);
    refreshUrl.pathname = "/api/refresh";
    axios.post(refreshUrl.toString(), {refresh_token: refreshToken}).then((response: AxiosResponse<AccessTokenResponse>) => {
      // set cookies and auth header
      setTokenResponseData(response.data);
      // attempt to fetch user
      let meUrl = new URL(BACKEND_URL);
      meUrl.pathname = "/api/users/@me";
      axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
        // authentication complete, send login
        window.login(response.data);
        resolve();
      });
    }).catch((err) => {
      console.log(err)
      console.log("invalid refresh token, not logging in");
      resolve();
    });
  });
}

export function setTokenResponseData(data: AccessTokenResponse){
  // set cookies with token data
  cookies.set("access_token", data.access_token, {
    path: "/", sameSite: "lax", maxAge: data.expires_in-1000
  });
  cookies.set("refresh_token", data.refresh_token, {
    path: "/", sameSite: "lax", maxAge: 2147483647
  });
  // set auth header for all axios
  axios.defaults.headers.common['Authorization'] = `Bearer ${data.access_token}`;
}

export function logout() {
  // remove cookies
  cookies.remove("access_token", {path: "/", sameSite: "lax"});
  cookies.remove("refresh_token", {path: "/", sameSite: "lax"});
  // call app logout function
  window.logout();
}