import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import SelectInput from '../components/selectInput';

describe('SelectInput Component', () => {
  it('renders correctly with given options', () => {
    const mockOnChange = vi.fn();
    const options = [
      { value: 'epub', text: 'EPUB Format' },
      { value: 'mobi', text: 'MOBI Format' }
    ];

    render(
      <SelectInput
        id="format-select"
        label="Format"
        value="epub"
        options={options}
        onChange={mockOnChange}
      />
    );

    expect(screen.getByLabelText('Format')).toBeInTheDocument();
    expect(screen.getByRole('combobox')).toHaveValue('epub');
    expect(screen.getByText('EPUB Format')).toBeInTheDocument();
    expect(screen.getByText('MOBI Format')).toBeInTheDocument();
  });

  it('calls onChange handler when selection changes', () => {
    const mockOnChange = vi.fn();
    const options = [
      { value: 'epub', text: 'EPUB Format' },
      { value: 'mobi', text: 'MOBI Format' }
    ];

    render(
      <SelectInput
        id="format-select"
        label="Format"
        value="epub"
        options={options}
        onChange={mockOnChange}
      />
    );

    fireEvent.change(screen.getByRole('combobox'), { target: { value: 'mobi' } });
    expect(mockOnChange).toHaveBeenCalledTimes(1);
  });
});
