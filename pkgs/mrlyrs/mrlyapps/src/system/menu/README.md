# menu

The launcher behind the desktop: a searchable index of every installed app. It reads the manifest list straight from the system, drops anything marked hidden - itself included - and filters the rest as you type. Matching is a simple case-insensitive substring test against each app's route, title and category.

## Using

- Type to filter the list; an empty query shows every app.
- A search for "tools" matches whole categories, not just names.
- The layout follows the shared *launchpad* setting: grid or list.
- The query is remembered, so the menu reopens exactly as you left it.
