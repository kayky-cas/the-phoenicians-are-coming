use std::{collections::VecDeque, str::FromStr};

use anyhow::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos(i32, i32);

impl Pos {
    fn to_index(&self, map_size: (usize, usize)) -> usize {
        self.1 as usize * map_size.0 + self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_pos(&self, pos: &Pos) -> Pos {
        match self {
            Direction::North => Pos(pos.0, pos.1 + 1),
            Direction::South => Pos(pos.0, pos.1 - 1),
            Direction::East => Pos(pos.0 + 1, pos.1),
            Direction::West => Pos(pos.0 - 1, pos.1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WorldMapNode {
    Water,
    Land,
    Port(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PosWithDistance(Pos, usize);

impl PartialOrd for PosWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PosWithDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

pub struct PhoenicianTrader {
    current_port: Pos,
    first_port: Pos,
    left_ports: Vec<Pos>,
    world_map: Vec<WorldMapNode>,
    map_size: (usize, usize),
    fuel_cost: usize,
}

impl Iterator for PhoenicianTrader {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut visited = vec![None; self.map_size.0 * self.map_size.1];
        let mut queue = VecDeque::new();

        let mut ports = Vec::new();

        let current_port_id = match self.world_map[self.current_port.to_index(self.map_size)] {
            WorldMapNode::Port(port_id) => port_id,
            _ => unreachable!("Should be a port"),
        };

        queue.push_front(PosWithDistance(self.current_port, 0));
        visited[self.current_port.to_index(self.map_size)] = Some(0);

        let mut left_ports = self.left_ports.clone();

        while let Some(PosWithDistance(node, distance)) = queue.pop_back() {
            if let WorldMapNode::Port(port) = self.world_map[node.to_index(self.map_size)] {
                if port > current_port_id {
                    ports.push((node, distance));

                    if self.current_port == self.first_port {
                        self.left_ports.push(node);
                    } else {
                        left_ports.retain(|&port| port != node);
                    }
                }
            }

            if left_ports.is_empty() && self.current_port != self.first_port {
                break;
            }

            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let next_node = direction.to_pos(&node);

                let Pos(x, y) = next_node;

                if x < 0 || y < 0 || x >= self.map_size.0 as i32 || y >= self.map_size.1 as i32 {
                    continue;
                }

                if let WorldMapNode::Land = self.world_map[next_node.to_index(self.map_size)] {
                    continue;
                }

                if visited[next_node.to_index(self.map_size)].is_none() {
                    queue.push_front(PosWithDistance(next_node, distance + 1));
                    visited[next_node.to_index(self.map_size)] = Some(distance + 1);
                }
            }
        }

        if self.first_port == self.current_port {
            let distance = match ports.iter().max_by_key(|(port, _)| {
                match self.world_map[port.to_index(self.map_size)] {
                    WorldMapNode::Port(current_port) => current_port,
                    _ => unreachable!("Should be a port"),
                }
            }) {
                Some((_, distance)) => *distance,
                None => return None,
            };

            self.fuel_cost += distance;
        }

        let (port, distance) = match ports.iter().min_by_key(|(port, _)| {
            match self.world_map[port.to_index(self.map_size)] {
                WorldMapNode::Port(current_port) => current_port,
                _ => unreachable!("Should be a port"),
            }
        }) {
            Some((port, distance)) => (*port, *distance),
            None => return None,
        };

        self.left_ports.retain(|p| *p != port);

        self.fuel_cost += distance;
        self.current_port = port;

        Some(self.fuel_cost)
    }
}

impl FromStr for PhoenicianTrader {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut world_map: Vec<WorldMapNode> = Vec::new();
        let mut ports = Vec::new();

        let mut lines = s.lines();

        let map_size: (usize, usize) = lines
            .next()
            .unwrap()
            .trim()
            .split_once(' ')
            .map(|(x, y)| (y.parse().unwrap(), x.parse().unwrap()))
            .unwrap();

        for (y, line) in lines.enumerate() {
            for (x, ch) in line.trim().chars().enumerate() {
                let pos = Pos(x as i32, y as i32);

                match ch {
                    '.' => {
                        world_map.push(WorldMapNode::Water);
                    }
                    '*' => {
                        world_map.push(WorldMapNode::Land);
                    }
                    '0'..='9' => {
                        world_map.push(WorldMapNode::Port(ch.to_digit(10).unwrap() as usize));
                        ports.push((pos, ch.to_digit(10).unwrap() as usize));
                    }
                    _ => unreachable!("Invalid character"),
                }
            }
        }

        let current_port = ports.iter().min_by_key(|(_, port)| *port).unwrap().0;

        Ok(Self {
            first_port: current_port,
            current_port,
            left_ports: Vec::new(),
            world_map,
            fuel_cost: 0,
            map_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_go_to_next_port() {
        const INPUT: &str = r"
            80 15
            .......................****************.........................................
            ...************..........************...........................................
            ....***************........*******.......*********..................1******.....
            ....***************.........**........***************..............*********....
            .....************.....................***************..............*********....
            ........*********.....................***************..............*********....
            ............*****......................**************...............*****.......
            .........................................************...........................
            .........................................************...........................
            **.............................**,.......************...........................
            *****.........................****.......***********............................
            ********.......................***.........********.............................
            ***************2................*,..........****................................
            *******************.............................................................
            ********************............................................................";

        let mut phoenicians: PhoenicianTrader = INPUT.parse().unwrap();

        phoenicians.next();

        assert_eq!(phoenicians.current_port, Pos(1, 14));
        assert_eq!(phoenicians.fuel_cost, 126);
    }

    #[test]
    fn test_input() {
        let input = fs::read_to_string("./cases/caso20.txt").unwrap();

        let mut phoenicians: PhoenicianTrader = input.parse().unwrap();

        while let Some(fuel_cost) = phoenicians.next() {
            println!("{}", fuel_cost);
        }

        dbg!(phoenicians.fuel_cost);
    }
}
