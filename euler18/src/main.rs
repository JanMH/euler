use std::fs::File;
use std::io::prelude::*;

fn read_path_cost(path: &str) -> Vec<Vec<u32>> {
    let mut f = File::open(path).unwrap();
    
    let mut contents = String::new();
    f.read_to_string(&mut contents);

    let mut result: Vec<Vec<u32>> = Vec::new();

    for line in contents.lines() {
        let converted_line = line.split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
        result.push(converted_line);

    }
    result
}

fn next_max_path_cost(prev_max_path: &Vec<u32>, current_path_cost: &Vec<u32>) -> Vec<u32> {
    assert!(prev_max_path.len() < current_path_cost.len());
    assert!(prev_max_path.len() > 0 );

    let mut result =vec![prev_max_path[0] + current_path_cost[0]];
    for i in 0..(prev_max_path.len()-1) {
        let prev_local_max = std::cmp::max(prev_max_path[i], prev_max_path[i+1]);
        result.push( prev_local_max + current_path_cost[i + 1])
    }
    result.push(prev_max_path.last().unwrap() + current_path_cost.last().unwrap());
    return result;
}

fn main() {
    let mut path_costs: Vec<Vec<u32>> = read_path_cost("/home/jan/file2.txt");
    let mut max_path_cost: Vec<Vec<u32>> = vec![path_costs.first().unwrap().clone()];
    for path_cost in path_costs[1..].iter() {
        let nmp = next_max_path_cost(max_path_cost.last().unwrap(), &path_cost);
        max_path_cost.push(nmp);
    }
    let res =  max_path_cost.last().and_then(|l| l.iter().max());
    println!("{:?}",res);


    println!("Hello, world!");
}
