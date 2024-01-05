use super::Color;
use crate::widget::interact::WidgetState;

#[derive(Clone)]
pub struct ColorSet
{
    pub cold: Color,
    pub hover: Color,
    pub hot: Color
}

impl ColorSet
{
    pub fn get(&self, state: WidgetState) -> Color
    {
        match state
        {
            WidgetState::Cold => self.cold,
            WidgetState::Hover => self.hover,
            WidgetState::Hot => self.hot
        }
    }
}

#[derive(Clone)]
pub struct StyleSet
{
    pub bg: ColorSet,
    pub top: Color,
    pub text: Color,
    pub data: ColorSet
}

impl Default for StyleSet
{
    fn default() -> Self
    {
        Self
        {
            bg: ColorSet
            {
                cold: Color::from_discrete_srgb(200, 200, 200, 255),
                hover: Color::from_discrete_srgb(150, 150, 250, 255),
                hot: Color::from_discrete_srgb(250, 150, 150, 255)
            },
            top: Color::from_discrete_srgb(170, 170, 170, 255),
            text: Color::from_discrete_srgb(0, 0, 0, 255),
            data: ColorSet
            {
                cold: Color::from_discrete_srgb(100, 100, 100, 255),
                hover: Color::from_discrete_srgb(50, 150, 50, 255),
                hot: Color::from_discrete_srgb(250, 200, 200, 255)
            }
        }
    }
}
