import React from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router';
import LandingPage from './components/LandingPage';
import LoginRegisterPage from './components/LoginRegisterPage';
import PurchasePointsPage from './components/PurchasePointsPage';
import { extractAndDecodeToken } from './utils/auth';

const App: React.FC = () => {
  const isAuthenticated = !!extractAndDecodeToken();

  return (
    <Router>
      <Routes>
        <Route path="/">
          {isAuthenticated ? <LandingPage /> : <Navigate to="/login" />}
        </Route>
        <Route path="/login">
          {isAuthenticated ? <Navigate to="/" /> : <LoginRegisterPage />}
        </Route>
        <Route path="/purchase">
          {isAuthenticated ? <PurchasePointsPage /> : <Navigate to="/login" />}
        </Route>
      </Routes>
    </Router>
  );
};

export default App;
