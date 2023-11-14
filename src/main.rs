#![allow(clippy::many_single_char_names)]

mod bash_viewr;
mod menu;

use menu::*;
use pancurses::*;

fn main() {
    let window = initscr();
    window_setup(&window);
    menu(&window);
    return;
}

/// Initialize the widnow important parameters, including color pairs that will be used
/// throughout the application.
fn window_setup(window: &Window) {
    if has_colors() {
        let mut bg_normal = COLOR_BLACK;
        let bg_unhovered = COLOR_CYAN;
        let bg_hovered = COLOR_GREEN;
        start_color();
        if use_default_colors() == OK {
            bg_normal = -1;
        }

        init_pair(1, COLOR_WHITE, bg_normal);
        init_pair(2, COLOR_WHITE, bg_hovered);
        init_pair(3, COLOR_BLACK, bg_unhovered);
    }

    nl();
    noecho();
    curs_set(0);
    window.timeout(0);
    window.keypad(true);
}
