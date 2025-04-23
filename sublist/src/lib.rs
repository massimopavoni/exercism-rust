#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

macro_rules! sublist_windows {
    ($fl:ident, $fl_size:ident, $sl:ident, $rel:ident) => {{
        if $sl.windows($fl_size).any(|window| window == $fl) {
            Comparison::$rel
        } else {
            Comparison::Unequal
        }
    }};
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    match (first_list.len(), second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (fl_len, sl_len) if fl_len < sl_len => {
            sublist_windows!(first_list, fl_len, second_list, Sublist)
        }
        (fl_len, sl_len) if fl_len > sl_len => {
            sublist_windows!(second_list, sl_len, first_list, Superlist)
        }
        (_, _) => {
            if first_list == second_list {
                Equal
            } else {
                Unequal
            }
        }
    }
}
