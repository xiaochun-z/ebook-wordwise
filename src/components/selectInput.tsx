import React from 'react';
import { Select } from 'flowbite-react';

interface SelectInputProps {
  id: string;
  label: string;
  value: string | number;
  options: { value: string | number; text: string }[];
  onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
}

const SelectInput: React.FC<SelectInputProps> = ({ id, label, value, options, onChange }) => {
  return (
    <div className="flex-auto">
      <label htmlFor={id} className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
        {label}
      </label>
      <Select
        id={id}
        value={value}
        onChange={onChange}
        sizing="sm"
        className="[&_select]:!py-1 [&_select]:!h-8 [&_select]:!px-2.5"
      >
        {options.map(({ value, text }) => (
          <option key={value} value={value}>
            {text}
          </option>
        ))}
      </Select>
    </div>
  );
};

export default SelectInput;