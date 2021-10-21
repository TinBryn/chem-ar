use std::{mem::size_of, slice};

#[derive(Debug)]
pub struct Obj {
    #[allow(dead_code)]
    data: Vec<gfx_maths::Vec3>,
}

#[allow(dead_code)]
pub fn parse_obj(input: &str) -> Option<Obj> {
    let mut data = vec![];

    let mut verticies: Vec<gfx_maths::Vec3> = vec![];
    let mut normals: Vec<gfx_maths::Vec3> = vec![];

    for line in input.lines() {
        parse_line(line, &mut verticies, &mut normals, &mut data);
    }
    Some(Obj { data })
}

fn parse_line(
    line: &str,
    verticies: &mut Vec<gfx_maths::Vec3>,
    normals: &mut Vec<gfx_maths::Vec3>,
    data: &mut Vec<gfx_maths::Vec3>,
) {
    let line = line.trim();
    if let Some(stripped) = line.strip_prefix("vn ") {
        let coords = stripped
            .trim()
            .split_ascii_whitespace()
            .map(|c| c.parse().expect("could not parse normal"))
            .collect::<Vec<f32>>();
        assert_eq!(coords.len(), 3);

        normals.push(gfx_maths::Vec3::new(coords[0], coords[1], coords[2]));
    } else if let Some(stripped) = line.strip_prefix("v ") {
        let coords = stripped
            .trim()
            .split_ascii_whitespace()
            .map(|c| c.parse().unwrap())
            .collect::<Vec<f32>>();
        assert_eq!(coords.len(), 3);

        verticies.push(gfx_maths::Vec3::new(coords[0], coords[1], coords[2]));
    } else if let Some(stripped) = line.strip_prefix("f ") {
        let coords = stripped
            .trim()
            .split_ascii_whitespace()
            .map(|c| {
                c.split('/')
                    .map(|i| {
                        if i.is_empty() {
                            usize::MAX
                        } else {
                            i.parse::<usize>().unwrap() - 1
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if coords.len() == 3 {
            for id in [0, 2, 1] {
                data.push(verticies[coords[id][0]]);
                data.push(normals[coords[id][2]]);
            }
        } else if coords.len() == 4 {
            for id in [0, 2, 1, 3, 2, 0] {
                data.push(verticies[coords[id][0]]);
                data.push(normals[coords[id][2]]);
            }
        }
    }
}

impl Obj {
    pub fn as_slice(&self) -> &[f32] {
        let vec_size = size_of::<gfx_maths::Vec3>();
        let float_size = size_of::<f32>();
        assert_eq!(vec_size % float_size, 0);
        let len = self.data.len() * vec_size / float_size;
        let ptr = self.data.as_ptr() as *const f32;

        // # Safety: Vec3 is #[repr(C)] so it is known that the f32s are layed out packed and 3 times the length of the containing Vec
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

#[cfg(test)]
mod test {
    use crate::parsers::parse_line;

    const CUBE: &str = include_str!("../models/cube.obj");

    #[test]
    fn parse_vertex() {
        let mut v = vec![];
        let mut n = vec![];
        let mut f = vec![];

        parse_line("v -1.0 -1.0 -1.0", &mut v, &mut n, &mut f);

        assert_eq!(v[0], gfx_maths::Vec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn parse_cube() {
        let _obj = crate::parsers::parse_obj(CUBE);

        println!("{:#?}", _obj);
    }
}
