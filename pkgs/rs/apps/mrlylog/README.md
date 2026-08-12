# log

A running transcript of the session: every action any app performs lands in a shared ring, and log reads it back newest first. Each entry carries its tick number, the verb that ran, and the arguments it ran with. The log stores nothing of its own - it only mirrors what the session has already recorded.

## Using

- Entries appear newest first, one per action taken this session.
- Each line shows the tick, the verb, and its arguments when there were any.
- **export** writes the whole ring to a markdown file named session.md - oldest first, one numbered line per action.
- Because the app keeps no state, an empty session means an empty log.
