import React, { useState } from 'react';
import { Button, Form, InputNumber, Layout, message, Typography } from 'antd';
import { DefaultApi } from '../api/apis/DefaultApi';

const { Header, Content } = Layout;
const { Title } = Typography;

const PurchasePointsPage: React.FC = () => {
  const [points, setPoints] = useState<number>(0);

  const handlePurchase = async () => {
    const api = new DefaultApi();
    try {
      const response = await api.purchasePoints({ purchaseRequest: { points } });
      message.success(`Points purchased successfully! Your new balance is ${response}`);
    } catch (error) {
      message.error('Failed to purchase points. Please try again.');
    }
  };

  return (
    <Layout>
      <Header>
        <Title level={2}>Purchase Points</Title>
      </Header>
      <Content>
        <Form layout="vertical" onFinish={handlePurchase}>
          <Form.Item label="Number of Points" required>
            <InputNumber min={1} value={points} onChange = {(value) => setPoints(value?.valueOf() || 0)} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit">
              Purchase
            </Button>
          </Form.Item>
        </Form>
      </Content>
    </Layout>
  );
};

export default PurchasePointsPage;
