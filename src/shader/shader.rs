
/// wrapper over an OpenGL "Shader Object" name.
#[derive(Debug)]
pub struct ShaderObject(u32);
impl ShaderObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

impl Drop for ShaderObject {
	fn drop(&mut self) {
		delete_shader(
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		);
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShaderType {
	VertexShader = gl::VERTEX_SHADER,
	FragmentShader = gl::FRAGMENT_SHADER,
	GeometryShader = gl::GEOMETRY_SHADER,
}

/// [`glCreateShader()`](https://docs.gl/gl3/glCreateShader)
pub fn create_shader(kind: ShaderType) -> Option<ShaderObject> {
	let out = unsafe { gl::CreateShader(kind as u32) };
	if out == 0 {
		None
	} else {
		Some(ShaderObject(out))
	}
}

/// [`glDeleteShader()`](https://docs.gl/gl3/glDeleteShader)
pub fn delete_shader(shader: ShaderObject) {
	unsafe {
		gl::DeleteShader(std::mem::ManuallyDrop::new(shader).handle());
	}
}

/// [`glIsShader()`](https://docs.gl/gl3/glIsShader)
pub fn is_shader(shader: &ShaderObject) -> bool {
	unsafe {
		gl::IsShader(shader.handle()) == gl::TRUE
	}
}

/// [`glShaderSource()`](https://docs.gl/gl3/glShaderSource)
pub fn shader_source<const N: usize>(shader: &ShaderObject, source: [&str; N]) {
	let count = source.len() as i32;
	let string_arr = source.map(|v| v.as_ptr() as *const i8);
	let string = string_arr.as_ptr();
	let length_arr = source.map(|v| v.len() as i32);
	let length = length_arr.as_ptr();

	unsafe {
		gl::ShaderSource(
			shader.handle(),
			count,
			string,
			length,
		);
	}
}

/// [`glCompileShader()`](https://docs.gl/gl3/glCompileShader)
pub fn compile_shader(shader: &ShaderObject) {
	unsafe {
		gl::CompileShader(shader.handle());
	}
}

