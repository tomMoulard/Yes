import React from 'react';
import { render, screen } from '@testing-library/react';
import App from '../src/App';

test('renders Bidding App heading', () => {
  render(<App />);
  const headingElement = screen.getByText(/Bidding App/i);
  expect(headingElement).toBeInTheDocument();
});

test('renders welcome message', () => {
  render(<App />);
  const welcomeElement = screen.getByText(/Welcome to the Bidding App!/i);
  expect(welcomeElement).toBeInTheDocument();
});
