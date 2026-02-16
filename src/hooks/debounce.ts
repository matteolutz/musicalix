import { useState, useEffect } from "react";

/**
 * Custom hook to debounce a value.
 *
 * @template T The type of the value being debounced.
 * @param {T} value The value to debounce.
 * @param {number} delay The debounce delay in milliseconds.
 * @returns {T} The debounced value.
 */
export const useDebounce = <T>(value: T, delay: number): T => {
  // State to store the debounced value
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    // Set a timeout to update the debounced value after the specified delay
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    // Cleanup function:
    // This will be called if the 'value' changes (before the timeout fires)
    // or if the component unmounts.
    // It clears the previous timeout to prevent the old value from being used
    // and ensures that only the latest 'value' is debounced.
    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]); // Re-run the effect if 'value' or 'delay' changes

  return debouncedValue;
};
