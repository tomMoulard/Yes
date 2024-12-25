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
        <Route path="/" element={isAuthenticated ? <LandingPage /> : <Navigate to="/login" />} />
        <Route path="/login" element={isAuthenticated ? <Navigate to="/" /> : <LoginRegisterPage />} />
        <Route path="/purchase" element={isAuthenticated ? <PurchasePointsPage /> : <Navigate to="/login" />} />
      </Routes>
    </Router>
  );
};

export default App;
