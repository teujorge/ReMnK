"use client";

import { useState } from "react";
import VisualController from "./VisualController";
import VisualMnK from "./VisualMnK";

export default function Visualize() {
  const [isShowingController, setIsShowingController] = useState(true);
  const [isShowingMnK, setIsShowingMnK] = useState(true);

  return (
    <div className="flex flex-col gap-4 w-full h-full">
      <div className="flex flex-col w-full h-full gap-2">
        <div className="flex flex-row items-center gap-2">
          <h2>Virtual Controller</h2>
          <button
            className={`flex items-center justify-center rounded-full h-7 w-7
                ${isShowingController ? "bg-green-500" : "bg-red-500"}
              `}
            onClick={() => setIsShowingController(!isShowingController)}
          >
            {isShowingController ? "on" : "off"}
          </button>
        </div>
        {isShowingController && <VisualController />}
      </div>

      <div className="flex flex-col w-full h-full gap-2">
        <div className="flex flex-row items-center gap-2">
          <h2>Virtual MnK</h2>
          <button
            className={`flex items-center justify-center rounded-full h-7 w-7
                ${isShowingMnK ? "bg-green-500" : "bg-red-500"}
              `}
            onClick={() => setIsShowingMnK(!isShowingMnK)}
          >
            {isShowingMnK ? "on" : "off"}
          </button>
        </div>
        {isShowingMnK && <VisualMnK />}
      </div>
    </div>
  );
}
