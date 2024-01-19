"use client";

import { useEffect, useState } from "react";
import { Event, listen } from "@tauri-apps/api/event";

export const MAX_ANALOG_VALUE = 10;

export type ControllerData = {
  padUp: boolean;
  padDown: boolean;
  padLeft: boolean;
  padRight: boolean;
  buttonSquare: boolean;
  buttonCross: boolean;
  buttonCircle: boolean;
  buttonTriangle: boolean;
  buttonL1: boolean;
  buttonL2: boolean;
  buttonL3: boolean;
  buttonR1: boolean;
  buttonR2: boolean;
  buttonR3: boolean;
  buttonSelect: boolean;
  buttonStart: boolean;
  analogLeftX: number;
  analogLeftY: number;
  analogRightX: number;
  analogRightY: number;
};

export function useControllerData() {
  const [controllerData, setControllerData] = useState<ControllerData>(
    initialControllerData()
  );

  useEffect(() => {
    const unListen = listen("controller-event", (event: Event<string>) => {
      // console.log("controller-event:", event.payload);
      handleCommand(event.payload, setControllerData);
    });

    return () => {
      unListen.then((fn) => fn());
    };
  }, []);

  return controllerData;
}

function handleCommand(
  command: string,
  setState: React.Dispatch<React.SetStateAction<ControllerData>>
) {
  setState((prev) => {
    const newState = { ...prev };

    switch (command) {
      // D-Pad
      case "press_pad_up":
        newState.padUp = true;
        break;
      case "release_pad_up":
        newState.padUp = false;
        break;
      case "press_pad_down":
        newState.padDown = true;
        break;
      case "release_pad_down":
        newState.padDown = false;
        break;
      case "press_pad_left":
        newState.padLeft = true;
        break;
      case "release_pad_left":
        newState.padLeft = false;
        break;
      case "press_pad_right":
        newState.padRight = true;
        break;
      case "release_pad_right":
        newState.padRight = false;
        break;

      // Buttons
      case "press_square":
        newState.buttonSquare = true;
        break;
      case "release_square":
        newState.buttonSquare = false;
        break;
      case "press_cross":
        newState.buttonCross = true;
        break;
      case "release_cross":
        newState.buttonCross = false;
        break;
      case "press_circle":
        newState.buttonCircle = true;
        break;
      case "release_circle":
        newState.buttonCircle = false;
        break;
      case "press_triangle":
        newState.buttonTriangle = true;
        break;
      case "release_triangle":
        newState.buttonTriangle = false;
        break;

      // Bumpers
      case "press_l1":
        newState.buttonL1 = true;
        break;
      case "release_l1":
        newState.buttonL1 = false;
        break;
      case "press_r1":
        newState.buttonR1 = true;
        break;
      case "release_r1":
        newState.buttonR1 = false;
        break;

      // Triggers
      case "press_l2":
        newState.buttonL2 = true;
        break;
      case "release_l2":
        newState.buttonL2 = false;
        break;
      case "press_r2":
        newState.buttonR2 = true;
        break;
      case "release_r2":
        newState.buttonR2 = false;
        break;

      // Select & Start
      case "press_select":
        newState.buttonSelect = true;
        break;
      case "release_select":
        newState.buttonSelect = false;
        break;
      case "press_start":
        newState.buttonStart = true;
        break;
      case "release_start":
        newState.buttonStart = false;
        break;

      // Left Analog
      case "press_l3_in":
        newState.buttonL3 = true;
        break;
      case "release_l3_in":
        newState.buttonL3 = false;
        break;
      case "press_l3_right":
        newState.analogLeftX = MAX_ANALOG_VALUE;
        break;
      case "release_l3_right":
        if (newState.analogLeftX == MAX_ANALOG_VALUE) newState.analogLeftX = 0;
        break;
      case "press_l3_left":
        newState.analogLeftX = -MAX_ANALOG_VALUE;
        break;
      case "release_l3_left":
        if (newState.analogLeftX == -MAX_ANALOG_VALUE) newState.analogLeftX = 0;
        break;
      case "press_l3_up":
        newState.analogLeftY = -MAX_ANALOG_VALUE;
        break;
      case "release_l3_up":
        if (newState.analogLeftY == -MAX_ANALOG_VALUE) newState.analogLeftY = 0;
        break;
      case "press_l3_down":
        newState.analogLeftY = MAX_ANALOG_VALUE;
        break;
      case "release_l3_down":
        if (newState.analogLeftY == MAX_ANALOG_VALUE) newState.analogLeftY = 0;
        break;

      // Right Analog
      default:
        if (command.includes("r3_")) {
          let aim_command = command.replace("r3_", "");
          let xy = aim_command.split("_");

          if (xy.length === 2) {
            newState.analogRightX = parseFloat(xy[0]);
            newState.analogRightY = parseFloat(xy[1]);

            // TODO: remove ... this is temporary and adjustments should be done in BE

            newState.analogRightX /= 100;
            newState.analogRightY /= 100;
          }
        }
        break;
    }

    if (command.includes("r3_")) {
      console.log(
        "r3_ case -> newState analog right:",
        newState.analogRightX,
        newState.analogRightY
      );
    }

    return newState;
  });
}

function initialControllerData(): ControllerData {
  return {
    padUp: false,
    padDown: false,
    padLeft: false,
    padRight: false,
    buttonSquare: false,
    buttonCross: false,
    buttonCircle: false,
    buttonTriangle: false,
    buttonL1: false,
    buttonL2: false,
    buttonL3: false,
    buttonR1: false,
    buttonR2: false,
    buttonR3: false,
    buttonSelect: false,
    buttonStart: false,
    analogLeftX: 0,
    analogLeftY: 0,
    analogRightX: 0,
    analogRightY: 0,
  };
}
