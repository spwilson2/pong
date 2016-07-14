space <!-- The representation of space for use in different subsystems. -->
=====
Will try to be a generic abstraction of 2D space allowing rectangular objects of 
  different sizes.
Should be generic enough that other systems will be able to provide wrappers around
  these 'objects' to suit their own needs.

physics
======
Will deal with movement and taking movement inputs from both the user and AI.
Will either need &mut access to the board or will need to develop a system for other
  readers to access the board.

user_interaction
================
Will be in charge of output to the terminal and dealing with ncurses.
Will need read access to the board state.

    start_menu <!-- The main menu -->
    ==========
    * Play
    * Play vs. AI
    * Options <- Add in support later..
    * Exit

    display_board <!-- How the board will be displayed -->
    =============
    Needs & access to the board.

    interact_user 
    =============
    Handle user input (passing? -- called by?) the correct system
    In menu: Will send to the menu system to move cursor around.
    In game: Will send to the physics engine to move player.


ai <!--A simple AI that should always just head to the same horizontal level as the ball. -->
==
Will need read access the board state.

Main <!-- Will be a loop calling into subsystems, likely will need some sort of callback -->
====
