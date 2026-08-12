# mrlyos

The Mrly kernel: one deterministic world of apps, verbs and journals, in plain data. An app is anything that names its verbs and answers calls; the kernel installs a set of them, routes each call to its app, and journals what happened. Nothing hides in objects - identity, calls, outcomes and the whole world all speak plain JSON.

Time never leaks in on its own: a call may carry its moment, and the world only moves when called. After any call the kernel folds the world into an envelope - the tick, the route, the focused view, the last outcome, plus any effects or notices - one frame ready for a screen or a wire.

- **App** is the contract: a route, verbs, calls, and state to save and load.
- **Os** hosts the apps, routes the calls, and keeps the journal ring.
- **Envelope** is one frame of the whole world in plain JSON.
- **Iden** says who is acting; **Manifest** is an app's listing.
