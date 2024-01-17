import {
  ControllerData,
  useControllerData,
} from "@/app/hooks/useControllerData";
import { MnkData, useMnkData } from "@/app/hooks/useMnkData";
import { createContext, useContext } from "react";

const ReMnkContext = createContext<
  { mnk: MnkData; controller: ControllerData } | undefined
>(undefined);

export function ReMnKDataProvider({ children }: { children: React.ReactNode }) {
  const mnkData = useMnkData();
  const controllerData = useControllerData();

  return (
    <ReMnkContext.Provider
      value={{
        mnk: mnkData,
        controller: controllerData,
      }}
    >
      {children}
    </ReMnkContext.Provider>
  );
}

export function useReMnkContext() {
  const context = useContext(ReMnkContext);

  if (!context) throw Error("Please use ReMnkContext within a ReMnkProvider!");

  return context;
}
