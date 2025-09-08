
use crate::msg::ERROR_OOB;

use super::framebuffer::FramebufferTarget;


/// wrapper over an OpenGL "Renderbuffer Object" name.
#[derive(Debug)]
pub struct RenderbufferObject(u32);
impl RenderbufferObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

pub fn gen_renderbuffers<const N: usize>() -> [RenderbufferObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenRenderbuffers(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(|v| RenderbufferObject(v))
}

pub fn delete_renderbuffers<const N: usize>(renderbuffers: [RenderbufferObject; N]) {
	let list = renderbuffers.map(|v| v.handle());
	unsafe {
		gl::DeleteRenderbuffers(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

pub fn is_renderbuffer(renderbuffer: &RenderbufferObject) -> bool {
	unsafe {
		gl::IsRenderbuffer(renderbuffer.handle()) == gl::TRUE
	}
}

pub fn bind_renderbuffer(renderbuffer: &RenderbufferObject) {
	unsafe {
		gl::BindRenderbuffer(gl::RENDERBUFFER, renderbuffer.handle());
	}
}

pub fn unbind_renderbuffer() {
	unsafe {
		gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
	}
}

// todo: enum
pub fn renderbuffer_storage(format: u32, width: usize, height: usize) {
	unsafe {
		gl::RenderbufferStorage(
			gl::RENDERBUFFER,
			format,
			width as i32,
			height as i32,
		);
	}
}

// todo: enum
pub fn framebuffer_renderbuffer(
	target: FramebufferTarget,
	attachment: u32,
	renderbuffer: &RenderbufferObject,
) {
	unsafe {
		gl::FramebufferRenderbuffer(
			target as u32,
			attachment,
			gl::RENDERBUFFER,
			renderbuffer.handle(),
		);
	}
}

