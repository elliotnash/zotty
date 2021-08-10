import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";
import Cookies from "universal-cookie";
import * as request from '../utils/request';

const cookies = new Cookies();
interface AuthorizeProps extends RouteComponentProps {}
interface AuthorizeStates{}
class Authorize extends React.Component<AuthorizeProps, AuthorizeStates> {
  render() {
    return null;
  }
  componentDidMount() {
    // parse url for auth token
    const authParams = new URLSearchParams(window.location.search);
    const authCode = authParams.get("code");
    const dcState = authParams.get("state");
    // get cookie state var and redirect var
    const cookieState = cookies.get("state");
    // delete cookies
    cookies.remove("state", {path: "/", sameSite: "lax"});
    cookies.remove("redirect_path", {path: "/", sameSite: "lax"});
    if (dcState !== cookieState) {
      // state not equal, redirect to login
      console.log(`Invalid state: state is ${cookieState} but returned ${dcState}`);
      this.props.history.push("/login");
      return;
    }

    request.login(authCode as string).then(() => {
      request.user().then((user) => {
        // authentication complete, close oauth window or redirect
        window.opener?.emiter.emit('login', user);
        window.close();
      });
    }).catch((err) => {
      // if invalid code, close window without login
      console.log(err.response.data);
      window.close();
    });
  }
}
export default withRouter(Authorize);