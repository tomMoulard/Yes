import React from 'react';
import { Card, Typography } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const AuthorCard = styled(Card)`
  margin: 20px;
  padding: 20px;
  background-color: #f0f2f5;
`;

const AuthorSection: React.FC = () => {
  return (
	<AuthorCard>
		<Title level={2}>
			About the Author
		</Title>

		<Paragraph>
			Tom Moulard is a passionate developer with years of experience in building web applications.
		</Paragraph>

		<Paragraph>
			He is dedicated to creating high-quality software and continuously improving his skills.
		</Paragraph>

		<Paragraph>
			Tom enjoys sharing his knowledge with the community and contributing to open-source projects.
		</Paragraph>
	</AuthorCard>
  );
};

export default AuthorSection;
