import { resolve } from "node:path"
import { routesOf } from "../src/lib/data"
import { scan as check } from "../src/lib/md"
import type { Miss } from "../src/lib/md"
import { scan } from "./scan"

// MAIN

const { site, docs } = scan(resolve(import.meta.dir, ".."))
const routes = routesOf(site)
const misses: Miss[] = []
for (const doc of docs) misses.push(...(await check(doc.route, doc.md, routes)))

for (const miss of misses) console.log(`${miss.route}: broken link ${miss.value}`)
console.log(`links: ${docs.length} docs, ${misses.length} broken`)
if (misses.length > 0) process.exit(1)
