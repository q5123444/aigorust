// Vector<T>
#[derive(Clone)]
struct Vector<T>(Vec<T>);
impl<T> Vector<T> {
    fn new() -> Self {
        Vector(Vec::<T>::new())
    }
}
// use std::ops::{Deref, DerefMut, Index, IndexMut};
impl<T> std::ops::Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
trait CastToUsize { fn cast_to_usize(self) -> usize; }
macro_rules! impl_cast_to_usize {
    ($($t : ty)+) => {
        $(impl CastToUsize for $t {
            fn cast_to_usize(self) -> usize {
                self as usize
            }
        })+
    }
}
impl_cast_to_usize!(i8 i16 i32 i64 i128 u8 u16 u32 u64 u128);

impl<T, I : CastToUsize> std::ops::Index<I> for Vector<T> {
    type Output = T;
    fn index(&self, idx: I) -> &Self::Output {
        &self.0[idx.cast_to_usize()]
    }
}
impl<T, I : CastToUsize> std::ops::IndexMut<I> for Vector<T> {
    fn index_mut(&mut self, idx: I) -> &mut Self::Output {
        &mut self.0[idx.cast_to_usize()]
    }
}
impl<T> std::fmt::Display for Vector<T> where T : std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.iter();
        write!(f, "{}", iter.next().unwrap())?;
        for i in iter {
            write!(f, ", {}", i)?;
        }
        write!(f, "]")
    }
}
impl<T> std::iter::FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut c = Self::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}
macro_rules! vector {
    () => { Vector(vec![])};
    ($($elem : expr),*) => { Vector(vec![ $($elem ,)* ]) };
    ($elem : expr ; $n : expr) => { Vector(vec!($elem;$n as usize)) };
    ($elem : expr ; $n : expr $(; $ns : expr)+) => { {
        let mut c = Vector(Vec::with_capacity($n as usize));
        for i in 0..$n {
            c.push(vector![$elem $(; $ns)+]);
        }
        c
    } };
}
// read
fn readln() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}
fn readi() -> i64 { readln().parse().unwrap() }
fn reada<const N: usize>() -> [i64; N] {
    let mut c : [i64; N] = [0; N];
    let ln = readln();
    let mut iter = ln.split(" ").take(N).map(|x| x.parse::<i64>().unwrap());
    for i in 0..N {
        c[i] = iter.next().unwrap();
    }
    c
}
fn readv() -> Vector<i64> { readln().split(" ").map(|x| x.parse::<i64>().unwrap()).collect() }
