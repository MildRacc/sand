pub mod particles;
use particles::{Particle, ParticleBase, ParticleType, ParticleState};
use particles::{EMPTY, SAND, WATER};


pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 200;


pub struct PixelMatrix
{
    pub screen: [[Particle; HEIGHT as usize]; WIDTH as usize],
}
impl PixelMatrix
{
   pub fn new() -> Self
   {

        let mut pixelbuf = [[ Particle::new(ParticleType::Empty); HEIGHT as usize]; WIDTH as usize];

        for i in 100..200
        {
            for j in HEIGHT as usize-100..HEIGHT as usize
            {
                pixelbuf[i][j] = Particle::new(ParticleType::Sand);
            }
        }
        for i in 300..350
        {
            for j in 0..45 as usize
            {
                pixelbuf[i][j] = Particle::new(ParticleType::Sand);
            }
        }

        for i in 300..350
        {
            for j in HEIGHT as usize-100..HEIGHT as usize
            {
                pixelbuf[i][j] = Particle::new(ParticleType::Water);
            }
        }
        for i in 100..150
        {
            for j in 0..50 as usize
            {
                pixelbuf[i][j] = Particle::new(ParticleType::Water);
            }
        }


        Self { screen: pixelbuf }
    }

    pub fn get(&self, x: usize, y: usize) -> Particle
    {
        self.screen[x][y]
    }

    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize)
    {
        if x1 == x2
        {
            self.screen[x1].swap(y1, y2);
        }
        else
        {
            let (a, b) = if x1 < x2
            {
                let (left, right) = self.screen.split_at_mut(x2);
                (&mut left[x1][y1], &mut right[0][y2])
            }
            else
            {
                let (left, right) = self.screen.split_at_mut(x1);
                (&mut left[x2][y2], &mut right[0][y1])
            };

            std::mem::swap(a, b);
        }
    }
}
