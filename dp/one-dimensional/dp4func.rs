use std::io::Read;
use std::fmt;
use std::iter;

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

// 関数型っぽい書き方
fn main() {
    input! {
        n: usize,
        m: usize,
        d: [usize; m],
    }

    // `arrived` ベクタを生成し、`fold` を使用して各マスに到達可能かどうかのブール値を計算する。
    // `fold` の第一引数は初期値（ここでは全ての要素が `false` で初期化されたベクタ）、
    // 第二引数は各要素に適用するクロージャ
    let arrived = (0..=n).fold(vec![false; n + 1], |mut acc, i| {
        // インデックス0（スタート位置）の場合、必ず到達可能として `true` に設定
        if i == 0 {
            acc[i] = true;
        } else {
            // インデックスが0以外の場合、全てのサイコロの目を評価し、
            // その目を使って現在のインデックスに到達可能かどうかを確認する。
            // `filter` はサイコロの目が現在のインデックスより小さい or 等しい場合に真を返す。
            // `any` は少なくとも一つのサイコロの目で現在のインデックスに到達可能な場合に真を返す
            acc[i] = d.iter()
                    .filter(|&&die| i >= die)  // 使用可能なサイコロの目をフィルタリング
                    .any(|&die| acc[i - die]); // 前の計算結果を使って到達可能かを確認
        }
        // 更新された `acc` を返します。これが次のイテレーションの入力となる。
        acc
    });
    println!("{}", if arrived[n] { "Yes" } else { "No" });
}