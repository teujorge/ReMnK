"use client";

import { useEffect, useState } from "react";
import { Event, listen } from "@tauri-apps/api/event";

export type MnkData = {
  keyPress: string;
  keyRelease: string;
  buttonPress: string;
  buttonRelease: string;
  mouseMove: { x: number; y: number };
  wheel: { dx: number; dy: number };
};

export function useMnkData() {
  const [mnkData, setMnkData] = useState<MnkData>({
    keyPress: "",
    keyRelease: "",
    buttonPress: "",
    buttonRelease: "",
    mouseMove: { x: -1, y: -1 },
    wheel: { dx: -1, dy: -1 },
  });

  useEffect(() => {
    const unListen = listen("mnk-event", (event: Event<string>) => {
      // console.log("mnk-event:", event.payload);

      try {
        const data = JSON.parse(event.payload);
        // console.log("Parsed data:", data);
        setMnkData((prevData) => ({ ...prevData, ...data }));
      } catch (error) {
        console.error("Error parsing JSON:", error);
      }
    });

    return () => {
      unListen.then((fn) => fn());
    };
  }, []);

  return mnkData;
}
