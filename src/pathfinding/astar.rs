use crate::world::World;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f32,
    heuristic: f32,
    pos: (i32, i32),
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_total = self.cost + self.heuristic;
        let other_total = other.cost + other.heuristic;
        // Inverse order for Min-Heap
        other_total
            .partial_cmp(&self_total)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn octile_distance(from: (i32, i32), to: (i32, i32)) -> f32 {
    let dx = (from.0 - to.0).abs() as f32;
    let dy = (from.1 - to.1).abs() as f32;
    let min = dx.min(dy);
    let max = dx.max(dy);
    (max - min) + 1.41421356 * min
}

/// A* pathfinding algorithm along the 3D surface relief of the world.
/// Returns the ordered sequence of coordinates from `start` to `goal`.
pub fn find_path(
    world: &World,
    start: (i32, i32),
    goal: (i32, i32),
) -> Option<Vec<(i32, i32)>> {
    if start == goal {
        return Some(vec![start]);
    }

    let goal_z = world.get_surface_z(goal.0, goal.1);

    // Check if the goal tile is walkable; if blocked, find closest accessible neighbor
    let actual_goal = if let Some(target_tile) = world.get_tile(goal.0, goal.1, goal_z) {
        if target_tile.walkable {
            goal
        } else {
            let mut best_neighbor = None;
            let mut min_dist = f32::MAX;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = goal.0 + dx;
                    let ny = goal.1 + dy;
                    let nz = world.get_surface_z(nx, ny);
                    if let Some(nt) = world.get_tile(nx, ny, nz) {
                        if nt.walkable {
                            let d = octile_distance(start, (nx, ny));
                            if d < min_dist {
                                min_dist = d;
                                best_neighbor = Some((nx, ny));
                            }
                        }
                    }
                }
            }
            match best_neighbor {
                Some(g) => g,
                None => return None,
            }
        }
    } else {
        return None;
    };

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();

    g_score.insert(start, 0.0);
    open_set.push(State {
        cost: 0.0,
        heuristic: octile_distance(start, actual_goal),
        pos: start,
    });

    let directions = [
        (1, 0, 1.0),
        (-1, 0, 1.0),
        (0, 1, 1.0),
        (0, -1, 1.0),
        (1, 1, 1.4142),
        (-1, 1, 1.4142),
        (1, -1, 1.4142),
        (-1, -1, 1.4142),
    ];

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 6000;

    while let Some(State { cost, pos, .. }) = open_set.pop() {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            break;
        }

        if pos == actual_goal {
            let mut path = vec![actual_goal];
            let mut curr = actual_goal;
            while let Some(&prev) = came_from.get(&curr) {
                path.push(prev);
                curr = prev;
                if curr == start {
                    break;
                }
            }
            path.reverse();
            return Some(path);
        }

        if let Some(&recorded_g) = g_score.get(&pos) {
            if cost > recorded_g + 0.0001 {
                continue;
            }
        }

        let curr_z = world.get_surface_z(pos.0, pos.1);

        for &(dx, dy, base_dist) in &directions {
            let next_pos = (pos.0 + dx, pos.1 + dy);
            let next_z = world.get_surface_z(next_pos.0, next_pos.1);

            // Slope relief check: height difference must be at most 1 (gentle slope/ramp)
            let dz = (next_z - curr_z).abs();
            if dz > 1 {
                continue; // Too steep cliff to climb
            }

            // Diagonal corner-cutting check
            if dx != 0 && dy != 0 {
                let z1 = world.get_surface_z(pos.0 + dx, pos.1);
                let z2 = world.get_surface_z(pos.0, pos.1 + dy);
                let check1 = world.get_tile(pos.0 + dx, pos.1, z1);
                let check2 = world.get_tile(pos.0, pos.1 + dy, z2);
                let block1 = check1.map(|t| !t.walkable).unwrap_or(true);
                let block2 = check2.map(|t| !t.walkable).unwrap_or(true);
                if block1 && block2 {
                    continue;
                }
            }

            if let Some(next_tile) = world.get_tile(next_pos.0, next_pos.1, next_z) {
                if !next_tile.walkable {
                    continue;
                }

                // Slope movement cost factor (uphill takes more effort, downhill is easier)
                let slope_factor = if next_z > curr_z {
                    1.35
                } else if next_z < curr_z {
                    0.90
                } else {
                    1.0
                };

                let step_cost = base_dist * next_tile.move_cost * slope_factor;
                let tentative_g = cost + step_cost;

                let is_better = match g_score.get(&next_pos) {
                    Some(&prev_g) => tentative_g < prev_g,
                    None => true,
                };

                if is_better {
                    g_score.insert(next_pos, tentative_g);
                    came_from.insert(next_pos, pos);
                    open_set.push(State {
                        cost: tentative_g,
                        heuristic: octile_distance(next_pos, actual_goal),
                        pos: next_pos,
                    });
                }
            }
        }
    }

    None
}
