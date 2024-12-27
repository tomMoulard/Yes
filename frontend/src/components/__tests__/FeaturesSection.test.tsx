import React from 'react';
import { render, screen } from '@testing-library/react';
import FeaturesSection from '../FeaturesSection';

describe('FeaturesSection', () => {
  test('renders FeaturesSection component', () => {
    render(<FeaturesSection />);
    const titleElements = screen.queryAllByText(/Features/i);
    expect(titleElements.length).toBeGreaterThan(0);
  });
});
