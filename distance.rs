use std::collections::VecDeque;

pub fn bfs(adjlist: Vec<Vec<usize>>, vertrices: usize, start:usize) ->Vec<Option<usize>> {
    // will return each node's degree of distance with other nodes
    // lecture 28
    let mut distance: Vec<Option<usize>> = vec![None;vertrices];
    distance[start] = Some(0); 
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { 
        for u in adjlist[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    return distance
}