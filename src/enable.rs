
/// [`glEnable(GL_BLEND)`](https://docs.gl/gl3/glEnable)
pub fn enable_blend() {
	unsafe {
		gl::Enable(gl::BLEND);
	}
}

/// [`glDisable(GL_BLEND)`](https://docs.gl/gl3/glEnable)
pub fn disable_blend() {
	unsafe {
		gl::Disable(gl::BLEND);
	}
}

/// [`glIsEnabled(GL_BLEND)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_blend() -> bool {
	unsafe {
		gl::IsEnabled(gl::BLEND) == gl::TRUE
	}
}

// todo: enable_clip_distance()

/// [`glEnable(GL_COLOR_LOGIC_OP)`](https://docs.gl/gl3/glEnable)
pub fn enable_color_logic_op() {
	unsafe {
		gl::Enable(gl::COLOR_LOGIC_OP);
	}
}

/// [`glDisable(GL_COLOR_LOGIC_OP)`](https://docs.gl/gl3/glEnable)
pub fn disable_color_logic_op() {
	unsafe {
		gl::Disable(gl::COLOR_LOGIC_OP);
	}
}

/// [`glIsEnabled(GL_COLOR_LOGIC_OP)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_color_logic_op() -> bool {
	unsafe {
		gl::IsEnabled(gl::COLOR_LOGIC_OP) == gl::TRUE
	}
}


/// [`glEnable(GL_CULL_FACE)`](https://docs.gl/gl3/glEnable)
pub fn enable_cull_face() {
	unsafe {
		gl::Enable(gl::CULL_FACE);
	}
}

/// [`glDisable(GL_CULL_FACE)`](https://docs.gl/gl3/glEnable)
pub fn disable_cull_face() {
	unsafe {
		gl::Disable(gl::CULL_FACE);
	}
}

/// [`glIsEnabled(GL_CULL_FACE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_cull_face() -> bool {
	unsafe {
		gl::IsEnabled(gl::CULL_FACE) == gl::TRUE
	}
}


/// [`glEnable(GL_DEPTH_CLAMP)`](https://docs.gl/gl3/glEnable)
pub fn enable_depth_clamp() {
	unsafe {
		gl::Enable(gl::DEPTH_CLAMP);
	}
}

/// [`glDisable(GL_DEPTH_CLAMP)`](https://docs.gl/gl3/glEnable)
pub fn disable_depth_clamp() {
	unsafe {
		gl::Disable(gl::DEPTH_CLAMP);
	}
}

/// [`glIsEnabled(GL_DEPTH_CLAMP)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_depth_clamp() -> bool {
	unsafe {
		gl::IsEnabled(gl::DEPTH_CLAMP) == gl::TRUE
	}
}


/// [`glEnable(GL_DEPTH_TEST)`](https://docs.gl/gl3/glEnable)
pub fn enable_depth_test() {
	unsafe {
		gl::Enable(gl::DEPTH_TEST);
	}
}

/// [`glDisable(GL_DEPTH_TEST)`](https://docs.gl/gl3/glEnable)
pub fn disable_depth_test() {
	unsafe {
		gl::Disable(gl::DEPTH_TEST);
	}
}

/// [`glIsEnabled(GL_DEPTH_TEST)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_depth_test() -> bool {
	unsafe {
		gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE
	}
}


/// [`glEnable(GL_DITHER)`](https://docs.gl/gl3/glEnable)
pub fn enable_dither() {
	unsafe {
		gl::Enable(gl::DITHER);
	}
}

/// [`glDisable(GL_DITHER)`](https://docs.gl/gl3/glEnable)
pub fn disable_dither() {
	unsafe {
		gl::Disable(gl::DITHER);
	}
}

/// [`glIsEnabled(GL_DITHER)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_dither() -> bool {
	unsafe {
		gl::IsEnabled(gl::DITHER) == gl::TRUE
	}
}


/// [`glEnable(GL_FRAMEBUFFER_SRGB)`](https://docs.gl/gl3/glEnable)
pub fn enable_framebuffer_srgb() {
	unsafe {
		gl::Enable(gl::FRAMEBUFFER_SRGB);
	}
}

/// [`glDisable(GL_FRAMEBUFFER_SRGB)`](https://docs.gl/gl3/glEnable)
pub fn disable_framebuffer_srgb() {
	unsafe {
		gl::Disable(gl::FRAMEBUFFER_SRGB);
	}
}

/// [`glIsEnabled(GL_FRAMEBUFFER_SRGB)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_framebuffer_srgb() -> bool {
	unsafe {
		gl::IsEnabled(gl::FRAMEBUFFER_SRGB) == gl::TRUE
	}
}


/// [`glEnable(GL_LINE_SMOOTH)`](https://docs.gl/gl3/glEnable)
pub fn enable_line_smooth() {
	unsafe {
		gl::Enable(gl::LINE_SMOOTH);
	}
}

/// [`glDisable(GL_LINE_SMOOTH)`](https://docs.gl/gl3/glEnable)
pub fn disable_line_smooth() {
	unsafe {
		gl::Disable(gl::LINE_SMOOTH);
	}
}

