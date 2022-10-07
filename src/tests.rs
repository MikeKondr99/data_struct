#[cfg(test)]
use crate::list::List;
//
#[test]
fn is_empty() {
    let list = List::<i32>::new();
    assert!(list.is_empty())
}

#[test]
fn empty_peek() {
    let list = List::<i32>::new();
    assert!(list.peek().is_none())
}

#[test]
fn peek() {
    let mut list = List::new();
    list.push(1);
    assert!(*list.peek().unwrap() == 1);
    list.push(2);
    assert!(*list.peek().unwrap() == 2);
}

#[test]
fn pop() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert!(list.pop().unwrap() == 3);
    assert!(list.pop().unwrap() == 2);
    assert!(list.pop().unwrap() == 1);
}
