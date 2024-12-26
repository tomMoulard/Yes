import { render, screen } from '@testing-library/react';
import App from './App';
import React from 'react';

describe('App', () => {
	test('make sure the app renders', () => {
		render(<App />);
	});

	test('renders learn react link', () => {
		render(<App />);
		const linkElement = screen.getByText(/learn react/iu);
		expect(linkElement).toBeInTheDocument();
	});
});
