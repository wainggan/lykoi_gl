
use std::{ffi::CString, marker::PhantomData};

use super::program::ProgramObject;

/// location of a uniform, for type safety.
#[derive(Debug)]
pub struct UniformLocation<'a>(u32, PhantomData<&'a ()>);
impl<'a> UniformLocation<'a> {
	pub fn get(&self) -> u32 {
		self.0
	}
}

pub fn get_uniform_location<'a>(
	program: &'a ProgramObject,
	name: &str
) -> Option<UniformLocation<'a>> {
	let input_name = CString::new(name).unwrap();
	let out = unsafe {
		gl::GetUniformLocation(
			program.handle(),
			input_name.as_ptr(),
		)
	};
	if out == -1 {
		None
	} else {
		Some(UniformLocation(out as u32, PhantomData))
	}
}

pub fn uniform_1d(uniform: &UniformLocation, x: f64) {
	unsafe {
		gl::Uniform1d(uniform.get() as i32, x);
	}
}

pub fn uniform_1f(uniform: &UniformLocation, x: f32) {
	unsafe {
		gl::Uniform1f(uniform.get() as i32, x);
	}
}

pub fn uniform_1i(uniform: &UniformLocation, x: i32) {
	unsafe {
		gl::Uniform1i(uniform.get() as i32, x);
	}
}

pub fn uniform_2f(uniform: &UniformLocation, x: f32, y: f32) {
	unsafe {
		gl::Uniform2f(uniform.get() as i32, x, y);
	}
}

pub fn uniform_3f(uniform: &UniformLocation, x: f32, y: f32, z: f32) {
	unsafe {
		gl::Uniform3f(uniform.get() as i32, x, y, z);
	}
}

pub fn uniform_4f(uniform: &UniformLocation, x: f32, y: f32, z: f32, w: f32) {
	unsafe {
		gl::Uniform4f(uniform.get() as i32, x, y, z, w);
	}
}

// is there a better api we can use?
pub fn uniform_matrix_4fv(uniform: &UniformLocation, transpose: bool, value: &[f32; 16]) {
	// TODO: array of matrix support
	unsafe {
		gl::UniformMatrix4fv(
			uniform.get() as i32,
			1,
			transpose as u8,
			value.as_ptr()
		);
	}
}

