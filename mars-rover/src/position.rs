use crate::coordinate::Coordinate;
use crate::direction::Direction;

pub struct Position {
    pub(crate) coordinate: Coordinate,
    pub(crate) direction: Direction,
}

impl Position {
    pub(crate) fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            direction,
            coordinate: Coordinate::new(x, y),
        }
    }

    pub(crate) fn turn_left(&mut self) {
        self.direction = self.direction.rotate_left();
    }

    pub(crate) fn turn_right(&mut self) {
        self.direction = self.direction.rotate_right();
    }

    pub(crate) fn forward(&mut self) {
        self.direction.forward(&mut self.coordinate);
    }
}

// package com.tw.dojo.marsRover;
//
// public class Position {
// private final Coordinate coordinate;
// private Direction direction;
//
// public Position(Integer x, Integer y, Direction direction) {
// this.direction = direction;
// this.coordinate = new Coordinate(x, y);
// }
//
// public void turnLeft() {
// this.direction = this.direction.rotateLeft();
// }
//
// public void turnRight() {
// this.direction = this.direction.rotateRight();
// }
//
// public void move() {
// this.direction.move(coordinate);
// }
//
// @Override
// public String toString() {
// return coordinate.getX() + " " + coordinate.getY() + " " + direction + "\n";
// }
// }
