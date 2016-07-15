use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub type Id = u32;
pub type Collision = (Id, Id);
pub type Line = (u32, u32);

struct CollisionLines {
    x_line: Line,
    y_line: Line,
}

struct CollisionBox {
    xmin: u32,
    ymin: u32,
    xmax: u32,
    ymax: u32,
}

#[derive(Debug)]
struct Plane {
    objects: HashMap<Id, Box<Object>>,

    // Save time by not indexing the hashmap for 
    // finding and then returning collidables, just
    // look in this "set".
    collide_obj_ids: HashSet<Id>, 
    id_counter: u32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Coords {
    x: u32,
    y: u32
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Velocity {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Movement {
    Fluid {velocity: Velocity},
    Rigid,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mass {
    Massful(u32),
    Massless,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Object {
    width: u32,
    height: u32,
    coords: Coords,
    movement: Movement,
    mass: Mass,
    is_collidable: bool,
    is_rigid: bool,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            objects: HashMap::new(),
            collide_obj_ids: HashSet::new(),
            id_counter: 0,
        }
    }

    pub fn attach_object(&mut self, obj: Object) -> Id {
        self.id_counter += 1;
        let new_obj_id = self.id_counter;

        if obj.is_collidable {
            // The value should not exists in the plane, as Id's are unique
            assert!(self.collide_obj_ids.insert(new_obj_id));
        }

        // The value should not exists in the plane, as Id's are unique
        assert!(self.objects.insert(new_obj_id, Box::new(obj)).is_none());

        new_obj_id
    }

    pub fn drop_object(&mut self, id: Id) -> Option<Object>{

        // Find the object in the hash map.
        let mut drop_obj =  match self.objects.remove(&id) {
            Some(drop_obj) => drop_obj,
            None => return None,
        };

        if drop_obj.is_collidable {
            self.collide_obj_ids.remove(&id);
        }

        Some(*drop_obj)
    }

    pub fn tick(&mut self) {

        let collisions = self.detect_collisions();

        for collision in collisions {
            self.execute_collisions();
        }

    }

    fn detect_collisions(&self) -> Vec<Collision> {
        //TODO: Iterate through the set of collidables detecting if there is a collision, this is
        //an incredibly naive approach. n^2 complexity.
        let mut collision_list = Vec::new();

        // Create a list of all ids so that we can remove those we've already dealt with.
        let mut id_list = VecDeque::with_capacity(self.collide_obj_ids.len());
        for id in self.collide_obj_ids.iter() {
            id_list.push_front(*id)
        }

        // Don't check collision with ourself.
        if id_list.len() > 0 {
            id_list.pop_back();
        }

        for id_1 in self.collide_obj_ids.iter() {
            for id_2 in id_list.iter() {
                // Detect the collision and add to list of collisions.
                if self.objects[id_1].detect_collision(self.objects[id_2].as_ref()) {
                    collision_list.push((*id_1, *id_2));
                }
            }

            // Take the first item out of the checking so we don't retest, or try to detect
            // collision with ourself.
            id_list.pop_back();
        }

        // TODO: Create a 2d rendering of objects and their borders, if we try to write an id on top of
        // another, this is a collision.
        collision_list
    }

    fn execute_collisions(&mut self) {
    }
}

impl Object {
    pub fn new_wall(width: u32, height: u32, x: u32, y: u32) -> Self{
        Object {
            height: height,
            width: width,
            coords: Coords{x:x, y:y},
            movement: Movement::Rigid,
            mass: Mass::Massless,
            is_collidable:  true,
            is_rigid: true,
        }
    }

    fn detect_collision(&self, other: &Self) -> bool {

        let self_bounds = self.bounds();
        let other_bounds = other.bounds();

        if Self::line_collision(self_bounds.x_line, other_bounds.x_line) {
            if Self::line_collision(self_bounds.y_line, other_bounds.y_line) {
                return true
            }
        }

        false
    }

    fn line_collision(line_1: Line, line_2: Line) -> bool {
        if (line_1.0 < line_2.0) && 
           (line_1.1 > line_2.1) {
               return true;
        }
        false
    }

    fn bounds(&self) -> CollisionLines {
        CollisionLines {
            x_line: (self.coords.x, self.coords.x + self.width),
            y_line: (self.coords.y, self.coords.y + self.height),
        }
    }

    fn collide (&mut self, other: &Self) {
        if self.is_collidable && !self.is_rigid{
        }
    }
}

#[cfg(test)]
#[test]
pub fn test_plane_init() {
    Plane::new();
}

#[test]
pub fn test_attach_wall_object_to_plane() {

    let mut plane = Plane::new();

    assert_eq!(plane.attach_object(Object::new_wall(1,1,0,0)), 1);
}

#[test]
pub fn test_drop_object_from_plane() {

    let mut plane = Plane::new();
    let mut wall = Object::new_wall(1,1,0,0);
    let wall_clone = wall.clone();

    let obj_id = plane.attach_object(wall);
    plane.drop_object(obj_id);

    assert_eq!(wall_clone, wall);
}
