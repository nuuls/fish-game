use crate::game::Triangle;
use delaunator::{triangulate, Point};
use svg;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::parser::Event;

pub struct Level {
    // vertices: Vec<f64>,
    // indices: Vec<u16>,
    triangles: Vec<Triangle>,
}

impl Level {
    pub fn load_from_svg(content: &str) -> Level {
        let points: Vec<Point> = Vec::new();
        let parser = svg::read(content).unwrap();

        for event in parser {
            match event {
                Event::Tag("path", _, attributes) => {
                    // println!(
                    //     "Path: {:?} Type: {:?} Attributes: {:?}",
                    //     path, type_, attributes
                    // );

                    let data = attributes.get("d").unwrap();
                    let data = Data::parse(data).unwrap();
                    // for command in data.iter() {
                    //     match command {
                    //         &Command::Move(position, parameters) => {
                    //             let Parameters(xD) = parameters;

                    //             println!("Move: {}, {}", x, y);
                    //             points.push(Point { x, y });
                    //         }
                    //         &Command::Line(position, parameters) => {
                    //             println!("Line: {}, {}", x, y);
                    //             points.push(Point { x, y });
                    //         }
                    //         _ => {}
                    //     }
                    // }
                }
                _ => {}
            }
        }

        // let mut indices = Vec::new();

        // Level { vertices, indices }
        Level { triangles: vec![] }
    }

    pub fn triangles(&self) -> Vec<Triangle> {
        vec![]
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

        let level = super::Level::load_from_svg(content);
    }
}
