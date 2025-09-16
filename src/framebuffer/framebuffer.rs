
use crate::{msg::ERROR_OOB, texture::TextureObject};


/// wrapper over an OpenGL "Framebuffer Object" name.
#[derive(Debug)]
pub struct FramebufferObject(u32);
impl FramebufferObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}
impl Drop for FramebufferObject {
	fn drop(&mut self) {
		delete_framebuffers([unsafe { std::ptr::read(self) }]);
	}
}

/// https://docs.gl/gl3/glGenFramebuffers
pub fn gen_framebuffers<const N: usize>() -> [FramebufferObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenFramebuffers(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(|v| FramebufferObject(v))
}

/// https://docs.gl/gl3/glDeleteFramebuffers
pub fn delete_framebuffers<const N: usize>(framebuffers: [FramebufferObject; N]) {
	// manually drop to avoid double free
	let list = framebuffers.map(|v| std::mem::ManuallyDrop::new(v).handle());
	unsafe {
		gl::DeleteFramebuffers(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

/// https://docs.gl/gl3/glIsFramebuffers
pub fn is_framebuffer(framebuffer: &FramebufferObject) -> bool {
	unsafe {
		gl::IsFramebuffer(framebuffer.handle()) == gl::TRUE
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FramebufferTarget {
	Framebuffer = gl::FRAMEBUFFER,
	ReadFramebuffer = gl::READ_FRAMEBUFFER,
	DrawFramebuffer = gl::DRAW_FRAMEBUFFER,
}

pub fn bind_framebuffer(target: FramebufferTarget, framebuffer: &FramebufferObject) {
	unsafe {
		gl::BindFramebuffer(target as u32, framebuffer.handle());
	}
}

pub fn unbind_framebuffer(target: FramebufferTarget) {
	unsafe {
		gl::BindFramebuffer(target as u32, 0);
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FramebufferStatusType {
	FramebufferComplete = gl::FRAMEBUFFER_COMPLETE,
}

// todo: return enum
pub fn check_framebuffer_status(target: FramebufferTarget) -> bool {
	let out = unsafe { gl::CheckFramebufferStatus(target as u32) };
	out == gl::FRAMEBUFFER_COMPLETE
}

#[repr(u32)]
pub enum FramebufferAttachment {
	ColorAttachment = gl::COLOR_ATTACHMENT0,
}

// todo: PLEASE do something about the enum problem
pub fn framebuffer_texture_2d(
	target: FramebufferTarget,
	attachment: FramebufferAttachment,
	textarget: u32,
	texture: &TextureObject,
) {
	unsafe {
		gl::FramebufferTexture2D(
			target as u32,
			attachment as u32,
			textarget,
			texture.handle(),
			0,
		);
	}
}

