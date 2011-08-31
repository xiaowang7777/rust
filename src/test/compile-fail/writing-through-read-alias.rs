// -*- rust -*-

// error-pattern:assigning to immutable field

type point = {x: int, y: int, z: int};

fn f(p: &point) { p.x = 13; }

fn main() { let x: point = {x: 10, y: 11, z: 12}; f(x); }
