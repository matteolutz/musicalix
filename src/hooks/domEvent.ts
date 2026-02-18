import { useEffect, useEffectEvent } from 'react';

// I only use the 3 types most of the time, so...
type DOMEventMapDefinitions = [
  [HTMLElement, HTMLElementEventMap],
  [Window, WindowEventMap],
  [Document, DocumentEventMap]
];

type DOMEventSubscriber = DOMEventMapDefinitions[number][0];
type MapDefinitionToEventMap<D extends { [K: number]: unknown[] }, T> = {
  [K in keyof D]: D[K] extends unknown[]
    ? T extends D[K][0]
      ? D[K][1]
      : never
    : never;
}[number];

type GetEventMapFromByElement<T extends DOMEventSubscriber> =
  MapDefinitionToEventMap<DOMEventMapDefinitions, T>;

const useEventListener = <
  TElement extends DOMEventSubscriber,
  TType extends keyof GetEventMapFromByElement<TElement>,
  TEvent extends GetEventMapFromByElement<TElement>[TType]
>(
  element: TElement,
  eventType: TType,
  listener: (event: TEvent) => unknown
) => {
  const eventHandler = useEffectEvent((event: TEvent) => listener(event));

  useEffect(() => {
    element.addEventListener(
      eventType as string,
      eventHandler as EventListener
    );
    return () => {
      element.removeEventListener(
        eventType as string,
        eventHandler as EventListener
      );
    };
  }, [eventType, element]);
};

export default useEventListener;
