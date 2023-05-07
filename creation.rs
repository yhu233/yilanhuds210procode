pub fn vert(file: Vec<(usize, usize)>) -> usize {
    // will calculate the total number of vertrices
    let length = file.len();
    let mut max_0 = 0;
    let mut max_1 = 0;
    let vertrices: usize;
    for i in 0..length {
        if file[i].0 > max_0 {
            max_0 = file[i].0
        }
    }
    for i in 0..length {
        if file[i].1 > max_1 {
            max_1 = file[i].1
        }
    }
    if max_0 >= max_1 {
        vertrices = max_0 + 1
    }
    else {
        vertrices = max_1 + 1
    }
    return vertrices
}

#[test]
fn test_vertrices() {
    let testfile = [(2,3),(9,12),(69,33),(32,35),(9,8)];
    let result = vert(testfile.to_vec());
    assert_eq!(result, 70, "Vertrices behave weird")
}

pub fn adj(file: Vec<(usize,usize)>, vertrices: usize)-> Vec<Vec<usize>> {
    // will return a collection of vertors 
    // showing the connection of each nodes
    let mut graphlist: Vec<Vec<usize>> = vec![vec![];vertrices];
    for (v,w) in file.iter() {
        graphlist[*v].push(*w);
        graphlist[*w].push(*v);
    }
    return graphlist
}
