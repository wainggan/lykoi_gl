
use super::SamplerObject;


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SamplerParameterName {
	TextureBorderColor = gl::TEXTURE_BORDER_COLOR,
	TextureCompareMode = gl::TEXTURE_COMPARE_MODE,
	TextureCompareFunc = gl::TEXTURE_COMPARE_FUNC,
	TextureMinLod = gl::TEXTURE_MIN_LOD,
	TextureMaxLod = gl::TEXTURE_MAX_LOD,
	TextureLodBias = gl::TEXTURE_LOD_BIAS,
	TextureMinFilter = gl::TEXTURE_MIN_FILTER,
	TextureMagFilter = gl::TEXTURE_MAG_FILTER,
	TextureWrapS = gl::TEXTURE_WRAP_S,
	TextureWrapT = gl::TEXTURE_WRAP_T,
	TextureWrapR = gl::TEXTURE_WRAP_R,
}

pub unsafe fn sampler_parameter_i(sampler: &SamplerObject, name: SamplerParameterName, value: i32) {
	unsafe {
		gl::SamplerParameteri(sampler.handle(), name as u32, value);
	}
}

pub unsafe fn sampler_parameter_f(sampler: &SamplerObject, name: SamplerParameterName, value: f32) {
	unsafe {
		gl::SamplerParameterf(sampler.handle(), name as u32, value);
	}
}

pub unsafe fn sampler_parameter_iv(sampler: &SamplerObject, name: SamplerParameterName, value: &[i32]) {
	let mut params = [0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::SamplerParameteriv(sampler.handle(), name as u32, params.as_ptr());
	}
}

pub unsafe fn sampler_parameter_fv(sampler: &SamplerObject, name: SamplerParameterName, value: &[f32]) {
	let mut params = [0.0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::SamplerParameterfv(sampler.handle(), name as u32, params.as_ptr());
	}
}

pub unsafe fn sampler_parameter_i_iv(sampler: &SamplerObject, name: SamplerParameterName, value: &[i32]) {
	let mut params = [0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::SamplerParameterIiv(sampler.handle(), name as u32, params.as_ptr());
	}
}

pub unsafe fn sampler_parameter_i_uiv(sampler: &SamplerObject, name: SamplerParameterName, value: &[u32]) {
	let mut params = [0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::SamplerParameterIuiv(sampler.handle(), name as u32, params.as_ptr());
	}
}




