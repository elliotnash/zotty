import Cookies from "universal-cookie";
import axios from "axios";
import { nanoid } from 'nanoid';
import * as request from "../utils/request";

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
  request.ping().then((oAuthInfo) => {
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
    const oauthUrl = new URL(oAuthInfo.api_url);
    oauthUrl.pathname = "/oauth2/authorize";
    oauthUrl.search = new URLSearchParams({
      client_id: oAuthInfo.client_id,
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
  return new Promise((resolve, reject) => {
    const accessToken: string | undefined = cookies.get("access_token");
    if (accessToken){
      resolve(accessToken);
    } else {
      console.log("access_token missing or expired");
      refresh().then((success) => {
        reject(success);
      });
    }
  });
}

export function cookieLogin(): Promise<boolean> {
  return new Promise((resolve) => {
    getAccessToken().then((accessToken) => {
      // we now have access token, set axios auth header and make req
      axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;
      // get user obj and set login state
      request.user().then((user) => {
        // authentication complete, update main state and resolve
        window.emiter.emit('login', user);
        resolve(true);
      }).catch((err) => {
        console.log(err);
        console.log("invalid access token, requesting new token using refresh code");
        cookies.remove("access_token", {path: "/", sameSite: "lax"});
        getAccessToken().then((accessToken) => {
          // we now have access token, set axios auth header and make req
          axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;
          request.user().then((user) => {
            // authentication complete, update main state
            window.emiter.emit('login', user);
            resolve(true);
          });
        }).catch(() => {
          console.log("refresh_token invalid");
          resolve(false);
        });
      });
    }).catch((success) => {
      resolve(success);
    });
  });
}

function refresh(): Promise<boolean> {
  return new Promise((resolve) => {
    const refreshToken: string | undefined = cookies.get("refresh_token");
    if (!refreshToken){
      console.log("no refresh_token cookie found");
      resolve(false);
      return;
    }
    request.refresh(refreshToken).then(() => {
      request.user().then((user) => {
        // set user cookie
        cookies.set("user", user, {
          path: "/", sameSite: "lax", maxAge: 2147483647
        });
        // authentication complete, send login
        window.emiter.emit('login', user);
        resolve(true);
      });
    }).catch((err) => {
      console.log(err);
      console.log("invalid refresh token, not logging in");
      resolve(false);
    });
  });
}

export function logout(): void {
  // remove cookies
  cookies.remove("access_token", {path: "/", sameSite: "lax"});
  cookies.remove("refresh_token", {path: "/", sameSite: "lax"});
  cookies.remove("user", {path: "/", sameSite: "lax"});
  // call app logout function
  window.emiter.emit('logout');
}