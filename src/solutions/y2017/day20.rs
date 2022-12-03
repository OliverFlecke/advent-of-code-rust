use std::{
    collections::HashSet,
    fmt::{Display, Error},
    str::FromStr,
};

use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day20;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3D {
    fn magnitude(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl FromStr for Vec3D {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(',');
        Ok(Vec3D {
            x: splits.next().unwrap().parse().unwrap(),
            y: splits.next().unwrap().parse().unwrap(),
            z: splits.next().unwrap().parse().unwrap(),
        })
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

impl std::ops::Add<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Vec3D> for Vec3D {
    fn add_assign(&mut self, rhs: Vec3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Particle {
    id: usize,
    position: Vec3D,
    velocity: Vec3D,
    acceleration: Vec3D,
}

const PARTICLE_PATTERN: &str =
    r"p=<(?P<position>[0-9,\-]*)>, v=<(?P<velocity>[0-9,\-]*)>, a=<(?P<acceleration>[0-9,\-]*)>";

impl Particle {
    fn parse(id: usize, input: &str) -> Self {
        let re = Regex::new(PARTICLE_PATTERN).unwrap();
        let captures = re.captures(input).unwrap();
        Particle {
            id,
            position: Vec3D::from_str(&captures["position"]).unwrap(),
            velocity: Vec3D::from_str(&captures["velocity"]).unwrap(),
            acceleration: Vec3D::from_str(&captures["acceleration"]).unwrap(),
        }
    }

    fn collide(&self, other: &Particle) -> bool {
        self.position == other.position
    }

    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id={}, p={}, v={}, a={}",
            self.id, self.position, self.velocity, self.acceleration
        )
    }
}

impl Day20 {
    fn parse(input: &str) -> Vec<Particle> {
        input
            .lines()
            .enumerate()
            .map(|(i, l)| Particle::parse(i, l))
            .collect()
    }
}

impl Solution for Day20 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let mut particles = Self::parse(input);

        // NOTE: This is not correct in every chase.
        // The general idea is: the particale that will remain the closest to the
        // origin is the one with the smallest acceleration. If two particles have
        // the same acceleration, it will be the one with the smallest velocity.
        // (And if the velocities are the same, the one initially closest to
        // the origin wins).
        //
        // However, here we only look at the magnitude for the velocity (which
        // works well enough for my input), which is not enough. If the
        // acceleration is in the same direction as the velocity, higher
        // velocity is better, while if it is the opposite direction, lower
        // is better. The same argument holds for the relationship between the
        // velocity and position.
        particles.sort_unstable_by_key(|p| {
            (
                p.acceleration.magnitude(),
                p.velocity.magnitude(),
                p.position.magnitude(),
            )
        });

        Some(particles[0].id.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        // NOTE: This has been chosen arbitrarily and adjusted until nothing
        // seemed to move around any more.
        const MAGIC_ITERATIONS: usize = 5_000;

        let mut particles = Self::parse(input);

        for _ in 0..MAGIC_ITERATIONS {
            let mut collided = HashSet::new();

            particles.sort_unstable_by_key(|p| p.position);
            for i in 0..(particles.len() - 1) {
                if particles[i].collide(&particles[i + 1]) {
                    collided.insert(particles[i]);
                    collided.insert(particles[i + 1]);
                }
            }

            particles.retain(|p| !collided.contains(p));
            for p in &mut particles {
                p.tick();
            }
        }

        Some(particles.len().into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_vec3d() {
        assert_eq!(
            Vec3D::from_str("1,2,-3").unwrap(),
            Vec3D { x: 1, y: 2, z: -3 }
        );
    }

    #[test]
    fn parse_particle() {
        let input = "p=<1,2,-3>, v=<4,-5,6>, a=<-7,8,9>";
        assert_eq!(
            Particle::parse(3, input),
            Particle {
                id: 3,
                position: Vec3D { x: 1, y: 2, z: -3 },
                velocity: Vec3D { x: 4, y: -5, z: 6 },
                acceleration: Vec3D { x: -7, y: 8, z: 9 },
            }
        );
    }

    #[test]
    fn test_a() {
        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

        assert_eq!(Day20 {}.solve_a(input), Some(Answer::UInt(0)))
    }

    #[test]
    fn test_b() {
        let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(Day20 {}.solve_b(input), Some(Answer::UInt(1)))
    }
}
