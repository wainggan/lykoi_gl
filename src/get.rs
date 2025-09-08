
/// [`glGetIntegerv(GL_ACTIVE_TEXTURE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_active_texture() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::ACTIVE_TEXTURE, &raw mut data);
	}
	data as u32
}

/// [`glGetFloatv(GL_ALIASED_LINE_WIDTH_RANGE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_aliased_line_width_range() -> (f32, f32) {
	let mut data = [0.0; 2];
	unsafe {
		gl::GetFloatv(gl::ALIASED_LINE_WIDTH_RANGE, data.as_mut_ptr());
	}
	data.into()
}

/// [`glGetFloatv(GL_SMOOTH_LINE_WIDTH_RANGE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_smooth_line_width_range() -> (f32, f32) {
	let mut data = [0.0; 2];
	unsafe {
		gl::GetFloatv(gl::SMOOTH_LINE_WIDTH_RANGE, data.as_mut_ptr());
	}
	data.into()
}

/// [`glGetFloatv(GL_SMOOTH_LINE_WIDTH_GRANULARITY, ...)`](https://docs.gl/gl3/glGet)
pub fn get_smooth_line_width_granularity() -> f32 {
	let mut data = 0.0;
	unsafe {
		gl::GetFloatv(gl::SMOOTH_LINE_WIDTH_GRANULARITY, &raw mut data);
	}
	data
}

/// [`glGetIntegerv(GL_ARRAY_BUFFER_BINDING, ...)`](https://docs.gl/gl3/glGet)
pub fn get_array_buffer_binding() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &raw mut data);
	}
	data as u32
}

/// [`glGetBooleanv(GL_BLEND, ...)`](https://docs.gl/gl3/glGet)
pub fn get_blend() -> bool {
	let mut data = 0;
	unsafe {
		gl::GetBooleanv(gl::BLEND, &raw mut data);
	}
	data == gl::TRUE
}

/// [`glGetFloatv(GL_BLEND_COLOR, ...)`](https://docs.gl/gl3/glGet)
pub fn get_blend_color() -> (f32, f32, f32, f32) {
	let mut data = [0.0; 4];
	unsafe {
		gl::GetFloatv(gl::BLEND_COLOR, data.as_mut_ptr());
	}
	data.into()
}

