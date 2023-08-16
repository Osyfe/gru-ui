use super::color::Color;

#[derive(Clone)]
pub struct StyleSet
{
    pub text: Color
}

impl Default for StyleSet
{
    fn default() -> Self
    {
        Self
        {
            text: Color::from_discrete_srgb(0, 0, 0, 255)
        }
    }
}
