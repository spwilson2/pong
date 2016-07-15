extern crate ncurses;
use ncurses::*;

pub fn init_display() {
  /* Initialize curses */
  initscr();
  start_color();
  cbreak();
  noecho();
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  keypad(stdscr, true);
  init_pair(1, COLOR_RED, COLOR_BLACK);
}

pub struct MainMenu {
    items: Vec<ITEM>,
    menu: MENU,
    window: WINDOW,
}

static EXIT_OPTION: &'static str  = "Exit";

impl MainMenu {
    pub fn new() -> Self {
        /* Create menu items */
        let mut items: Vec<ITEM> = Vec::new();

        items.push(new_item("Play", "Play versus another player."));
        items.push(new_item("(Options)", "Configure controls."));
        items.push(new_item("Play vs. AI", "Play versus a sentient AI."));
        items.push(new_item(EXIT_OPTION, "Exit Pong."));

        /* Crate menu */
        let my_menu = new_menu(&mut items);
        menu_opts_off(my_menu, O_SHOWDESC);

        let my_menu_win = newwin(9, 18, 4, 4);
        keypad(my_menu_win, true);

        /* Set main window and sub window */
        set_menu_win(my_menu, my_menu_win);
        set_menu_sub(my_menu, derwin(my_menu_win, 5, 0, 2, 2));

        /* Set menu mark to the string " * " */
        set_menu_mark(my_menu, " * ");

        /* Print a border around the main window */
        box_(my_menu_win, 0, 0);
        mvprintw(LINES - 3, 0, "Press <ENTER> to see the option selected");
        mvprintw(LINES - 2, 0, "F1 to exit");
        refresh();

        /* Post the menu */
        post_menu(my_menu);
        wrefresh(my_menu_win);

        MainMenu {
            items:items,
            menu:my_menu,
            window:my_menu_win,
        }
    }

    pub fn interact_loop(&mut self) {
        //let ref KEY_F1 = KEY_F(1);
        loop {
            let ch = getch();
            match ch {
                KEY_F1 => break,

                KEY_UP => {
                    menu_driver(self.menu, REQ_UP_ITEM);
                },

                KEY_DOWN => {
                    menu_driver(self.menu, REQ_DOWN_ITEM);
                },

                10 => {/* Enter */
                    mv(20, 0);
                    clrtoeol();
                    pos_menu_cursor(self.menu);

                    match item_name(current_item(self.menu)) {
                        ref item if item == EXIT_OPTION => break,
                        _ => {}
                    }
                },

                _ => {}
            }
            wrefresh(self.window);
        }
    }

    #[cfg(test)]
    pub fn refresh(&mut self) {
        wrefresh(self.window);
    }
    #[cfg(test)]
    pub fn moveup (&mut self) {
        menu_driver(self.menu, REQ_UP_ITEM);
    }
    #[cfg(test)]
    pub fn movedown (&mut self) {
        menu_driver(self.menu, REQ_DOWN_ITEM);
    }
}

impl Drop for MainMenu {
    fn drop(&mut self) {        
        unpost_menu(self.menu);

        /* free items */
        for &item in self.items.iter() {
            free_item(item);
        }

        free_menu(self.menu);
        endwin();
    }
}

