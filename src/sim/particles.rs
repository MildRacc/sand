use crate::sim::{HEIGHT, PixelMatrix, WIDTH, rand::rand};

pub static EMPTY: ParticleBase = ParticleBase::new(0);
pub static SAND: ParticleBase = ParticleBase::new(2);
pub static WATER: ParticleBase = ParticleBase::new(1);
pub static GAS: ParticleBase = ParticleBase::new(-1);




#[derive(Clone, Copy, Debug)]
pub enum ParticleType
{
    Empty,
    Sand,
    Water
}


#[derive(Clone, Copy, Debug)]
pub enum ParticleState
{
    Solid,
    Liquid,
    Gas,
    Plasma,
}



#[derive(Clone, Copy, Debug)]
pub struct Particle
{
    pub base: &'static ParticleBase,
    pub ptype: ParticleType,
    pub pstate: ParticleState,
    pub just_moved: bool,
}
impl Particle
{
    pub fn new(ptype: ParticleType) -> Self
    {
        let (base, pstate): (&ParticleBase, ParticleState) = match ptype
        {
            ParticleType::Empty => {(&EMPTY, ParticleState::Solid)},
            ParticleType::Sand => {(&SAND, ParticleState::Solid)},
            ParticleType::Water => {(&WATER, ParticleState::Liquid)},
            _ => {(&EMPTY, ParticleState::Solid)}
        };


        Self
        {
            base,
            ptype,
            pstate,
            just_moved: false
        }
    }


    pub fn update(&mut self, x: usize, y: usize, matrix: &mut PixelMatrix)
    {
        if matrix.screen[x][y].just_moved {return}

        match self.pstate
        {
            ParticleState::Solid => {self.solid_move(x, y, matrix)},
            ParticleState::Liquid => {self.liquid_move(x, y, matrix);},
            ParticleState::Gas => {},
            ParticleState::Plasma => {},
        }
    }


    pub fn solid_move(&mut self, x: usize, y: usize, mat: &mut PixelMatrix)
    {
        if y+1 >= HEIGHT as usize {return}
       
        let curr = &mat.get(x, y);


        if mat.get(x, y+1).base.density < curr.base.density
        {
            mat.swap(x, y, x, y+1);
            mat.screen[x][y+1].just_moved = true;
            return;
        }

        let dr_goable: bool = x+1 < WIDTH as usize && mat.get(x+1, y+1).base.density < curr.base.density;
        let dl_goable: bool = x > 0 && mat.get(x-1, y+1).base.density < curr.base.density;

        if dr_goable && dl_goable
        {
            if rand() & 1 == 0
            { // Go Down Right
                mat.swap(x, y, x+1, y+1);
                mat.screen[x+1][y+1].just_moved = true;
                return;
            }
            else
            { // Go Down Left
                mat.swap(x, y, x-1, y+1);
                mat.screen[x-1][y+1].just_moved = true;
                return;
            }
        }

        if dr_goable
        {
            mat.swap(x, y, x+1, y+1);
            mat.screen[x+1][y+1].just_moved = true;
            return;
        }
        if x > 0 && mat.get(x-1, y+1).base.density < curr.base.density
        {
            mat.swap(x, y, x-1, y+1);
            mat.screen[x-1][y+1].just_moved = true;
            return;
        }

    }


    pub fn liquid_move(&mut self, x: usize, y: usize, mat: &mut PixelMatrix)
    {
        if y+1 >= HEIGHT as usize {return}
       
        self.just_moved = true;
        let curr = &mat.get(x, y);
        let curr_density = curr.base.density;


        if mat.get(x, y+1).base.density < curr.base.density
        {
            mat.swap(x, y, x, y+1);
            mat.screen[x][y+1].just_moved = true;
            return;
        }


        if y > 0 && mat.get(x, y-1).base.density < curr_density
        {
            let ur_goable = x+1 < WIDTH as usize && mat.get(x+1, y-1).base.density < curr_density;
            let ul_goable = x > 0 && mat.get(x-1, y-1).base.density < curr_density;
        
            if ur_goable && ul

        }


        if x+1 < WIDTH as usize && mat.get(x+1, y+1).base.density < curr_density 
        {
            mat.swap(x, y, x+1, y+1);
            mat.screen[x+1][y+1].just_moved = true;
            return;
        }
        if x > 0 && mat.get(x-1, y+1).base.density < curr_density 
        {
            mat.swap(x, y, x-1, y+1);
            mat.screen[x-1][y+1].just_moved = true;
            return;
        }

        if x+1 < WIDTH as usize && mat.get(x+1, y).base.density == 0
        {
            mat.swap(x, y, x+1, y);
            mat.screen[x][y+1].just_moved = true;
            return
        }
        if x > 0 && mat.get(x-1, y).base.density == 0
        {
            mat.swap(x, y, x-1, y);
            mat.screen[x-1][y].just_moved = true;
            return
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct ParticleBase
{
    pub density: i16, 
}
impl ParticleBase
{
    pub const fn new(density: i16) -> Self
    {
        Self 
        {
            density
        }
    }
}
