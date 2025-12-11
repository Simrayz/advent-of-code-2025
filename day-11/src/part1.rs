/*
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
*/

pub fn process(input: &str) -> miette::Result<String> {
    let graph = input.lines().map(|line| {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_string();
        let rest = parts.next().unwrap();
        let children: Vec<String> = rest.split(' ').map(|s| s.to_string()).collect();
        (name, children)
    }).collect::<std::collections::HashMap<String, Vec<String>>>();

    let paths = dfs(
        "you",
        &graph,
        &mut Vec::new(),
        &mut std::collections::HashSet::new(),
    );
    Ok(paths.len().to_string())
}

/* Find all paths from "you" to "out" */
fn dfs(
    node: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    visited: &mut std::collections::HashSet<String>,
) -> Vec<Vec<String>> {
    if visited.contains(node) {
        return vec![];
    }
    visited.insert(node.to_string());
    path.push(node.to_string());

    let mut paths = Vec::new();

    if node == "out" {
        paths.push(path.clone());
    } else if let Some(children) = graph.get(node) {
        for child in children {
            let mut child_paths = dfs(child, graph, path, visited);
            paths.append(&mut child_paths);
        }
    }

    path.pop();
    visited.remove(node);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("5", result?);
        Ok(())
    }
}
