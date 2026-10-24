import { hide, Lede, show } from "../../shell/parts.jsx";

export const layout = {};

export function render(row, _site, host) {
  show(
    host,
    <>
      <Lede id="cart" title={row.title} lead={row.lead} />
      <p>mrly.net has no shop yet.</p>
      <p>
        <a href="/">Back to the home page</a>.
      </p>
    </>,
  );
}

export const unmount = hide;
