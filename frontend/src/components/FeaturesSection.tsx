import React from 'react';
import { Card, Typography } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const FeaturesSection: React.FC = () => {
  return (
	<Card>
		<Title level={2}>
			Features
		</Title>

		<Paragraph>
			Our application offers a wide range of features to enhance your experience.
		</Paragraph>

		<Paragraph>
			Some of the key features include:
		</Paragraph>

		<ul>
			<li>
				Feature 1: Description of feature 1.
			</li>

			<li>
				Feature 2: Description of feature 2.
			</li>

			<li>
				Feature 3: Description of feature 3.
			</li>

			<li>
				Feature 4: Description of feature 4.
			</li>

			<li>
				Feature 5: Description of feature 5.
			</li>
		</ul>
	</Card>
  );
};

export default FeaturesSection;
