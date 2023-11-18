use pancurses::*;

use crate::bash_viewr::bash_viewer;
use std::process::exit;

#[derive(Clone)]
pub struct Options {
    options: Vec<String>,
}

const VALID_COMMANDS: [&str; 10] = [" grep", " awk ", " sed ", " echo ", " cat ", " ls ", " find ", " head ", " tail ", " wc "];

pub fn menu(window: &crate::Window) {
    let mut curr_selector = 0;
    let max_selector = VALID_COMMANDS.len() as i32 - 1;

    setup_menu(&window, curr_selector);

    loop {
        match window.getch() {
            Some(Input::Character(q)) if q == 'q' || q == 'Q' => {
                curs_set(1);
                endwin();
                exit(0);
            }
            Some(Input::KeyResize) => {
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            Some(Input::Character(j)) if j == 'j' || j == 'J' => {
                if curr_selector != 0 {
                    curr_selector -= 1;
                }
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            Some(Input::KeyUp) => {
                if curr_selector != 0 {
                    curr_selector -= 1;
                }
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            Some(Input::Character(k)) if k == 'k' || k == 'K' => {
                if curr_selector < max_selector {
                    curr_selector += 1;
                }
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            Some(Input::KeyDown) => {
                if curr_selector < max_selector {
                    curr_selector += 1;
                }
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            Some(Input::Character('\n')) => {
                break;
            }
            _ => {}
        }
    }
    bash_viewer(window, VALID_COMMANDS[curr_selector as usize]);
}

/// takes care of primary window setup, inclusive of borders and title
pub fn base_menu(window: &crate::Window, options: Vec<String>) {
    window.attrset(COLOR_PAIR(3));
    draw_border(window);
    window.mvaddstr(0, window.get_max_x() / 3, "Bashtastic Visualizer");
    let menu_keybindings = Options { options: options };

    window.mv(window.get_max_y() - 1, 0);

    for option in menu_keybindings.options.iter() {
        window.attron(COLOR_PAIR(3));
        window.printw(&option);
        window.attron(COLOR_PAIR(1));
        window.printw(" ");
    }
}

/// Takes care of setting up the main manu, with the given optios for the user and Instructions.
fn setup_menu(window: &crate::Window, selector: i32) {
    window.clear();
    base_menu(
        window,
        vec![
            String::from(" Q: Quit "),
            String::from(" J or Key: Up "),
            String::from(" K or Key: Down "),
            String::from(" Select: Enter "),
        ],
    );

    let valid_commands: Vec<String> = VALID_COMMANDS.iter().map(|&s| s.to_string()).collect();
    let menu_selections = Options {
        options: valid_commands,
    };

    let mid_screen_y = window.get_max_y() / 3;
    let mid_screen_x = 2;

    for (y_increment, option) in menu_selections.options.iter().enumerate() {
        if selector == y_increment as i32 {
            window.attrset(COLOR_PAIR(2));
        } else {
            window.attrset(COLOR_PAIR(1));
        }
        window.mvaddstr(mid_screen_y + y_increment as i32, mid_screen_x, option);
    }
}

/// Draw generic borders of the same type for consistency
pub fn draw_border(window: &crate::Window) {
    window.border(' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ');
}
