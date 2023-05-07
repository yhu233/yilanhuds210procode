use crate::distance::bfs;

pub fn count(adjlist: Vec<Vec<usize>>, vertrices: usize, max:usize) -> Vec<(usize,usize)> {
        let mut count = vec![(0,0);max+1];
        //println!("{:?}", count);
        for i in 0..=max{
            let tuple = (i,0);
            count[i] = tuple;
        }
        //println!("{:?}", count);
        for i in 0..vertrices {
            let distance = bfs(adjlist.clone(), vertrices, i);
            for m in 0..vertrices {
                count[distance[m].unwrap()].1 += 1
            }
        }
        //println!("{:?}", count);
        for i in 0..=max {
            if count[i].0 != 0 {
                count[i].1 = count[i].1 / 2;
            }
        }
        //println!("{:?}", count);
        return count
    }