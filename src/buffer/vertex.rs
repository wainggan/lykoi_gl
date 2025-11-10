
use std::ffi::c_void;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VertexAttribPointerType {
	/// GL_BYTE
	Byte = gl::BYTE,
	/// GL_UNSIGNED_BYTE
	UnsignedByte = gl::UNSIGNED_BYTE,
	/// GL_SHORT
	Short = gl::SHORT,
	/// GL_UNSIGNED_SHORT
	UnsignedShort = gl::UNSIGNED_SHORT,
	/// GL_INT
	Int = gl::INT,
	/// GL_UNSIGNED_INT
	UnsignedInt = gl::UNSIGNED_INT,
	/// GL_HALF_FLOAT
	HalfFloat = gl::HALF_FLOAT,
	/// GL_FLOAT
	Float = gl::FLOAT,
	/// GL_DOUBLE
	Double = gl::DOUBLE,
	/// GL_FIXED
	Fixed = gl::FIXED,
}

pub fn vertex_attrib_pointer(
	target: u32,
	size: u8,
	kind: VertexAttribPointerType,
	normalized: bool,
	stride: u32,
	offset: u32,
) {
	assert!(matches!(size, 1..=4));
	unsafe {
		gl::VertexAttribPointer(
			target,
			size as i32,
			kind as u32,
			normalized as u8,
			stride as i32,
			offset as *const c_void,
		);
	}
}

pub fn enable_vertex_attrib_array(index: u32) {
	unsafe {
		gl::EnableVertexAttribArray(index);
	}
}



