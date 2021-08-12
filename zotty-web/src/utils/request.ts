import axios, { AxiosResponse } from "axios";
import Cookies from "universal-cookie";

const cookies = new Cookies();

export const BACKEND_URL = getBackendUrl();
function getBackendUrl(): string {
  if (window.location.port === "3000")
    return "http://localhost:8000";
  else
    return window.location.origin;
}

export interface DiscordUser {
  id: number,
  username: string,
  discriminator: string,
  avatar?: string
}
export interface AccessTokenResponse{
  access_token: string,
  token_type: string,
  expires_in: number,
  refresh_token: string,
  scope: string
}

export interface OAuthInfo{
  api_url: string,
  client_id: string
}

export interface PartialGuild {
  id: string,
  name: string,
  icon?: string,
  owner: boolean,
  permissions: string,
}

const pingUrl = new URL(BACKEND_URL);
pingUrl.pathname = "/api/ping";
export function ping(): Promise<OAuthInfo> {
  return new Promise((resolve, reject) => {
    axios.get(
      pingUrl.toString()
    ).then((response) => {
      resolve(response.data);
    }).catch((err) => {
      reject(err);
    });
  });
}

const redirectUrl = new URL(window.location.origin);
redirectUrl.pathname = window.location.pathname;
const loginUrl = new URL(BACKEND_URL);
loginUrl.pathname = "/api/login";
export function login(code: string): Promise<void> {
  return new Promise((resolve, reject) => {
    axios.post(loginUrl.toString(), {
      code,
      redirect_uri: redirectUrl.toString()
    }).then((response: AxiosResponse<AccessTokenResponse>) => {
      // set cookies and auth header
      setTokenResponseData(response.data);
      // resolve
      resolve();
    }).catch((err) => {
      reject(err);
    });
  });
}

const refreshUrl = new URL(BACKEND_URL);
refreshUrl.pathname = "/api/refresh";
export function refresh(token: string): Promise<void> {
  return new Promise((resolve, reject) => {
    axios.post(refreshUrl.toString(), {refresh_token: token}).then((response: AxiosResponse<AccessTokenResponse>) => {
      // set cookies and auth header
      setTokenResponseData(response.data);
      // resolve
      resolve();
    }).catch((err) => {
      reject(err);
    });
  });
}

const meUrl = new URL(BACKEND_URL);
meUrl.pathname = "/api/users/@me";
export function user(): Promise<DiscordUser> {
  return new Promise((resolve, reject) => {
    axios.get(meUrl.toString()).then((response: AxiosResponse<DiscordUser>) => {
      // set user cookie
      cookies.set("user", response.data, {
        path: "/", sameSite: "lax", maxAge: 2147483647
      });
      // authentication complete, resolve
      resolve(response.data);
    }).catch((err) => {
      reject(err);
    });
  });
}

const guildsUrl = new URL(BACKEND_URL);
guildsUrl.pathname = "/api/users/@me/guilds";
export function guilds(): Promise<PartialGuild[]> {
  return new Promise((resolve, reject) => {
    axios.get(guildsUrl.toString()).then((response: AxiosResponse<PartialGuild[]>) => {
      // set user cookie
      cookies.set("guilds", response.data, {
        path: "/", sameSite: "lax", maxAge: 2147483647
      });
      // authentication complete, resolve
      //resolve(response.data);
      resolve([
        {
            id: "743256836014342166",
            name: "ETech Developer Server",
            icon: "2187417166b60abf02c3b4bf85db2dfa",
            owner: false,
            permissions: "274877906943"
        },
        {
            id: "812805217543782430",
            name: "cheese",
            owner: true,
            permissions: "274877906943"
        },
        {
          id: "743256836014342166",
          name: "ETech Developer Server",
          icon: "2187417166b60abf02c3b4bf85db2dfa",
          owner: false,
          permissions: "274877906943"
        },
        {
          id: "812805217543782430",
          name: "cheese",
          owner: true,
          permissions: "274877906943"
        },
      ]);
    }).catch((err) => {
      reject(err);
    });
  });
}

function setTokenResponseData(data: AccessTokenResponse): void {
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
