extern crate glm;
extern crate rand;

use Emiter::rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub enum ParticleType {
    Sprite(String),
    Line(u32),
    Point,
} 

// Struct to describe a particle that will initially be
// emited from one of the base or custom emiters.
pub struct Particle {
    mPosition : glm::Vector3<f32>,
    mSpeed : glm::Vector3<f32>,
    mSize : u32,
    mType : ParticleType,
}

pub struct ParticleDescription {
    mMinSize : u32,
    mMaxSize : u32,
    mMinSpeed : f32,
    mMaxSpeed : f32,
    mMaxLifeTime : u32,
    mType : ParticleType
}

// Users may add custom structs that implement the emiter trait 
pub trait Emiter {
    fn Emit(&self) -> Particle;
}


// example/basic emiters

pub struct PlaneEmiter {
    mNormal : glm::Vector3<f32>,
    mPosition : glm::Vector3<f32>,
    mX : f32,
    mY : f32,
    mDesc : ParticleDescription
}

impl PlaneEmiter {
    pub fn new( position : glm::Vector3<f32>, 
                normal : glm::Vector3<f32>, 
                x : f32, 
                y : f32, 
                desc : ParticleDescription) -> PlaneEmiter {
        PlaneEmiter{mPosition : position, mNormal : normal, mX : x, mY : y, mDesc : desc}
    }
}


impl Emiter for PlaneEmiter {
    fn Emit(&self) -> Particle {
        let mut rng = thread_rng();
        let x_position : f32 = rng.gen_range(self.mPosition.x - self.mX, self.mPosition.x + self.mX);
        let y_position : f32 = rng.gen_range(self.mPosition.y - self.mY, self.mPosition.y + self.mY);
        let speed : f32 = rng.gen_range(self.mDesc.mMinSpeed, self.mDesc.mMaxSpeed);
        let size : u32  = rng.gen_range(self.mDesc.mMinSize, self.mDesc.mMaxSize);

        Particle{mPosition : glm::Vector3::<f32>::new(x_position, y_position, self.mPosition.z),
                 mSpeed : glm::Vector3::<f32>::new(speed, speed, speed) * self.mNormal,
                 mSize : size,
                 mType : self.mDesc.mType.clone()
        }
    }
}


pub struct StreamEmiter {
    mDirection : glm::Vector3<f32>,
    mPosition : glm::Vector3<f32>,
    mAngle : f32,
    mDesc : ParticleDescription
}

impl StreamEmiter {
    pub fn new( position : glm::Vector3<f32>, 
                direction : glm::Vector3<f32>, 
                angle : f32,
                desc : ParticleDescription) -> StreamEmiter {
        StreamEmiter{mPosition : position, mDirection : direction, mAngle : angle, mDesc : desc}
    }
}


impl Emiter for StreamEmiter {
    fn Emit(&self) -> Particle {

        unimplemented!();
    }
}

pub struct SphereEmiter {
    mCenter : glm::Vector3<f32>,
    mPosition : glm::Vector3<f32>,
    mRadius : f32,
    mDesc : ParticleDescription
}

impl SphereEmiter {
    pub fn new( position : glm::Vector3<f32>,
                center : glm::Vector3<f32>, 
                radius : f32,
                desc : ParticleDescription) -> SphereEmiter {
        SphereEmiter{mPosition : position, mCenter : center, mRadius : radius, mDesc : desc}
    }
}


impl Emiter for SphereEmiter {
    fn Emit(&self) -> Particle {

        unimplemented!();
    }
}
