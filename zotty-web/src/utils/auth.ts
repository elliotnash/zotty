import Cookies from "universal-cookie";
import axios, { AxiosResponse } from "axios";
import { nanoid } from 'nanoid';
import {BACKEND_URL} from "..";
import { AccessTokenResponse, DiscordUser } from "../types";

interface OAuthInfo{
  api_url: string,
  client_id: string
}

const cookies = new Cookies();
export function newLogin(): void {
  // create oauth window, need to do it now else calling window.open 
  // from other context gets blocked by browser popup blocker
  // we'll set content later
  const width = 500;
  const height = 800;
  const left = window.screenX + (window.outerWidth - width) / 2;
  const top = window.screenY + (window.outerHeight - height) / 2.5;

  const windowFeatures = `toolbar=0,scrollbars=0,status=0,resizable=0,location=0,menuBar=0,width=${width},height=${height},top=${top},left=${left}`;
  const oauthWindow = window.open(
      "",
      "Login",
      windowFeatures
  );
  // ping api, recieve oauth info
  const pingUrl = new URL(BACKEND_URL);
  pingUrl.pathname = "/api/ping";
  axios.get(
    pingUrl.toString()
  ).then((response) => {
    const oauthInfo = response.data as OAuthInfo;
    // generate state var
    const state = nanoid();
    // set state cookie
    cookies.set("state", state, {
      path: "/", sameSite: "lax", maxAge: 2147483647
    });
    // construct redirect url
    const redirectUrl = new URL(window.location.origin);
    redirectUrl.pathname = "/authorize";
    // construct discord oauth url
    const oauthUrl = new URL(oauthInfo.api_url);
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
    oauthWindow?.location.replace(oauthUrl.toString());
  });
}

function getAccessToken(): Promise<string> {
  return new Promise<string>((resolve, reject) => {
    const accessToken: string | undefined = cookies.get("access_token");
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

export function cookieLogin(): Promise<boolean> {
  return new Promise<boolean>((resolve) => {
    getAccessToken().then((accessToken) => {
      // we now have access token, set axios auth header and make req
      axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;
      // get user obj and set login state
      const meUrl = new URL(BACKEND_URL);
      meUrl.pathname = "/api/users/@me";
      axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
        // set user cookie
        cookies.set("user", response.data, {
          path: "/", sameSite: "lax", maxAge: 2147483647
        });
        // authentication complete, update main state and resolve
        window.login(response.data);
        resolve(true);
      }).catch((err) => {
        console.log(err);
        console.log("invalid access token, requesting new token using refresh code");
        cookies.remove("access_token", {path: "/", sameSite: "lax"});
        getAccessToken().then((accessToken) => {
          // we now have access token, set axios auth header and make req
          axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;
          axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
            // set user cookie
            cookies.set("user", response.data, {
              path: "/", sameSite: "lax", maxAge: 2147483647
            });
            // authentication complete, update main state
            window.login(response.data);
            resolve(true);
          });
        }).catch(() => {
          console.log("refresh_token invalid");
          resolve(false);
        });
      });
    }).catch(() => {
      resolve(false);
    });
  });
}

function refresh(): Promise<void> {
  return new Promise<void>((resolve) => {
    const refreshToken: string | undefined = cookies.get("refresh_token");
    if (!refreshToken){
      console.log("no refresh_token cookie found");
      resolve();
      return;
    }
    const refreshUrl = new URL(BACKEND_URL);
    refreshUrl.pathname = "/api/refresh";
    axios.post(refreshUrl.toString(), {refresh_token: refreshToken}).then((response: AxiosResponse<AccessTokenResponse>) => {
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
        // authentication complete, send login
        window.login(response.data);
        resolve();
      });
    }).catch((err) => {
      console.log(err);
      console.log("invalid refresh token, not logging in");
      resolve();
    });
  });
}

export function setTokenResponseData(data: AccessTokenResponse): void {
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

export function logout(): void {
  // remove cookies
  cookies.remove("access_token", {path: "/", sameSite: "lax"});
  cookies.remove("refresh_token", {path: "/", sameSite: "lax"});
  cookies.remove("user", {path: "/", sameSite: "lax"});
  // call app logout function
  window.logout();
}