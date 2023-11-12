use crate::{
    menu::{base_menu, draw_border},
    Window,
};
use pancurses::*;
use std::process::{exit, Command};

pub fn bash_viewer(window: &Window, command_type: &str) {
    let mut result = String::new();
    let mut command = Command::new("sh")
        .spawn()
        .expect("sh command failed to start");

    setup_bash_window(window);
    let subwindow = window
        .subwin((window.get_max_y() / 4) * 3, window.get_max_x() - 2, 1, 1)
        .expect("Couldn't create subwindow");

    setup_inner_bash_window(&subwindow, command_type);
    subwindow.refresh();

    let input_text_window = window
        .subwin(
            window.get_max_y() / 4,
            window.get_max_x() - 2,
            subwindow.get_max_y(),
            1,
        )
        .expect("Couldn't create subwindow");
    setup_inner_bash_window(&input_text_window, "Input terminal: Type below");
    input_text_window.mv(1, 1);
    input_text_window.keypad(true);
    input_text_window.nodelay(true);
    input_text_window.refresh();
    curs_set(2);

    loop {
        let current_y = input_text_window.get_cur_y();
        let current_x = input_text_window.get_cur_x();

        match input_text_window.getch() {
            Some(Input::Character(enter)) if enter == '\n' => {}
            Some(Input::Character(esc)) if esc == '\x1b' => {
                curs_set(1);
                endwin();
                exit(0);
            }
            Some(Input::Character(q)) => {
                input_text_window.addch(q);
                input_text_window.refresh();
            }
            Some(Input::KeyBackspace) => {
                if current_x != 1 {
                    input_text_window.mvaddch(current_y, current_x - 1, ' ');
                    input_text_window.mv(current_y, current_x - 1);
                }
            }
            _ => {}
        }
    }
}

fn setup_bash_window(window: &Window) {
    window.clear();
    base_menu(
        window,
        vec![
            String::from(" Esc: Quit "),
            String::from(" Select: Visualize "),
        ],
    );
    window.refresh();
}

fn setup_inner_bash_window(window: &Window, title: &str) {
    window.attrset(COLOR_PAIR(2));
    draw_border(window);
    window.mvaddstr(0, 0, title);
    window.refresh();
    window.attrset(COLOR_PAIR(1));
}
