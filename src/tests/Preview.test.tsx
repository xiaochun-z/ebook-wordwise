import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import Preview from '../components/Preview';
import { platform } from '@tauri-apps/plugin-os';

// Mock Tauri API
vi.mock('@tauri-apps/plugin-os', () => ({
  platform: vi.fn(),
}));

describe('Preview Component', () => {
  it('renders HTML content correctly', () => {
    vi.mocked(platform).mockResolvedValue('linux');
    
    render(<Preview innerHTML="<p data-testid='inner-html'>Test Content</p>" />);
    
    expect(screen.getByTestId('inner-html')).toBeInTheDocument();
    expect(screen.getByTestId('inner-html')).toHaveTextContent('Test Content');
  });

  it('sets maxHeight based on platform windows', async () => {
    vi.mocked(platform).mockResolvedValue('windows');
    
    const { container } = render(<Preview innerHTML="<p>Test</p>" />);
    
    await waitFor(() => {
      expect(container.firstChild).toHaveStyle({ height: '215px' });
    });
  });

  it('sets maxHeight based on platform macos', async () => {
    vi.mocked(platform).mockResolvedValue('macos');
    
    const { container } = render(<Preview innerHTML="<p>Test</p>" />);
    
    await waitFor(() => {
      expect(container.firstChild).toHaveStyle({ height: '190px' });
    });
  });
});
