const SIZE: usize = 19 * 19;

pub fn right(index: usize) -> usize {
    index + 1
}

pub fn left(index: usize) -> usize {
    index - 1
}

pub fn top(index: usize) -> usize {
    index - SIZE
}

pub fn bottom(index: usize) -> usize {
    index + SIZE
}

pub fn top_left(index: usize) -> usize {
    top(left(index))
}

pub fn top_right(index: usize) -> usize {
    top(right(index))
}

pub fn bot_left(index: usize) -> usize {
    bot(left(index))
}

pub fn bot_right(index: usize) -> usize {
    bot(right(index))
}
