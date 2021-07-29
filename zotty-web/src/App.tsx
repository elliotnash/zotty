import {
  BrowserRouter as Router,
  Route,
  Link,
  RouteComponentProps
} from "react-router-dom";
import Login from "./Login";

function Index() {
  return (
    <div>
      <span>HOME</span>
      <br/>
      <Link to="/login">login</Link>
    </div>
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
        <Route path="/" exact component={Index}/>
        <Route path="/login" exact component={Login}/>
        <Route path="/guild/:guild_id" exact component={ServerPage}/>
      </div>
    </Router>
  );
}
export default AppRouter;
