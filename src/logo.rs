//! Distro/OS ASCII logos, picked for the system rfetch runs on.
//! Linux art adapted from pfetch (MIT).

const ALPINE: &[&str] = &[
    "   /\\ /\\",
    "  // \\  \\",
    " //   \\  \\",
    "///    \\  \\",
    "//      \\  \\",
    "         \\",
];

const ANDROID: &[&str] = &[
    "  ;,           ,;",
    "   ';,.-----.,;'",
    "  ,'           ',",
    " /    O     O    \\",
    "|                 |",
    "'-----------------'",
];

const ARCH: &[&str] = &[
    "       /\\",
    "      /  \\",
    "     /\\   \\",
    "    /      \\",
    "   /   ,,   \\",
    "  /   |  |  -\\",
    " /_-''    ''-_\\",
];

const ARTIX: &[&str] = &[
    "      /\\",
    "     /  \\",
    "    /`'.,\\",
    "   /     ',",
    "  /      ,`\\",
    " /   ,.'`.  \\",
    "/.,'`     `'.\\",
];

const CENTOS: &[&str] = &[
    " ____^____",
    " |\\  |  /|",
    " | \\ | / |",
    "<---- ---->",
    " | / | \\ |",
    " |/__|__\\|",
    "     v",
];

const DEBIAN: &[&str] = &[
    "  _____",
    " /  __ \\",
    "|  /    |",
    "|  \\___-",
    "-_",
    "  --_",
];

const ENDEAVOUROS: &[&str] = &[
    "      /\\",
    "    //  \\\\",
    "   //    \\ \\",
    " / /     _) )",
    "/_/___-- __-",
    " /____--",
];

const ELEMENTARY: &[&str] = &[
    "  _______",
    " / ____  \\",
    "/  |  /  /\\",
    "|__\\ /  / |",
    "\\   /__/  /",
    " \\_______/",
];

const FEDORA: &[&str] = &[
    "        ,'''''.",
    "       |   ,.  |",
    "       |  |  '_'",
    "  ,....|  |..",
    ".'  ,_;|   ..'",
    "|  |   |  |",
    "|  ',_,'  |",
    " '.     ,'",
    "   '''''",
];

const GARUDA: &[&str] = &[
    "         _______",
    "      __/       \\_",
    "    _/     /      \\_",
    "  _/      /_________\\",
    "_/                  |",
    "\\     ____________",
    " \\_            __/",
    "   \\__________/",
];

const GENTOO: &[&str] = &[
    " _-----_",
    "(       \\",
    "\\    0   \\",
    " \\        )",
    " /      _/",
    "(     _-",
    "\\____-",
];

const LINUX: &[&str] = &[
    "    ___",
    "   (.. |",
    "   (<> |",
    "  / __  \\",
    " ( /  \\ /|",
    "_/\\ __)/_)",
    "\\/-____\\/",
];

const LINUXMINT: &[&str] = &[
    " ___________",
    "|_          \\",
    "  | | _____ |",
    "  | | | | | |",
    "  | | | | | |",
    "  | \\_____/ |",
    "  \\_________/",
];

const MANJARO: &[&str] = &[
    "||||||||| ||||",
    "||||||||| ||||",
    "||||      ||||",
    "|||| |||| ||||",
    "|||| |||| ||||",
    "|||| |||| ||||",
    "|||| |||| ||||",
];

const NIXOS: &[&str] = &[
    "  \\\\  \\\\ //",
    " ==\\\\__\\\\/ //",
    "   //   \\\\//",
    "==//     //==",
    " //\\\\___//",
    "// /\\\\  \\\\==",
    "  // \\\\  \\\\",
];

const OPENSUSE: &[&str] = &[
    "  _______",
    "__|   __ \\",
    "     / .\\ \\",
    "     \\__/ |",
    "   _______|",
    "   \\_______",
    "__________/",
];

const POP: &[&str] = &[
    "______",
    "\\   _ \\        __",
    " \\ \\ \\ \\      / /",
    "  \\ \\_\\ \\    / /",
    "   \\  ___\\  /_/",
    "    \\ \\    _",
    "   __\\_\\__(_)_",
    "  (___________)",
];

const RASPBERRY: &[&str] = &[
    "  __  __",
    " (_\\)(/_)",
    " (_(__)_)",
    "(_(_)(_)_)",
    " (_(__)_)",
    "   (__)",
];

const UBUNTU: &[&str] = &[
    "         _",
    "     ---(_)",
    " _/  ---  \\",
    "(_) |   |",
    "  \\  --- _/",
    "     ---(_)",
];

const VOID: &[&str] = &[
    "    _______",
    " _ \\______ -",
    "| \\  ___  \\ |",
    "| | /   \\ | |",
    "| | \\___/ | |",
    "| \\______ \\_|",
    " -_______\\",
];

const SLACKWARE: &[&str] = &[
    "   ________",
    "  /  ______|",
    "  | |______",
    "  \\______  \\",
    "   ______| |",
    "| |________/",
    "|____________",
];

