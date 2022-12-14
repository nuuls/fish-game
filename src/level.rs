use crate::game::Triangle;
use crate::log;
use svg;
use svg::node::element::path::{Command, Data, Position};
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

        for event in parser {
            match event {
                Event::Tag("path", _, attributes) => {
                    let data = attributes.get("d").unwrap();
                    let data = Data::parse(data).unwrap();

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
                _ => {}
            }
        }

        let mut triangles: Vec<Triangle> = vec![];

        for path in points.iter() {
            // not even a triangle
            if path.len() < 5 {
                continue;
            }

            let path_triangles = earcutr::earcut(path.as_slice(), &vec![], 2);

            for [a, b, c] in path_triangles.array_chunks::<3>() {
                triangles.push([
                    path[*a * 2] as f32,
                    path[(*a + 1) * 2] as f32,
                    0.0 as f32,
                    path[*b * 2] as f32,
                    path[(*b + 1) * 2] as f32,
                    0.0 as f32,
                    path[*c * 2] as f32,
                    path[(*c + 1) * 2] as f32,
                    0.0 as f32,
                ]);
            }
        }

        Level { triangles }
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
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

        insta::assert_debug_snapshot!(&level.triangles);
    }
}
