pub const CUBE_VERT: &str = include_str!("../shaders/cube.vert");
pub const CUBE_FRAG: &str = include_str!("../shaders/cube.frag");

use std::mem::size_of;

use gfx_maths::{Mat4, Quaternion, Vec3};
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlUniformLocation};
use WebGl2RenderingContext as GL;

use crate::app_state;

pub struct Cube {
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    u_transform: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    array_len: usize,
}

#[allow(dead_code)]
impl Cube {
    const CUBE: &'static str = include_str!("../models/cube.obj");

    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let program = ::shaders::link_program(gl, CUBE_VERT, CUBE_FRAG).unwrap();

        let obj = obj_parser::parse_obj(Self::CUBE).unwrap();

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let verticies = obj.as_slice();

        let vertices_location = verticies.as_ptr() as u32 / size_of::<f32>() as u32;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + verticies.len() as u32,
        );

        let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let u_transform = gl.get_uniform_location(&program, "uTransform").unwrap();
        let u_projection = gl.get_uniform_location(&program, "uProjection").unwrap();

        Self {
            program,
            vertex_buffer,
            u_projection,
            u_transform,
            array_len: verticies.len(),
        }
    }

    pub fn render(
        &self,
        gl: &WebGl2RenderingContext,
        canvas: app_state::Canvas,
        angles: app_state::Angles,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));

        // setup position attribute
        configure_vec3_attribute(gl, 0, 0);

        // setup normal attribute
        configure_vec3_attribute(gl, 1, 3);

        let rotation_x = Quaternion::axis_angle(Vec3::new(1.0, 0.0, 0.0), angles.x);
        let rotation_y = Quaternion::axis_angle(Vec3::new(0.0, 1.0, 0.0), angles.y);
        let rotation_z = Quaternion::axis_angle(Vec3::new(0.0, 0.0, 1.0), angles.z);
        let rotation = rotation_x * rotation_y * rotation_z;

        let rotation = Mat4::rotate(rotation);
        let translation = Mat4::translate(Vec3::new(0.0, 0.0, 5.0));
        let scale = Mat4::scale(Vec3::new(2.0, 2.0, 2.0));

        let transform = translation * scale * rotation;

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform.values);

        let aspect_ratio = canvas.width / canvas.height;
        let unit_dist_fov = 1.5f32
            * if canvas.width > canvas.height {
                1.0
            } else {
                1.0 / aspect_ratio
            };

        /*
        The unit_dist_fov is already exactly what I want the perspective to use and in the
        coordinate space that makes things easy to work with. But the perspective method
        does some maths on its arguments to make it "user friendly". So I need to do some
        calculations that are as complicated as what it's trying to hide from me, but in
        effect it forces me to do it when in a good abstraction neither I nor the library
        needed to do anything this complicated at all.
         */
        let fov_rad = unit_dist_fov.atan() * 2.0;

        let projection = Mat4::perspective_opengl(fov_rad, 0.1, 10.0, aspect_ratio);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &projection.values);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.array_len / 6) as i32);
    }
}

fn configure_vec3_attribute(gl: &GL, indx: u32, offset: i32) {
    gl.vertex_attrib_pointer_with_i32(
        indx,
        3,
        GL::FLOAT,
        false,
        6 * size_of::<f32>() as i32,
        offset * size_of::<f32>() as i32,
    );
    gl.enable_vertex_attrib_array(indx);
}
