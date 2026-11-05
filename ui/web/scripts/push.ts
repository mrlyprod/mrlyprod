import { main } from "@mrlyprod/os/hosts/cloud/push.ts";
import { build } from "./build.ts";
import { HOME } from "./home.ts";
import { locked } from "./locked.ts";

await main(HOME, () => build().then((done) => locked(done)));
