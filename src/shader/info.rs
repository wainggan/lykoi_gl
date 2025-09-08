
use std::ffi::CStr;

use super::{ProgramObject, ShaderObject, ShaderType};


/// [`glGetShaderiv(_, GL_SHADER_TYPE, ...)`](https://docs.gl/gl3/glGetShader)
pub fn get_shader_shader_type(shader: &ShaderObject) -> ShaderType {
	let mut out = 0;
	unsafe {
		gl::GetShaderiv(shader.handle(), gl::SHADER_TYPE, &mut out);
	}
	match out as u32 {
		gl::FRAGMENT_SHADER => ShaderType::FragmentShader,
		gl::VERTEX_SHADER => ShaderType::VertexShader,
		gl::GEOMETRY_SHADER => ShaderType::GeometryShader,
		_ => unreachable!(),
	}
}

/// [`glGetShaderiv(_, GL_DELETE_STATUS, ...)`](https://docs.gl/gl3/glGetShader)
pub fn get_shader_delete_status(shader: &ShaderObject) -> bool {
	let mut out = 0;
	unsafe {
		gl::GetShaderiv(shader.handle(), gl::DELETE_STATUS, &mut out);
	}
	out as u8 == gl::TRUE
}

/// [`glGetShaderiv(_, GL_COMPILE_STATUS, ...)`](https://docs.gl/gl3/glGetShader)
pub fn get_shader_compile_status(shader: &ShaderObject) -> bool {
	let mut out = 0;
	unsafe {
		gl::GetShaderiv(shader.handle(), gl::COMPILE_STATUS, &mut out);
	}
	out as u8 == gl::TRUE
}

/// [`glGetShaderiv(_, GL_INFO_LOG_LENGTH, ...)`](https://docs.gl/gl3/glGetShader)
pub fn get_shader_info_log_length(shader: &ShaderObject) -> Option<u32> {
	let mut out = 0;
	unsafe {
		gl::GetShaderiv(shader.handle(), gl::INFO_LOG_LENGTH, &mut out);
	}
	if out == 0 {
		None
	} else {
		Some(out as u32)
	}
}

/// [`glGetShaderiv(_, GL_SHADER_SOURCE_LENGTH, ...)`](https://docs.gl/gl3/glGetShader)
pub fn get_shader_shader_source_length(shader: &ShaderObject) -> Option<u32> {
	let mut out = 0;
	unsafe {
		gl::GetShaderiv(shader.handle(), gl::SHADER_SOURCE_LENGTH, &mut out);
	}
	if out == 0 {
		None
	} else {
		Some(out as u32)
	}
}

/// [`glGetShaderInfoLog()`](https://docs.gl/gl3/glGetShaderInfoLog)
pub fn get_shader_info_log(shader: &ShaderObject) -> String {
	let mut length = 0;
	let mut out_string = [0u8; 512];
	unsafe {
		gl::GetShaderInfoLog(
			shader.handle(),
			out_string.len() as i32,
			&mut length,
			// TODO: this might be UB? figure out a way to validate the string
			out_string.as_mut_ptr() as *mut i8,
		);
	}
	let out = CStr::from_bytes_until_nul(&out_string).unwrap();
	out.to_str().unwrap().to_string()
}


/// [`glGetProgramiv(_, GL_DELETE_STATUS, ...)`](https://docs.gl/gl3/glGetProgram)
pub fn get_program_delete_status(program: &ProgramObject) -> bool {
	let mut out = 0;
	unsafe {
		gl::GetProgramiv(program.handle(), gl::DELETE_STATUS, &mut out);
	}
	out as u8 == gl::TRUE
}

/// [`glGetProgramiv(_, GL_LINK_STATUS, ...)`](https://docs.gl/gl3/glGetProgram)
pub fn get_program_link_status(program: &ProgramObject) -> bool {
	let mut out = 0;
	unsafe {
		gl::GetProgramiv(program.handle(), gl::LINK_STATUS, &mut out);
	}
	out as u8 == gl::TRUE
}

/// [`glGetProgramiv(_, GL_ACTIVE_UNIFORMS, ...)`](https://docs.gl/gl3/glGetProgram)
pub fn get_program_active_uniforms(program: &ProgramObject) -> u32 {
	let mut out = 0;
	unsafe {
		gl::GetProgramiv(program.handle(), gl::ACTIVE_UNIFORMS, &mut out);
	}
	out as u32
}


