import React from "react";
import {
  BrowserRouter as Router,
  Route,
  Link,
  RouteComponentProps
} from "react-router-dom";

function Index() {
  return <h2>Home</h2>;
}

type TParams = {id: string};
function Page({match}: RouteComponentProps<TParams>) {
  return <h2>This is a page with ID: {match.params.id} </h2>;
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
              <Link to="/page/1">First Product</Link>
            </li>
            <li>
              <Link to="/page/2">Second Product</Link>
            </li>
          </ul>
        </nav>
        <Route path="/" exact component={Index} />
        <Route path="/page/:id" component={Page} />
      </div>
    </Router>
  );
}
export default AppRouter;
