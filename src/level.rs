use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::types::{Color, Entity, ShaderId, Triangle};
use regex::Regex;
use svg;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::Style;
use svg::parser::Event;

pub struct Level {
    id: String,
    triangles: Vec<Triangle>,
    player_pos: (f32, f32),
    ground: (f32, f32, f32, f32),
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
        // points, color, wireframe
        let polygons: RefCell<Vec<(Vec<f32>, Color, bool)>> =
            RefCell::new(vec![(vec![0.0, 0.0], [1.0, 0.0, 1.0, 1.0], false)]);
        let mut player_pos = (0.0, 0.0);
        let mut hitbox = (0.0, 0.0, 0.0, 0.0);

        for event in parser {
            match event {
                Event::Tag("path", _, attributes) => {
                    let data = attributes.get("d").unwrap();
                    let data = Data::parse(data).unwrap();

                    let color = color_from_style(
                        attributes.get("style").map(|v| Style::new(v.to_string())),
                    )
                    .unwrap_or([1.0, 0.0, 1.0, 1.0]);

                    let current_pos = RefCell::new((0.0, 0.0));

                    let push_point = |position: Position, x: Option<f32>, y: Option<f32>| {
                        let p =
                            update_point(current_pos.borrow_mut().borrow_mut(), position, (x, y));

                        if let Some(polygon) = polygons.borrow_mut().last_mut() {
                            polygon.0.push(p.0);
                            polygon.0.push(p.1);
                        };
                    };

                    for command in data.iter() {
                        match command {
                            Command::Move(position, parameters) => {
                                let mut point_it = parameters.array_chunks::<2>();

                                if let Some([x, y]) = point_it.next() {
                                    let mut current_pos = current_pos.borrow_mut();

                                    update_point(
                                        current_pos.borrow_mut(),
                                        *position,
                                        (Some(*x), Some(*y)),
                                    );
                                    polygons.borrow_mut().push((
                                        vec![current_pos.0, current_pos.1],
                                        color,
                                        false,
                                    ));
                                }

                                for [x, y] in point_it {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::Line(position, parameters) => {
                                for [x, y] in parameters.array_chunks::<2>() {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::HorizontalLine(position, parameters) => {
                                for x in parameters.iter() {
                                    push_point(*position, Some(*x), None);
                                }
                            }
                            Command::VerticalLine(position, parameters) => {
                                for y in parameters.iter() {
                                    push_point(*position, None, Some(*y));
                                }
                            }
                            Command::CubicCurve(position, parameters) => {
                                for [_x1, _y1, _x2, _y2, x, y] in parameters.array_chunks::<6>() {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::SmoothCubicCurve(position, parameters) => {
                                for [_x1, _x2, x, y] in parameters.array_chunks::<4>() {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::QuadraticCurve(position, parameters) => {
                                for [_x1, _y1, x, y] in parameters.array_chunks::<4>() {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::SmoothQuadraticCurve(position, parameters) => {
                                for [x, y] in parameters.array_chunks::<2>() {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::EllipticalArc(position, parameters) => {
                                for [_rx, _ry, _x_axis_rotation, _large_arc_flag, _sweep_flag, x, y] in
                                    parameters.array_chunks::<7>()
                                {
                                    push_point(*position, Some(*x), Some(*y));
                                }
                            }
                            Command::Close => {
                                polygons.borrow_mut().push((
                                    vec![0.0, 0.0],
                                    [1.0, 0.0, 1.0, 1.0],
                                    false,
                                ));
                            }
                        }
                    }
                }
                Event::Tag("rect", _, attributes) => {
                    let x = attributes.get("x").unwrap().parse::<f32>().unwrap();
                    let y = attributes.get("y").unwrap().parse::<f32>().unwrap();
                    let width = attributes.get("width").unwrap().parse::<f32>().unwrap();
                    let height = attributes.get("height").unwrap().parse::<f32>().unwrap();

                    let color = color_from_style(
                        attributes.get("style").map(|v| Style::new(v.to_string())),
                    )
                    .unwrap_or([1.0, 0.0, 1.0, 1.0]);

                    let mut wireframe = false;

                    if let Some(id) = attributes.get("id").map(|v| v.to_string()) {
                        if id == "player" {
                            player_pos = (x, y);
                            wireframe = true;
                        } else if id.starts_with("hitbox") {
                            hitbox = (x, y, width, height);
                            wireframe = true;
                        }
                    }

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

                    polygons.borrow_mut().push((path, color, wireframe));
                }
                _ => {}
            }
        }

        let mut triangles: Vec<Triangle> = vec![];

        for (points, color, wireframe) in polygons.borrow().iter() {
            // not even a triangle
            if points.len() < 6 {
                continue;
            }

            let path_triangles = earcutr::earcut(points.as_slice(), &vec![], 2);

            for [a, b, c] in path_triangles.array_chunks::<3>() {
                let mut triangle = Triangle {
                    coords: [
                        points[*a * 2] as f32,
                        points[*a * 2 + 1] as f32,
                        0.0 as f32,
                        points[*b * 2] as f32,
                        points[*b * 2 + 1] as f32,
                        0.0 as f32,
                        points[*c * 2] as f32,
                        points[*c * 2 + 1] as f32,
                        0.0 as f32,
                    ],
                    color: *color,
                    shader_id: ShaderId::Default,
                    wireframe: *wireframe,
                };

                if triangle.color[0] < 0.0001 && triangle.color[1] < triangle.color[2] {
                    triangle.shader_id = ShaderId::Water;
                }

                triangles.push(triangle);
            }
        }

        Level {
            id: "level".to_string(),
            triangles,
            player_pos,
            ground: hitbox,
        }
    }

    pub fn player_pos(&self) -> (f32, f32) {
        self.player_pos
    }

    pub fn ground(&self) -> (f32, f32, f32, f32) {
        self.ground
    }
}

impl Entity for Level {
    fn id(&self) -> &String {
        &self.id
    }
    fn triangles(&self) -> &Vec<Triangle> {
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

    #[test]
    fn bigger_example() {
        let content = include_str!("../test_data/map1.svg");

        let level = super::Level::load_from_svg_str(content);

        insta::assert_debug_snapshot!(&level
            .triangles
            .iter()
            .map(|t| t.coords)
            .collect::<Vec<_>>());
    }
}
