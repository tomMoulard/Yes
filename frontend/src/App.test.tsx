import { render, screen } from '@testing-library/react';
import App from './App';
import React from 'react';
import { act } from 'react';

describe('App', () => {
	test('make sure the app renders', () => {
		act(() => {
			render(<App />);
		});
	});

	test('renders learn react link', () => {
		act(() => {
			render(<App />);
		});
		const linkElement = screen.getByText(/learn react/iu);
		expect(linkElement).toBeInTheDocument();
	});
});
