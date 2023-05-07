use rand::Rng;
use std::fs::File;
use std::io::Write;

pub fn generate_file(path: &str, n: usize, neighbornum: usize) {
    // Generate a random file of edges for vertices 0.n
    let mut file = File::create(path).expect("Unable to create file");
    for i in 0..n {
        // How many neighbors will this node have
        let rng = rand::thread_rng().gen_range(1..neighbornum) as usize;
        for _j in 0..rng {
            // Randomly select a neighbor (even with duplicates but not to ourselves)
            let neighbor = rand::thread_rng().gen_range(1..n) as usize;
            if neighbor != i {
                let s: String = format!("{} {}\n", i, neighbor);
                file.write_all(s.as_bytes()).expect("Unable to write file");
            }
        }
    }
}