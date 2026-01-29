//! # Plan:
//!
//! calculate the shortest path between every pair of numbers. there are 8 numbers,
//! paths are symmetrical, so there are 28 paths.
//! Then, create a higher-level graph where the edges are the distance between nodes.
//! On this path, we can then apply the traveling salesman problem.

use aoc2016::graph::a_star_rev;
use aoc2016::vec2_hamming_dist;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::iter::once;
use std::ops::{Index, IndexMut};
use vecmath::Vector2;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Maze {
    /// wall = true, hallway = false
    cells: Vec<bool>,
    width: u32,
    height: u32,
    /// `0` is the start
    waypoints: Vec<[i32; 2]>,
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let waypoints: HashMap<_, _> = self
            .waypoints
            .iter()
            .copied()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect();
        let width = self.width;
        for y in 0..self.height {
            if y > 0 {
                writeln!(f)?;
            }
            for x in 0..width {
                if self.cells[(y * width + x) as usize] {
                    f.write_char('#')?;
                } else if let Some(&wp) = waypoints.get(&[x as i32, y as i32]) {
                    f.write_char((b'0' + wp as u8) as char)?;
                } else {
                    f.write_char('.')?;
                }
            }
        }
        Ok(())
    }
}

impl Maze {
    fn idx(&self, [x, y]: [i32; 2]) -> usize {
        (y * self.width as i32 + x) as usize
    }
}

impl Index<[i32; 2]> for Maze {
    type Output = bool;

    fn index(&self, index: [i32; 2]) -> &Self::Output {
        &self.cells[self.idx(index)]
    }
}

impl IndexMut<[i32; 2]> for Maze {
    fn index_mut(&mut self, index: [i32; 2]) -> &mut Self::Output {
        let i = self.idx(index);
        &mut self.cells[i]
    }
}

fn from_lines<'i>(lines: impl IntoIterator<Item = &'i str>) -> Maze {
    let mut it = lines.into_iter();
    let r1 = it.next().unwrap();
    let width = r1.len() as u32;
    let mut waypoints = Vec::new();
    let cells: Vec<_> = once(r1)
        .chain(it)
        .flat_map(str::chars)
        .enumerate()
        .map(|(index, c): (usize, char)| match c {
            '#' => true,
            '.' => false,
            '0'..='9' => {
                let i = (c as u8 - b'0') as usize;
                if waypoints.len() <= i {
                    waypoints.resize(i + 1, [-1, -1]);
                }
                waypoints[i] = [(index as u32 % width) as i32, (index as u32 / width) as i32];
                false
            }
            _ => panic!("wronge"),
        })
        .collect();
    let height = cells.len() as u32 / width;

    Maze {
        cells,
        width,
        height,
        waypoints,
    }
}

fn shortest_path(maze: &Maze, from: [i32; 2], to: [i32; 2]) -> Vec<Vector2<i32>> {
    let (path, to) = a_star_rev(
        &from,
        |p| *p == to,
        |&[x, y]| {
            [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]]
                .into_iter()
                .filter(|&[x, y]| {
                    (0..maze.width as i32).contains(&x) && (0..maze.height as i32).contains(&y)
                })
                .filter(|&p| !maze[p])
                .map(|p| (p, ()))
        },
        |&n| vec2_hamming_dist(n, to) as i64,
        |_, _, _| 1,
    )
    .unwrap();
    path.into_iter().map(|(p, _)| p).rev().chain([to]).collect()
}

fn main() {
    let input = include_str!("d24.txt");
    let maze = from_lines(input.lines());
    let p = shortest_path(&maze, maze.waypoints[0], maze.waypoints[1]);
    println!("{p:?}");
}
