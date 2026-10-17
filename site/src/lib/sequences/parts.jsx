export const shift = (hit) => (hit?.shift ? ` ${hit.shift > 0 ? '+' : ''}${hit.shift}` : '');

export const Record = ({ hit }) => (
  <>
    <a href={`https://oeis.org/${hit.id}`} target="_blank" rel="noopener">{hit.id}</a>
    {shift(hit)}
  </>
);

export const Found = ({ found }) =>
  found.length
    ? found.map(({ record, at }, i) => (
        <span key={record.id}>
          {i ? ' · ' : ''}
          <a href={`https://oeis.org/${record.id}`} target="_blank" rel="noopener">{record.id}</a> at {at}: {record.name}
        </span>
      ))
    : 'no curated record holds these terms';
