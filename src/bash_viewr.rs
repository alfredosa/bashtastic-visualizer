use crate::{Window, menu::{
    base_menu, draw_border
}};
use pancurses::*;
use std::process::{exit, Command};

pub fn bash_viewer(window: &Window, command_type: &str) {
    setup_bash_window(window);
    let subwindow = window.subwin((window.get_max_y()/4)*3, window.get_max_x()-2, 1, 1).expect("Couldn't create subwindow");
    subwindow.printw("Hello, World!");
    setup_inner_bash_window(&subwindow);
    subwindow.refresh();

    let input_text_window = window.subwin(window.get_max_y()/4, window.get_max_x()-2, subwindow.get_max_y(), 1).expect("Couldn't create subwindow");
    setup_inner_bash_window(&input_text_window);
    input_text_window.mv(1, 1);
    input_text_window.refresh();
    curs_set(2);

    loop {
        match input_text_window.getch() {
            Some(Input::Character(esc)) if esc == '\x1b' => { // '\x1b' is the escape character
                curs_set(1);
                endwin();
                exit(0);
            }
            Some(Input::Character(q)) => {
                input_text_window.addch(q);
            }
            _ => {

            }
        }
    }

}

fn setup_bash_window(window: &Window) {
    window.clear();
    base_menu(window, vec![
        String::from(" Esc: Quit "),
        String::from(" Select: Visualize "),
    ]);
    window.refresh();
}

fn setup_inner_bash_window(window: &Window) {
    window.attrset(COLOR_PAIR(2));
    draw_border(window);
    window.refresh();
}