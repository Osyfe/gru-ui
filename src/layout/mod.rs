use crate::math::*;

const ERR: f32 = 1e-3;

pub enum ScaleWish
{
    AspectFree { min: Vec2, max: Vec2 }, //absolute sizes
    AspectFixed { min: f32, max: f32 } //relative factors
}

pub struct SizeWish
{
    pub pref: Vec2,
    pub scale: ScaleWish
}

impl SizeWish
{
    pub fn new_fixed(size: Vec2) -> Self
    {
        Self
        {
            pref: size,
            scale: ScaleWish::AspectFree { min: size, max: size }
        }
    }

    pub fn fits(&self, size: Vec2) -> bool
    {
        match &self.scale
        {
            ScaleWish::AspectFree { min, max } => size.0 >= min.0 && size.0 <= max.0 && size.1 >= min.1 && size.1 <= max.1,
            ScaleWish::AspectFixed { min, max } =>
            {
                let aspect_wish = self.pref.0 / self.pref.1;
                let aspect_size = size.0 / size.1;
                let size_ratio = aspect_size / aspect_wish;
                (aspect_size - aspect_wish) / aspect_wish < ERR && size_ratio >= *min && size_ratio <= *max
            }
        }
    }
}
