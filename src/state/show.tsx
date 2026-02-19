import {
  createContext,
  FC,
  PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react";
import { commands, Show, ShowState } from "../bindings";
import { useKeyedEnumTauriEventHandler } from "../hooks/events";

type ShowContext = {
  show: Show;
  showState: ShowState;
};

export const ShowContext = createContext<ShowContext | null>(null);

export const useShow = () => {
  const show = useContext(ShowContext);

  if (!show) {
    throw new Error("useShow must be used within a ShowProvider");
  }

  return show.show;
};

export const useShowState = () => {
  const showState = useContext(ShowContext);

  if (!showState) {
    throw new Error("useShowState must be used within a ShowProvider");
  }

  return showState.showState;
};

export const ShowProvider: FC<PropsWithChildren> = ({ children }) => {
  const [show, setShow] = useState<Show | null>(null);
  const [showState, setShowState] = useState<ShowState | null>(null);

  const requestShow = async () => {
    const showRes = await commands.getShow();
    if (showRes.status != "ok") {
      return;
    }

    const [show, showState] = showRes.data;
    setShow(show);
    setShowState(showState);
  };

  useEffect(() => {
    requestShow();
  }, []);

  useKeyedEnumTauriEventHandler(
    "showEvent",
    {
      Loaded: (show) => {
        setShow(show);
      },
      CueAdded: ([idx, cue]) => {
        if (show === null) return;

        // insert cue at idx in show.cues
        show.cues.splice(idx, 0, cue);
        setShow({ ...show });
      },
    },
    [show],
  );

  useKeyedEnumTauriEventHandler("showStateEvent", {
    Update: (showState) => {
      setShowState(showState);
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

  useKeyedEnumTauriEventHandler(
    "groupEvent",
    {
      Added: ([id, actor]) => {
        if (show === null) return;

        show.mixConfig.groups[id] = actor;
        setShow({ ...show });
      },
      Removed: (id) => {
        if (show === null) return;

        delete show.mixConfig.groups[id];
        setShow({ ...show });
      },
    },
    [show],
  );

  if (!show || !showState) {
    return <div>Loading...</div>;
  }

  return (
    <ShowContext.Provider value={{ show, showState }}>
      {children}
    </ShowContext.Provider>
  );
};
