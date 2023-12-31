use std::rc::Rc;

mod models;

mod db;
use db::JiraDatabase;

mod ui;

mod io_utils;
use io_utils::{get_trimed_user_input, wait_for_key_press};

mod navigator;
use navigator::Navigator;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {error}\nPress any key to continue...");
                wait_for_key_press();
            };

            let user_input = get_trimed_user_input();

            match page.handle_input(user_input.as_str()) {
                Err(error) => {
                    println!("Error getting user input: {error}\nPress any key to continue...");
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {error}\nPress any key to continue...");
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
