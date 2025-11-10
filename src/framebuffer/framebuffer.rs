
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
		delete_framebuffers([
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		]);
	}
}

/// https://docs.gl/gl3/glGenFramebuffers
pub fn gen_framebuffers<const N: usize>() -> [FramebufferObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenFramebuffers(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(FramebufferObject)
}

/// https://docs.gl/gl3/glDeleteFramebuffers
pub fn delete_framebuffers<const N: usize>(framebuffers: [FramebufferObject; N]) {
	// manually drop to avoid double free
	let list = framebuffers.map(|v| std::mem::ManuallyDrop::new(v).handle());
	unsafe {
		gl::DeleteFramebuffers(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

/// [`glIsFramebuffer()`](https://docs.gl/gl3/glIsFramebuffer)
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

/// [`glBindFramebuffer()`](https://docs.gl/gl3/glBindFramebuffer)
pub fn bind_framebuffer(target: FramebufferTarget, framebuffer: &FramebufferObject) {
	unsafe {
		gl::BindFramebuffer(target as u32, framebuffer.handle());
	}
}

/// [`glBindFramebuffer()`](https://docs.gl/gl3/glBindFramebuffer)
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

pub enum FramebufferAttachment {
	ColorAttachment(u8),
	DepthAttachment,
	StencilAttachment,
	DepthStencilAttachment,
}
impl FramebufferAttachment {
	pub(crate) fn out(self) -> u32 {
		match self {
			Self::ColorAttachment(x) => {
				debug_assert!(
					(x as u32) < crate::get_max_color_attachments(),
					"color attachment index must be less than {}",
					crate::get_max_color_attachments(),
				);
				gl::COLOR_ATTACHMENT0 + x as u32
			}
			Self::DepthAttachment => gl::DEPTH_ATTACHMENT,
			Self::StencilAttachment => gl::STENCIL_ATTACHMENT,
			Self::DepthStencilAttachment => gl::DEPTH_STENCIL_ATTACHMENT,
		}
	}
}

/// [`glFramebufferTexture1D()`](https://docs.gl/gl3/glFramebufferTexture)
pub fn framebuffer_texture_1d(
	target: FramebufferTarget,
	attachment: FramebufferAttachment,
	textarget: crate::TexImage1DTarget,
	texture: Option<&TextureObject>,
	level: u16,
) {
	// todo: are these validations correct?
	
	debug_assert!(
		if texture.is_some() {
			matches!(textarget, crate::TexImage1DTarget::Texture1D)
		} else {
			true
		}
	);

	unsafe {
		gl::FramebufferTexture1D(
			target as u32,
			attachment.out(),
			textarget as u32,
			texture.map(|x| x.handle()).unwrap_or(0),
			level as i32,
		);
	}
}


/// [`glFramebufferTexture2D()`](https://docs.gl/gl3/glFramebufferTexture)
pub fn framebuffer_texture_2d(
	target: FramebufferTarget,
	attachment: FramebufferAttachment,
	textarget: crate::TexImage2DTarget,
	texture: Option<&TextureObject>,
	level: u16,
) {
	// todo: are these validations correct?

	debug_assert!(
		if texture.is_some() {
			matches!(
				textarget,
				| crate::TexImage2DTarget::Texture2D
				| crate::TexImage2DTarget::TextureRectangle
				| crate::TexImage2DTarget::TextureCubeMapNegativeX
				| crate::TexImage2DTarget::TextureCubeMapPositiveX
				| crate::TexImage2DTarget::TextureCubeMapNegativeY
				| crate::TexImage2DTarget::TextureCubeMapPositiveY
				| crate::TexImage2DTarget::TextureCubeMapNegativeZ
				| crate::TexImage2DTarget::TextureCubeMapPositiveZ
			)
		} else {
			true
		}
	);

	debug_assert!(
		if matches!(textarget, crate::TexImage2DTarget::TextureRectangle) {
			level == 0
		} else {
			true
		},
		"level must be 0 if textarget is TextureRectangle",
	);

	debug_assert!(
		match textarget {
			| crate::TexImage2DTarget::TextureCubeMapNegativeX
			| crate::TexImage2DTarget::TextureCubeMapPositiveX
			| crate::TexImage2DTarget::TextureCubeMapNegativeY
			| crate::TexImage2DTarget::TextureCubeMapPositiveY
			| crate::TexImage2DTarget::TextureCubeMapNegativeZ
			| crate::TexImage2DTarget::TextureCubeMapPositiveZ => {
				(level as u32) <= crate::get_max_cube_map_texture_size().ilog2()
			}
			_ => {
				(level as u32) <= crate::get_max_texture_size().ilog2()
			}
		}
	);

	unsafe {
		gl::FramebufferTexture2D(
			target as u32,
			attachment.out(),
			textarget as u32,
			texture.map(|x| x.handle()).unwrap_or(0),
			level as i32,
		);
	}
}

