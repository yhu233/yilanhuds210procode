mod file;
mod creation;
mod maximum;
mod distance;
mod distribution;
mod result;
mod genfile;


fn read_file(path: &str) -> Vec<(usize, usize)> {
    let result: Vec<(usize, usize)> = file::transfer(path);
    return result;
}

fn vertrices(file: Vec<(usize, usize)>) -> usize {
    // will calculate the total number of vertrices
    let vertrices:usize = creation::vert(file);
    return vertrices
}

#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<usize>>,
    vert: usize,
}

impl Graph {
    fn adjlist(file: Vec<(usize,usize)>, vertrices: usize)-> Vec<Vec<usize>> { 
        // showing the connection of each nodes, adjacency list
        let graphlist: Vec<Vec<usize>> = creation::adj(file, vertrices);
        return graphlist
    }
    
    fn creategraph(file: Vec<(usize,usize)>, vertrices: usize) -> Graph {
        let adjacency = Self::adjlist(file, vertrices);
        Graph{adj:adjacency, vert:vertrices}
    }

    fn data_maximum(&self) -> usize {
        // returning the maximum distance the dataset has
        let max: usize = maximum::total_maximum(self.adj.clone(), self.vert.clone());
        return max
    }
    
    fn distribution(&self, max:usize) -> Vec<(usize,usize)> {
        let count: Vec<(usize,usize)> = distribution::count(self.adj.clone(), self.vert.clone(), max);
        return count
    }
    
    fn data_ave(&self) -> f32 {
        let ave: f32 = result::data_ave(self.adj.clone(), self.vert.clone());
        return ave
    }  

    fn data_median(&self) -> f32 {
        let median:f32 = result::data_median(self.adj.clone(), self.vert.clone());
        return median
    }

    fn mostoccurdist(&self) -> usize {
        let occurence: usize = result::mostoccur(self.adj.clone(), self.vert.clone());
        return occurence
    }
}

fn main() {
    let file = read_file("facebook_combined.txt");
    let edges = vertrices(file.clone());
    let graph = Graph::creategraph(file.clone(), edges);
    let max = graph.data_maximum();
    println!("the longest degree of distance is {:?}", max);
    let facebook_distri = graph.distribution(max);
    for i in 0..facebook_distri.len() {
        println!("the degree of distance {:?} has the frequency of {:?} times", facebook_distri[i].0, facebook_distri[i].1);
    }
    println!("the distribution of degree is {:?}", facebook_distri);
    let facebook_aver = graph.data_ave();
    println!("the average degree of distance for facebook dataset is {:?}",facebook_aver);
    let facebook_median = graph.data_median();
    println!("the median degree of distance for facebook dataset is {:?}",facebook_median);
    let facebook_occur = graph.mostoccurdist();
    println!("the most occurred degree of distance for facebook dataset is {:?}",facebook_occur);

    println!("-------------------------------------------------");

    //genfile::generate_file("selfgen_data2.txt", 4039, 10); 
    let newfile2 = read_file("selfgen_data2.txt");
    let edges2 = vertrices(newfile2.clone());
    let graph2 = Graph::creategraph(newfile2.clone(), edges2);
    let max2 = graph2.data_maximum();
    println!("the longest degree of distance for the first self generated dataset is {:?}", max2);
    let distri2 = graph2.distribution(max2);
    for i in 0..distri2.len() {
        println!("the degree of distance {:?} has the frequency of {:?} times", distri2[i].0, distri2[i].1);
    }
    println!("the distribution of degree for the first self generated dataset is {:?}", distri2);
    let aver2 = graph2.data_ave();
    println!("the average degree of distance for first self generated dataset is {:?}",aver2);
    let median2 = graph2.data_median();
    println!("the median degree of distance for first self generated dataset is {:?}", median2);
    let occur2 = graph2.mostoccurdist();
    println!("the most occurred degree of distance for first self generated dataset is {:?}", occur2);

    println!("-------------------------------------------------");

    //genfile::generate_file("selfgen_data3.txt", 500, 10); 
    let newfile3 = read_file("selfgen_data3.txt");
    let edges3 = vertrices(newfile3.clone());
    let graph3 = Graph::creategraph(newfile3.clone(), edges3);
    let max3 = graph3.data_maximum();
    println!("the longest degree of distance for the second self generated dataset is {:?}", max3);
    let distri3 = graph3.distribution(max3);
    for i in 0..distri3.len() {
        println!("the degree of distance {:?} has the frequency of {:?} times", distri3[i].0, distri3[i].1);
    }
    println!("the distribution of degree for the second self generated dataset is {:?}", distri3);
    let aver3 = graph3.data_ave();
    println!("the average degree of distance for second self created dataset is {:?}",aver3);
    let median3 = graph3.data_median();
    println!("the median degree of distance for second self generated dataset is {:?}", median3);
    let occur3 = graph3.mostoccurdist();
    println!("the most occurred degree of distance for second self generated dataset is {:?}", occur3);
}

//citation:
//https://en.wikipedia.org/wiki/Six_degrees_of_separation
//https://stackoverflow.com/questions/64398106/move-occurs-because-value-has-type-vect-which-does-not-implement-the-copy-t
//https://www.ics.uci.edu/~goodrich/teach/cs165/notes/NetworkAlgsIntro.pdf
//https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/casting-between-types.html 
//https://doc.rust-lang.org/book/ch05-01-defining-structs.html
//https://doc.rust-lang.org/book/ch05-02-example-structs.html  
//https://doc.rust-lang.org/book/ch05-03-method-syntax.html  
//https://doc.rust-lang.org/book/ch06-03-if-let.html
//https://stackoverflow.com/questions/48071513/how-to-use-one-module-from-another-module-in-a-rust-cargo-project
//https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch03-02-data-types.html  
//lecture note 18, 20, 28
//J. McAuley and J. Leskovec. Learning to Discover Social Circles in Ego Networks. NIPS, 2012.