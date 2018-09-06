extern crate glm;

pub enum ParticleType {
    Sprite(String),
    Line(u32),
    Point,
} 

pub struct Particle {
    mPosition : glm::Vector3<f32>,
    mAcceleration : glm::Vector3<f32>,
    mSize : u32,
    mType : ParticleType,
}

pub trait Emiter {
    fn Emit(&self) -> Particle;
}


// example/basic emiters

pub struct PlaneEmiter {
    mNormal : glm::Vector3<f32>,
    mX : u32,
    mY : u32,
}

impl PlaneEmiter {
    pub fn new(normal : glm::Vector3<f32>, x : u32, y : u32) -> PlaneEmiter {
        PlaneEmiter{mNormal : normal, mX : x, mY : y}
    }
}

pub struct StreamEmiter {
    mDirection : glm::Vector3<f32>,
    mAngle : f32,
}

impl StreamEmiter {
    pub fn new(direction : glm::Vector3<f32>, angle : f32) -> StreamEmiter {
        StreamEmiter{mDirection : direction, mAngle : angle}
    }
}

pub struct SphereEmiter {
    mCenter : glm::Vector3<f32>,
    mRadius : f32,
}

impl SphereEmiter {
    pub fn new(center : glm::Vector3<f32>, radius : f32) -> SphereEmiter {
        SphereEmiter{mCenter : center, mRadius : radius}
    }
}
