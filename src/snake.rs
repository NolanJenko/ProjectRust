use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

// Enum to specify direction of movement for the snake
enum Direction{
    up,
    down,
    left,
    right,
}

// A point struct which will store the x and y coordinates of the snake components
struct Point{
    x: f32,
    y: f32,
}

// Snake structure which will store list of points and the direction of the snake
struct Snake{
    head: Point,
    pieces: Vec<Point>,
    size: i32,
    direction: Direction,
}


fn game_loop(){
    let snake = Snake{
        head: Point{x: 0.0, y: 0.0},
        pieces: Vec::<Point>::new(),
        size: 1,
        direction: Direction::right,
    };
    snake.pieces.push(snake.head);

    loop{

        let i = 0;

    }
}


fn main(){
    game_loop();
}