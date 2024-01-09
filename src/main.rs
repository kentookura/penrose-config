//! penrose :: minimal configuration
//!
//! This file will give you a functional if incredibly minimal window manager that
//! has multiple workspaces and simple client / workspace movement.
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::hooks::{
        add_ewmh_hooks, add_named_scratchpads, manage::FloatingCentered, NamedScratchPad,
        ToggleNamedScratchPad,
    },
    map, stack,
    x::query::ClassName,
    x11rb::RustConn,
    Result,
};
use penrose_config::{
    bar::status_bar, ReserveBottom, BAR_HEIGHT_PX, BLACK, BLUE, INNER_PX, MAX_MAIN, OUTER_PX,
    RATIO, RATIO_STEP, WHITE,
};
use penrose_ui::core::TextStyle;
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn raw_key_bindings(
    toggle_mail: ToggleNamedScratchPad,
) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-h" => send_layout_message(|| ShrinkMain),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-m" => Box::new(toggle_mail),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-S-c" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-f" => modify_with(|cs| cs.next_layout()),
        //"M-S-grave" => modify_with(|cs| cs.previous_layout()),
        "M-S-h" => send_layout_message(|| IncMain(1)),
        "M-S-l" => send_layout_message(|| IncMain(-1)),
        "M-semicolon" => spawn("dmenu_run"),
        "M-Return" => spawn("wezterm"),
        "M-b" => spawn("qutebrowser"),
        "M-A-q" => exit(),
    };

    for tag in &["1", "2", "3", "4", "5"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        //MainAndStack::side_mirrored(MAX_MAIN, RATIO, RATIO_STEP),
        //MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveBottom::wrap(layout, BAR_HEIGHT_PX))
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();
    let (mail_sp, toggle_mail) = NamedScratchPad::new(
        "mail",
        "wezterm -e neomutt",
        ClassName("MailScratchpad"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings(toggle_mail))?;
    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        focus_follow_mouse: false,
        focused_border: BLUE.into(),
        ..Config::default()
    });
    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };

    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;
    let bar = status_bar().unwrap();
    let wm = bar.add_to(wm);
    let wm = add_named_scratchpads(wm, vec![mail_sp]);

    wm.run()
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn bindings_parse_correctly_with_xmodmap() {
//        let res = parse_keybindings_with_xmodmap(raw_key_bindings());
//
//        if let Err(e) = res {
//            panic!("{e}");
//        }
//    }
//}
//
