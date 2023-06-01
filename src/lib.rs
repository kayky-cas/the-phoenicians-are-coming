use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::Result;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pos(i32, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

enum WorldMapNode {
    Water,
    Land,
    Port(usize),
}

pub struct PhoenicianTrader {
    current_port: Pos,
    world_map: HashMap<Pos, WorldMapNode>,
    fuel_cost: usize,
    map_size: (usize, usize),
}

impl Iterator for PhoenicianTrader {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut visited: HashMap<Pos, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        let mut ports = Vec::new();

        let current_port_id = match self.world_map.get(&self.current_port) {
            Some(WorldMapNode::Port(port_id)) => *port_id,
            _ => unreachable!("Should be a port"),
        };

        queue.push_back((self.current_port, 0));

        while let Some((node, distance)) = queue.pop_front() {
            if visited.get(&node).is_some() {
                continue;
            }

            visited.insert(node, distance);

            if let WorldMapNode::Port(port) = self.world_map.get(&node).unwrap() {
                if *port > current_port_id {
                    ports.push((node, distance));
                }
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

                if let Some(WorldMapNode::Land) = self.world_map.get(&next_node) {
                    continue;
                }

                if visited.get(&next_node).is_none() {
                    queue.push_back((next_node, distance + 1));
                }
            }
        }

        let (port, distance) =
            match ports
                .iter()
                .min_by_key(|(port, _)| match self.world_map.get(&port) {
                    Some(WorldMapNode::Port(current_port)) => current_port,
                    _ => unreachable!("Should be a port"),
                }) {
                Some((port, distance)) => (*port, *distance),
                None => return None,
            };

        self.fuel_cost += distance;
        self.current_port = port;

        Some(self.fuel_cost)
    }
}

impl FromStr for PhoenicianTrader {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut map_size = (0, 0);
        let mut world_map: HashMap<Pos, WorldMapNode> = HashMap::new();

        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, ch) in line.trim().chars().enumerate() {
                let pos = Pos(x as i32, y as i32);

                if x > map_size.0 {
                    map_size.0 = x;
                }
                if y > map_size.1 {
                    map_size.1 = y;
                }

                match ch {
                    '.' => {
                        world_map.insert(pos, WorldMapNode::Water);
                    }
                    '*' => {
                        world_map.insert(pos, WorldMapNode::Land);
                    }
                    '0'..='9' => {
                        world_map
                            .insert(pos, WorldMapNode::Port(ch.to_digit(10).unwrap() as usize));
                    }
                    _ => unreachable!("Invalid character"),
                }
            }
        }

        let current_port = world_map
            .iter()
            .find_map(|(pos, node)| match node {
                WorldMapNode::Port(1) => Some(*pos),
                _ => None,
            })
            .unwrap();

        Ok(Self {
            current_port,
            world_map,
            fuel_cost: 0,
            map_size: (map_size.0 + 1, map_size.1 + 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_go_to_next_port() {
        const INPUT: &str = r"80 15
            ............****..............**......................................**....*1..
            ..........*******.............****............*...........********....****......
            ..........*******.............****..........****..........********....****......
            ..........*******.............****.........******..........********....****.....
            ..........*******.............****.........******..........********....****.....
            ..........*******.............****.........******..........********....****.....
            ..........*******.............****........********.......**********....****.....
            ..........*******.............****.......*********.......**********....****.....
            ..........*******.............****.......*********.......**********....****.....
            ..........*******.............****.......*********.......**********....****.....
            ..............................****.......*********.......**********....****.....
            ..........*******.............****.......*********.......**********....****.....
            ..........*******.............****.......*********.......**********....****.....
            ..........*******.............****.......*********.......**********....****.....
            .2........*******........................*********.......**********.............";

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
