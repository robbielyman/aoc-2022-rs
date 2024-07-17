use std::{collections::HashMap, hash::DefaultHasher, num::Saturating};

use itertools::Itertools;

struct Node {
    children: Vec<usize>,
    flow_rate: u32,
}

fn distances(graph: &Graph) -> Vec<Vec<usize>> {
    // for i in 0..graph.len() {
        // let inner = vec![None; graph.len()];
        // for j in graph[i].children.iter() {
            // inner[*j] = Some(1);
        // }
    // }
    let mut dists = vec![vec![None; graph.len()]; graph.len()];
    for (i, node) in graph.iter().enumerate() {
        for j in node.children.iter() {
            if *j == 5 {
                println!("i, j: {}, {}", i, j);
            }
            dists[i][*j] = Some(1);
        }
    }
    for i in 0..graph.len() {
        dists[i][i] = Some(0);
    }
    println!("{:?}", dists);
    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                match dists[i][j] {
                    None => match dists[i][k] {
                        None => {}
                        Some(d) => match dists[k][j] {
                            None => {}
                            Some(e) => dists[i][j] = Some(d + e),
                        },
                    },
                    Some(f) => match dists[i][k] {
                        None => {}
                        Some(d) => match dists[k][j] {
                            None => {}
                            Some(e) => dists[i][j] = Some(f.min(d + e)),
                        },
                    },
                }
            }
        }
    }
    // println!("{:?}", dists);
    dists
        .into_iter()
        .map(|opt| opt.into_iter().map(|o| o.unwrap()).collect())
        .collect()
}

type Graph = Vec<Node>;

fn parse(input: &str) -> (Vec<Node>, usize) {
    let mut counter: usize = 0;
    let mut hash_map = HashMap::new();
    let graph: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let relevant: Vec<_> = line
                .split_whitespace()
                .enumerate()
                .filter(|(n, _)| *n == 1 || *n == 4 || *n >= 9)
                .map(|(_, token)| token)
                .collect();
            println!("{:?}", relevant);
            let name = relevant[0];
            if !hash_map.contains_key(name) {
                hash_map.insert(name, counter);
                counter += 1;
            }
            let flow_rate = relevant[1]
                .trim_start_matches("rate=")
                .trim_end_matches(";")
                .parse()
                .ok()?;
            let children: Vec<_> = relevant[2..]
                .iter()
                .map(|token| {
                    let trimmed = token.trim_end_matches(",");
                    if !hash_map.contains_key(trimmed) {
                        hash_map.insert(trimmed, counter);
                        counter += 1;
                    }
                    *hash_map.get(trimmed).unwrap()
                })
                .collect();
            println!("name: {}, children: {:?}", name, children);
            Some(Node {
                children,
                flow_rate,
            })
        })
        .collect();
    // println!("{:?}", hash_map);
    (graph, *hash_map.get("AA").unwrap())
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    flipped: Vec<bool>,
    location: usize,
}

impl State {
    fn next(&self, score: u32, remaining: u32, graph: &Vec<Node>) -> Vec<(State, u32)> {
        let mut ret: Vec<_> = graph[self.location]
            .children
            .clone()
            .into_iter()
            .map(|location| {
                (
                    State {
                        flipped: self.flipped.clone(),
                        location,
                    },
                    score,
                )
            })
            .collect();
        if !self.flipped[self.location] && graph[self.location].flow_rate > 0 {
            let mut new_flipped = self.flipped.clone();
            new_flipped[self.location] = true;
            let new_score = score + graph[self.location].flow_rate * remaining;
            ret.push((
                State {
                    flipped: new_flipped,
                    location: self.location,
                },
                new_score,
            ));
        }
        ret
    }
}

fn score(start_loc: usize, chain: &Vec<&(usize, u32)>, distances: &Vec<Vec<usize>>) -> usize {
    let mut a = start_loc;
    let mut time_remaining = 30;
    let mut score = 0;
    for (idx, flow_rate) in chain.iter() {
        let dist = distances[a][*idx];
        a = *idx;
        if dist + 1 > time_remaining {
            return score;
        }
        time_remaining -= (dist + 1);
        // println!("time remaining: {}, flow_rate: {}", time_remaining, *flow_rate);
        score += (time_remaining * (*flow_rate as usize));
    }
    score
    // chain.iter()
    //     .fold(0, |score, (b, flow_rate)| {
    //         let distance = distances[a][*b];
    //         a = *b;
    //         if distance + 1 > time_remaining {
    //             time_remaining = 0;
    //             score
    //         } else {
    //             time_remaining -= (distance + 1);
    //             println!("time remaining: {}, flow_rate: {}", time_remaining, *flow_rate);
    //             score + (time_remaining * (*flow_rate as usize))
    //         }
    //     })
}

fn solve(graph: &Graph, start_loc: usize) -> usize {
    let distances = distances(graph);
    let vec: Vec<_> = graph
        .iter()
        .enumerate()
        .filter_map(|(i, node)| {
            if node.flow_rate == 0 {
                None
            } else {
                Some((i, node.flow_rate))
            }
        })
        .collect();
    vec.iter()
        .permutations(vec.len())
        .inspect(|chain| {
            println!(
                "{:?} with score {}",
                chain,
                score(start_loc, chain, &distances)
            )
        })
        .map(|chain| score(start_loc, &chain, &distances))
        .max()
        .unwrap()
}

// fn solve(graph: &Vec<Node>, start_loc: usize) -> u32 {
//     let mut paths = vec![(
//         State {
//             flipped: vec![false; graph.len()],
//             location: start_loc,
//         },
//         0,
//     )];
//     for i in (0..30).rev() {
//         paths = paths
//             .into_iter()
//             .flat_map(|(state, score)| state.next(score, i, graph))
//             .fold(Vec::<(State, u32)>::new(), |ret, (state, score)| {
//                 let mut found = false;
//                 let mut new_ret: Vec<_> = ret.into_iter()
//                     .map(|(other_state, other_score)| {
//                         if other_state == state {
//                             found = true;
//                             (other_state, other_score.max(score))
//                         } else {
//                             (other_state, other_score)
//                         }
//                     })
//                     .collect();
//                 if !found { new_ret.push((state, score)); }
//                 new_ret
//             });
//     }
//     paths.iter()
//         .map(|(_, score)| *score)
//         .max().unwrap()
// }

#[cfg(test)]
mod tests {
    use crate::{parse, solve};

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn big_graph() {
        let (graph, start) = parse(INPUT);
        let score = solve(&graph, start);
        assert_eq!(score, 1651);
    }
}
