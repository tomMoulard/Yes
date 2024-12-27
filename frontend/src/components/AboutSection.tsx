import React from 'react';
import { Card, Typography } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const AboutCard = styled(Card)`
  margin: 20px;
  padding: 20px;
  background-color: #f0f2f5;
`;

const AboutSection: React.FC = () => {
  return (
	<AboutCard>
		<Title level={2}>
			About Us
		</Title>

		<Paragraph>
			Welcome to our application! We are dedicated to providing the best service possible.
		</Paragraph>

		<Paragraph>
			Our team is composed of experienced professionals who are passionate about what they do.
		</Paragraph>

		<Paragraph>
			We continuously strive to improve and innovate to meet the needs of our users.
		</Paragraph>
	</AboutCard>
  );
};

export default AboutSection;
