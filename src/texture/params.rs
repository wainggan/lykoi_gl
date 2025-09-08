
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterTarget {
	Texture1D = gl::TEXTURE_1D,
	Texture2D = gl::TEXTURE_2D,
	Texture3D = gl::TEXTURE_3D,
	Texture1DArray = gl::TEXTURE_1D_ARRAY,
	Texture2DArray = gl::TEXTURE_2D_ARRAY,
	TextureRectangle = gl::TEXTURE_RECTANGLE,
	TextureCubeMap = gl::TEXTURE_CUBE_MAP,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterName {
	BaseLevel = gl::TEXTURE_BASE_LEVEL,
	BorderColor = gl::TEXTURE_BORDER_COLOR,
	CompareMode = gl::TEXTURE_COMPARE_MODE,
	CompareFunc = gl::TEXTURE_COMPARE_FUNC,
	MaxLod = gl::TEXTURE_MAX_LOD,
	MinLod = gl::TEXTURE_MIN_LOD,
	LodBias = gl::TEXTURE_LOD_BIAS,
	MagFilter = gl::TEXTURE_MAG_FILTER,
	MinFilter = gl::TEXTURE_MIN_FILTER,
	MaxLevel = gl::TEXTURE_MAX_LEVEL,
	SwizzleR = gl::TEXTURE_SWIZZLE_R,
	SwizzleG = gl::TEXTURE_SWIZZLE_G,
	SwizzleB = gl::TEXTURE_SWIZZLE_B,
	SwizzleA = gl::TEXTURE_SWIZZLE_A,
	SwizzleRGBA = gl::TEXTURE_SWIZZLE_RGBA,
	WrapS = gl::TEXTURE_WRAP_S,
	WrapT = gl::TEXTURE_WRAP_T,
	WrapR = gl::TEXTURE_WRAP_R,
}

pub unsafe fn tex_parameter_i(target: TexParameterTarget, name: TexParameterName, value: i32) {
	unsafe {
		gl::TexParameteri(target as u32, name as u32, value);
	}
}

pub unsafe fn tex_parameter_f(target: TexParameterTarget, name: TexParameterName, value: f32) {
	unsafe {
		gl::TexParameterf(target as u32, name as u32, value);
	}
}

pub unsafe fn tex_parameter_iv(target: TexParameterTarget, name: TexParameterName, value: &[i32]) {
	let mut params = [0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::TexParameteriv(target as u32, name as u32, params.as_ptr());
	}
}

pub unsafe fn tex_parameter_fv(target: TexParameterTarget, name: TexParameterName, value: &[f32]) {
	let mut params = [0.0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::TexParameterfv(target as u32, name as u32, params.as_ptr());
	}
}

pub unsafe fn tex_parameter_i_iv(target: TexParameterTarget, name: TexParameterName, value: &[i32]) {
	let mut params = [0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::TexParameterIiv(target as u32, name as u32, params.as_ptr());
	}
}

pub unsafe fn tex_parameter_i_uiv(target: TexParameterTarget, name: TexParameterName, value: &[u32]) {
	let mut params = [0; 4];
	for i in 0..4.min(value.len()) {
		params[i] = value[i];
	}
	unsafe {
		gl::TexParameterIuiv(target as u32, name as u32, params.as_ptr());
	}
}


/// [`glTexParameteri(_, GL_TEXTURE_BASE_LEVEL, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_base_level(target: TexParameterTarget, value: u32) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_BASE_LEVEL, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_BASE_LEVEL, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_base_level(target: TexParameterTarget) -> u32 {
	let mut out = 0;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_BASE_LEVEL, &mut out as *mut _);
	}
	out as u32
}

pub enum TexParameterBorderColor {
	I32I([i32; 4]),
	U32I([u32; 4]),
	I32F([i32; 4]),
	F32F([f32; 4]),
}

