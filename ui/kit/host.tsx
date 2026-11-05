import type { ReactNode } from "react";

export type FigureProps = { name: string; label?: string; className?: string; caption?: ReactNode };

export function Figure({ name, label = "", className, caption }: FigureProps) {
  return (
    <figure className={className} data-figure={name}>
      <canvas role={label ? "img" : undefined} aria-label={label || undefined} aria-hidden={label ? undefined : true} />
      {caption !== undefined && <figcaption>{caption}</figcaption>}
    </figure>
  );
}
