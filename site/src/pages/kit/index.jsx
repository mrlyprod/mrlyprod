import { rows } from "site:routes";
import { island } from "../../lib/app/island.jsx";
import { Kit } from "./kit.jsx";

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return {
    node: <Kit row={rows.find((one) => one.meta?.id === "kit")} />,
    close: () => still && host.after(still),
  };
});
