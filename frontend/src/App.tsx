import React from 'react';
import { BrowserRouter as Router, Route, Switch, Redirect } from 'react-router-dom';
import LandingPage from './components/LandingPage';
import LoginRegisterPage from './components/LoginRegisterPage';
import PurchasePointsPage from './components/PurchasePointsPage';
import { extractAndDecodeToken } from './utils/auth';

const App: React.FC = () => {
  const isAuthenticated = !!extractAndDecodeToken();

  return (
    <Router>
      <Switch>
        <Route path="/login">
          {isAuthenticated ? <Redirect to="/" /> : <LoginRegisterPage />}
        </Route>
        <Route path="/purchase">
          {isAuthenticated ? <PurchasePointsPage /> : <Redirect to="/login" />}
        </Route>
        <Route path="/">
          {isAuthenticated ? <LandingPage /> : <Redirect to="/login" />}
        </Route>
      </Switch>
    </Router>
  );
};

export default App;
