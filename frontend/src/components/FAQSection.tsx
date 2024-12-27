import React from 'react';
import { Card, Typography } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const FAQSection: React.FC = () => {
  return (
	<Card>
		<Title level={2}>
			Frequently Asked Questions
		</Title>

		<Paragraph>
			Here are some of the most frequently asked questions about our application.
		</Paragraph>

		<ul>
			<li>
				<strong>
					Question 1:
				</strong>

				{' '}
				Answer to question 1.
			</li>

			<li>
				<strong>
					Question 2:
				</strong>

				{' '}
				Answer to question 2.
			</li>

			<li>
				<strong>
					Question 3:
				</strong>

				{' '}
				Answer to question 3.
			</li>

			<li>
				<strong>
					Question 4:
				</strong>

				{' '}
				Answer to question 4.
			</li>

			<li>
				<strong>
					Question 5:
				</strong>

				{' '}
				Answer to question 5.
			</li>
		</ul>
	</Card>
  );
};

export default FAQSection;
