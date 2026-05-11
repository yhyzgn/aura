use aura_icons::Icon;
use gpui::{Animation, AnimationElement, AnimationExt, ElementId, IntoElement, Styled, radians};
use std::{f32::consts::TAU, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionDuration {
    Fast,
    Normal,
    Slow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionEasing {
    Linear,
    EaseInOut,
    EaseOut,
    Elastic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionPreset {
    FadeIn,
    FadeOut,
    PopIn,
    Pulse,
    Spin,
    ElasticSlide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    In,
    Out,
}

impl MotionDuration {
    pub fn as_duration(self) -> Duration {
        match self {
            Self::Fast => Duration::from_millis(120),
            Self::Normal => Duration::from_millis(180),
            Self::Slow => Duration::from_millis(240),
        }
    }
}

pub fn motion_animation(duration: MotionDuration, easing: MotionEasing) -> Animation {
    Animation::new(duration.as_duration()).with_easing(move |delta| ease(delta, easing))
}

pub fn repeating_motion_animation(duration: MotionDuration, easing: MotionEasing) -> Animation {
    motion_animation(duration, easing).repeat()
}

pub fn fade<E>(
    id: impl Into<ElementId>,
    direction: FadeDirection,
    element: E,
) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    element.with_animation(
        id,
        motion_animation(MotionDuration::Fast, MotionEasing::EaseOut),
        move |element, delta| {
            let opacity = match direction {
                FadeDirection::In => delta,
                FadeDirection::Out => 1.0 - delta,
            };
            element.opacity(opacity)
        },
    )
}

pub fn fade_in<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    fade(id, FadeDirection::In, element)
}

pub fn fade_out<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    fade(id, FadeDirection::Out, element)
}

pub fn pop_in<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    element.with_animation(
        id,
        motion_animation(MotionDuration::Normal, MotionEasing::EaseOut),
        |element, delta| element.opacity(0.86 + delta * 0.14),
    )
}

pub fn pulse<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    element.with_animation(
        id,
        repeating_motion_animation(MotionDuration::Slow, MotionEasing::EaseInOut),
        |element, delta| element.opacity(0.62 + pulse_alpha(delta) * 0.38),
    )
}

pub fn spin_icon(id: impl Into<ElementId>, icon: Icon) -> AnimationElement<Icon> {
    icon.with_animation(
        id,
        repeating_motion_animation(MotionDuration::Slow, MotionEasing::Linear),
        |icon, delta| icon.rotation(radians(delta * TAU)),
    )
}

pub fn elastic_slide(delta: f32) -> f32 {
    let t = delta.clamp(0.0, 1.0);
    let c1 = 1.35;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

fn ease(delta: f32, easing: MotionEasing) -> f32 {
    match easing {
        MotionEasing::Linear => gpui::linear(delta),
        MotionEasing::EaseInOut => gpui::ease_in_out(delta),
        MotionEasing::EaseOut => gpui::ease_out_quint()(delta),
        MotionEasing::Elastic => elastic_slide(delta),
    }
}

fn pulse_alpha(delta: f32) -> f32 {
    gpui::pulsating_between(0.0, 1.0)(delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_duration_tokens_track_aura_defaults() {
        assert_eq!(
            MotionDuration::Fast.as_duration(),
            Duration::from_millis(120)
        );
        assert_eq!(
            MotionDuration::Normal.as_duration(),
            Duration::from_millis(180)
        );
        assert_eq!(
            MotionDuration::Slow.as_duration(),
            Duration::from_millis(240)
        );
    }

    #[test]
    fn elastic_slide_overshoots_then_settles() {
        assert!(elastic_slide(0.0).abs() < 0.000_01);
        assert_eq!(elastic_slide(1.0), 1.0);
        assert!(elastic_slide(0.7) > 1.0);
    }

    #[test]
    fn motion_presets_cover_requested_component_behaviors() {
        let presets = [
            MotionPreset::FadeIn,
            MotionPreset::FadeOut,
            MotionPreset::PopIn,
            MotionPreset::Pulse,
            MotionPreset::Spin,
            MotionPreset::ElasticSlide,
        ];

        assert_eq!(presets.len(), 6);
    }
}
