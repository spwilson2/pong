client
======
Will be the javascript and html library which will connect to the websocket and draw 
objects.

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

ai <!--A simple AI that should always just head to the same horizontal level as the ball. -->
==
Will need read access the board state.

Main <!-- Will be a loop calling into subsystems, likely will need some sort of callback -->
====
Call into the provided gamescript.

GameScript
=========
Will initialize objects for the game itself and be the backend of the socket api.
Needs to provide hooks into the engine to do actions availiable via the api.

server
======
Will be in charge of starting a thread of the game, and sending the data board via websocket
to the client. While receiving player interaction via websocket.


    SocketAPI
    =========
    Offer hooks in the GameScript doing things such as:
    * Moving player(s)
    * Starting the game

