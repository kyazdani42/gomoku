static SIZE: usize = 19*19;

pub fn right(index: usize) -> usize {
	return index + 1;
}

pub fn left(index: usize) -> usize {
	return index - 1;
}

pub fn top(index: usize) -> usize {
	return index - SIZE;
}

pub fn bottom(index: usize) -> usize {
	return index + SIZE;
}

pub fn top_left(index: usize) -> usize {
	return top(left(index));
}

pub fn top_right(index: usize) -> usize {
	return top(right(index));
}

pub fn bot_left(index: usize) -> usize {
	return bot(left(index));
}

pub fn bot_right(index: usize) -> usize {
	return bot(right(index));
}