const MACOS: &[&str] = &[
    "       .:'",
    "    _ :'_",
    " .'`_`-'_``.",
    ":________.-'",
    ":_______:",
    " :_______`-;",
    "  `._.-._.'",
];

/// Omarchy emblem.
const OMARCHY: &[&str] = &[
    "███████████████████████████",
    "██           ██          ██",
    "██  ███████████    ████  ██",
    "██  ██               ██  ██",
    "██  ██               ██  ██",
    "██  ██               ██  ██",
    "██████               ██  ██",
    "██  ██               ██  ██",
    "██  ██               ██  ██",
    "██  ██               ██  ██",
    "██  ███████████████████  ██",
    "██           ██          ██",
    "███████████████  ██████████",
];

/// Four-pane window (Windows).
const WINDOWS: &[&str] = &[
    " ████ ████ ",
    " ████ ████ ",
    " ████ ████ ",
    "           ",
    " ████ ████ ",
    " ████ ████ ",
    " ████ ████ ",
];


/// Map a distro identifier to a logo by substring.
fn from_id(id: &str) -> Option<&'static [&'static str]> {
    let id = id.to_ascii_lowercase();
    Some(match id.as_str() {
        _ if id.contains("omarchy") => OMARCHY,
        _ if id.contains("cachyos") => ARCH,
        _ if id.contains("arch") => ARCH,
        _ if id.contains("artix") => ARTIX,
        _ if id.contains("endeavour") => ENDEAVOUROS,
        _ if id.contains("garuda") => GARUDA,
        _ if id.contains("manjaro") => MANJARO,
        _ if id.contains("alpine") => ALPINE,
        _ if id.contains("raspbian") || id.contains("raspberry") => RASPBERRY,
        _ if id.contains("ubuntu") => UBUNTU,
        _ if id.contains("mint") => LINUXMINT,
        _ if id.contains("pop") => POP,
        _ if id.contains("elementary") => ELEMENTARY,
        _ if id.contains("kali") || id.contains("debian") => DEBIAN,
        _ if id.contains("fedora") => FEDORA,
        _ if id.contains("centos")
            || id.contains("rhel")
            || id.contains("rocky")
            || id.contains("alma") =>
        {
            CENTOS
        }
        _ if id.contains("suse") => OPENSUSE,
        _ if id.contains("nixos") => NIXOS,
        _ if id.contains("gentoo") => GENTOO,
        _ if id.contains("void") => VOID,
        _ if id.contains("slackware") => SLACKWARE,
        _ if id.contains("android") => ANDROID,
        _ => return None,
    })
}

#[cfg(target_os = "linux")]
fn detect_distro() -> Option<&'static [&'static str]> {
    let text = std::fs::read_to_string("/etc/os-release").ok()?;
    let mut id = None;
    let mut like = None;
    for line in text.lines() {
        if let Some(v) = line.strip_prefix("ID=") {
            id = Some(v.trim().trim_matches('"').to_string());
        } else if let Some(v) = line.strip_prefix("ID_LIKE=") {
            like = Some(v.trim().trim_matches('"').to_string());
        }
    }
    if let Some(id) = &id
        && let Some(l) = from_id(id)
    {
        return Some(l);
    }
    if let Some(like) = &like {
        for tok in like.split_whitespace() {
            if let Some(l) = from_id(tok) {
                return Some(l);
            }
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn detect_distro() -> Option<&'static [&'static str]> {
    None
}

/// Logos addressable by name (used by `RFETCH_ASCII`).
fn known(name: &str) -> Option<&'static [&'static str]> {
    Some(match name {
        "omarchy" => OMARCHY,
        "linux" => LINUX,
        "macos" => MACOS,
        "windows" => WINDOWS,
        "arch" => ARCH,
        "artix" => ARTIX,
        "cachyos" => ARCH,
        "endeavouros" | "endeavour" => ENDEAVOUROS,
        "garuda" => GARUDA,
        "manjaro" => MANJARO,
        "alpine" => ALPINE,
        "debian" => DEBIAN,
        "kali" => DEBIAN,
        "ubuntu" => UBUNTU,
        "linuxmint" | "mint" => LINUXMINT,
        "pop" | "popos" => POP,
        "raspbian" | "raspberry" => RASPBERRY,
        "elementary" => ELEMENTARY,
        "fedora" => FEDORA,
        "centos" | "rhel" | "rocky" | "alma" => CENTOS,
        "opensuse" | "suse" => OPENSUSE,
        "nixos" => NIXOS,
        "gentoo" => GENTOO,
        "void" => VOID,
        "slackware" => SLACKWARE,
        "android" => ANDROID,
        _ => return None,
    })
}

/// Returns the logo for the current OS/distro. The `RFETCH_ASCII` environment
/// variable (an OS or distro name, e.g. `ubuntu`) overrides auto-detection.
pub fn ascii() -> &'static [&'static str] {
    if let Ok(v) = std::env::var("RFETCH_ASCII")
        && let Some(l) = known(&v.to_ascii_lowercase())
    {
        return l;
    }
    match std::env::consts::OS {
        "windows" => WINDOWS,
        "macos" => MACOS,
        _ => detect_distro().unwrap_or(LINUX),
    }
}
