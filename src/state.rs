
/// [`glViewport()`](https://docs.gl/gl3/glViewport)
pub fn viewport(x: i32, y: i32, width: usize, height: usize) {
	unsafe {
		gl::Viewport(x, y, width as i32, height as i32);
	}
}


/// [`glBlendFunc()`](https://docs.gl/gl3/glBlendFunc)
// todo: enum
pub fn blend_func(src: u32, dst: u32) {
	unsafe {
		gl::BlendFunc(src, dst);
	}
}

/// [`glBlendFuncSeperate()`](https://docs.gl/gl3/glBlendFuncSeperate)
pub fn blend_func_seperate(src_c: u32, dst_c: u32, src_a: u32, dst_a: u32) {
	unsafe {
		gl::BlendFuncSeparate(src_c, dst_c, src_a, dst_a);
	}
}

/// [`glBlendColor()`](https://docs.gl/gl3/glBlendColor)
pub fn blend_color(r: f32, g: f32, b: f32, a: f32) {
	unsafe {
		gl::BlendColor(r, g, b, a);
	}
}


/// [`glEnable()`](https://docs.gl/gl3/glEnable)
// todo: enum
pub fn enable(cap: u32) {
	unsafe {
		gl::Enable(cap);
	}
}

/// [`glDisable()`](https://docs.gl/gl3/glDisable)
// todo: enum
pub fn disable(cap: u32) {
	unsafe {
		gl::Disable(cap);
	}
}

