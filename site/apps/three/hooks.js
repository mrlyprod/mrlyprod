import { useEffect, useRef, useState } from 'react';

export function useUnit(given, load) {
  const [math, setMath] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    load().then(({ math: one, ready }) => ready.then(() => on && setMath(() => one)));
    return () => {
      on = false;
    };
  }, [given]);
  return given ?? math;
}

export function useThree(given) {
  const [three, setThree] = useState(null);
  useEffect(() => {
    if (given || three) return undefined;
    let on = true;
    import('three').then((mod) => on && setThree(mod));
    return () => {
      on = false;
    };
  }, [given, three]);
  return given ?? three;
}

export function useReroll(seed, roll) {
  const was = useRef(seed);
  const fn = useRef(roll);
  fn.current = roll;
  useEffect(() => {
    if (was.current === seed) return;
    was.current = seed;
    fn.current(seed);
  }, [seed]);
}