/// [`glGetProgramInfoLog()`](https://docs.gl/gl3/glGetProgramInfoLog)
pub fn get_program_info_log(program: &ProgramObject) -> String {
	let mut length = 0;
	let mut out_string = [0u8; 512];
	unsafe {
		gl::GetProgramInfoLog(
			program.handle(),
			out_string.len() as i32,
			&mut length,
			// TODO: this might be UB? figure out a way to validate the string
			out_string.as_mut_ptr() as *mut i8,
		);
	}
	let out = CStr::from_bytes_until_nul(&out_string).unwrap();
	out.to_str().unwrap().to_string()
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UniformTypes {
	Float = gl::FLOAT,
	FloatVec2 = gl::FLOAT_VEC2,
	FloatVec3 = gl::FLOAT_VEC3,
	FloatVec4 = gl::FLOAT_VEC4,
	Int = gl::INT,
	IntVec2 = gl::INT_VEC2,
	IntVec3 = gl::INT_VEC3,
	IntVec4 = gl::INT_VEC4,
	UnsignedInt = gl::UNSIGNED_INT,
	UnsignedIntVec2 = gl::UNSIGNED_INT_VEC2,
	UnsignedIntVec3 = gl::UNSIGNED_INT_VEC3,
	UnsignedIntVec4 = gl::UNSIGNED_INT_VEC4,
	FloatMat2x2 = gl::FLOAT_MAT2,
	FloatMat2x3 = gl::FLOAT_MAT2x3,
	FloatMat2x4 = gl::FLOAT_MAT2x4,
	FloatMat3x2 = gl::FLOAT_MAT3x2,
	FloatMat3x3 = gl::FLOAT_MAT3,
	FloatMat3x4 = gl::FLOAT_MAT3x4,
	FloatMat4x2 = gl::FLOAT_MAT4x2,
	FloatMat4x3 = gl::FLOAT_MAT4x3,
	FloatMat4x4 = gl::FLOAT_MAT4,
	Sampler1D = gl::SAMPLER_1D,
	Sampler2D = gl::SAMPLER_2D,
	Sampler3D = gl::SAMPLER_3D,
}

/// [`glGetActiveUniform()`](https://docs.gl/gl3/glGetActiveUniform)
pub fn get_active_uniform(program: &ProgramObject, index: u32) -> (String, UniformTypes, usize) {
	let mut out_name = Vec::<u8>::with_capacity(64);
	let mut out_len = 63;
	let mut out_size = 0;
	let mut out_type = 0;
	unsafe {
		gl::GetActiveUniform(
			program.handle(),
			index,
			out_len,
			&mut out_len,
			&mut out_size,
			&mut out_type,
			out_name.as_mut_ptr() as *mut i8,
		);
		out_name.set_len(out_len as usize);
	}
	let name = String::from_utf8(out_name).unwrap();
	let kind = match out_type {
		gl::FLOAT => UniformTypes::Float,
		gl::FLOAT_VEC2 => UniformTypes::FloatVec2,
		gl::FLOAT_VEC3 => UniformTypes::FloatVec3,
		gl::FLOAT_VEC4 => UniformTypes::FloatVec4,
		
		gl::INT => UniformTypes::Int,
		gl::INT_VEC2 => UniformTypes::IntVec2,
		gl::INT_VEC3 => UniformTypes::IntVec3,
		gl::INT_VEC4 => UniformTypes::IntVec4,

		gl::UNSIGNED_INT => UniformTypes::UnsignedInt,
		gl::UNSIGNED_INT_VEC2 => UniformTypes::UnsignedIntVec2,
		gl::UNSIGNED_INT_VEC3 => UniformTypes::UnsignedIntVec3,
		gl::UNSIGNED_INT_VEC4 => UniformTypes::UnsignedIntVec4,

		gl::FLOAT_MAT2 => UniformTypes::FloatMat2x2,
		gl::FLOAT_MAT2x3 => UniformTypes::FloatMat2x3,
		gl::FLOAT_MAT2x4 => UniformTypes::FloatMat2x4,

		gl::FLOAT_MAT3x2 => UniformTypes::FloatMat3x2,
		gl::FLOAT_MAT3 => UniformTypes::FloatMat3x3,
		gl::FLOAT_MAT3x4 => UniformTypes::FloatMat3x4,

		gl::FLOAT_MAT4x2 => UniformTypes::FloatMat4x2,
		gl::FLOAT_MAT4x3 => UniformTypes::FloatMat4x3,
		gl::FLOAT_MAT4 => UniformTypes::FloatMat4x4,
		
		gl::SAMPLER_1D => UniformTypes::Sampler1D,
		gl::SAMPLER_2D => UniformTypes::Sampler2D,
		gl::SAMPLER_3D => UniformTypes::Sampler3D,
		_ => todo!(),
	};
	(name, kind, out_size as usize)
}

