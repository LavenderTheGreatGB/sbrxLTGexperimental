#![allow(dead_code)]

extern crate nfd;

use conrod;
use self::nfd::Response;
use std;

use self::super::*;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub struct GuiState {
    chosen_file: String,
}

impl GuiState {
    pub fn new() -> Self {
        GuiState {
            chosen_file: "no ROM open".to_string(),
        }
    }
}

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "sbrx theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_RED,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

// create the ids
widget_ids! {
    pub struct Ids {
        canvas,
        canvas_scrollbar,

        title,
        subtitle,

        file_chooser_button,
        file_chooser_text,
    }
}

pub fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut GuiState) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 32;

    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    widget::Text::new("sbrx")
        .font_size(TITLE_SIZE)
        .top_left_of(ids.canvas)
        .set(ids.title, ui);

    widget::Text::new(&format!("version {} by phase", VERSION))
        .padded_w_of(ids.canvas, MARGIN)
        .top_right_of(ids.canvas)
        .down(5.0)
        .line_spacing(5.0)
        .set(ids.subtitle, ui);

    for _press in widget::Button::new()
        .label("Open ROM")
        .small_font(ui)
        .top_right_of(ids.canvas)
        .w_h(70.0, 35.0)
        .set(ids.file_chooser_button, ui)
        {
            let result = nfd::dialog().filter("gba").open().unwrap_or_else(|e| {
                panic!(e);
            });
            match result {
                Response::Okay(file_path) => {
                    println!("File path = {:?}", file_path);
                    app.chosen_file = file_path;
                }
                Response::Cancel => println!("User canceled"),
                _ => (),
            }
        }

    widget::Text::new(&app.chosen_file)
        .bottom_right_of(ids.file_chooser_button)
        .font_size(10)
        .down(5.0)
        .align_right()
        .set(ids.file_chooser_text, ui);

    widget::Scrollbar::y_axis(ids.canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);
}
