import React from 'react';
import { Card, Typography } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const HomeSection: React.FC = () => {
  return (
	<Card>
		<Title level={2}>
			Welcome to Our Application
		</Title>

		<Paragraph>
			This is the home section of our application. Here you can find a brief introduction and welcome message.
		</Paragraph>

		<Paragraph>
			We are excited to have you here and hope you enjoy using our application.
		</Paragraph>
	</Card>
  );
};

export default HomeSection;
