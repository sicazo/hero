import { useEffect, useMemo, useRef, useState } from "react";

type RectObserver = Omit<DOMRectReadOnly, 'toJSON'>;

const default_state: RectObserver = {
  x: 0,
  y: 0,
  width: 0,
  height: 0,
  top: 0,
  left: 0,
  bottom: 0,
  right: 0,
};


export function useResize<T extends HTMLElement = any>(options?: ResizeObserverOptions) {
  const frame_id = useRef(0);
  const ref = useRef<T>(null);

  const [rect, setRect] = useState<RectObserver>(default_state);

  const observer = useMemo(
    () => typeof window !== "undefined"
      ? new ResizeObserver((entries: any) => {
        const entry = entries[0];

        if (entry) {
          cancelAnimationFrame(frame_id.current);

          frame_id.current = requestAnimationFrame(() => {
            if (ref.current) {
              setRect(entry.contentRect);
            }
          })
        }
      })
      : null,
    []
  )

  useEffect(() => {
    if (ref.current) {
      observer?.observe(ref.current, options)
    }

    return () => {
      observer?.disconnect();

      if (frame_id.current) {
        cancelAnimationFrame(frame_id.current);
      }
    }
  }, [ref.current])

  return [ref, rect] as const;
}


export function useSize<T extends HTMLElement = any>(options?: ResizeObserverOptions) {
  const [ref, { width, height }] = useResize<T>(options);
  return { ref, width, height }
}
