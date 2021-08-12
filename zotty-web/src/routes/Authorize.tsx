import React from "react";
import { RouteComponentProps, withRouter } from "react-router-dom";

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
    // emit authorize to opener and close
    window.opener?.emiter.emit('authorize', authCode, dcState);
    window.close();
  }
}
export default withRouter(Authorize);