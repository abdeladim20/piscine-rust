pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_len = source.chars().count();
    let target_len = target.chars().count();
    if source_len == 0 { return target_len; }
    if target_len == 0 { return source_len; }

    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let mut dp = vec![vec![0; target_len + 1]; source_len + 1];

    for i in 1..=source_len {
        dp[i][0] = i;
    }
    for j in 1..=target_len {
        dp[0][j] = j;
    }

    for i in 1..=source_len {
        for j in 1..=target_len {
            let substitution_cost = if source_chars[i - 1] == target_chars[j - 1] { 0 } else { 1 };

            let substitution = dp[i - 1][j - 1] + substitution_cost;
            let deletion = dp[i - 1][j] + 1;
            let insertion = dp[i][j - 1] + 1;

            dp[i][j] = substitution.min(deletion).min(insertion);
        }
    }

    dp[source_len][target_len]
}