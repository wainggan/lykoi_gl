
#[derive(Debug)]
pub enum GlError {
	InvalidEnum,
	InvalidValue,
	InvalidOperation,
	InvalidFramebufferOperation,
	OutOfMemory,
}
impl std::fmt::Display for GlError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Self::InvalidEnum => "Invalid Enum",
			Self::InvalidValue => "Invalid Value",
			Self::InvalidOperation => "Invalid Operation",
			Self::InvalidFramebufferOperation => "Invalid Framebuffer Operation",
			Self::OutOfMemory => "Out Of Memory",
		})
	}
}
impl std::error::Error for GlError {}

/// [`glGetError()`](https://docs.gl/gl3/glGetError)
pub fn get_error() -> Result<(), GlError> {
	match unsafe { gl::GetError() } {
		gl::NO_ERROR => Ok(()),
		gl::INVALID_ENUM => Err(GlError::InvalidEnum),
		gl::INVALID_VALUE => Err(GlError::InvalidValue),
		gl::INVALID_OPERATION => Err(GlError::InvalidOperation),
		gl::INVALID_FRAMEBUFFER_OPERATION => Err(GlError::InvalidFramebufferOperation),
		gl::OUT_OF_MEMORY => Err(GlError::OutOfMemory),
		_ => unreachable!(),
	}
}