/// [`glTexParameterIiv(_, GL_TEXTURE_BORDER_COLOR, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_border_color(target: TexParameterTarget, value: TexParameterBorderColor) {
	match value {
		TexParameterBorderColor::I32I(v) => unsafe {
			gl::TexParameterIiv(
				target as u32,
				gl::TEXTURE_BORDER_COLOR,
				v.as_ptr(),
			);
		},
		TexParameterBorderColor::U32I(v) => unsafe {
			gl::TexParameterIuiv(
				target as u32,
				gl::TEXTURE_BORDER_COLOR,
				v.as_ptr(),
			);
		},
		TexParameterBorderColor::I32F(v) => unsafe {
			gl::TexParameteriv(
				target as u32,
				gl::TEXTURE_BORDER_COLOR,
				v.as_ptr(),
			);
		},
		TexParameterBorderColor::F32F(v) => unsafe {
			gl::TexParameterfv(
				target as u32,
				gl::TEXTURE_BORDER_COLOR,
				v.as_ptr(),
			);
		},
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterCompareMode {
	None = gl::NONE,
	CompareRefToTexture = gl::COMPARE_REF_TO_TEXTURE,
}

/// [`glTexParameteri(_, GL_TEXTURE_COMPARE_MODE, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_compare_mode(target: TexParameterTarget, value: TexParameterCompareMode) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_COMPARE_MODE, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_COMPARE_MODE, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_compare_mode(target: TexParameterTarget) -> TexParameterCompareMode {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_COMPARE_MODE, &mut out as *mut _);
	}
	match out as u32 {
		gl::NONE => TexParameterCompareMode::None,
		gl::COMPARE_REF_TO_TEXTURE => TexParameterCompareMode::CompareRefToTexture,
		_ => unreachable!(),
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterCompareFunc {
	LEqual = gl::LEQUAL,
	GEqual = gl::GEQUAL,
	Less = gl::LESS,
	Greater = gl::GREATER,
	Equal = gl::EQUAL,
	NotEqual = gl::NOTEQUAL,
	Always = gl::ALWAYS,
	Never = gl::NEVER,
}

/// [`glTexParameteri(_, GL_TEXTURE_COMPARE_FUNC, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_compare_func(target: TexParameterTarget, value: TexParameterCompareFunc) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_COMPARE_FUNC, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_COMPARE_FUNC, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_compare_func(target: TexParameterTarget) -> TexParameterCompareFunc {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_COMPARE_FUNC, &mut out as *mut _);
	}
	match out as u32 {
		gl::LEQUAL => TexParameterCompareFunc::LEqual,
		gl::GEQUAL => TexParameterCompareFunc::GEqual,
		gl::LESS => TexParameterCompareFunc::Less,
		gl::GREATER => TexParameterCompareFunc::Greater,
		gl::EQUAL => TexParameterCompareFunc::Equal,
		gl::NOTEQUAL => TexParameterCompareFunc::NotEqual,
		gl::ALWAYS => TexParameterCompareFunc::Always,
		gl::NEVER => TexParameterCompareFunc::Never,
		_ => unreachable!(),
	}
}

/// [`glTexParameterf(_, GL_TEXTURE_LOD_BIAS, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_lod_bias(target: TexParameterTarget, value: f32) {
	unsafe {
		gl::TexParameterf(target as u32, gl::TEXTURE_LOD_BIAS, value);
	}
}

/// [`glGetTexParameterfv(_, GL_TEXTURE_LOD_BIAS, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_lod_bias(target: TexParameterTarget) -> f32 {
	let mut out = 0.0;
	unsafe {
		gl::GetTexParameterfv(target as u32, gl::TEXTURE_LOD_BIAS, &mut out as *mut _);
	}
	out
}

/// [`glTexParameterf(_, GL_TEXTURE_MAX_LOD, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_max_lod(target: TexParameterTarget, value: f32) {
	unsafe {
		gl::TexParameterf(target as u32, gl::TEXTURE_MAX_LOD, value);
	}
}

/// [`glGetTexParameterfv(_, GL_TEXTURE_MAX_LOD, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_max_lod(target: TexParameterTarget) -> f32 {
	let mut out = 0.0;
	unsafe {
		gl::GetTexParameterfv(target as u32, gl::TEXTURE_MAX_LOD, &mut out as *mut _);
	}
	out
}

/// [`glTexParameterf(_, GL_TEXTURE_MIN_LOD, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_min_lod(target: TexParameterTarget, value: f32) {
	unsafe {
		gl::TexParameterf(target as u32, gl::TEXTURE_MIN_LOD, value);
	}
}

/// [`glGetTexParameterfv(_, GL_TEXTURE_MIN_LOD, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_min_lod(target: TexParameterTarget) -> f32 {
	let mut out = 0.0;
	unsafe {
		gl::GetTexParameterfv(target as u32, gl::TEXTURE_MIN_LOD, &mut out as *mut _);
	}
	out
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterMagFilter {
	Nearest = gl::NEAREST,
	Linear = gl::LINEAR,
}

