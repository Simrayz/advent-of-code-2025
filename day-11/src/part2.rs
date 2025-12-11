use std::collections::HashMap;

pub fn process(input: &str) -> miette::Result<String> {
    let graph = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap().to_string();
            let rest = parts.next().unwrap();
            let children: Vec<String> = rest.split(' ').map(|s| s.to_string()).collect();
            (name, children)
        })
        .collect::<HashMap<String, Vec<String>>>();
    
    // Running the code reveals that there are no paths where "dac" is visited before "fft". 
    // As such, we only compute the paths where "fft" is visited before "dac".
    let sub_paths = vec![
        ("svr", "fft", "dac"),
        ("fft", "dac", "  "),
        ("dac", "out", "fft"),
    ];
    let total = get_total_paths_from_sub_paths(&graph, &sub_paths)?;

    Ok(total.to_string())
}

fn get_total_paths_from_sub_paths(
    graph: &HashMap<String, Vec<String>>,
    sub_paths: &Vec<(&str, &str, &str)>,
) -> miette::Result<usize> {
    let mut total = 1usize;

    for (from, to, avoid) in sub_paths {
        let count = dfs(from, to, avoid, graph, &mut HashMap::<String, usize>::new());
        if count == 0 {
            return Err(miette::miette!("No paths found from {} to {}", from, to));
        }
        total *= count;
    }

    Ok(total)
}

/* Count all paths from "from" to "to" */
fn dfs(
    from: &str,
    to: &str,
    avoid: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if from == avoid {
        return 0;
    }
    if from == to {
        return 1;
    }
    if let Some(&count) = memo.get(from) {
        return count;
    }

    let mut path_count = 0;
    if let Some(outputs) = graph.get(from) {
        for output in outputs {
            path_count += dfs(output, to, avoid, graph, memo);
        }
    }

    memo.insert(from.to_string(), path_count);
    path_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example2.txt"));
        assert_eq!("2", result?);
        Ok(())
    }
}
