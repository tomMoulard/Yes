import React from 'react';
import { render, screen } from '@testing-library/react';
import AuthorSection from '../AuthorSection';

describe('AuthorSection', () => {
  test('renders AuthorSection component', () => {
    render(<AuthorSection />);
    const titleElements = screen.queryAllByText(/About the Author/i);
    expect(titleElements.length).toBeGreaterThan(0);
  });
});
