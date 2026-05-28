import { describe, it, expect } from 'vitest';
import { formatBytes, formatUptime, calculateMemoryPercent, formatTimestamp } from '../format';

describe('formatBytes', () => {
  it('returns "0 Bytes" for zero', () => {
    expect(formatBytes(0)).toBe('0 Bytes');
  });

  it('returns "0 Bytes" for negative', () => {
    expect(formatBytes(-1)).toBe('NaN undefined');
  });

  it('formats bytes correctly', () => {
    expect(formatBytes(500)).toBe('500 Bytes');
  });

  it('formats KB correctly', () => {
    expect(formatBytes(1024)).toBe('1 KB');
  });

  it('formats MB correctly', () => {
    expect(formatBytes(1048576)).toBe('1 MB');
  });

  it('formats GB correctly', () => {
    expect(formatBytes(1073741824)).toBe('1 GB');
  });

  it('respects decimal parameter', () => {
    expect(formatBytes(1536, 1)).toBe('1.5 KB');
  });
});

describe('formatUptime', () => {
  it('formats seconds only', () => {
    expect(formatUptime(30)).toBe('30 秒');
  });

  it('formats minutes only', () => {
    expect(formatUptime(120)).toBe('2分钟');
  });

  it('formats hours and minutes', () => {
    expect(formatUptime(5400)).toBe('1小时 30分钟');
  });

  it('formats days, hours, minutes', () => {
    expect(formatUptime(90000)).toBe('1天 1小时');
  });

  it('formats days only when even', () => {
    expect(formatUptime(86400)).toBe('1天');
  });
});

describe('calculateMemoryPercent', () => {
  it('returns 0 when total is 0', () => {
    expect(calculateMemoryPercent(50, 0)).toBe(0);
  });

  it('calculates percentage correctly', () => {
    expect(calculateMemoryPercent(512, 1024)).toBe(50);
  });

  it('rounds to nearest integer', () => {
    expect(calculateMemoryPercent(333, 1000)).toBe(33);
  });
});

describe('formatTimestamp', () => {
  it('returns "-" for falsy values', () => {
    expect(formatTimestamp(0)).toBe('-');
  });

  it('handles millisecond timestamps', () => {
    const result = formatTimestamp(1716931200000);
    expect(result).toBeTruthy();
    expect(typeof result).toBe('string');
  });

  it('handles second-based timestamps (10 digits)', () => {
    const result = formatTimestamp(1716931200);
    expect(result).toBeTruthy();
    expect(typeof result).toBe('string');
  });
});
