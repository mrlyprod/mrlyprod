import { main } from "@mrlyprod/os/hosts/cloud/push.ts";
import { build } from "./build.ts";
import { HOME } from "./home.ts";

await main(HOME, () => build());
