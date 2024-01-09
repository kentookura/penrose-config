#![warn(clippy::all)]
#![warn(future_incompatible, rust_2018_idioms)]
use penrose::{
    core::bindings::KeyEventHandler,
    core::layout::{Layout, LayoutTransformer},
    pure::geometry::Rect,
    x11rb::RustConn,
};

pub mod bar;

pub type KeyHandler = Box<dyn KeyEventHandler<RustConn>>;

pub const FONT: &str = "Dina";
pub const BLACK: u32 = 0x282828ff;
pub const WHITE: u32 = 0xebdbb2ff;
pub const GREY: u32 = 0x3c3836ff;
//pub const BLUE: u32 = 0x458588ff;
pub const BLUE: u32 = 0x7fbbb3ff;

pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.6;
pub const RATIO_STEP: f32 = 0.1;
pub const OUTER_PX: u32 = 5;
pub const INNER_PX: u32 = 5;
pub const BAR_HEIGHT_PX: u32 = 20;
pub const MAX_ACTIVE_WINDOW_CHARS: usize = 50;

pub const DEBUG_ENV_VAR: &str = "PENROSE_DEBUG";

pub const MON_1: &str = "eDP-1";
pub const MON_2: &str = "HDMI-2";

/// Reserve `px` pixels at the top of the screen.
///
/// Typically used for providing space for a status bar.
#[derive(Debug, Clone)]
pub struct ReserveBottom {
    /// The wrapped inner layout
    pub layout: Box<dyn Layout>,
    /// The number of pixels to reserve at the top of the screen
    pub px: u32,
}

impl ReserveBottom {
    /// Wrap an existing [Layout] with the given reserved area.
    pub fn wrap(layout: Box<dyn Layout>, px: u32) -> Box<dyn Layout> {
        Box::new(Self { layout, px })
    }
}

impl LayoutTransformer for ReserveBottom {
    fn transformed_name(&self) -> String {
        self.layout.name()
    }

    fn inner_mut(&mut self) -> &mut Box<dyn Layout> {
        &mut self.layout
    }

    fn transform_initial(&self, mut r: Rect) -> Rect {
        if r.w == 0 || r.h == 0 {
            return r;
        }

        //r.y = self.px;
        r.h -= self.px;

        r
    }
}
