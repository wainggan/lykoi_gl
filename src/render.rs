
/// [`glClearColor()`](https://docs.gl/gl3/glClearColor)
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
	unsafe {
		gl::ClearColor(r, g, b, a);
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BufferBit {
	/// GL_COLOR_BUFFER_BIT
	ColorBufferBit = gl::COLOR_BUFFER_BIT,
	/// GL_STENCIL_BUFFER_BIT
	StencilBufferBit = gl::STENCIL_BUFFER_BIT,
	/// GL_DEPTH_BUFFER_BIT
	DepthBufferBit = gl::DEPTH_BUFFER_BIT,
}

/// [`glClear()`](https://docs.gl/gl3/glClear)
pub fn clear(bits: &[BufferBit]) {
	let mask = bits
		.iter()
		.fold(0b0, |a, b| a | *b as u32);
	unsafe {
		gl::Clear(mask);
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawPrimitives {
	/// GL_POINTS
	Points = gl::POINTS,
	/// GL_LINE_STRIP
	LineStrip = gl::LINE_STRIP,
	/// GL_LINE_LOOP
	LineLoop = gl::LINE_LOOP,
	/// GL_LINES
	Lines = gl::LINES,
	/// GL_LINE_STRIP_ADJACENCY
	/// gl >= 3.2
	LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
	/// GL_LINES_ADJACENCY
	/// gl >= 3.2
	LinesAdjacency = gl::LINES_ADJACENCY,
	/// GL_TRIANGLE_STRIP
	TriangleStrip = gl::TRIANGLE_STRIP,
	/// GL_TRIANGLE_FAN
	TriangleFan = gl::TRIANGLE_FAN,
	/// GL_TRIANGLES
	Triangles = gl::TRIANGLES,
	/// GL_TRIANGLE_STRIP_ADJACENCY
	/// gl >= 3.2
	TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
	/// GL_TRIANGLES_ADJACENCY
	/// gl >= 3.2
	TrianglesAdjacency = gl::TRIANGLES_ADJACENCY,
	/// GL_PATCHES
	Patches = gl::PATCHES,
}

/// [`glDrawArrays()`](https://docs.gl/gl3/glDrawArrays)
pub fn draw_arrays(mode: DrawPrimitives, first: u32, count: u32) {
	unsafe {
		gl::DrawArrays(mode as u32, first as i32, count as i32);
	}
}

