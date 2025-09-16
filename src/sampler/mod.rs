
mod params;
pub use params::*;

use super::{msg::ERROR_OOB, texture::TextureObject};


/// wrapper over an OpenGL "Sampler Object" name.
#[derive(Debug)]
pub struct SamplerObject(u32);
impl SamplerObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

impl Drop for SamplerObject {
	fn drop(&mut self) {
		delete_samplers([
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		]);
	}
}

pub fn gen_samplers<const N: usize>() -> [SamplerObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenSamplers(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(|v| SamplerObject(v))
}

pub fn delete_samplers<const N: usize>(samplers: [SamplerObject; N]) {
	let list = samplers.map(|v| std::mem::ManuallyDrop::new(v).handle());
	unsafe {
		gl::DeleteSamplers(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

pub fn is_sampler(sampler: &SamplerObject) -> bool {
	unsafe {
		gl::IsSampler(sampler.handle()) == gl::TRUE
	}
}

pub fn bind_sampler(texture: &TextureObject, sampler: &SamplerObject) {
	unsafe {
		gl::BindSampler(texture.handle(), sampler.handle());
	}
}

pub fn unbind_sampler(texture: &TextureObject) {
	unsafe {
		gl::BindSampler(texture.handle(), 0);
	}
}

