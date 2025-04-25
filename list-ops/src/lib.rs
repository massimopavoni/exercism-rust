use std::iter::from_fn;

pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    from_fn(move || a.next().or_else(|| b.next()))
}

pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut current_iter: Option<I::Item> = None;

    from_fn(move || {
        loop {
            if let Some(inner_next) = current_iter.as_mut().and_then(|iter| iter.next()) {
                return Some(inner_next);
            }

            if let Some(inner_iter) = nested_iter.next() {
                current_iter = Some(inner_iter);
            } else {
                break;
            }
        }

        None
    })
}

pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    from_fn(move || iter.find(|next| predicate(next)))
}

pub fn length<I: Iterator>(iter: I) -> usize {
    foldl(iter, 0, |acc, _| acc + 1)
}

pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    from_fn(move || iter.next().map(&function))
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut accumulator = initial;

    for next in iter {
        accumulator = function(accumulator, next);
    }

    accumulator
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    foldl(reverse(iter), initial, function)
}

pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    from_fn(move || iter.next_back())
}
