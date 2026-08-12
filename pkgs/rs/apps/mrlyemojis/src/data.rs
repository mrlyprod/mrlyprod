/// A named group of emoji in the catalog.
pub struct Category {
    /// The label the picker shows and matches on.
    pub name: &'static str,
    /// The characters the category offers, in grid order.
    pub emojis: &'static [&'static str],
}

/// The full catalog, in picker order.
pub const CATEGORIES: &[Category] = &[
    Category {
        name: "smileys",
        emojis: &[
            "😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "😊", "😇", "🙂", "🙃", "😉", "😌",
            "😍", "🥰", "😘", "😋", "😛", "😜", "🤪", "🤨", "🧐", "🤓", "😎", "🥳", "😏", "😒",
            "😔", "😟", "🙁", "😣", "😖", "😫", "😩", "🥺", "😢", "😭", "😤", "😠", "😡", "🤯",
            "😳", "🥵", "🥶", "😱", "😨", "😰", "😥", "🤗", "🤔", "🤭", "🤫", "🤥", "😶", "😐",
            "😑", "😬", "🙄", "😴", "🤤", "🤢", "🤮", "🤧", "😷", "🤒", "🤕", "🤠", "🥴", "🥱",
            "😪", "😵", "🤐", "🥸", "🤩", "🥲", "😈", "👿", "👹", "👺", "🤡", "👻", "💀", "👽",
            "👾", "🤖", "💩", "😺", "😸", "😹", "😻", "😼", "😽", "🙀", "😿", "😾",
        ],
    },
    Category {
        name: "gestures",
        emojis: &[
            "👋", "🤚", "👌", "🤌", "🤏", "🤞", "🤟", "🤘", "🤙", "👈", "👉", "👆", "👇", "👍",
            "👎", "✊", "👊", "🤛", "🤜", "👏", "🙌", "👐", "🤲", "🤝", "🙏", "💪", "🤳", "🖕",
            "👂", "👃", "👀", "👅", "👄", "🧠", "🦷", "🦴", "👶", "🧒", "👦", "👧", "🧑", "👨",
            "👩", "🧓", "👴", "👵",
        ],
    },
    Category {
        name: "animals",
        emojis: &[
            "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐸",
            "🐵", "🙈", "🙉", "🙊", "🐒", "🐔", "🐧", "🐦", "🐤", "🐣", "🐥", "🦆", "🦅", "🦉",
            "🦇", "🐺", "🐗", "🐴", "🦄", "🐝", "🐛", "🦋", "🐌", "🐞", "🐜", "🦗", "🦂", "🐢",
            "🐍", "🦎", "🐙", "🦑", "🦐", "🦞", "🦀", "🐡", "🐠", "🐟", "🐬", "🐳", "🐋", "🦈",
            "🐊", "🐅", "🐆", "🦓", "🦍", "🦧", "🐘", "🦛", "🦏", "🐪", "🐫", "🦒", "🦘", "🐃",
            "🐂", "🐄", "🐎", "🐖", "🐏", "🐑", "🦙", "🐐", "🦌", "🐕", "🐩", "🦮", "🐈", "🐓",
            "🦃", "🦚", "🦜", "🦢", "🦩", "🐇", "🦝", "🦨", "🦡", "🦦", "🦥", "🐁", "🐀", "🐿",
            "🦔", "🐾", "🐉", "🐲",
        ],
    },
    Category {
        name: "food",
        emojis: &[
            "🍎", "🍏", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🫐", "🍈", "🍒", "🍑", "🥭",
            "🍍", "🥥", "🥝", "🍅", "🍆", "🥑", "🥦", "🥬", "🥒", "🌽", "🥕", "🧄", "🧅", "🥔",
            "🍠", "🥐", "🥯", "🍞", "🥖", "🥨", "🧀", "🥚", "🍳", "🥞", "🧇", "🥓", "🥩", "🍗",
            "🍖", "🌭", "🍔", "🍟", "🍕", "🥪", "🌮", "🌯", "🥙", "🧆", "🥘", "🍝", "🍜", "🍲",
            "🍛", "🍣", "🍱", "🥟", "🍤", "🍙", "🍚", "🍘", "🍥", "🥠", "🍢", "🍡", "🍧", "🍨",
            "🍦", "🥧", "🧁", "🍰", "🎂", "🍮", "🍭", "🍬", "🍫", "🍿", "🍩", "🍪", "🌰", "🥜",
            "🍯", "🥛", "🍼", "🍵", "🧃", "🥤", "🍶", "🍺", "🍻", "🥂", "🍷", "🥃", "🍸", "🍹",
            "🍾", "🧉",
        ],
    },
    Category {
        name: "nature",
        emojis: &[
            "🌵", "🎄", "🌲", "🌳", "🌴", "🌱", "🌿", "🍀", "🎍", "🎋", "🍃", "🍂", "🍁", "🍄",
            "🌾", "💐", "🌷", "🌹", "🥀", "🌺", "🌸", "🌼", "🌻", "🌞", "🌝", "🌛", "🌜", "🌚",
            "🌕", "🌖", "🌗", "🌘", "🌑", "🌒", "🌓", "🌔", "🌙", "🌎", "🌍", "🌏", "🪐", "💫",
            "⭐", "🌟", "🔥", "💥", "🌈", "🌊", "💧", "💦", "🌋",
        ],
    },
    Category {
        name: "activities",
        emojis: &[
            "⚽", "🏀", "🏈", "⚾", "🥎", "🎾", "🏐", "🏉", "🥏", "🎱", "🪀", "🏓", "🏸", "🏒",
            "🏑", "🥍", "🏏", "🪁", "🏹", "🎣", "🥊", "🥋", "🎽", "🛹", "🥌", "🎿", "🏂", "🤼",
            "🤸", "🤺", "🤾", "🏄", "🏊", "🤽", "🚣", "🧗", "🚴", "🚵", "🏆", "🥇", "🥈", "🥉",
            "🏅", "🎪", "🤹", "🎭", "🎨", "🎬", "🎤", "🎧", "🎼", "🎹", "🥁", "🎷", "🎺", "🎸",
            "🪕", "🎻", "🎲", "🎯", "🎳", "🎮", "🎰", "🧩",
        ],
    },
    Category {
        name: "travel",
        emojis: &[
            "🚗", "🚕", "🚙", "🚌", "🚎", "🚓", "🚑", "🚒", "🚐", "🚚", "🚛", "🚜", "🛴", "🚲",
            "🛵", "🛺", "🚨", "🚀", "🛸", "🚁", "⛵", "🚤", "🛳", "🚢", "🚂", "🚆", "🚄", "🚅",
            "🚈", "🚇", "🚊", "🚉", "🏰", "🏯", "🎡", "🎢", "🎠", "🗼", "🗽", "🗿", "🏠", "🏡",
            "🏢", "🏥", "🏦", "🏨", "🏪", "🏫", "🏬", "🏭", "🌋", "🏕", "🏖", "🏝", "⚓", "🧭", "🧳",
        ],
    },
    Category {
        name: "objects",
        emojis: &[
            "⌚", "📱", "💻", "📷", "📸", "📹", "🎥", "📞", "📟", "📠", "📺", "📻", "⏰", "📡",
            "🔋", "🔌", "💡", "🔦", "🧯", "💸", "💵", "💰", "💳", "💎", "🔧", "🔨", "🔩", "🧰",
            "🔫", "💣", "🧨", "🔪", "🔮", "📿", "🧿", "🔭", "🔬", "💊", "💉", "🩸", "🧬", "🦠",
            "🧫", "🧪", "🧹", "🧺", "🧻", "🚽", "🚿", "🛁", "🧼", "🧽", "🔑", "🚪", "🛏", "🧸", "🛍",
            "🛒", "🎁", "🎈", "🎀", "🎉", "🎊", "📩", "💌", "📦", "📜", "📄", "📊", "📈", "📉",
            "📅", "📋", "📁", "📂", "📰", "📕", "📗", "📘", "📙", "📚", "📖", "🔖", "🔗", "📎",
            "📌", "📍", "📝", "🔍", "🔎", "🔒", "🔓",
        ],
    },
    Category {
        name: "symbols",
        emojis: &[
            "💛", "💚", "💙", "💜", "🖤", "🤍", "🤎", "💔", "💕", "💞", "💓", "💗", "💖", "💘",
            "💝", "💟", "💯", "💢", "💤", "⭕", "❌", "🚫", "🔴", "🟠", "🟡", "🟢", "🔵", "🟣",
            "🟤", "⚫", "⚪", "🔺", "🔻", "🔶", "🔷", "🔸", "🔹", "💠", "🔘", "🎵", "🎶", "💲",
            "🔔", "💬", "💭", "✅", "❎", "🆔", "🆕", "🆒", "🔟", "🔢", "🟥", "🟧", "🟨", "🟩",
            "🟦", "🟪", "⬛", "⬜", "🟫",
        ],
    },
];

