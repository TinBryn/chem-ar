pub mod error;

use std::{mem::size_of, num::ParseIntError, slice};

use crate::error::Error;

#[derive(Debug)]
pub struct Obj {
    data: Vec<[f32; 3]>,
}

fn lex_line(index: usize, line: &str) -> Result<(&str, Vec<&str>), error::Error> {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();

    match tokens[..] {
        [prefix, ..] => Ok((prefix, tokens[1..].to_owned())),
        _ => Err(error::Error::invalid(format!(
            "bad lex on line {}: {}",
            index, line
        ))),
    }
}

pub fn parse_obj(input: &str) -> Result<Obj, error::Error> {
    let mut data = vec![];

    let mut verticies: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];

    for (index, line) in input.lines().enumerate() {
        if !line.is_empty() {
            parse_line(index, line, &mut verticies, &mut normals, &mut data)?;
        }
    }
    Ok(Obj { data })
}

fn parse_line(
    index: usize,
    line: &str,
    verticies: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    data: &mut Vec<[f32; 3]>,
) -> Result<(), error::Error> {
    let line = line.trim();
    let (prefix, args) = lex_line(index, line)?;
    match prefix {
        "vn" => match args[..] {
            [_x, _y, _z] => {
                let coords: Result<Vec<f32>, _> = args.into_iter().map(|c| c.parse()).collect();
                if let [x, y, z] = coords.map_err(|err| {
                    error::Error::new(err.into(), format!("line {}, {}", index, line))
                })?[..]
                {
                    normals.push([x, y, z])
                }
                return Ok(());
            }
            _ => return Err(Error::invalid(format!("line {}, {:?}", index, line))),
        },
        "v" => match args[..] {
            [_, _, _] => {
                let coords: Result<Vec<f32>, _> = args.into_iter().map(|c| c.parse()).collect();
                if let [x, y, z] = coords.map_err(|err| {
                    error::Error::new(err.into(), format!("line {}, {}", index, line))
                })?[..]
                {
                    verticies.push([x, y, z])
                }
                return Ok(());
            }
            _ => return Err(Error::invalid(format!("line {}, {}", index, line))),
        },
        "f" => match args[..] {
            [_, _, _] | [_, _, _, _] => {
                let coords: Result<Vec<Vec<_>>, _> = args
                    .into_iter()
                    .filter_map(|c| -> Option<Result<Vec<usize>, ParseIntError>> {
                        c.split('/')
                            .map(|i| {
                                if i.is_empty() {
                                    Some(Ok(usize::MAX))
                                } else {
                                    Some(i.parse::<usize>())
                                }
                            })
                            .collect()
                    })
                    .collect();
                let coords: Vec<_> = coords.map_err(|err| {
                    error::Error::new(err.into(), format!("line {}, {}", index, line))
                })?;

                if coords.len() == 3 {
                    for id in [0, 2, 1] {
                        data.push(verticies[coords[id][0] - 1]);
                        data.push(normals[coords[id][2] - 1]);
                    }
                } else if coords.len() == 4 {
                    for id in [0, 2, 1, 3, 2, 0] {
                        data.push(verticies[coords[id][0] - 1]);
                        data.push(normals[coords[id][2] - 1]);
                    }
                }
                return Ok(());
            }
            _ => return Err(Error::invalid(format!("line {}, {}", index, line))),
        },
        _ => return Ok(()), // ignore any other lines
    }
}

impl Obj {
    pub fn as_slice(&self) -> &[f32] {
        let vec_size = size_of::<[f32; 3]>();
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
    use crate::parse_line;

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
