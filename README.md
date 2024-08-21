# Input
アルゴ式では、Proconioマクロが使えないため、以下の記事から拝借したマクロを使用することで同じような使用感で問題を解く
https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
```rust
use std::io::Read;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        s: [String; n]
    }

    println!("{} {} {}", n, x, y);
    println!("{:?}", s);
}
```
Rust公式 println!
https://doc.rust-lang.org/std/fmt/index.html

※Atcoderでは proconioマクロが使用できるため、
```rust
use proconio::input;
```
して入力受け取りを行う。

# 配列とベクタ

配列の初期化（可変長 vec!を使用）
```rust
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut numbers = vec![0; n];
```
※配列を[x; N]で初期化する場合、要素数Nはコンパイル時に定まっている必要がある。
そのため、下記のように標準入力から受け取った値を用いて、その要素数の配列を生成することはできない。
```rust
    input! {
        n: usize,
    }

    let mut numbers : [0; n];
```
配列を使用した場合は、以下のように書く
```rust
fn main() {
    input! {
        n: usize,
    }
    // 10000は、問題に合わせて任意の数を設定する
    let mut numbers: [usize; 10000] = [0; 10000];
}

```

型
型|説明
-|-
i8|8ビット符号あり整数（-127 ~ 127）
i16|16ビット符号あり整数（−32768 ~ 32767）
i32|32ビット符号あり整数（-2147483648 ~ 2147483647）
i64|64ビット符号あり整数（-9223372036854775808 ~ 9223372036854775807）
i128|128ビット符号あり整数（-(2**127) ~ (2**127)−1）
isize|CPUアーキテクチャのビットサイズ
-|-
u8|8ビット符号なし整数（~ 255）
u16|16ビット符号なし整数（~ 65535）
u32|32ビット符号なし整数（~ 4294967295）
u64|64ビット符号なし整数（~ 18446744073709551615）
u128|128ビット符号なし整数（~ (2**128)−1）
usize|CPUアーキテクチャのビットサイズ
