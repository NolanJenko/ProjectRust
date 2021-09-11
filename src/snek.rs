pub enum Direction {
	Up,
	Down,
	Left, 
	Right,
}

pub enum Command {
	Quit, 
	Turn(Direction),
}

pub struct Point {
	pub x: u16,
	pub y: u16,
}
impl Point {
	pub fn new(x: u16, y: u16) -> Self {
		Self {x, y}
	}
	pub fn transform(&self, direction: Direction, times: u16) -> Self {
		let times = times as i16;
		let transformation = match direction {
			Direction::Up => (0, -times),
			Direction::Down => (0, times),
			Direction::Left => (-times, 0),
			Direction::Right => (times, 0),

		};
		Self::new(
			Self::transform_value(self.x, transformation.0),
			Self::transform_value(self.y, transformation.1),
		)
	}
	fn transform_value(value: u16, by: i16) -> u16 {
		if by.is_negative() && by.abs() as u16 > value {
			panic!("Transforming value {} by {} is a negative number", value. by);
		} else {
			(value as i16 + by) as u16
		}
	}
}
/*
	Body is a Vec over the point type which can be thought of as a list that holds a value of that type Point. Each (x, y) item in the body represents a point in the game's grid that is occupied by a part of the snake. The very first item in the body is always the HEAD of the snake.

	Direction represents the direction which the snake is currently facing using the direction enum that was declared earlier. The snake will always be facing up, down, left or right.

	digesting represents whether or not the snake has just eaten some food. Use this flag to indicate that the snake should grow when it next moves.	
*/
pub struct Snake {
	body: vec<Point>,
	direction: Direction,
	digesting: bool,
}
