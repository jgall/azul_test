use std::cell::RefCell;
use std::rc::Rc;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
}

impl Particle {
    fn distance(&self, other: &Self) -> f64 {
        let dy = self.y - other.y;
        let dx = self.x - other.x;
        (dy.powf(2 as f64) + dx.powf(2 as f64)).sqrt()
    }
}

pub struct Spring {
    p1: Rc<RefCell<Particle>>,
    p2: Rc<RefCell<Particle>>,
    base_len: f64,
    k: f64,
}

pub struct Mesh {
    pub particles: Vec<Rc<RefCell<Particle>>>,
    pub springs: Vec<Spring>,
    max_vel: f64,
    damping: f64,
}

impl Mesh {
    pub fn step(&mut self, time_step: f64) {
        for spring in &self.springs {
            let mut p1 = spring.p1.borrow_mut();
            let mut p2 = spring.p2.borrow_mut();
            let dist = p1.distance(&p2);
            let force = spring.k * (dist - spring.base_len);
            p1.vx += time_step * force * (p2.x - p1.x);
            p1.vy += time_step * force * (p2.y - p1.y);
            p2.vx -= time_step * force * (p2.x - p1.x);
            p2.vy -= time_step * force * (p2.y - p1.y);
        }

        let vdamp = self.damping.powf(time_step);
        for p in &self.particles {
            let mut p = p.borrow_mut();
            p.vx *= vdamp;
            p.vy *= vdamp;
            let vmag = (p.vx * p.vx + p.vy * p.vy).sqrt();
            if vmag > self.max_vel {
                let scale = self.max_vel / vmag;
                p.vx *= scale;
                p.vy *= scale;
            }
            p.x += time_step * p.vx;
            p.y += time_step * p.vy;
        }
    }

    pub fn new_grid(spacing: f64, x: f64, y: f64, rows: i64, cols: i64) -> Self {
        let mut particles = vec![];
        let mut springs = vec![];
        for row in 0..rows {
            for col in 0..cols {
                let p = Rc::new(RefCell::new(Particle {
                    x: x + col as f64 * spacing,
                    y: y + row as f64 * spacing,
                    vx: 0.0,
                    vy: 0.0,
                }));
                particles.push(p.clone());
                if col > 0 {
                    let p1 = particles[(row * cols + col - 1) as usize].clone();
                    let p2 = p.clone();
                    springs.push(Spring {
                        p1: p1.clone(),
                        p2: p2.clone(),
                        base_len: p1.borrow().distance(&p2.borrow()),
                        k: 10.0,
                    });
                }
                if row > 0 {
                    let p1 = particles[((row - 1) * cols + col) as usize].clone();
                    let p2 = p.clone();
                    springs.push(Spring {
                        p1: p1.clone(),
                        p2: p2.clone(),
                        base_len: p1.borrow().distance(&p2.borrow()),
                        k: 10.0,
                    });
                }
            }
        }
        let edges = particles
            .iter()
            .enumerate()
            .filter(|(i, _p)| {
                *i < rows as usize
                    || *i % cols as usize == 0
                    || *i > (cols * (rows - 1)) as usize
                    || *i % cols as usize == cols as usize - 1
            })
            .collect::<Vec<_>>();
        for (i1, particle) in particles.iter().enumerate() {
            for (i2, edge) in &edges {
                if i1 != *i2 {
                    let base_len = particle.borrow().distance(&edge.borrow());
                    springs.push(Spring {
                        p1: particle.clone(),
                        p2: (*edge).clone(),
                        base_len: base_len,
                        k: 10.0 / (base_len * base_len),
                    })
                }
            }
        }
        Mesh {
            particles: particles,
            springs: springs,
            max_vel: 100.0,
            damping: 0.01,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_grid() {
        let mut mesh = Mesh::new_grid(1.0, 1.0, 1.0, 5, 5);
        mesh.step(5.0);
    }
}
