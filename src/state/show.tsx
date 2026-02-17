import {
  createContext,
  FC,
  PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react";
import { commands, Show } from "../bindings";
import { useKeyedEnumTauriEventHandler } from "../hooks/events";

export const ShowContext = createContext<Show | null>(null);

export const useShow = () => {
  const show = useContext(ShowContext);

  if (!show) {
    throw new Error("useShow must be used within a ShowProvider");
  }

  return show;
};

export const ShowProvider: FC<PropsWithChildren> = ({ children }) => {
  const [show, setShow] = useState<Show | null>(null);

  const requestShow = async () => {
    const showRes = await commands.getShow();
    if (showRes.status != "ok") {
      return;
    }

    setShow(showRes.data);
  };

  useEffect(() => {
    requestShow();
  }, []);

  useKeyedEnumTauriEventHandler("showEvent", {
    Loaded: (show) => {
      setShow(show);
    },
  });

  useKeyedEnumTauriEventHandler(
    "actorEvent",
    {
      Added: ([id, actor]) => {
        if (show === null) return;

        show.mixConfig.actors[id] = actor;
        setShow({ ...show });
      },
      Removed: (id) => {
        if (show === null) return;

        delete show.mixConfig.actors[id];
        setShow({ ...show });
      },
    },
    [show],
  );

  if (!show) {
    return <div>Loading...</div>;
  }

  return <ShowContext.Provider value={show}>{children}</ShowContext.Provider>;
};
