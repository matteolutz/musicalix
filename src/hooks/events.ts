import { DependencyList, useEffect } from "react";
import { events } from "../bindings";

type EventListener<T> = {
  listen: (cb: (event: { payload: T }) => void) => Promise<() => void>;
};

type EventKey = keyof typeof events;

export const useTauriEventHandler = <T>(
  eventListener: EventListener<T>,
  callback: (payload: T) => void,
  deps?: DependencyList,
) => {
  useEffect(() => {
    console.log("listening to eventListener");
    const unlisten = eventListener.listen((event) => callback(event.payload));

    return () => {
      console.log("unlistening");
      unlisten.then((unlisten) => unlisten());
    };
  }, deps ?? []);
};

export const useKeyedTauriEventHandler = <E extends EventKey>(
  event: E,
  callback: (
    payload: (typeof events)[E] extends EventListener<infer T> ? T : never,
  ) => void,
  deps?: DependencyList,
) => {
  const eventListener = events[event] as EventListener<any>;
  useTauriEventHandler(eventListener, callback, deps);
};

type PayloadOf<E extends EventKey> =
  (typeof events)[E] extends EventListener<infer T> ? T : never;

type VariantKeys<U> = U extends U ? keyof U : never;

type ExtractVariant<U, K extends PropertyKey> =
  U extends Record<K, infer V> ? V : never;

type VariantHandlers<E extends EventKey> = {
  [K in VariantKeys<PayloadOf<E>>]: (
    payload: ExtractVariant<PayloadOf<E>, K>,
  ) => void;
};

export const useKeyedEnumTauriEventHandler = <E extends EventKey>(
  event: E,
  callbacks: VariantHandlers<E>,
  deps?: DependencyList,
) => {
  useKeyedTauriEventHandler(
    event,
    (payload) => {
      const payloads = payload as Record<string, unknown>;
      const eventType = Object.keys(payloads)[0];

      callbacks[eventType as VariantKeys<PayloadOf<E>>](
        payloads[eventType] as any,
      );
    },
    deps,
  );
};
