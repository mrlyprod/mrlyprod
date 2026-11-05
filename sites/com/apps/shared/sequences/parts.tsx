import type { Hit, Match } from "./engine.ts";

export const shift = (hit: Hit | null | undefined) => (hit?.shift ? ` ${hit.shift > 0 ? "+" : ""}${hit.shift}` : "");

export const Record = ({ hit }: { hit: Hit }) => (
  <>
    <a href={`https://oeis.org/${hit.id}`} target="_blank" rel="noopener">
      {hit.id}
    </a>
    {shift(hit)}
  </>
);

export const Found = ({ found }: { found: Match[] }) =>
  found.length
    ? found.map(({ record, at }, i) => (
        <span key={record.id}>
          {i ? " · " : ""}
          <a href={`https://oeis.org/${record.id}`} target="_blank" rel="noopener">
            {record.id}
          </a>{" "}
          at {at}: {record.name}
        </span>
      ))
    : "no curated record holds these terms";
