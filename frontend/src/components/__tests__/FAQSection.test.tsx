import React from 'react';
import { render, screen } from '@testing-library/react';
import FAQSection from '../FAQSection';

describe('FAQSection', () => {
  test('renders FAQSection component', () => {
    render(<FAQSection />);
    const titleElements = screen.queryAllByText(/Frequently Asked Questions/i);
    expect(titleElements.length).toBeGreaterThan(0);
  });
});
