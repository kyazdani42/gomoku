const SIZE: i32 = 19;

pub fn right(index: i32) -> i32 {
	return index + 1;
}

pub fn left(index: i32) -> i32 {
	return index - 1;
}

pub fn top(index: i32) -> i32 {
	return index - SIZE;
}

pub fn bot(index: i32) -> i32 {
	return index + SIZE;
}

pub fn top_left(index: i32) -> i32 {
	return index - SIZE - 1;
}

pub fn top_right(index: i32) -> i32 {
	return index - SIZE + 1;
}

pub fn bot_left(index: i32) -> i32 {
	return index + SIZE - 1;
}

pub fn bot_right(index: i32) -> i32 {
	return index + SIZE + 1;
}
