pub struct Coordinate {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coordinate {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub(crate) fn up(&mut self) {
        self.y += 1;
    }

    pub(crate) fn down(&mut self) {
        self.y -= 1;
    }

    pub(crate) fn left(&mut self) {
        self.x -= 1;
    }

    pub(crate) fn right(&mut self) {
        self.x += 1;
    }
}

//
// public class Coordinate {
// private Integer x;
// private Integer y;
//
// public Coordinate(Integer x, Integer y) {
// this.x = x;
// this.y = y;
// }
//
// public void up() {
// this.y += 1;
// }
//
// public void down() {
// this.y -= 1;
// }
//
// public void left() {
// this.x -= 1;
// }
//
// public void right() {
// this.x += 1;
// }
//
// public Integer getX() {
// return x;
// }
//
// public Integer getY() {
// return y;
// }
// }
