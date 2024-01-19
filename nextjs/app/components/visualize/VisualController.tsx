"use client";

import {
  MAX_ANALOG_VALUE,
  useControllerData,
} from "@/app/hooks/useControllerData";

export default function VisualController() {
  const controllerData = useControllerData();

  const { x: leftX, y: leftY } = transformLeftJoystick(
    controllerData.analogLeftX,
    controllerData.analogLeftY
  );

  return (
    <div className="controller">
      {/* D-Pad */}
      <div className={`button ${controllerData.padUp ? "active" : ""}`}>↑</div>
      <div className={`button ${controllerData.padDown ? "active" : ""}`}>
        ↓
      </div>
      <div className={`button ${controllerData.padLeft ? "active" : ""}`}>
        ←
      </div>
      <div className={`button ${controllerData.padRight ? "active" : ""}`}>
        →
      </div>

      {/* Buttons */}
      <div className={`button ${controllerData.buttonCross ? "active" : ""}`}>
        X
      </div>
      <div className={`button ${controllerData.buttonCircle ? "active" : ""}`}>
        O
      </div>
      <div
        className={`button ${controllerData.buttonTriangle ? "active" : ""}`}
      >
        △
      </div>
      <div className={`button ${controllerData.buttonSquare ? "active" : ""}`}>
        □
      </div>

      {/* Bumpers and Triggers */}
      <div className={`button ${controllerData.buttonL1 ? "active" : ""}`}>
        L1
      </div>
      <div className={`button ${controllerData.buttonL2 ? "active" : ""}`}>
        L2
      </div>
      <div className={`button ${controllerData.buttonR2 ? "active" : ""}`}>
        R2
      </div>
      <div className={`button ${controllerData.buttonR1 ? "active" : ""}`}>
        R1
      </div>

      {/* Joystick Left */}
      <div className={`joystick ${controllerData.buttonL3 ? "active" : ""}`}>
        <div
          className="joystick__inner"
          style={{ transform: `translate(${leftX}px, ${leftY}px)` }}
        />
      </div>

      <div className={`button ${controllerData.buttonStart ? "active" : ""}`}>
        Strt
      </div>
      <div className={`button ${controllerData.buttonSelect ? "active" : ""}`}>
        Slct
      </div>

      {/* Joystick Right */}
      <div className={`joystick ${controllerData.buttonR3 ? "active" : ""}`}>
        <div
          className="joystick__inner"
          style={{
            transform: `translate(${Math.min(
              controllerData.analogRightX,
              MAX_ANALOG_VALUE
            )}px, ${Math.min(
              controllerData.analogRightY,
              MAX_ANALOG_VALUE
            )}px)`,
          }}
        />
      </div>
    </div>
  );
}

// function to transform the square bounds of the left joystick into a circle
function transformLeftJoystick(x: number, y: number): { x: number; y: number } {
  const isXMoved = Math.abs(x) > 0;
  const isYMoved = Math.abs(y) > 0;

  if (isXMoved && isYMoved) {
    const angle = Math.atan2(y, x);
    x = Math.cos(angle) * MAX_ANALOG_VALUE;
    y = Math.sin(angle) * MAX_ANALOG_VALUE;
  }

  return { x, y };
}
