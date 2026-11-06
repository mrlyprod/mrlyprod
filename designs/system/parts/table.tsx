import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Table({ head, rows, align = [], caption }: Props["table"]) {
  return (
    <div className="tabular">
      <table>
        {caption && <caption>{caption}</caption>}
        <thead>
          <tr>
            {head.map((name, c) => (
              <th key={c} scope="col" data-align={align[c]}>
                {name}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, r) => (
            <tr key={r}>
              {row.map((cell, c) => (
                <td key={c} data-align={align[c]}>
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
