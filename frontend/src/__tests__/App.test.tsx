import React from 'react';
import { render } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import App from '../App';

test('renders App component without crashing', () => {
  const { getByText } = render(<App />);
  expect(getByText('Landing Page')).toBeInTheDocument();
});
