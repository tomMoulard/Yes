import React from 'react';
import { render, screen } from '@testing-library/react';
import TestimonialsSection from '../TestimonialsSection';

describe('TestimonialsSection', () => {
  test('renders TestimonialsSection component', () => {
    render(<TestimonialsSection />);
    const titleElements = screen.queryAllByText(/Testimonials/i);
    expect(titleElements.length).toBeGreaterThan(0);
  });
});
