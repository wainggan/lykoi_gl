
/// [`glViewport()`](https://docs.gl/gl3/glViewport)
pub fn viewport(x: i32, y: i32, width: usize, height: usize) {
	unsafe {
		gl::Viewport(x, y, width as i32, height as i32);
	}
}

#[repr(u32)]
pub enum Blend {
	/// GL_ZERO
	Zero = gl::ZERO,
	/// GL_ONE
	One = gl::ONE,
	/// GL_SRC_COLOR
	SrcColor = gl::SRC_COLOR,
	/// GL_ONE_MINUS_SRC_COLOR
	OneMinusSrcColor = gl::ONE_MINUS_SRC_COLOR,
	/// GL_DST_COLOR
	DstColor = gl::DST_COLOR,
	/// GL_ONE_MINUS_DST_COLOR
	OneMinusDstColor = gl::ONE_MINUS_DST_COLOR,
	/// GL_SRC_ALPHA
	SrcAlpha = gl::SRC_ALPHA,
	/// GL_ONE_MINUS_SRC_ALPHA
	OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA,
	/// GL_DST_ALPHA
	DstAlpha = gl::DST_ALPHA,
	/// GL_ONE_MINUS_DST_ALPHA
	OneMinusDstAlpha = gl::ONE_MINUS_DST_ALPHA,
	/// GL_CONSTANT_COLOR
	ConstantColor = gl::CONSTANT_COLOR,
	/// GL_ONE_MINUS_CONSTANT_COLOR
	OneMinusConstantColor = gl::ONE_MINUS_CONSTANT_COLOR,
	/// GL_CONSTANT_ALPHA
	ConstantAlpha = gl::CONSTANT_ALPHA,
	/// GL_ONE_MINUS_CONSTANT_ALPHA
	OneMinusConstantAlpha = gl::ONE_MINUS_CONSTANT_ALPHA,
}

/// [`glBlendFunc()`](https://docs.gl/gl3/glBlendFunc)
pub fn blend_func(src: Blend, dst: Blend) {
	unsafe {
		gl::BlendFunc(
			src as u32,
			dst as u32,
		);
	}
}

/// [`glBlendFuncSeparate()`](https://docs.gl/gl3/glBlendFuncSeparate)
pub fn blend_func_separate(src_c: Blend, dst_c: Blend, src_a: Blend, dst_a: Blend) {
	unsafe {
		gl::BlendFuncSeparate(
			src_c as u32,
			dst_c as u32,
			src_a as u32,
			dst_a as u32,
		);
	}
}

/// [`glBlendColor()`](https://docs.gl/gl3/glBlendColor)
pub fn blend_color(r: f32, g: f32, b: f32, a: f32) {
	unsafe {
		gl::BlendColor(r, g, b, a);
	}
}

