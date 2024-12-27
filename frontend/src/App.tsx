import './App.css';
import React, { Suspense, lazy } from 'react';
import { Layout } from 'antd';

const { Content } = Layout;

const HomeSection = lazy(() => import('./components/HomeSection'));
const AboutSection = lazy(() => import('./components/AboutSection'));
const AuthorSection = lazy(() => import('./components/AuthorSection'));
const FeaturesSection = lazy(() => import('./components/FeaturesSection'));
const TestimonialsSection = lazy(() => import('./components/TestimonialsSection'));
const FAQSection = lazy(() => import('./components/FAQSection'));

export default function App() {
  return (
	<Layout>
		<Content style={{ padding: '0 50px', marginTop: 64 }}>
			<Suspense fallback={<div>
				Loading...
			</div>}
			>
				<div id="home">
					<HomeSection />
				</div>

				<div id="about">
					<AboutSection />
				</div>

				<div id="author">
					<AuthorSection />
				</div>

				<div id="features">
					<FeaturesSection />
				</div>

				<div id="testimonials">
					<TestimonialsSection />
				</div>

				<div id="faq">
					<FAQSection />
				</div>
			</Suspense>
		</Content>
	</Layout>
  );
}
