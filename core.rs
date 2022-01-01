fn readln() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}
fn readi() -> i64 { readln().parse().unwrap() }
fn readv() -> Vec<i64> { readln().split(" ").map(|x| x.parse::<i64>().unwrap()).collect() }

struct Vector<T> {
    vec : Vec<T>
}
use std::ops::{Deref, DerefMut, Index, IndexMut};
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
impl<T, I : Copy + ?Sized> Index<I> for Vector<T> {
    type Output = T;
    fn index(&self, idx: I) -> &Self::Output {
        &self.vec[unsafe {
            let index : usize = 0;
            *(&index as *const _ as *mut I) = idx;
            index
        }] // Hurray!
    }
}
macro_rules! vector {
    [$($elem:expr),*] => { Vector{ vec: vec![ $($elem ,)* ] } }
}
