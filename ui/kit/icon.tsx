import { ICONS, VIEWBOX } from "./icons.gen.js";

export type IconName = keyof typeof ICONS;

export type IconProps = { name: IconName; size?: number };

export function Icon({ name, size = 20 }: IconProps) {
  return (
    <svg viewBox={VIEWBOX} width={size} height={size} fill="currentColor" aria-hidden="true">
      <path d={ICONS[name]} />
    </svg>
  );
}
