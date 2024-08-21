use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 0..n {
        let ch = s.chars().nth(i).unwrap();
        match ch {
            'R' => {
                // あいこ
                dp[i + 1][0] = max(dp[i + 1][0], dp[i][1]);
                dp[i + 1][0] = max(dp[i + 1][0], dp[i][2]);
                // 勝ち
                dp[i + 1][1] = max(dp[i + 1][1], dp[i][0] + 1);
                dp[i + 1][1] = max(dp[i + 1][1], dp[i][2] + 1);
            }
            'P' => {
                // あいこ
                dp[i + 1][1] = max(dp[i + 1][1], dp[i][0]);
                dp[i + 1][1] = max(dp[i + 1][1], dp[i][2]);
                // 勝ち
                dp[i + 1][2] = max(dp[i + 1][2], dp[i][0] + 1);
                dp[i + 1][2] = max(dp[i + 1][2], dp[i][1] + 1);
            }
            'S' => {
                // あいこ
                dp[i + 1][0] = max(dp[i + 1][0], dp[i][1] + 1);
                dp[i + 1][0] = max(dp[i + 1][0], dp[i][2] + 1);
                // 勝ち
                dp[i + 1][2] = max(dp[i + 1][2], dp[i][0]);
                dp[i + 1][2] = max(dp[i + 1][2], dp[i][1]);
            }
            _ => unreachable!(),
        }
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
