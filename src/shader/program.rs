
use super::shader::ShaderObject;


/// wrapper over an OpenGL "Program Object" name.
#[derive(Debug)]
pub struct ProgramObject(u32);
impl ProgramObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

impl Drop for ProgramObject {
	fn drop(&mut self) {
		delete_program(
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		);
	}
}

/// [`glCreateProgram()`](https://docs.gl/gl3/glCreateProgram)
pub fn create_program() -> Result<ProgramObject, ()> {
	let out = unsafe { gl::CreateProgram() };
	if out == 0 {
		Err(())
	} else {
		Ok(ProgramObject(out))
	}
}

/// [`glDeleteProgram()`](https://docs.gl/gl3/glDeleteProgram)
pub fn delete_program(program: ProgramObject) {
	unsafe {
		gl::DeleteProgram(std::mem::ManuallyDrop::new(program).handle());
	}
}

/// [`glIsProgram()`](https://docs.gl/gl3/glIsProgram)
pub fn is_program(program: &ProgramObject) -> bool {
	unsafe {
		gl::IsProgram(program.handle()) == gl::TRUE
	}
}

/// [`glAttachShader()`](https://docs.gl/gl3/glAttachShader)
pub fn attach_shader(program: &ProgramObject, shader: &ShaderObject) {
	unsafe {
		gl::AttachShader(program.handle(), shader.handle());
	}
}

/// [`glLinkProgram()`](https://docs.gl/gl3/glLinkProgram)
pub fn link_program(program: &ProgramObject) {
	unsafe {
		gl::LinkProgram(program.handle());
	}
}

/// [`glUseProgram()`](https://docs.gl/gl3/glUseProgram)
pub fn use_program(program: &ProgramObject) {
	unsafe {
		gl::UseProgram(program.handle());
	}
}

