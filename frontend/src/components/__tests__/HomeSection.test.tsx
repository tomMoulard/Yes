import React from 'react';
import { render, screen } from '@testing-library/react';
import HomeSection from '../HomeSection';

describe('HomeSection', () => {
  test('renders HomeSection component', () => {
    render(<HomeSection />);
    const titleElements = screen.queryAllByText(/Welcome to Our Application/i);
    expect(titleElements.length).toBeGreaterThan(0);
  });
});
