extern crate glm;
extern crate rand;

use Emiter::rand::distributions::{IndependentSample, Range};

pub enum ParticleType {
    Sprite(String),
    Line(u32),
    Point,
} 

// Struct to describe a particle that will initially be
// emited from one of the base or custom emiters.
pub struct Particle {
    mPosition : glm::Vector3<f32>,
    mAcceleration : glm::Vector3<f32>,
    mSize : u32,
    mType : ParticleType,
}

pub struct ParticleDescription {
    mMinSize : u32,
    mMaxSize : u32,
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
    mX : u32,
    mY : u32,
    mDesc : ParticleDescription,
    mRng : rand::XorShiftRng,
}

impl PlaneEmiter {
    pub fn new( position : glm::Vector3<f32>, 
                normal : glm::Vector3<f32>, 
                x : u32, 
                y : u32, 
                desc : ParticleDescription) -> PlaneEmiter {
        PlaneEmiter{mPosition : position, mNormal : normal, mX : x, mY : y, mDesc : desc, mRng : rand::XorShiftRng::new_unseeded()}
    }
}

pub struct StreamEmiter {
    mDirection : glm::Vector3<f32>,
    mPosition : glm::Vector3<f32>,
    mAngle : f32,
    mDesc : ParticleDescription,
    mRng : rand::XorShiftRng,
}

impl StreamEmiter {
    pub fn new( position : glm::Vector3<f32>, 
                direction : glm::Vector3<f32>, 
                angle : f32,
                desc : ParticleDescription) -> StreamEmiter {
        StreamEmiter{mPosition : position, mDirection : direction, mAngle : angle, mDesc : desc, mRng : rand::XorShiftRng::new_unseeded()}
    }
}

pub struct SphereEmiter {
    mCenter : glm::Vector3<f32>,
    mPosition : glm::Vector3<f32>,
    mRadius : f32,
    mDesc : ParticleDescription,
    mRng : rand::XorShiftRng,
}

impl SphereEmiter {
    pub fn new( position : glm::Vector3<f32>,
                center : glm::Vector3<f32>, 
                radius : f32,
                desc : ParticleDescription) -> SphereEmiter {
        SphereEmiter{mPosition : position, mCenter : center, mRadius : radius, mDesc : desc, mRng : rand::XorShiftRng::new_unseeded()}
    }
}
