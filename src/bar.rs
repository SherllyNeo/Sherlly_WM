use crate::{BAR_HEIGHT_PX, BLACK, BLUE, FONT, GREY, MAX_ACTIVE_WINDOW_CHARS, WHITE,RED};
use penrose::{x::XConn, Color};
use penrose_ui::{
    bar::{
        widgets::{
            current_date_and_time, wifi_network, ActiveWindowName,
            CurrentLayout, Workspaces,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};
use penrose_ui::bar::widgets::{RefreshText};
use std::fs;
use crate::reader::reader;
use crate::api_call::api_call;
use penrose::util::spawn_for_output_with_args;

fn battery_sum(bat: &'static str, style: &TextStyle) -> RefreshText {
        RefreshText::new(style, move || battery_text(bat).unwrap_or_default())
}

fn battery_text(bat: &str) -> Option<String> {
        let status = read_sys_file(bat, "status")?;
        let charge: u32 = read_sys_file(bat,"capacity")?.parse().ok()?;



        let icon = if status == "Charging" {
                       ""
                   } else if charge >= 90 || status == "Full" {
                    ""
                   } else if charge >= 70 {
                    ""
                  } else if charge >= 50 {
                   ""
                   } else if charge >= 20 {
                       ""
                     } else {
                   ""
                       };

         Some(format!("{icon} {charge}% | "))
}
pub fn dt(style: &TextStyle) -> RefreshText {
    RefreshText::new(style, || {
        spawn_for_output_with_args("date", &["+%F (%a) %R"])
            .unwrap_or_default()
            .trim()
            .to_string()
    })
}
pub fn name(style: &TextStyle) -> RefreshText {
    RefreshText::new(style, || {
        spawn_for_output_with_args("sb-hostname",&["Sherlly"])
            .unwrap_or_default()
            .trim()
            .to_string()
    })
}

fn read_sys_file(bat: &str, fname: &str) -> Option<String> {
        fs::read_to_string(format!("/sys/class/power_supply/{bat}/{fname}"))
                    .ok()
                            .map(|s| s.trim().to_string())
}

fn weather_sum(style: &TextStyle) -> RefreshText {
        RefreshText::new(style, move || api_call("https://wttr.in/?format=4").unwrap_or_default())
}

// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = RED.into();
    let empty_ws: Color = GREY.into();

    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 10,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };

    let padded_style = TextStyle {
        padding: (4.0, 2.0),
        ..style.clone()
    };

    StatusBar::try_new(
        Position::Top,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        &[&style.font],
        vec![
            Box::new(Workspaces::new(&style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(&style)),
            // Box::new(penrose_bar::widgets::debug::StateSummary::new(style)),
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                &TextStyle {
                    bg: Some(highlight),
                    padding: (6.0, 4.0),
                    ..style.clone()
                },
                true,
                false,
            )),
            Box::new(name(&padded_style)),
            Box::new(wifi_network(&padded_style)),
            Box::new(battery_sum("BAT1", &padded_style)),
            Box::new(battery_sum("BAT0", &padded_style)),
            Box::new(weather_sum(&padded_style)),
            //Box::new(amixer_volume("Master", &padded_style)),
            Box::new(dt(&padded_style)),
        ],
    )
}
