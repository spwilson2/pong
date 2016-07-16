use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::i32::MAX as I32MAX;
use std::i32::MIN as I32MIN;

pub type Id = u32;
pub type Collision = (Id, Id);
pub type Line = (i32, i32);

const MASS_DRAG: u32 = 1;
const MASS_MAX: u32  = I32MAX as u32;

// The different amount of velocity applied per difference in mass.
const MASS_DIF_VEL: u32 = 1;
//const I32MIN :i32 = std::i32::MIN;
//const I32MAX :i32 = std::i32::MAX;

struct CollisionLines {
    x_line: Line,
    y_line: Line,
}

struct CollisionBox {
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
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
    x: i32,
    y: i32
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Velocity {
    x: i32,
    y: i32,
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
    width: i32,
    height: i32,
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
        
        self.update_objects_physics();
    }

    fn update_objects_physics(&mut self) {
        for object in self.objects.values_mut() {
            object.update_physics();
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
    pub fn new_wall(width: i32, height: i32, x: i32, y: i32) -> Self{
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
        if (line_1.0 <= line_2.0) && 
           (line_1.1 >= line_2.1) {
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

    fn update_physics(&mut self) {

        // Ignore rigid movement types, they must be moved by function/field access.
        let velocity = match self.movement {
            Movement::Rigid => return,
            Movement::Fluid {velocity: ref mut velocity} => velocity,
        };

        let ref mut x_vel = velocity.x;
        let ref mut y_vel = velocity.y;

        // Skip further calculations if no velocity exists.
        if (*x_vel == 0) && (*y_vel == 0) {return}

        apply_velocity(&mut self.coords.x, *x_vel);
        apply_velocity(&mut self.coords.y, *y_vel);

        let mass = match self.mass {
            Mass::Massful(mass) => mass,
            Mass::Massless =>      0,
        };

        apply_drag(x_vel, mass);
        apply_drag(x_vel, mass);

    }
}

fn apply_drag(mut vel: &mut i32, mass: u32) {

    let vel_sign = match (*vel) > 0 {
        true => 1,
        false => -1,
    };

    assert!(mass <= MASS_MAX);

    // Drag is in the opposite direction of velocity.
    let drag = (mass as i32).checked_mul(MASS_DRAG as i32).unwrap_or(I32MAX);
    
    // Magnitude is unsigned.
    let old_magnitude = (*vel) * vel_sign;

    (*vel) = vel_sign * old_magnitude.checked_sub(drag).unwrap_or(0);

    (*vel) = match (*vel <= 0) {
        true => 0,
        false => *vel
    };
}


fn apply_velocity(mut coord: &mut i32, vel: i32) {
    *coord = match vel >= 0 {
        true => coord.checked_add(vel).unwrap_or(I32MAX),
        false => match coord.checked_add(vel) {
            Some(coord) => coord,
            None        => I32MIN,
        }
    }
}

#[test]
pub fn test_apply_drag_drags_to_zero() {
    let mut i = 2;
    apply_drag(&mut i, MASS_MAX);
    assert_eq!(i, 0);
}

#[test]
pub fn test_apply_velocity_doesnt_overflow_coord() {
    let mut coord = I32MAX;
    apply_velocity(&mut coord, 500);
    assert!(coord >= 0);
}

#[test]
pub fn test_apply_velocity_maxes_out() {
    let mut coord = I32MAX;
    apply_velocity(&mut coord, I32MAX);
    assert_eq!(coord, I32MAX);
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

#[test]
pub fn test_plane_detects_overlapping_walls() {

    let mut plane = Plane::new();
    let mut wall_1 = Object::new_wall(1,1,0,0);
    let mut wall_2 = Object::new_wall(1,1,0,0);

    let id_1 = plane.attach_object(wall_1);
    let id_2 = plane.attach_object(wall_2);

    assert_eq!(plane.detect_collisions().len(), 1);

    // Recieve walls back as a collision, order of detection 
    // doesn't matter.
    let collision_success = match plane.detect_collisions()[0] {
        (col_id_1, col_id_2) if (col_id_1 == id_1) && (col_id_2 == id_2) => true,
        (col_id_2, col_id_1) if (col_id_1 == id_1) && (col_id_2 == id_2) => true,
        _ => false,
    };
    assert!(collision_success);
}
