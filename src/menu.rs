
use pancurses::*;

use crate::Options;
use std::process::exit;

pub fn menu_setup(window: &crate::Window) {
    let mut curr_selector = 0;
    let max_selector = 2;

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
            Some(Input::Character(q)) if q == 'j' || q == 'J' => {
                if curr_selector != 0 {
                    curr_selector -= 1;
                }
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            Some(Input::Character(q)) if q == 'k' || q == 'K' => {
                if curr_selector != max_selector {
                    curr_selector += 1;
                }
                window.erase();
                setup_menu(&window, curr_selector);
                window.refresh();
            }
            _ => {}
        }
    }

}

pub fn setup_menu(window: &crate::Window, selector: i32) {
    let menu_keybindings = Options {
        options: vec![
            String::from(" Q: Quit "),
            // String::from(" H: Left "),
            // String::from(" L: Right "),
            String::from(" J: Up "),
            String::from(" K: Down "),
            String::from(" Select: Enter "),
        ],
    };

    window.mv(window.get_max_y() - 1, 0);

    for option in menu_keybindings.options.iter() {
        window.attron(COLOR_PAIR(3));
        window.printw(&option);
        window.attron(COLOR_PAIR(1));
        window.printw(" ");

    }

    let menu_selections = Options {
        options: vec![
            String::from(" 1: Todo's "),
            String::from(" 2: Reminders "),
            String::from(" 3: Birthdays"),
        ]
    };  
    
    let mid_screen_y = window.get_max_y()/2;
    let mid_screen_x = window.get_max_x()/3;

    for (y_increment, option) in menu_selections.options.iter().enumerate() {
        if selector == y_increment as i32 {
            window.attrset(COLOR_PAIR(2));
        } else {
            window.attrset(COLOR_PAIR(1));
        }
        window.mvaddstr(mid_screen_y+y_increment as i32, mid_screen_x, option);
    }
}