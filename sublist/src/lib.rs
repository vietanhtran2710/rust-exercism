#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if _first_list == _second_list {
            Comparison::Equal
        }
        else {
            Comparison::Unequal
        }
    }
    else {
        if _first_list.len() < _second_list.len() {
            for i in 0.._second_list.len() - _first_list.len() + 1 {
                if &_second_list[i..i + _first_list.len()] == _first_list {
                    return Comparison::Sublist
                }
            }
        }
        else {
            for i in 0.._first_list.len() - _second_list.len() + 1 {
                if &_second_list[i..i + _second_list.len()] == _second_list {
                    return Comparison::Superlist
                }
            }
        }
        Comparison::Unequal
    }
}
