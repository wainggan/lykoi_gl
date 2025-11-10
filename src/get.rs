
/// current active texture unit.
/// 
/// [`glGetIntegerv(GL_ACTIVE_TEXTURE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_active_texture() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::ACTIVE_TEXTURE, &raw mut data);
	}
	data as u32
}

/// range of widths supported for aliased lines.
/// 
/// [`glGetFloatv(GL_ALIASED_LINE_WIDTH_RANGE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_aliased_line_width_range() -> (f32, f32) {
	let mut data = [0.0; 2];
	unsafe {
		gl::GetFloatv(gl::ALIASED_LINE_WIDTH_RANGE, data.as_mut_ptr());
	}
	data.into()
}

/// range of widths supported for antialiased lines.
/// 
/// [`glGetFloatv(GL_SMOOTH_LINE_WIDTH_RANGE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_smooth_line_width_range() -> (f32, f32) {
	let mut data = [0.0; 2];
	unsafe {
		gl::GetFloatv(gl::SMOOTH_LINE_WIDTH_RANGE, data.as_mut_ptr());
	}
	data.into()
}

/// level of quantization applied to antialiased line widths.
/// 
/// [`glGetFloatv(GL_SMOOTH_LINE_WIDTH_GRANULARITY, ...)`](https://docs.gl/gl3/glGet)
pub fn get_smooth_line_width_granularity() -> f32 {
	let mut data = 0.0;
	unsafe {
		gl::GetFloatv(gl::SMOOTH_LINE_WIDTH_GRANULARITY, &raw mut data);
	}
	data
}

/// currently bound buffer handle. if no buffer is bound, `None` is returned.
/// see [`crate::BufferObject::handle()`].
/// 
/// [`glGetIntegerv(GL_ARRAY_BUFFER_BINDING, ...)`](https://docs.gl/gl3/glGet)
pub fn get_array_buffer_binding() -> Option<u32> {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &raw mut data);
	}
	if data == 0 {
		None
	} else {
		Some(data as u32)
	}
}

/// whether blending is enabled.
/// 
/// [`glGetBooleanv(GL_BLEND, ...)`](https://docs.gl/gl3/glGet)
pub fn get_blend() -> bool {
	let mut data = 0;
	unsafe {
		gl::GetBooleanv(gl::BLEND, &raw mut data);
	}
	data == gl::TRUE
}

/// the blend color's red, green, blue, and alpha component values.
/// 
/// [`glGetFloatv(GL_BLEND_COLOR, ...)`](https://docs.gl/gl3/glGet)
pub fn get_blend_color() -> (f32, f32, f32, f32) {
	let mut data = [0.0; 4];
	unsafe {
		gl::GetFloatv(gl::BLEND_COLOR, data.as_mut_ptr());
	}
	data.into()
}

/// maximum supported texture units.
/// will be at least 48.
/// 
/// [`glGetIntegerv(GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS, ...)`](https://docs.gl/gl3/glGet)
pub fn get_max_combined_texture_image_units() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS, &raw mut data);
	}
	data as u32
}

/// rough estimate of the largest texture GL can handle.
/// will be at least 1024.
/// 
/// [`glGetIntegerv(GL_MAX_TEXTURE_SIZE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_max_texture_size() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &raw mut data);
	}
	data as u32
}

/// largest size of renderbuffer GL can handle.
/// 
/// [`glGetIntegerv(GL_MAX_RENDERBUFFER_SIZE, ...)`](https://docs.gl/gl3/glGet)
pub fn get_max_renderbuffer_size() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::MAX_RENDERBUFFER_SIZE, &raw mut data);
	}
	data as u32
}

/// maximum number of layers allowed in an array texture.
/// will be at least 256.
/// 
/// [`glGetIntegerv(GL_MAX_ARRAY_TEXTURE_LAYERS, ...)`](https://docs.gl/gl3/glGet)
pub fn get_max_array_texture_layers() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::MAX_ARRAY_TEXTURE_LAYERS, &raw mut data);
	}
	data as u32
}

/// rough estimate of the largest cubemap texture GL can handle.
/// will be at least 1024.
/// 
/// [`glGetIntegerv(GL_MAX_ARRAY_TEXTURE_LAYERS, ...)`](https://docs.gl/gl3/glGet)
pub fn get_max_cube_map_texture_size() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::MAX_CUBE_MAP_TEXTURE_SIZE, &raw mut data);
	}
	data as u32
}

/// maximum number of color attachments in a framebuffer object.
/// will be at least 4.
/// 
/// [`glGetIntegerv(GL_MAX_COLOR_ATTACHMENTS, ...)`](https://docs.gl/gl3/glGet)
pub fn get_max_color_attachments() -> u32 {
	let mut data = 0;
	unsafe {
		gl::GetIntegerv(gl::MAX_COLOR_ATTACHMENTS, &raw mut data);
	}
	data as u32
}

