#![warn(clippy::all)]
#![warn(future_incompatible, rust_2018_idioms)]
use penrose::{core::bindings::KeyEventHandler, x11rb::RustConn};

pub mod actions;
pub mod bar;
pub mod bindings;
pub mod layouts;
pub mod reader;
pub mod api_call;

pub type KeyHandler = Box<dyn KeyEventHandler<RustConn>>;

pub const FONT: &str = "monospace:size=20";
pub const BLACK: u32 = 0x282828ff;
pub const WHITE: u32 = 0xebdbb2ff;
pub const GREY: u32 = 0x3c3836ff;
pub const BLUE: u32 = 0x458588ff;
pub const RED: u32 = 0xd83917ff;

pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.6; //changed from 0.6
pub const RATIO_STEP: f32 = 0.1;
pub const OUTER_PX: u32 = 10; //changed from 5
pub const INNER_PX: u32 = 10; //changed from 5
pub const BAR_HEIGHT_PX: u32 = 25; //changed from 18
pub const MAX_ACTIVE_WINDOW_CHARS: usize = 50;

pub const DEBUG_ENV_VAR: &str = "PENROSE_DEBUG";

pub const MON_1: &str = "eDP-1";
pub const MON_2: &str = "HDMI-2";

pub const TERMINAL: &str = "st";
pub const BROWSER: &str = "librewolf";
