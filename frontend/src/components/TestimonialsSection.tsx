import React from 'react';
import { Card, Typography, Carousel } from 'antd';
import styled from 'styled-components';

const { Title, Paragraph } = Typography;

const TestimonialsCard = styled(Card)`
  margin: 20px;
  padding: 20px;
  background-color: #f0f2f5;
`;

const testimonials = [
  {
    name: 'John Doe',
    feedback: 'This application is amazing! It has helped me a lot.',
  },
  {
    name: 'Jane Smith',
    feedback: 'I love using this app. It is very user-friendly and efficient.',
  },
  {
    name: 'Sam Wilson',
    feedback: 'Great app with excellent features. Highly recommended!',
  },
];

const TestimonialsSection: React.FC = () => {
  return (
	<TestimonialsCard>
		<Title level={2}>
			Testimonials
		</Title>

		<Carousel autoplay>
			{testimonials.map((testimonial, index) => (
				<div key={index}>
					<Paragraph>
						{testimonial.feedback}
					</Paragraph>

					<Paragraph>
						-
						{testimonial.name}
					</Paragraph>
				</div>
        ))}
		</Carousel>
	</TestimonialsCard>
  );
};

export default TestimonialsSection;
