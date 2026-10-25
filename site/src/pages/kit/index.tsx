import { rows } from "site:routes";
import { island } from "../../lib/app/island.tsx";
import { Kit } from "./kit.tsx";

export const { mount, unmount } = island((host) => {
  const still = host.nextElementSibling;
  still?.remove();
  return {
    node: <Kit row={rows.find((one) => one.meta?.id === "kit")!} />,
    close: () => still && host.after(still),
  };
});
