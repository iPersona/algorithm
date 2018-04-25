use ndarray::Array2;

pub fn get_longest_common_substring<'a>(source: &str, candidates: &Vec<&'a str>) -> Vec<&'a str> {
    let mut len_list = Vec::new();
    for c in candidates.iter() {
        let len = calc_common_substring_len(source, c);
        len_list.push(len);
    }

    let max = *len_list.iter().max().unwrap();
    candidates
        .iter()
        .enumerate()
        .filter(|&(i, _)| len_list[i] == max)
        .map(|(_, c)| *c)
        .collect()
}

fn calc_common_substring_len(source: &str, candidate: &str) -> u32 {
    let mut largest_sublen = 0_u32;
    let source_len = source.chars().count();
    let candidate_len = candidate.chars().count();
    let mut char_table = Array2::<u32>::default((source_len, candidate_len));
    for i in 0..source_len {
        for j in 0..candidate_len {
            if source.chars().nth(i).unwrap() == candidate.chars().nth(j).unwrap() {
                if i == 0 || j == 0 {
                    char_table[[i, j]] = 1;
                } else {
                    char_table[[i, j]] = char_table[[i - 1, j - 1]] + 1;
                }
            } else {
                char_table[[i, j]] = 0;
            }
            if char_table[[i, j]] > largest_sublen {
                largest_sublen = char_table[[i, j]];
            }
        }
    }
    print_char_table(&char_table, candidate, source);

    largest_sublen
}

fn print_char_table(
    char_table: &Array2<u32>,
    candidate: &str,
    source: &str
) {
    let col_width = 2;

    let mut title = format!("{: <1$}\t", "", col_width);
    let mut chars = candidate.chars();
    loop {
        let c = chars.next();
        if c == None {
            break;
        }
        let info = format!("{: <1$}\t", c.unwrap(), col_width);
        title.push_str(&info);
    }
    println!("{}", title);
    chars = source.chars();
    for i in 0..char_table.rows() {
        let mut row = "".to_string();
        for j in 0..char_table.cols() {
            row.push_str(&format!("{: <1$}|\t", char_table[[i, j]], col_width));
        }
        println!("{: <2$}|\t{}", chars.next().unwrap(), row, col_width);
    }
    println!("=====================================");
}
