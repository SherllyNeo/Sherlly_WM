use crate::actions::power_menu;
use crate::KeyHandler;
use penrose::{
    builtin::{
        actions::{
            floating::{float_focused, reposition, resize, sink_all, sink_focused},
            modify_with, send_layout_message, spawn,
        },
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    extensions::hooks::ToggleNamedScratchPad,
    map,
};
use std::collections::HashMap;
use tracing_subscriber::{reload::Handle, EnvFilter};

// Delta for moving / resizing floating windows
const DELTA: i32 = 10;

// Generate a raw key binding map in terms of parsable string key bindings rather than resolved key codes
pub fn raw_key_bindings<L, S>(
    toggle_scratch: ToggleNamedScratchPad,
    _handle: Handle<L, S>,
) -> HashMap<String, KeyHandler>
where
    L: From<EnvFilter> + 'static,
    S: 'static,
{
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_owned();

        // Windows
        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-q" => modify_with(|cs| cs.kill_focused()),

        // Workspaces
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-S-bracketright" => modify_with(|cs| cs.drag_workspace_forward()),
        "M-S-bracketleft" => modify_with(|cs| cs.drag_workspace_backward()),

        // Layouts
        "M-grave" => modify_with(|cs| cs.next_layout()),
        "M-S-grave" => modify_with(|cs| cs.previous_layout()),
        "M-Up" => send_layout_message(|| IncMain(1)),
        "M-Down" => send_layout_message(|| IncMain(-1)),
        "M-Right" => send_layout_message(|| ExpandMain),
        "M-Left" => send_layout_message(|| ShrinkMain),

        // Launchers
        "M-A-s" => spawn("flameshot gui"), //screenshot
        "M-d" => spawn("dmenu_run"),
        "M-t" => spawn("st"),
        "M-S-t" => spawn("to_do"), //to do program
        "M-slash" => Box::new(toggle_scratch),
        "M-w" => spawn("librewolf"), // browser
        "M-S-w" => spawn("st -e nmtui"), // wifi
        "M-n" => spawn("st -e newsboat"), //news
        "M-S-n" => spawn("rssadd_yt"), //news
        "M-r" => spawn("st -e joshuto"), // file explorer
        "M-c" => spawn("st -e bc"), //calculator
        "M-p" => spawn("st -e abook"), //phone book
        "M-m" => spawn("st -e bashtop"), //manager
        "M-z" => spawn("st -e pass_manager"), //password manger
        "M-i" => spawn("display_bookmarks"), //open bookmarks
        "M-b" => spawn("bookmark_clipped"), //bookmark whats copied
        "M-S-m" => spawn("st -e ncmpcpp"), //music player
        "M-e" => spawn("st -e neomutt"), //terminal email
        "M-S-r" => spawn("dmenurecord"), //recording
        "M-S-i" => spawn("dmenuunicode"), //terminal emoji
        "M-x" => spawn("zathura ~/.local/share/jars_help.pdf "), //help menu
        "M-h" => spawn("templates_tex"), //tex templates helper
        "M-g" => spawn("games"), //games launcher from ~/Games folder
        "M-s" => spawn("study_notes"), //study notes




        // Session management
        "M-A-l" => spawn("slock"),
        "M-A-Escape" => power_menu(),

        // Floating management
        "M-C-f" => float_focused(),
        "M-C-s" => sink_focused(),
        "M-C-S-s" => sink_all(),
        // Floating resize
        "M-C-Right" => resize(DELTA, 0),
        "M-C-Left" => resize(-DELTA, 0),
        "M-C-Up" => resize(0, -DELTA),
        "M-C-Down" => resize(0, DELTA),
        // Floating position
        "M-C-l" => reposition(DELTA, 0),
        "M-C-h" => reposition(-DELTA, 0),
        "M-C-k" => reposition(0, -DELTA),
        "M-C-j" => reposition(0, DELTA),

    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.pull_tag_to_screen(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}
