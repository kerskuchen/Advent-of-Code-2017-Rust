#[macro_use]
extern crate text_io;

use std::collections::HashSet;
use std::fs::File;
use std::io::*;
use std::ops::AddAssign;

fn main() {
    let particles: Vec<Particle> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(index, line)| Particle::from_str(index, &line).unwrap())
        .collect();

    part_one(particles.clone());
    part_two(particles);
}

fn part_one(mut particles: Vec<Particle>) {
    // We simulate particles and sort them by distance to origin, until the closest particle
    // stays the same for 10_000 iterations
    let mut closest_particle_id = 0;
    let mut num_simulation_steps_without_change = 0;
    loop {
        for particle in &mut particles {
            particle.vel += particle.acc;
            particle.pos += particle.vel;
        }
        particles.sort_by(|a, b| a.acc.abs().cmp(&b.acc.abs()));

        if closest_particle_id != particles[0].id {
            closest_particle_id = particles[0].id;
            num_simulation_steps_without_change = 0;
        }

        num_simulation_steps_without_change += 1;
        if num_simulation_steps_without_change > 10_000 {
            break;
        }
    }
    println!(
        "The particle which will stay closest to position <0,0,0> is particle {}",
        particles[0].id
    );
}

fn part_two(mut particles: Vec<Particle>) {
    // We simulate and remove collided particles until the number of particles stays the same
    // for 10_000 iterations
    let mut num_simulation_steps_without_change = 0;
    loop {
        for particle in &mut particles {
            particle.vel += particle.acc;
            particle.pos += particle.vel;
        }

        let (collision_happened, particles_without_collision) = remove_collided(particles);
        particles = particles_without_collision;
        if collision_happened {
            num_simulation_steps_without_change = 0;
        }

        num_simulation_steps_without_change += 1;
        if num_simulation_steps_without_change > 10_000 {
            break;
        }
    }
    println!(
        "The particle number of particles left after all collisions are resolved is {}",
        particles.len()
    );
}

/// Removes all colliding particles form a given particle vector by returning a
/// new vector without them. Returns true if collisions have happened.
fn remove_collided(mut particles: Vec<Particle>) -> (bool, Vec<Particle>) {
    // Collect all positions where collisions are happening
    // NOTE: Through sorting by position, we achieve that particle collision groups are consecutive
    //       in the vector
    particles.sort_by(|a, b| a.pos.cmp(&b.pos));
    let mut collision_points = HashSet::new();
    for possible_collision_point in particles.iter().map(|particle| particle.pos) {
        if particles
            .iter()
            .filter(|particle| particle.pos == possible_collision_point)
            .count() > 1
        {
            collision_points.insert(possible_collision_point);
        }
    }
    if collision_points.is_empty() {
        return (false, particles);
    }

    // Remove all particles that are located at collision points
    let particles_without_collision = particles
        .into_iter()
        .filter(|particle| {
            collision_points
                .iter()
                .all(|&col_point| particle.pos != col_point)
        })
        .collect();
    (true, particles_without_collision)
}

#[derive(Clone, Copy)]
struct Particle {
    id: usize,
    acc: Vec3,
    vel: Vec3,
    pos: Vec3,
}

impl Particle {
    fn from_str(id: usize, string: &str) -> Result<Self> {
        let mut pos = Vec3::new(0, 0, 0);
        let mut vel = Vec3::new(0, 0, 0);
        let mut acc = Vec3::new(0, 0, 0);
        scan!(
            string.bytes() =>
            "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            pos.x,
            pos.y,
            pos.z,
            vel.x,
            vel.y,
            vel.z,
            acc.x,
            acc.y,
            acc.z
        );
        Ok(Particle { id, pos, vel, acc })
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3 { x, y, z }
    }
    fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