/// [`glIsEnabled(GL_LINE_SMOOTH)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_line_smooth() -> bool {
	unsafe {
		gl::IsEnabled(gl::LINE_SMOOTH) == gl::TRUE
	}
}


/// [`glEnable(GL_BLEND)`](https://docs.gl/gl3/glEnable)
pub fn enable_multisample() {
	unsafe {
		gl::Enable(gl::MULTISAMPLE);
	}
}

/// [`glDisable(GL_BLEND)`](https://docs.gl/gl3/glEnable)
pub fn disable_multisample() {
	unsafe {
		gl::Disable(gl::MULTISAMPLE);
	}
}

/// [`glIsEnabled(GL_MULTISAMPLE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_multisample() -> bool {
	unsafe {
		gl::IsEnabled(gl::MULTISAMPLE) == gl::TRUE
	}
}


/// [`glEnable(GL_POLYGON_OFFSET_FILL)`](https://docs.gl/gl3/glEnable)
pub fn enable_polygon_offset_fill() {
	unsafe {
		gl::Enable(gl::POLYGON_OFFSET_FILL);
	}
}

/// [`glDisable(GL_POLYGON_OFFSET_FILL)`](https://docs.gl/gl3/glEnable)
pub fn disable_polygon_offset_fill() {
	unsafe {
		gl::Disable(gl::POLYGON_OFFSET_FILL);
	}
}

/// [`glIsEnabled(GL_POLYGON_OFFSET_FILL)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_polygon_offset_fill() -> bool {
	unsafe {
		gl::IsEnabled(gl::POLYGON_OFFSET_FILL) == gl::TRUE
	}
}


/// [`glEnable(GL_POLYGON_OFFSET_LINE)`](https://docs.gl/gl3/glEnable)
pub fn enable_polygon_offset_line() {
	unsafe {
		gl::Enable(gl::POLYGON_OFFSET_LINE);
	}
}

/// [`glDisable(GL_POLYGON_OFFSET_LINE)`](https://docs.gl/gl3/glEnable)
pub fn disable_polygon_offset_line() {
	unsafe {
		gl::Disable(gl::POLYGON_OFFSET_LINE);
	}
}

/// [`glIsEnabled(GL_POLYGON_OFFSET_LINE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_polygon_offset_line() -> bool {
	unsafe {
		gl::IsEnabled(gl::POLYGON_OFFSET_LINE) == gl::TRUE
	}
}


/// [`glEnable(GL_POLYGON_OFFSET_POINT)`](https://docs.gl/gl3/glEnable)
pub fn enable_polygon_offset_point() {
	unsafe {
		gl::Enable(gl::POLYGON_OFFSET_POINT);
	}
}

/// [`glDisable(GL_POLYGON_OFFSET_POINT)`](https://docs.gl/gl3/glEnable)
pub fn disable_polygon_offset_point() {
	unsafe {
		gl::Disable(gl::POLYGON_OFFSET_POINT);
	}
}

/// [`glIsEnabled(GL_POLYGON_OFFSET_POINT)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_polygon_offset_point() -> bool {
	unsafe {
		gl::IsEnabled(gl::POLYGON_OFFSET_POINT) == gl::TRUE
	}
}


/// [`glEnable(GL_POLYGON_SMOOTH)`](https://docs.gl/gl3/glEnable)
pub fn enable_polygon_smooth() {
	unsafe {
		gl::Enable(gl::POLYGON_SMOOTH);
	}
}

/// [`glDisable(GL_POLYGON_SMOOTH)`](https://docs.gl/gl3/glEnable)
pub fn disable_polygon_smooth() {
	unsafe {
		gl::Disable(gl::POLYGON_SMOOTH);
	}
}

/// [`glIsEnabled(GL_POLYGON_SMOOTH)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_polygon_smooth() -> bool {
	unsafe {
		gl::IsEnabled(gl::POLYGON_SMOOTH) == gl::TRUE
	}
}


/// [`glEnable(GL_PRIMITIVE_RESTART)`](https://docs.gl/gl3/glEnable)
pub fn enable_primitive_restart() {
	unsafe {
		gl::Enable(gl::PRIMITIVE_RESTART);
	}
}

/// [`glDisable(GL_PRIMITIVE_RESTART)`](https://docs.gl/gl3/glEnable)
pub fn disable_primitive_restart() {
	unsafe {
		gl::Disable(gl::PRIMITIVE_RESTART);
	}
}

/// [`glIsEnabled(GL_PRIMITIVE_RESTART)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_primitive_restart() -> bool {
	unsafe {
		gl::IsEnabled(gl::PRIMITIVE_RESTART) == gl::TRUE
	}
}


/// [`glEnable(GL_SAMPLE_ALPHA_TO_COVERAGE)`](https://docs.gl/gl3/glEnable)
pub fn enable_sample_alpha_to_coverage() {
	unsafe {
		gl::Enable(gl::SAMPLE_ALPHA_TO_COVERAGE);
	}
}

