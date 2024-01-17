"use client";

import { useControllerData } from "@/app/hooks/useControllerData";

export default function VisualController() {
  const controllerData = useControllerData();

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
          style={{
            transform: `translate(${controllerData.analogLeftX}px, ${controllerData.analogLeftY}px)`,
          }}
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
            transform: `translate(${controllerData.analogRightX}px, ${controllerData.analogRightY}px)`,
          }}
        />
      </div>
    </div>
  );
}
