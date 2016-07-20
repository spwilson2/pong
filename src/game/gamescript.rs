use physics::plane;
use std::cell::RefCell;
use std::cell::Cell;

const PADDLE_WIDTH: i32 = 5;
const PADDLE_HEIGHT: i32 = 50;

const TOP_LEFT: plane::Coords = plane::Coords{x:0,y:0};
const TOP_RIGHT : plane::Coords = plane::Coords{x:1000, y:0};

const PADDLE_TOP_RIGHT: plane::Coords = plane::Coords {x:TOP_RIGHT.x-PADDLE_WIDTH, y: TOP_RIGHT.y};

//TODO: PongGameScript should implement some traits TODO:tbd.
struct PongGameScript;

struct GameState {
    board: plane::Plane,
    ball_id: Option<i32>,
    player_1: Player,
    player_2: Player,
}

struct Player {
    paddle_id: plane::Id,
    score: i32,
    just_scored: RefCell<Cell<bool>>,
}

impl GameState {
    /// Initialize the Plane, Paddles, and Score.
    pub fn new() -> Self {
        let mut plane = plane::Plane::new();

        // Create the borders. TODO: Add a collision callback with the ball to count a point.

        // Create the paddles
        let mut paddle_1 = plane::Object {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            coords: TOP_LEFT,
            movement: plane::Movement::Rigid,
            mass: plane::Mass::Massless,
            is_collidable: true,
            is_rigid: true,
        };

        let mut paddle_2 = plane::Object {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            coords: PADDLE_TOP_RIGHT,
            movement: plane::Movement::Rigid,
            mass: plane::Mass::Massless,
            is_collidable: true,
            is_rigid: true,
        };

        let paddle_1_id = plane.attach_object(paddle_1);
        let paddle_2_id = plane.attach_object(paddle_2);
        
        GameState {
            board: plane,
            ball_id: None,
            player_1: Player::new(paddle_1_id),
            player_2: Player::new(paddle_2_id),
        }
    }
    
    /// Move the Paddles to their starting positions, move the ball to the scoring
    /// player.
    fn ResetPoint(&mut self) {
    }
    
    /// Update the game a tick.
    pub fn Tick(&mut self) {
        //TODO: Check if a player scored.
        
    }

    fn get_scored(player_scored: &mut RefCell<Cell<bool>>) -> bool {
        player_scored.borrow().get()
    }
    
    pub fn MovePlayer(&mut self) {}
    
    /// Return the positions of paddle, players, and the score.
    pub fn GetBoard(&mut self) {}
    
    /// Reset the Board, and the score.
    pub fn ResetGame(&mut self) {}
    
}

impl Player {
    pub fn new(paddle_id: plane::Id) -> Self {
            Player {
                paddle_id: paddle_id,
                score: 0,
                just_scored: RefCell::new(Cell::new(false)),
            }
    }
}

// TODO: A callback function that will be attached to the wall and ball, which will make
// a player score on collide. Callback updates a global that is in its own struct, so
// don't have to worry about trying to hold a mutable reference to the gamestate object 
// in the callback. Then after a tick, we check to see if this was incremented, if so,
// reset, and count the point.
fn set_scored(scored: &Cell<bool>) {
    // It doesn't *really* matter that this op is atomic, if we miss a tick or so. But 
    // in a single threaded environment this will seem atomic.
    scored.set(true);
}


// TODO: This should be part of a trait that main can rely on us for.
pub fn get_callbacks() {

}
