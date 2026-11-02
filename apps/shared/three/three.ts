import { useEffect, useState } from "react";

export type Three = typeof import("three");

export function useThree(wanted: boolean, given?: Three | null): Three | null {
  const [three, setThree] = useState<Three | null>(null);
  useEffect(() => {
    if (given || !wanted || three) return undefined;
    let on = true;
    import("three").then((mod) => on && setThree(mod));
    return () => {
      on = false;
    };
  }, [given, wanted, three]);
  return given ?? three;
}
