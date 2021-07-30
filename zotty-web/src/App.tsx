import {
  BrowserRouter as Router,
  Route,
  Link
} from "react-router-dom";
import Login from "./Login";
import Authorize from "./Authorize";

function Index() {
  return (
    <div>
      <span>HOME</span>
      <br/>
      <Link to="/login">login</Link>
    </div>
  );
}

function AppRouter() {
  return (
    <Router>
      <div>
        <Route path="/" exact component={Index}/>
        <Route path="/login" exact component={Login}/>
        <Route path="/authorize" exact component={Authorize}/>
      </div>
    </Router>
  );
}
export default AppRouter;
