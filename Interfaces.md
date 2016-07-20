Space:
    Plane:
        /// Create an empy Plane with no Objects attached.
        * new()
        /// Attach the given Object to the plane. 
        /// Return the ID assigned to the object.
        * attach_object(&mut self, obj: Object) -> Id

        /// Drop the object with id from the plane, return it if exists.
        * drop_object(&mut self, id: Id) -> Option<Object>

        /// Execute a tick of collisions and movement for the plane.
        * tick()

    struct Object {
        width: i32,
        height: i32,
        coords: Coords,
        movement: Movement,
        mass: Mass,
        is_collidable: bool,
        on_collide: Vector<>, //TODO: Need to be able to run callbacks on collision.
        is_rigid: bool,
    }

GameScript:
    /// Initialize the Plane, Paddles, and ball.
    * Init()

    /// Move the Paddles to their starting positions, move the ball to the scored
    /// player, .
    * private ResetPoint()

    /// Update the game a tick.
    * Tick()

    * MovePlayer()

    /// Return the positions of paddle, players, and the score.
    * GetBoard()

    /// Reset the Board, and the score.
    * ResetGame()

Server:
    /// Initialize the server, add the list of callbacks which can be called by the
    /// client using JSON.
    ///
    /// {command:"CALLBACK COMMAND",
    ///  args: {**kwargs}
    /// }
    /// This should be called by the main function. Main should get the callbacks from 
    /// The Game script.
    * Init(Callbacks: Vector<Map<str, fn>>)

Client:
