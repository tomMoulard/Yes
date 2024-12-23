import React, { useState } from 'react';
import { Form, Input, Button, Typography, message } from 'antd';
import { UserApi } from '../api/apis/UserApi';
import { User } from '../api/models/User';
import { useHistory } from 'react-router-dom';

const { Title } = Typography;

const LoginRegisterPage: React.FC = () => {
  const [isLogin, setIsLogin] = useState(true);
  const history = useHistory();
  const userApi = new UserApi();

  const onFinish = async (values: User) => {
    try {
      if (isLogin) {
        const token = await userApi.loginUser({ user: values });
        localStorage.setItem('token', token);
        history.push('/');
      } else {
        const token = await userApi.registerUser({ user: values });
        localStorage.setItem('token', token);
        history.push('/');
      }
    } catch (error) {
      if (error.response && error.response.status === 401) {
        message.error('Unauthorized: Incorrect email or password.');
      } else if (error.response && error.response.status === 404) {
        message.error('Not Found: User does not exist.');
      } else {
        message.error('An error occurred. Please try again.');
      }
    }
  };

  return (
    <div>
      <Title level={2}>{isLogin ? 'Login' : 'Register'}</Title>
      <Form
        name="login_register"
        onFinish={onFinish}
      >
        <Form.Item
          name="email"
          rules={[{ required: true, message: 'Please input your email!' }]}
        >
          <Input placeholder="Email" />
        </Form.Item>
        <Form.Item
          name="password"
          rules={[{ required: true, message: 'Please input your password!' }]}
        >
          <Input.Password placeholder="Password" />
        </Form.Item>
        {!isLogin && (
          <Form.Item
            name="username"
            rules={[{ required: true, message: 'Please input your username!' }]}
          >
            <Input placeholder="Username" />
          </Form.Item>
        )}
        <Form.Item>
          <Button type="primary" htmlType="submit">
            {isLogin ? 'Login' : 'Register'}
          </Button>
          <Button type="link" onClick={() => setIsLogin(!isLogin)}>
            {isLogin ? 'Register' : 'Login'} instead
          </Button>
        </Form.Item>
      </Form>
    </div>
  );
};

export default LoginRegisterPage;
