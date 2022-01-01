# aigorust
AigoRust is an useful ***solving tool*** for **algorithm problems**.

# Vector<T> and vector![]

Use `vector![]` to make `Vector<T>`. `Vector<T>` is inherited from `Vec<T>`, so it is slower than `Vec<T>`. But it doesn't matter.
```rust
fn main() {
    let a = vector![1, 2, 3]; // new feature : Vector<T> and vector![...]
    let b = a.clone();
    let c : Vector<i32> = a.iter()
                           .zip(b.iter())
                           .map(|(x,y)| x*y)
                           .collect();
    let d : i32 = c.iter().sum();
    println!("{}, {}", &c, d); // [1, 4, 9], 14
```
Index number doesn't need to be `usize`. It's comfortable. Get convenience with **Run-Time Overhead**!
```rust
    let mut a = vector![false;5]; // [false, false, true, false, false]
    a[2i16 /* as usize */] = true; // bye bye ~ "as usize" 
    println!("{}", &a);
```
You make 2D or 3D or ND vector by `vector![]`. Make easy.
```ruct
    let (r, c) = (2i64, 3i64);
    let mut a = vector![-1.5;r;c]; // wow! 2D vector!!
    for i in 0..r {
        for j in 0..c {
            a[i][j] += (i+j) as f64;
        }
    }
    println!("{}", &a); // [[-1.5, -0.5, 0.5], [-0.5, 0.5, 1.5]] // wow printable 2D vector!!
```
It's so convenient to write code that reads the input!
```ruct
    let l = readln(); // read one line excluding trailing "\n"
    let x = readi(); // read one line and return i64
    let y = reada::<10>(); // read one line and return [i64; 10] ( not better than below readv() )
    let [a, b] = reada(); // read one line and return two variables!!
    let v = readv(); // read one line and return Vector<i64>
}
```
I recommend `reada()`. It reads very well regardless of the number of variables.
```rust
fn main() {
    let [a] = reada();
    let [a, b] = reada();
    let [a, b, c] = reada();
    let [a, b, c, d] = reada();
    let [a, b, c, d, e] = reada();
}
```
Features : `Vector<T>, vector![], readln(), readi(), reada(), readv()`

I show an example to solve a problem from Cordeforces. ( https://codeforces.com/contest/1591/problem/C )
```rust
// .. skip above implementation of AigoRust

use std::cmp::max;
fn main() {
    let t = readi();
    for T in 1..=t {
        let [n, k] = reada();
        let mut x = readv();
        let mut ans = 0i64;
        let mut pos : Vector<i64> = x.iter().filter(|&&x| x > 0).map(|&x|  x).collect();
        let mut neg : Vector<i64> = x.iter().filter(|&&x| x < 0).map(|&x| -x).collect();
        pos.sort(); pos.reverse();
        neg.sort(); neg.reverse();
        ans += [&pos, &neg]
            .iter()
            .map(|v| 2*v.iter().step_by(k as usize).sum::<i64>())
            .sum::<i64>();
        ans -= max(pos.first().unwrap_or(&0), neg.first().unwrap_or(&0));
        println!("{}", ans);
    }
}
```
It is not better than cpp, but **hipper** than cpp.