/// Lists every category label in catalog order.
pub fn names() -> Vec<&'static str> {
    CATEGORIES.iter().map(|c| c.name).collect()
}

/// Reports whether the catalog holds a category by this label.
pub fn has(name: &str) -> bool {
    CATEGORIES.iter().any(|c| c.name == name)
}

/// Reports whether any category contains this character.
pub fn known(value: &str) -> bool {
    CATEGORIES.iter().any(|c| c.emojis.contains(&value))
}

/// Hands back the labeled category's emoji, empty when no such category exists.
pub fn grid(name: &str) -> &'static [&'static str] {
    CATEGORIES
        .iter()
        .find(|c| c.name == name)
        .map(|c| c.emojis)
        .unwrap_or(&[])
}

/// Names the category the picker opens on.
pub fn first() -> &'static str {
    CATEGORIES[0].name
}

/// Reports whether a string is emoji-shaped: one to eight scalars, none blank or control.
pub fn is_grapheme(value: &str) -> bool {
    let count = value.chars().count();
    (1..=8).contains(&count) && !value.chars().any(|c| c.is_whitespace() || c.is_control())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_single_scalar() {
        let mut bad: Vec<String> = Vec::new();
        for cat in CATEGORIES {
            for e in cat.emojis {
                if e.chars().count() != 1 {
                    bad.push(format!("{}:{e} ({} scalars)", cat.name, e.chars().count()));
                }
            }
        }
        assert!(bad.is_empty(), "multi-scalar entries: {}", bad.join(", "));
    }
    #[test]
    fn categories_are_named_and_unique() {
        assert!(CATEGORIES.len() >= 6);
        let mut labels = names();
        let n = labels.len();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), n);
        for cat in CATEGORIES {
            assert!(!cat.emojis.is_empty(), "{} is empty", cat.name);
        }
    }
    #[test]
    fn count_is_a_few_hundred() {
        let total: usize = CATEGORIES.iter().map(|c| c.emojis.len()).sum();
        assert!(total >= 200, "only {total} emojis");
    }
    #[test]
    fn grid_and_has_agree() {
        assert!(has(first()));
        assert!(!has("nope"));
        assert!(!grid(first()).is_empty());
        assert!(grid("nope").is_empty());
    }
    #[test]
    fn known_spots_catalog_members() {
        assert!(known("🍎"));
        assert!(known("🎲"));
        assert!(!known("x"));
        assert!(!known(""));
    }
    #[test]
    fn is_grapheme_accepts_emoji_rejects_junk() {
        assert!(is_grapheme("😀"));
        assert!(is_grapheme("❤️"));
        assert!(is_grapheme("a"));
        assert!(!is_grapheme(""));
        assert!(!is_grapheme("a b"));
        assert!(!is_grapheme("aaaaaaaaa"));
    }
}
