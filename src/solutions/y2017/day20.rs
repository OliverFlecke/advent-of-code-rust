use std::fmt::Display;

use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day20;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3D {
    fn parse(input: &str) -> Vec3D {
        let mut splits = input.split(',');
        Vec3D {
            x: splits.next().unwrap().parse().unwrap(),
            y: splits.next().unwrap().parse().unwrap(),
            z: splits.next().unwrap().parse().unwrap(),
        }
    }

    fn magnitude(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

impl std::ops::Add<&Vec3D> for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: &Vec3D) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, PartialEq)]
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
            position: Vec3D::parse(&captures["position"]),
            velocity: Vec3D::parse(&captures["velocity"]),
            acceleration: Vec3D::parse(&captures["acceleration"]),
        }
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
    fn solve_a(&self, input: &str) -> Answer {
        let mut particles = Self::parse(input);
        particles.sort_unstable_by_key(|p| {
            (
                p.acceleration.magnitude(),
                p.velocity.magnitude(),
                p.position.magnitude(),
            )
        });

        particles[0].id.into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_vec3d() {
        assert_eq!(Vec3D::parse("1,2,-3"), Vec3D { x: 1, y: 2, z: -3 });
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

        assert_eq!(Day20 {}.solve_a(input), Answer::UInt(0));
    }
}
