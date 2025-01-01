import React from 'react';
import { render, screen } from '@testing-library/react';
import AboutSection from '../AboutSection';

describe('AboutSection', () => {
  test('renders AboutSection component', () => {
    render(<AboutSection />);
    const titleElements = screen.queryAllByText(/About Us/i);
    expect(titleElements.length).toBeGreaterThan(0);
  });
});
