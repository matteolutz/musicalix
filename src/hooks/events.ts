import { useEffect } from "react";
import { events } from "../bindings";

type EventListener<T> = {
  listen: (cb: (event: { payload: T }) => void) => Promise<() => void>;
};

type EventKey = keyof typeof events;

export const useTauriEventHandler = <T>(
  eventListener: EventListener<T>,
  callback: (payload: T) => void,
) => {
  useEffect(() => {
    const unlisten = eventListener.listen((event) => callback(event.payload));

    return () => {
      unlisten.then((unlisten) => unlisten());
    };
  }, []);
};

export const useKeyedTauriEventHandler = <E extends EventKey>(
  event: E,
  callback: (
    payload: (typeof events)[E] extends EventListener<infer T> ? T : never,
  ) => void,
) => {
  const eventListener = events[event] as EventListener<any>;
  useTauriEventHandler(eventListener, callback);
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
) => {
  useKeyedTauriEventHandler(event, (payload) => {
    const payloads = payload as Record<string, unknown>;
    const eventType = Object.keys(payloads)[0];

    callbacks[eventType as VariantKeys<PayloadOf<E>>](
      payloads[eventType] as any,
    );
  });
};