/// [`glTexParameteri(_, GL_TEXTURE_MAG_FILTER, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_mag_filter(target: TexParameterTarget, value: TexParameterMagFilter) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_MAG_FILTER, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_MAG_FILTER, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_mag_filter(target: TexParameterTarget) -> TexParameterMagFilter {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_MAG_FILTER, &mut out as *mut _);
	}
	match out as u32 {
		gl::NEAREST => TexParameterMagFilter::Nearest,
		gl::LINEAR => TexParameterMagFilter::Linear,
		_ => unreachable!(),
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterMinFilter {
	Nearest = gl::NEAREST,
	Linear = gl::LINEAR,
	NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
	NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
	LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
	LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}

/// [`glTexParameteri(_, GL_TEXTURE_MIN_FILTER, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_min_filter(target: TexParameterTarget, value: TexParameterMinFilter) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_MIN_FILTER, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_MIN_FILTER, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_min_filter(target: TexParameterTarget) -> TexParameterMinFilter {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_MIN_FILTER, &mut out as *mut _);
	}
	match out as u32 {
		gl::NEAREST => TexParameterMinFilter::Nearest,
		gl::LINEAR => TexParameterMinFilter::Linear,
		gl::NEAREST_MIPMAP_NEAREST => TexParameterMinFilter::NearestMipmapNearest,
		gl::NEAREST_MIPMAP_LINEAR => TexParameterMinFilter::NearestMipmapLinear,
		gl::LINEAR_MIPMAP_NEAREST => TexParameterMinFilter::LinearMipmapNearest,
		gl::LINEAR_MIPMAP_LINEAR => TexParameterMinFilter::LinearMipmapLinear,
		_ => unreachable!(),
	}
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterSwizzle {
	Red = gl::RED,
	Green = gl::GREEN,
	Blue = gl::BLUE,
	Alpha = gl::ALPHA,
	Zero = gl::ZERO,
	One = gl::ONE,
}

/// [`glTexParameteri(_, GL_TEXTURE_SWIZZLE_R, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_swizzle_r(target: TexParameterTarget, value: TexParameterSwizzle) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_SWIZZLE_R, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_SWIZZLE_R, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_swizzle_r(target: TexParameterTarget) -> TexParameterSwizzle {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_SWIZZLE_R, &mut out as *mut _);
	}
	match out as u32 {
		gl::RED => TexParameterSwizzle::Red,
		gl::GREEN => TexParameterSwizzle::Green,
		gl::BLUE => TexParameterSwizzle::Blue,
		gl::ALPHA => TexParameterSwizzle::Alpha,
		gl::ZERO => TexParameterSwizzle::Zero,
		gl::ONE => TexParameterSwizzle::One,
		_ => unreachable!(),
	}
}

/// [`glTexParameteri(_, GL_TEXTURE_SWIZZLE_G, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_swizzle_g(target: TexParameterTarget, value: TexParameterSwizzle) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_SWIZZLE_G, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_SWIZZLE_G, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_swizzle_g(target: TexParameterTarget) -> TexParameterSwizzle {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_SWIZZLE_G, &mut out as *mut _);
	}
	match out as u32 {
		gl::RED => TexParameterSwizzle::Red,
		gl::GREEN => TexParameterSwizzle::Green,
		gl::BLUE => TexParameterSwizzle::Blue,
		gl::ALPHA => TexParameterSwizzle::Alpha,
		gl::ZERO => TexParameterSwizzle::Zero,
		gl::ONE => TexParameterSwizzle::One,
		_ => unreachable!(),
	}
}

/// [`glTexParameteri(_, GL_TEXTURE_SWIZZLE_B, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_swizzle_b(target: TexParameterTarget, value: TexParameterSwizzle) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_SWIZZLE_B, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_SWIZZLE_B, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_swizzle_b(target: TexParameterTarget) -> TexParameterSwizzle {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_SWIZZLE_B, &mut out as *mut _);
	}
	match out as u32 {
		gl::RED => TexParameterSwizzle::Red,
		gl::GREEN => TexParameterSwizzle::Green,
		gl::BLUE => TexParameterSwizzle::Blue,
		gl::ALPHA => TexParameterSwizzle::Alpha,
		gl::ZERO => TexParameterSwizzle::Zero,
		gl::ONE => TexParameterSwizzle::One,
		_ => unreachable!(),
	}
}

