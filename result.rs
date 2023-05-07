use crate::maximum::total_maximum;
use crate::distribution::count;

pub fn average(degreelist:Vec<(usize,usize)>, max: usize) -> f32 {
    let mut total = 0;
    let mut number = 0;
    for i in 0..=max {
        number += degreelist[i].1;
        total += degreelist[i].0 * degreelist[i].1; 
    }
    let total_fl = total as f32;
    let number_fl = number as f32;
    let outcome = total_fl / number_fl;
    return outcome
}

#[test]
pub fn test_average() {
    let degreelist = [(0,1),(1,2),(2,3),(3,4),(4,0)];
    let max = 4;
    let test_a = average(degreelist.to_vec(), max);
    assert_eq!(test_a, 2.0, "Not correct")
}

pub fn data_ave(adjlist: Vec<Vec<usize>>, vertrices: usize) -> f32 {
    let max = total_maximum(adjlist.clone(), vertrices);
    let degreelist = count(adjlist.clone(), vertrices, max);
    let ave = average(degreelist, max);
    return ave
} 

pub fn median(degreelist:Vec<(usize,usize)>) -> f32 {
    let mut total = 0;
    let median_num1: usize;
    let median_num2: usize;
    let mut median1: usize = 0;
    let mut median2: usize = 0;
    let mut median: f32 = 0.0;
    let mut distance: Vec<usize> = vec![0;degreelist.len()];
    distance[0] = degreelist[0].1;
    
    for i in 0..degreelist.len() {
        total += degreelist[i].1
    }

    for i in 1..degreelist.len() {
        distance[i] = degreelist[i].1 + distance[i-1]
    }

    if total % 2 == 1 {
        median_num1 = (total + 1) / 2;
        for i in 0..degreelist.len() {
            if median_num1 < distance[i] && median_num1 > distance[i-1] {
                median = degreelist[i].1 as f32
            }
        }
    }
    else {
        median_num1 = total / 2;
        median_num2 = (total / 2) + 1;
        for i in 0..degreelist.len() {
            if median_num1 <= distance[i] && median_num1 > distance[i-1] {
                median1 = degreelist[i].0
            }
        }
        for i in 0..degreelist.len() {
            if median_num2 <= distance[i] && median_num2 > distance[i-1] {
                median2 = degreelist[i].0
            }
        }
        let median1f = median1 as f32;
        let median2f = median2 as f32;
        median = (median1f + median2f)/2.0;
    }
    return median
}

#[test]
pub fn test_median() {
    let degreelist = [(0,9),(1,1),(2,6),(3,2),(4,2)];
    let testb = median(degreelist.to_vec());
    assert_eq!(testb, 1.5, "Not correct")
}

pub fn data_median(adjlist: Vec<Vec<usize>>, vertrices: usize) -> f32 {
    let max = total_maximum(adjlist.clone(),vertrices);
    let degreelist = count(adjlist.clone(), vertrices, max);
    let data_med = median(degreelist);
    return data_med
}

pub fn mostoccur(adjlist: Vec<Vec<usize>>, vertrices: usize) -> usize {
    let max = total_maximum(adjlist.clone(),vertrices);
    let degreelist = count(adjlist.clone(), vertrices, max);
    let mut most:usize = 0;
    let mut mostdist: usize = 0;
    for i in 0..max {
        if most < degreelist[i].1 {
            most = degreelist[i].1;
            mostdist = degreelist[i].0;
        }
    }
    return mostdist
}
    