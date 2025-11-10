
use crate::msg::ERROR_OOB;


/// wrapper over an OpenGL "Renderbuffer Object" name.
#[derive(Debug)]
pub struct RenderbufferObject(u32);
impl RenderbufferObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

impl Drop for RenderbufferObject {
	fn drop(&mut self) {
		delete_renderbuffers([
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		]);
	}
}

/// [`glGenRenderbuffers()`](https://docs.gl/gl3/glGenRenderbuffers)
pub fn gen_renderbuffers<const N: usize>() -> [RenderbufferObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenRenderbuffers(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(RenderbufferObject)
}

/// [`glDeleteRenderbuffers()`](https://docs.gl/gl3/glDeleteRenderbuffers)
pub fn delete_renderbuffers<const N: usize>(renderbuffers: [RenderbufferObject; N]) {
	let list = renderbuffers.map(|v| std::mem::ManuallyDrop::new(v).handle());
	unsafe {
		gl::DeleteRenderbuffers(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

/// [`glIsRenderbuffer()`](https://docs.gl/gl3/glIsRenderbuffer)
pub fn is_renderbuffer(renderbuffer: &RenderbufferObject) -> bool {
	unsafe {
		gl::IsRenderbuffer(renderbuffer.handle()) == gl::TRUE
	}
}

/// [`glBindRenderbuffer()`](https://docs.gl/gl3/glBindRenderbuffer)
pub fn bind_renderbuffer(renderbuffer: &RenderbufferObject) {
	unsafe {
		gl::BindRenderbuffer(gl::RENDERBUFFER, renderbuffer.handle());
	}
}

/// [`glBindRenderbuffer()`](https://docs.gl/gl3/glBindRenderbuffer)
pub fn unbind_renderbuffer() {
	unsafe {
		gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
	}
}

/// [`glRenderbufferStorage()`](https://docs.gl/gl3/glRenderbufferStorage)
pub fn renderbuffer_storage(format: crate::TexImageInnerFormat, width: usize, height: usize) {
	debug_assert!(width <= crate::get_max_renderbuffer_size() as usize);

	debug_assert!(height <= crate::get_max_renderbuffer_size() as usize);
	
	unsafe {
		gl::RenderbufferStorage(
			gl::RENDERBUFFER,
			format as u32,
			width as i32,
			height as i32,
		);
	}
}

/// [`glFramebufferRenderbuffer()`](https://docs.gl/gl3/glFramebufferRenderbuffer)
pub fn framebuffer_renderbuffer(
	target: crate::FramebufferTarget,
	attachment: crate::FramebufferAttachment,
	renderbuffer: &RenderbufferObject,
) {
	unsafe {
		gl::FramebufferRenderbuffer(
			target as u32,
			attachment.out(),
			gl::RENDERBUFFER,
			renderbuffer.handle(),
		);
	}
}

