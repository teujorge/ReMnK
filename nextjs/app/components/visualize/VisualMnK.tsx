"use client";

import { MnkData, useMnkData } from "@/app/hooks/useMnkData";

export default function VisualMnK() {
  const mnkData = useMnkData();

  function mnkValueToString(value: Required<MnkData>[keyof MnkData]): string {
    // mouseMove and wheel are objects
    if (typeof value === "object") {
      return Object.entries(value)
        .map(([key, value]) => `${key}: ${value.toFixed(0)}`)
        .join(", ");
    }

    // others
    return value.toString();
  }

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 w-full">
      {Object.entries(mnkData).map(([key, value]) => (
        <div
          className="p-3 border border-gray-300 rounded-md flex flex-grow flex-col w-full h-20"
          key={key}
        >
          <strong className="mb-2">{key}:</strong>
          <div className="w-full">{mnkValueToString(value)}</div>
        </div>
      ))}
    </div>
  );
}
