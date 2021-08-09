import axios, { AxiosResponse } from "axios";
import Cookies from "universal-cookie";

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

const cookies = new Cookies();

const redirectUrl = new URL(window.location.origin);
redirectUrl.pathname = window.location.pathname;
const loginUrl = new URL(BACKEND_URL);
loginUrl.pathname = "/api/login";
export function login(code: string): Promise<void> {
  return new Promise<void>((resolve, reject) => {
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

const meUrl = new URL(BACKEND_URL);
meUrl.pathname = "/api/users/@me";
export function user(): Promise<DiscordUser> {
  return new Promise<DiscordUser>((resolve, reject) => {
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
