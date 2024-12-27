import React from 'react';
import { Card, Typography, Carousel } from 'antd';

const { Title, Paragraph } = Typography;

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
		<Card>
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
		</Card>
	);
};

export default TestimonialsSection;