/// [`glDisable(GL_SAMPLE_ALPHA_TO_COVERAGE)`](https://docs.gl/gl3/glEnable)
pub fn disable_sample_alpha_to_coverage() {
	unsafe {
		gl::Disable(gl::SAMPLE_ALPHA_TO_COVERAGE);
	}
}

/// [`glIsEnabled(GL_SAMPLE_ALPHA_TO_COVERAGE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_sample_alpha_to_coverage() -> bool {
	unsafe {
		gl::IsEnabled(gl::SAMPLE_ALPHA_TO_COVERAGE) == gl::TRUE
	}
}


/// [`glEnable(GL_SAMPLE_ALPHA_TO_ONE)`](https://docs.gl/gl3/glEnable)
pub fn enable_sample_alpha_to_one() {
	unsafe {
		gl::Enable(gl::SAMPLE_ALPHA_TO_ONE);
	}
}

/// [`glDisable(GL_SAMPLE_ALPHA_TO_ONE)`](https://docs.gl/gl3/glEnable)
pub fn disable_sample_alpha_to_one() {
	unsafe {
		gl::Disable(gl::SAMPLE_ALPHA_TO_ONE);
	}
}

/// [`glIsEnabled(GL_SAMPLE_ALPHA_TO_ONE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_sample_alpha_to_one() -> bool {
	unsafe {
		gl::IsEnabled(gl::SAMPLE_ALPHA_TO_ONE) == gl::TRUE
	}
}


/// [`glEnable(GL_SAMPLE_COVERAGE)`](https://docs.gl/gl3/glEnable)
pub fn enable_sample_coverage() {
	unsafe {
		gl::Enable(gl::SAMPLE_COVERAGE);
	}
}

/// [`glDisable(GL_SAMPLE_COVERAGE)`](https://docs.gl/gl3/glEnable)
pub fn disable_sample_coverage() {
	unsafe {
		gl::Disable(gl::SAMPLE_COVERAGE);
	}
}

/// [`glIsEnabled(GL_SAMPLE_COVERAGE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_sample_coverage() -> bool {
	unsafe {
		gl::IsEnabled(gl::SAMPLE_COVERAGE) == gl::TRUE
	}
}


/// [`glEnable(GL_SCISSOR_TEST)`](https://docs.gl/gl3/glEnable)
pub fn enable_scissor_test() {
	unsafe {
		gl::Enable(gl::SCISSOR_TEST);
	}
}

/// [`glDisable(GL_SCISSOR_TEST)`](https://docs.gl/gl3/glEnable)
pub fn disable_scissor_test() {
	unsafe {
		gl::Disable(gl::SCISSOR_TEST);
	}
}

/// [`glIsEnabled(GL_SCISSOR_TEST)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_scissor_test() -> bool {
	unsafe {
		gl::IsEnabled(gl::SCISSOR_TEST) == gl::TRUE
	}
}


/// [`glEnable(GL_STENCIL_TEST)`](https://docs.gl/gl3/glEnable)
pub fn enable_stencil_test() {
	unsafe {
		gl::Enable(gl::STENCIL_TEST);
	}
}

/// [`glDisable(GL_STENCIL_TEST)`](https://docs.gl/gl3/glEnable)
pub fn disable_stencil_test() {
	unsafe {
		gl::Disable(gl::STENCIL_TEST);
	}
}

/// [`glIsEnabled(GL_STENCIL_TEST)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_stencil_test() -> bool {
	unsafe {
		gl::IsEnabled(gl::STENCIL_TEST) == gl::TRUE
	}
}


/// [`glEnable(GL_TEXTURE_CUBE_MAP_SEAMLESS)`](https://docs.gl/gl3/glEnable)
pub fn enable_texture_cube_map_seamless() {
	unsafe {
		gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
	}
}

/// [`glDisable(GL_TEXTURE_CUBE_MAP_SEAMLESS)`](https://docs.gl/gl3/glEnable)
pub fn disable_texture_cube_map_seamless() {
	unsafe {
		gl::Disable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
	}
}

/// [`glIsEnabled(GL_TEXTURE_CUBE_MAP_SEAMLESS)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_texture_cube_map_seamless() -> bool {
	unsafe {
		gl::IsEnabled(gl::TEXTURE_CUBE_MAP_SEAMLESS) == gl::TRUE
	}
}


/// [`glEnable(GL_PROGRAM_POINT_SIZE)`](https://docs.gl/gl3/glEnable)
pub fn enable_program_point_size() {
	unsafe {
		gl::Enable(gl::PROGRAM_POINT_SIZE);
	}
}

/// [`glDisable(GL_PROGRAM_POINT_SIZE)`](https://docs.gl/gl3/glEnable)
pub fn disable_program_point_size() {
	unsafe {
		gl::Disable(gl::PROGRAM_POINT_SIZE);
	}
}

/// [`glIsEnabled(GL_PROGRAM_POINT_SIZE)`](https://docs.gl/gl3/glIsEnabled)
pub fn is_enabled_program_point_size() -> bool {
	unsafe {
		gl::IsEnabled(gl::PROGRAM_POINT_SIZE) == gl::TRUE
	}
}

