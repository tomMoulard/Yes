import React, { useEffect, useState } from 'react';
import { jwtDecode } from "jwt-decode";
import { Button, Layout, Typography } from 'antd';
import { useNavigate, Link } from 'react-router';

const { Header, Content } = Layout;
const { Title } = Typography;

interface User {
  email: string;
  username: string;
  points: number;
}

const LandingPage: React.FC = () => {
  const [user, setUser] = useState<User | null>(null);
  const history = useNavigate();

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      const decodedToken: User = jwtDecode(token);
      setUser(decodedToken);
    }
  }, []);

  const handleLogout = () => {
    localStorage.removeItem('token');
    history('/login');
  };

  return (
    <Layout>
      <Header>
        {user ? (
          <div>
            <Title level={2}>Welcome, {user.username}!</Title>
            <p>Email: {user.email}</p>
            <p>Points: {user.points}</p>
            <Button type="primary" onClick={handleLogout}>
              Logout
            </Button>
          </div>
        ) : (
          <div>
            <Link to="/login">Login</Link> | <Link to="/register">Register</Link>
          </div>
        )}
      </Header>
      <Content>
        <Title level={1}>Landing Page</Title>
        <p>This is the landing page content.</p>
      </Content>
    </Layout>
  );
};

export default LandingPage;
