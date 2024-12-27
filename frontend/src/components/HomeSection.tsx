import React from 'react';
import { Card, Typography } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const HomeCard = styled(Card)`
  margin: 20px;
  padding: 20px;
  background-color: #f0f2f5;
`;

const HomeSection: React.FC = () => {
  return (
	<HomeCard>
		<Title level={2}>
			Welcome to Our Application
		</Title>

		<Paragraph>
			This is the home section of our application. Here you can find a brief introduction and welcome message.
		</Paragraph>

		<Paragraph>
			We are excited to have you here and hope you enjoy using our application.
		</Paragraph>
	</HomeCard>
  );
};

export default HomeSection;
