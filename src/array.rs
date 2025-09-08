
use super::msg::ERROR_OOB;


/// wrapper over an OpenGL "Vertex Array Object" name.
#[derive(Debug)]
pub struct VertexArrayObject(u32);
impl VertexArrayObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}


/// [`glGenVertexArrays()`](https://docs.gl/gl3/glGenVertexArrays)
pub fn gen_vertex_arrays<const N: usize>() -> [VertexArrayObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenVertexArrays(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(|v| VertexArrayObject(v))
}

/// [`glDeleteVertexArrays()`](https://docs.gl/gl3/glDeleteVertexArrays)
pub fn delete_vertex_arrays<const N: usize>(arrays: [VertexArrayObject; N]) {
	let list = arrays.map(|v| v.handle());
	unsafe {
		gl::DeleteVertexArrays(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

/// [`glIsVertexArray()`](https://docs.gl/gl3/glIsVertexArray)
pub fn is_vertex_array(array: &VertexArrayObject) -> bool {
	unsafe {
		gl::IsVertexArray(array.handle()) == gl::TRUE
	}
}

/// [`glBindVertexArray()`](https://docs.gl/gl3/glBindVertexArray)
pub fn bind_vertex_array(array: &VertexArrayObject) {
	unsafe {
		gl::BindVertexArray(array.handle());
	}
}

/// [`glBindVertexArray(_, 0)`](https://docs.gl/gl3/glBindVertexArray)
pub fn unbind_vertex_array() {
	unsafe {
		gl::BindVertexArray(0);
	}
}


