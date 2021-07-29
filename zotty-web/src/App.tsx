import React from "react";
import {
  BrowserRouter as Router,
  Route,
  Link,
  RouteComponentProps
} from "react-router-dom";
import { BACKEND_URL } from ".";

function Index() {
  return (
    <h2>{BACKEND_URL}</h2>
  );
}

type TParams = {guild_id: string};
function ServerPage({match}: RouteComponentProps<TParams>) {
  return <h2>Servers guild id is: {match.params.guild_id} </h2>;
}

function AppRouter() {
  return (
    <Router>
      <div>
        <nav>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/123213">First Guild</Link>
            </li>
            <li>
              <Link to="/213232">Second Guild</Link>
            </li>
          </ul>
        </nav>
        <Route path="/" exact component={Index}/>
        <Route path="/:guild_id" exact component={ServerPage}/>
      </div>
    </Router>
  );
}
export default AppRouter;
