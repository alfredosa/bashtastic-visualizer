use crate::{
    menu::{base_menu, draw_border},
    Window,
};
use pancurses::*;
use std::process::{exit, Command};

/// This function takes care of the Input Window in the TUI.
pub fn bash_viewer(window: &Window, command_type: &str) {
    setup_bash_window(window);
    let mut subwindow = setup_bash_viewr_subwindow(window, command_type);
    let mut input_text_window = setup_bash_viewr_input_text(window, &subwindow);
    subwindow.refresh();
    input_text_window.refresh();

    curs_set(2);

    loop {
        let current_y = input_text_window.get_cur_y();
        let current_x = input_text_window.get_cur_x();

        match input_text_window.getch() {
            Some(Input::Character(enter)) if enter == '\n' => {}
            Some(Input::Character(backspace)) if backspace == '\u{7f}' => {
                if current_x != 1 {
                    input_text_window.mvaddch(current_y, current_x - 1, ' ');
                    input_text_window.mv(current_y, current_x - 1);
                    update_windows(&subwindow, &input_text_window, command_type);
                }
            }
            Some(Input::Character(esc)) if esc == '\x1b' => {
                curs_set(1);
                endwin();
                exit(0);
            }
            Some(Input::Character(q)) => {
                input_text_window.addch(q);
                input_text_window.refresh();
                update_windows(&subwindow, &input_text_window, command_type);
            }
            Some(Input::KeyBackspace) => {
                if current_x != 1 {
                    input_text_window.mvaddch(current_y, current_x - 1, ' ');
                    input_text_window.mv(current_y, current_x - 1);
                    update_windows(&subwindow, &input_text_window, command_type);
                }
            }
            Some(Input::KeyResize) => {
                // we teardown and rebuild the windows
                let current_query = read_current_query(&input_text_window);
                setup_bash_window(window);
                input_text_window.delwin();
                subwindow.delwin();
                subwindow = setup_bash_viewr_subwindow(window, command_type);
                input_text_window = setup_bash_viewr_input_text(window, &subwindow);

                input_text_window.mvaddstr(1, 1, &current_query);
                update_windows(&subwindow, &input_text_window, command_type);
                input_text_window.refresh();
            }
            Some(Input::KeyRight) => {
                if current_x < input_text_window.get_max_x() - 2 {
                    input_text_window.mv(current_y, current_x + 1);
                }
            }
            Some(Input::KeyLeft) => {
                if current_x > 1 {
                    input_text_window.mv(current_y, current_x - 1);
                }
            }
            _ => {}
        }
    }
}

fn setup_bash_viewr_subwindow(window: &Window, command_type: &str) -> Window {
    let subwindow = window
        .subwin((window.get_max_y() / 4) * 3, window.get_max_x() - 2, 1, 1)
        .expect("Couldn't create command visualizer subwindow");

    setup_inner_bash_window(&subwindow, command_type);
    subwindow.refresh();

    subwindow
}

fn setup_bash_viewr_input_text(window: &Window, subwindow: &Window) -> Window {

    let input_text_window = window
        .subwin(
            window.get_max_y() / 4 - 1,
            window.get_max_x() - 2,
            subwindow.get_max_y() + 1,
            1,
        )
        .expect("Couldn't create Input Text subwindow");
    setup_inner_input_window(&input_text_window, "Input terminal");
    input_text_window.mv(1, 1);
    input_text_window.keypad(true);
    input_text_window.nodelay(true);
    input_text_window.refresh();

    input_text_window
}

fn setup_bash_window(window: &Window) {
    window.clear();
    base_menu(
        window,
        vec![
            String::from(" Esc: Quit "),
            String::from(" Select: Visualize "),
            String::from(" Backspace: Delete "),
            String::from(" <-: Left "),
            String::from(" ->: Right "),
        ],
    );
    window.refresh();
}

fn read_current_query(window: &Window) -> String {
    window.mv(1, 1);
    let mut prev_char_empty = true;
    let mut current_query = String::new();
    
    for x in 1..window.get_max_x() - 1 {
        let ch = window.mvinch(1, x);
        match ch as u8 as char {
            ' ' => {
                if !prev_char_empty {
                    current_query.push(' ');
                    prev_char_empty = true;
                }
                else {
                    break;
                }
            }
            _ => {
                prev_char_empty = false;
                current_query.push(ch as u8 as char);
            }
        }
}
    current_query
}

fn setup_inner_bash_window(window: &Window, title: &str) {
    window.attrset(COLOR_PAIR(2));

    window.mvaddstr(0, 0, title.trim());
    window.refresh();
    window.attrset(COLOR_PAIR(1));
}

fn setup_inner_input_window(window: &Window, title: &str) {
    window.attrset(COLOR_PAIR(2));
    draw_border(window);
    window.mvaddstr(0, 0, title);
    window.refresh();
    window.attrset(COLOR_PAIR(1));
}

fn update_windows(subwindow: &Window, input_text_window: &Window, command_type: &str) {
    let mut result = String::new();
    let (prev_y, prev_x) = input_text_window.get_cur_yx();
    for x in 1..input_text_window.get_max_x() - 1 {
        let ch = input_text_window.mvinch(1, x);
        result.push(ch as u8 as char);
    }
    input_text_window.mv(prev_y, prev_x);
    let args = result.trim();

    let output_result = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", command_type.trim(), args))
        .output();
    subwindow.clear();
    match output_result {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout).into_owned();
            subwindow.attrset(COLOR_PAIR(2));
            subwindow.mvaddstr(0, 0, format!("Executing: '{} {}'", command_type.trim(), args));
            subwindow.attrset(COLOR_PAIR(1));
            subwindow.mvaddstr(2, 0, &output_string);
            subwindow.refresh();
        }
        Err(e) => {
            let error_message = format!("Failed to execute command {:?}: {}", command_type, e);
            subwindow.mvaddstr(1, 1, &error_message);
            subwindow.refresh();
        }
    }
    input_text_window.refresh();
}