#![allow(unused_imports)]
#![feature(associated_consts)]
extern crate ncurses;
mod ui;
use ui::menu;

#[cfg(feature="menu")]
#[cfg(not(test))]
fn main() {
    menu::init_display();
    let mut main_menu = menu::MainMenu::new();
    main_menu.interact_loop()
}

#[cfg(not(feature="menu"))]
fn main()
{
}

#[cfg(test)]
#[test]
fn main() {
    use std::thread;
    use std::time::Duration;
    {
        menu::init_display();
        let mut main_menu = menu::MainMenu::new();
        for i in 0..1_000 {
            if i % 6 > 2 {
                main_menu.moveup();
            }
            else {
                main_menu.movedown();
            }
            main_menu.refresh();
            thread::sleep(Duration::from_millis(50))
        }
    }
    println!("Done");
}
