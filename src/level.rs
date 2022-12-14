use crate::game::Triangle;
use crate::log;
use regex::Regex;
use svg;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::Style;
use svg::parser::Event;

pub struct Level {
    triangles: Vec<Triangle>,
}

fn update_point(
    (x, y): &mut (f32, f32),
    position: Position,
    (new_x, new_y): (Option<f32>, Option<f32>),
) -> (f32, f32) {
    match position {
        Position::Absolute => {
            *x = new_x.unwrap_or(*x);
            *y = new_y.unwrap_or(*y);
        }
        Position::Relative => {
            *x += new_x.unwrap_or(0.0);
            *y += new_y.unwrap_or(0.0);
        }
    }
    (*x, *y)
}

impl Level {
    #[allow(dead_code)]
    pub fn load_from_svg_str(content: &str) -> Level {
        Self::parse(svg::read(content).unwrap())
    }

    fn parse(parser: svg::parser::Parser) -> Level {
        let mut points: Vec<Vec<f32>> = vec![vec![0.0, 0.0]];
        let mut color = [1.0, 0.0, 1.0, 1.0];

        for event in parser {
            match event {
                Event::Tag("path", _, attributes) => {
                    let data = attributes.get("d").unwrap();
                    let data = Data::parse(data).unwrap();

                    color = color_from_style(
                        attributes.get("style").map(|v| Style::new(v.to_string())),
                    )
                    .unwrap_or(color);

                    let mut current_pos = (0.0, 0.0);

                    for command in data.iter() {
                        match command {
                            Command::Move(position, parameters) => {
                                let start = parameters
                                    .array_chunks::<2>()
                                    .last()
                                    .map(|[x, y]| {
                                        update_point(
                                            &mut current_pos,
                                            *position,
                                            (Some(*x), Some(*y)),
                                        )
                                    })
                                    .unwrap_or((0.0, 0.0));

                                points.push(vec![start.0, start.1]);
                            }
                            Command::Line(position, parameters) => {
                                parameters.array_chunks::<2>().for_each(|[x, y]| {
                                    if let Some(path) = points.last_mut() {
                                        let p = update_point(
                                            &mut current_pos,
                                            *position,
                                            (Some(*x), Some(*y)),
                                        );

                                        path.push(p.0);
                                        path.push(p.1);
                                    };
                                });
                            }
                            Command::HorizontalLine(position, parameters) => {
                                parameters.iter().for_each(|x| {
                                    if let Some(path) = points.last_mut() {
                                        let p = update_point(
                                            &mut current_pos,
                                            *position,
                                            (Some(*x), None),
                                        );

                                        path.push(p.0);
                                        path.push(p.1);
                                    };
                                });
                            }
                            Command::VerticalLine(position, parameters) => {
                                parameters.iter().for_each(|y| {
                                    if let Some(path) = points.last_mut() {
                                        let p = update_point(
                                            &mut current_pos,
                                            *position,
                                            (None, Some(*y)),
                                        );

                                        path.push(p.0);
                                        path.push(p.1);
                                    };
                                });
                            }
                            Command::Close => {
                                points.push(vec![0.0, 0.0]);
                            }
                            _ => {
                                log!("unsupported svg path command: {:?}", command);
                            }
                        }
                    }
                }
                Event::Tag("rect", _, attributes) => {
                    let x = attributes.get("x").unwrap().parse::<f32>().unwrap();
                    let y = attributes.get("y").unwrap().parse::<f32>().unwrap();
                    let width = attributes.get("width").unwrap().parse::<f32>().unwrap();
                    let height = attributes.get("height").unwrap().parse::<f32>().unwrap();

                    color = color_from_style(
                        attributes.get("style").map(|v| Style::new(v.to_string())),
                    )
                    .unwrap_or(color);

                    let path = vec![
                        // first triangle
                        x,
                        y,
                        x + width,
                        y,
                        x + width,
                        y + height,
                        // second triangle
                        x,
                        y,
                        x + width,
                        y + height,
                        x,
                        y + height,
                    ];

                    points.push(path);
                }
                _ => {}
            }
        }

        let mut triangles: Vec<Triangle> = vec![];

        for path in points.iter() {
            // not even a triangle
            if path.len() < 6 {
                continue;
            }

            let path_triangles = earcutr::earcut(path.as_slice(), &vec![], 2);

            for [a, b, c] in path_triangles.array_chunks::<3>() {
                triangles.push(Triangle {
                    coords: [
                        path[*a * 2] as f32,
                        path[*a * 2 + 1] as f32,
                        0.0 as f32,
                        path[*b * 2] as f32,
                        path[*b * 2 + 1] as f32,
                        0.0 as f32,
                        path[*c * 2] as f32,
                        path[*c * 2 + 1] as f32,
                        0.0 as f32,
                    ],
                    color,
                });
            }
        }

        Level { triangles }
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
}

fn color_from_style(style: Option<Style>) -> Option<[f32; 4]> {
    let style = style?;
    let style_str = style.to_string();

    let re = Regex::new(r"fill:#([0-9a-z]{6})").unwrap();

    let rgb_str = re.captures(&style_str)?.get(1)?.as_str();

    let rgb = hex::decode(&rgb_str.as_bytes()).unwrap();
    let vals: Vec<f32> = rgb.into_iter().map(|v| v as f32 / 255.0).collect();

    let mut out = [1.0; 4];
    for i in 0..3 {
        out[i] = vals[i];
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let content = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="100%" height="100%" viewBox="0 0 300 300" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
    <path d="M49,144L141,146L152,173L172,187L207,191L241,186L255,165L259,147L292,147L289,268L42,274L49,144Z" style="fill:none;stroke:black;stroke-width:1px;"/>
</svg>"#;

        let level = super::Level::load_from_svg_str(content);

        insta::assert_debug_snapshot!(&level
            .triangles
            .iter()
            .map(|t| t.coords)
            .collect::<Vec<_>>());
    }
}