/// [`glTexParameteri(_, GL_TEXTURE_SWIZZLE_A, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_swizzle_a(target: TexParameterTarget, value: TexParameterSwizzle) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_SWIZZLE_A, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_SWIZZLE_A, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_swizzle_a(target: TexParameterTarget) -> TexParameterSwizzle {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_SWIZZLE_A, &mut out as *mut _);
	}
	match out as u32 {
		gl::RED => TexParameterSwizzle::Red,
		gl::GREEN => TexParameterSwizzle::Green,
		gl::BLUE => TexParameterSwizzle::Blue,
		gl::ALPHA => TexParameterSwizzle::Alpha,
		gl::ZERO => TexParameterSwizzle::Zero,
		gl::ONE => TexParameterSwizzle::One,
		_ => unreachable!(),
	}
}

/// [`glTexParameteri(_, GL_TEXTURE_SWIZZLE_RGBA, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_swizzle_rgba(target: TexParameterTarget, value: [TexParameterSwizzle; 4]) {
	let pass = value.map(|v| v as i32);
	unsafe {
		gl::TexParameteriv(target as u32, gl::TEXTURE_SWIZZLE_RGBA, pass.as_ptr());
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_SWIZZLE_RGBA, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_swizzle_rgba(target: TexParameterTarget) -> [TexParameterSwizzle; 4] {
	let mut out = [0i32; 4];
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_SWIZZLE_RGBA, out.as_mut_ptr());
	}
	out.map(|v| match v as u32 {
		gl::RED => TexParameterSwizzle::Red,
		gl::GREEN => TexParameterSwizzle::Green,
		gl::BLUE => TexParameterSwizzle::Blue,
		gl::ALPHA => TexParameterSwizzle::Alpha,
		gl::ZERO => TexParameterSwizzle::Zero,
		gl::ONE => TexParameterSwizzle::One,
		_ => unreachable!(),
	})
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexParameterWrap {
	ClampToEdge = gl::CLAMP_TO_EDGE,
	ClampToBorder = gl::CLAMP_TO_BORDER,
	Repeat = gl::REPEAT,
	MirroredRepeat = gl::MIRRORED_REPEAT,
}

/// [`glTexParameteri(_, GL_TEXTURE_WRAP_S, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_wrap_s(target: TexParameterTarget, value: TexParameterWrap) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_S, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_WRAP_S, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_wrap_s(target: TexParameterTarget) -> TexParameterWrap {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_WRAP_S, &mut out as *mut _);
	}
	match out as u32 {
		gl::CLAMP_TO_EDGE => TexParameterWrap::ClampToEdge,
		gl::CLAMP_TO_BORDER => TexParameterWrap::ClampToBorder,
		gl::REPEAT => TexParameterWrap::Repeat,
		gl::MIRRORED_REPEAT => TexParameterWrap::MirroredRepeat,
		_ => unreachable!(),
	}
}

/// [`glTexParameteri(_, GL_TEXTURE_WRAP_T, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_wrap_t(target: TexParameterTarget, value: TexParameterWrap) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_T, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_WRAP_T, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_wrap_t(target: TexParameterTarget) -> TexParameterWrap {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_WRAP_T, &mut out as *mut _);
	}
	match out as u32 {
		gl::CLAMP_TO_EDGE => TexParameterWrap::ClampToEdge,
		gl::CLAMP_TO_BORDER => TexParameterWrap::ClampToBorder,
		gl::REPEAT => TexParameterWrap::Repeat,
		gl::MIRRORED_REPEAT => TexParameterWrap::MirroredRepeat,
		_ => unreachable!(),
	}
}

/// [`glTexParameteri(_, GL_TEXTURE_WRAP_R, ...)`](https://docs.gl/gl3/glTexParameter)
pub fn tex_parameter_wrap_r(target: TexParameterTarget, value: TexParameterWrap) {
	unsafe {
		gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_R, value as i32);
	}
}

/// [`glGetTexParameteriv(_, GL_TEXTURE_WRAP_R, ...)`](https://docs.gl/gl3/glGetTexParameter)
pub fn get_tex_parameter_wrap_r(target: TexParameterTarget) -> TexParameterWrap {
	let mut out = 0i32;
	unsafe {
		gl::GetTexParameteriv(target as u32, gl::TEXTURE_WRAP_R, &mut out as *mut _);
	}
	match out as u32 {
		gl::CLAMP_TO_EDGE => TexParameterWrap::ClampToEdge,
		gl::CLAMP_TO_BORDER => TexParameterWrap::ClampToBorder,
		gl::REPEAT => TexParameterWrap::Repeat,
		gl::MIRRORED_REPEAT => TexParameterWrap::MirroredRepeat,
		_ => unreachable!(),
	}
}

