struct Plane {
    objects: Vec<Object> //TODO: Probably will want to use a map.
}

struct Coords {
    x: u32,
    y: u32
}

struct Object {
    coords: Coords,
    len: u32,
    width: u32,
}
