
use std::ffi::c_void;

use crate::msg::ERROR_OOB;


/// wrapper over an OpenGL "Buffer Object" name.
#[derive(Debug)]
pub struct BufferObject(u32);
impl BufferObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

impl Drop for BufferObject {
	fn drop(&mut self) {
		delete_buffers([
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		]);
	}
}

pub fn gen_buffers<const N: usize>() -> [BufferObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenBuffers(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(BufferObject)
}

pub fn delete_buffers<const N: usize>(buffers: [BufferObject; N]) {
	let list = buffers.map(|v| std::mem::ManuallyDrop::new(v).handle());
	unsafe {
		gl::DeleteBuffers(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

pub fn is_buffer(buffer: &BufferObject) -> bool {
	unsafe {
		gl::IsBuffer(buffer.handle()) == gl::TRUE
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BindBufferTarget {
	ArrayBuffer = gl::ARRAY_BUFFER,
	CopyReadBuffer = gl::COPY_READ_BUFFER,
	CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
	ReadBuffer = gl::READ_BUFFER,
	ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
	PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
	PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
	TextureBuffer = gl::TEXTURE_BUFFER,
	TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
	UniformBuffer = gl::UNIFORM_BUFFER,
}

pub fn bind_buffer(target: BindBufferTarget, buffer: &BufferObject) {
	unsafe {
		gl::BindBuffer(target as u32, buffer.handle());
	}
}

pub fn unbind_buffer(target: BindBufferTarget) {
	unsafe {
		gl::BindBuffer(target as u32, 0);
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BufferDataUsage {
	StreamDraw = gl::STREAM_DRAW,
	/// GL_STREAM_READ
	StreamRead = gl::STREAM_READ,
	/// GL_STREAM_COPY
	StreamCopy = gl::STREAM_COPY,
	/// GL_STATIC_DRAW
	StaticDraw = gl::STATIC_DRAW,
	/// GL_STATIC_READ
	StaticRead = gl::STATIC_READ,
	/// GL_STATIC_COPY
	StaticCopy = gl::STATIC_COPY,
	/// GL_DYNAMIC_DRAW
	DynamicDraw = gl::DYNAMIC_DRAW,
	/// GL_DYNAMIC_READ
	DynamicRead = gl::DYNAMIC_READ,
	/// GL_DYNAMIC_COPY
	DynamicCopy = gl::DYNAMIC_COPY,
}

pub fn buffer_data<T>(target: BindBufferTarget, data: &[T], usage: BufferDataUsage) {
	unsafe {
		gl::BufferData(
			target as u32,
			size_of_val(data) as isize,
			data.as_ptr() as *const c_void,
			usage as u32,
		);
	}
}

