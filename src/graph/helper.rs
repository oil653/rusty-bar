use crate::graph::graph::*;
use iced::{Color, Element, Font, Length, widget::Canvas};
use std::{f32, str::FromStr};

pub fn graph<'a>(
    width: impl Into<f32>,
    height: impl Into<f32>,

    scale_line_color: Color,
    scale_line_width: impl Into<f32>,
    
    font_color: Color,

    // The labels on the bottom
    labels: impl IntoIterator<Item = impl Into<String>>,
    label_height: impl Into<f32>,

    // The data displayed
    series: impl IntoIterator<Item = impl Into<Series>>,
    min: Option<impl Into<f32>>,
    max: Option<impl Into<f32>>,
    value_steps: Option<impl Into<i32>>,
    serires_line_width: impl Into<f32>,

    hover_color: Option<Color>
) -> Element<'a, crate::Message> {
    let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
    let series: Vec<Series> = series.into_iter().map(Into::into).collect();

    let min: f32 = match min {
        Some(v) => v.into(),
        None => {
            let mut overal_min = f32::INFINITY;

            for serie in &series {
                let values: Vec<f32> = serie.values.iter().map(|value: &(f32, f32)| value.0.clone()).collect();
                let min = values.iter().fold(f32::INFINITY, |acc, val| acc.min(val.clone()));
                overal_min = min.min(overal_min);
            }

            overal_min
        }
    };

    let max: f32 = match max {
        Some(v) => v.into(),
        None => {
            let mut overal_max = f32::INFINITY;

            for serie in &series {
                let values: Vec<f32> = serie.values.iter().map(|value: &(f32, f32)| value.0.clone()).collect();
                let max = values.iter().fold(f32::INFINITY, |acc, val| acc.max(val.clone()));
                overal_max = max.min(overal_max);
            }

            overal_max
        }
    };

    let steps = match value_steps {
        Some(v) => v.into(),
        None => {
            let range = (max - min).abs() as i32;

            if range >= 10 {
                10
            } else if range >= 6 {
                2
            } else {
                1
            }
        }
    };

    // Adjust min and max to be on a whole value in steps
    let min = {
        ((min % steps as f32) + 1.0) * steps as f32
    };

    let max = {
        ((max % steps as f32) + 1.0) * steps as f32
    };

    let graph = Graph {
        scale_line_color,
        scale_line_width: scale_line_width.into(),
        scale_hmargin: 30.0,
        top_padding: 8.0,

        labels,
        label_hmargin: 8.0,
        bottom_label_height: label_height.into(),

        series,
        
        font: Font::DEFAULT,
        font_color,

        max_value: max,
        min_value: min,

        value_steps: steps,

        series_line_width: serires_line_width.into(),

        hover_radius: 9.0,
        hover_color: hover_color.unwrap_or(Color::from_str("#cba6f7").expect("Failed to convert '#cba6f7' to color"))
    };

    Canvas::new(graph)
    .width(width.into())
    .height(height.into())
    .into()
}