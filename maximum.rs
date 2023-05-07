use crate::distance::bfs;

pub fn node_maximum(distance: Vec<Option<usize>>, vertrices:usize) -> usize {
    // will calculate the longest distance that the node has
    let mut max = 0;
    for i in 0..vertrices {
        if distance[i].unwrap() > max {
            max = distance[i].unwrap();
        }
    }
    return max
}

pub fn total_maximum(adjlist: Vec<Vec<usize>>, vertrices: usize ) -> usize {
    // returning the maximum distance the dataset has
    let mut max = 0;
    for i in 0..vertrices {
        let distance = bfs(adjlist.clone(), vertrices, i);
        let length = node_maximum(distance, vertrices);
        if length > max {
            max = length
        }
    }
    return max
}
