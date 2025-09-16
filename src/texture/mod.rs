
mod params;
pub use params::*;

use std::ffi::c_void;

use super::msg::ERROR_OOB;


/// wrapper over an OpenGL "Texture Object" name.
#[derive(Debug)]
pub struct TextureObject(u32);
impl TextureObject {
	pub fn handle(&self) -> u32 {
		self.0
	}
}

impl Drop for TextureObject {
	fn drop(&mut self) {
		delete_textures([
			// safety: obviously safe
			unsafe { std::ptr::read(self) }
		]);
	}
}

/// [`glGenTextures()`](https://docs.gl/gl3/glGenTextures)
pub fn gen_textures<const N: usize>() -> [TextureObject; N] {
	let mut list = [0; N];
	unsafe {
		gl::GenTextures(N.try_into().expect(ERROR_OOB), list.as_mut_ptr());
	}
	list.map(|v| TextureObject(v))
}

/// [`glDeleteTextures()`](https://docs.gl/gl3/glDeleteTextures)
pub fn delete_textures<const N: usize>(textures: [TextureObject; N]) {
	let list = textures.map(|v| std::mem::ManuallyDrop::new(v).handle());
	unsafe {
		gl::DeleteTextures(N.try_into().expect(ERROR_OOB), list.as_ptr());
	}
}

/// [`glIsTexture()`](https://docs.gl/gl3/glIsTexture)
pub fn is_texture(texture: &TextureObject) -> bool {
	unsafe {
		gl::IsTexture(texture.handle()) == gl::TRUE
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BindTextureTarget {
	/// GL_TEXTURE_1D
	Texture1D = gl::TEXTURE_1D,
	/// GL_TEXTURE_1D_ARRAY
	Texture1DArray = gl::TEXTURE_1D_ARRAY,
	/// GL_TEXTURE_2D
	Texture2D = gl::TEXTURE_2D,
	/// GL_TEXTURE_2D_ARRAY
	Texture2DArray = gl::TEXTURE_2D_ARRAY,
	/// GL_TEXTURE_2D_MULTISAMPLE
	Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
	/// GL_TEXTURE_2D_MULTISAMPLE_ARRAY
	Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
	/// GL_TEXTURE_3D
	Texture3D = gl::TEXTURE_3D,
	/// GL_TEXTURE_CUBE_MAP
	TextureCubeMap = gl::TEXTURE_CUBE_MAP,
	/// GL_TEXTURE_CUBE_MAP_ARRAY
	TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
	/// GL_TEXTURE_RECTANGLE
	TextureRectangle = gl::TEXTURE_RECTANGLE,
}

/// [`glBindTexture()`](https://docs.gl/gl3/glBindTexture)
pub fn bind_texture(target: BindTextureTarget, texture: &TextureObject) {
	unsafe {
		gl::BindTexture(target as u32, texture.handle());
	}
	// todo: GL_INVALID_OPERATION error
}

/// [`glBindTexture(_, 0)`](https://docs.gl/gl3/glBindTexture)
pub fn unbind_texture(target: BindTextureTarget) {
	unsafe {
		gl::BindTexture(target as u32, 0);
	}
}


/// [`glActiveTexture()`](https://docs.gl/gl3/glActiveTexture)
pub fn active_texture(unit: u32) {
	// todo: support more?
	assert!(gl::TEXTURE0 <= unit && unit <= gl::TEXTURE15);
	unsafe {
		gl::ActiveTexture(unit);
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelStoreNames {
	UnpackSwapBytes = gl::UNPACK_SWAP_BYTES,
	UnpackLsbFirst = gl::UNPACK_LSB_FIRST,
	UnpackRowLength = gl::UNPACK_ROW_LENGTH,
	UnpackSkipRows = gl::UNPACK_SKIP_ROWS,
	UnpackSkipPixels = gl::UNPACK_SKIP_PIXELS,
	UnpackAlignment = gl::UNPACK_ALIGNMENT,
	UnpackImageHeight = gl::UNPACK_IMAGE_HEIGHT,
	UnpackSkipImages = gl::UNPACK_SKIP_IMAGES,
}

pub unsafe fn pixel_store_i(name: PixelStoreNames, value: i32) {
	// todo: input validation
	unsafe {
		gl::PixelStorei(name as u32, value);
	}
}

pub unsafe fn pixel_store_f(name: PixelStoreNames, value: f32) {
	unsafe {
		gl::PixelStoref(name as u32, value);
	}
}



#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexImageInnerFormat {
	// base formats
	/// GL_RED
	Red = gl::RED,
	/// GL_RG
	RG = gl::RG,
	/// GL_RGB
	RGB = gl::RGB,
	/// GL_RGBA
	RGBA = gl::RGBA,
	/// GL_DEPTH_COMPONENT
	DepthComponent = gl::DEPTH_COMPONENT,
	/// GL_DEPTH_STENCIL
	DepthStencil = gl::DEPTH_STENCIL,

	// texture and renderbuffer color formats
	RGBA32F = gl::RGBA32F,
	RGBA32I = gl::RGBA32I,
	RGBA32UI = gl::RGBA32UI,
	RGBA16 = gl::RGBA16,
	RGBA16F = gl::RGBA16F,
	RGBA16I = gl::RGBA16I,
	RGBA16UI = gl::RGBA16UI,
	RGBA8 = gl::RGBA8,
	RGBA8I = gl::RGBA8I,
	RGBA8UI = gl::RGBA8UI,
	SRGB8Alpha8 = gl::SRGB8_ALPHA8,
	RGB10A2 = gl::RGB10_A2,
	RGB10A2UI = gl::RGB10_A2UI,

	R11FG11FB10F = gl::R11F_G11F_B10F,

	RG32F = gl::RG32F,
	RG32I = gl::RG32I,
	RG32UI = gl::RG32UI,
	RG16 = gl::RG16,
	RG16F = gl::RG16F,
	RG16I = gl::RG16I,
	RG16UI = gl::RG16UI,
	RG8 = gl::RG8,
	RG8I = gl::RG8I,
	RG8UI = gl::RG8UI,
	
	// texture-only color formats
	RGBA16SNorm = gl::RGBA16_SNORM,
	RGBA8SNorm = gl::RGBA8_SNORM,

	RGB32F = gl::RGB32F,
	RGB32I = gl::RGB32I,
	RGB32UI = gl::RGB32UI,

	RGB16SNorm = gl::RGB16_SNORM,
	RGB16F = gl::RGB16F,
	RGB16I = gl::RGB16I,
	RGB16UI = gl::RGB16UI,
	RGB16 = gl::RGB16,

	RGB8SNorm = gl::RGB8_SNORM,
	RGB8 = gl::RGB8,
	RGB8I = gl::RGB8I,
	RGB8UI = gl::RGB8UI,
	SRGB8 = gl::SRGB8,

	RGB9E5 = gl::RGB9_E5,

	RG16SNorm = gl::RG16_SNORM,
	RG8SNorm = gl::RG8_SNORM,

	R16SNorm = gl::R16_SNORM,
	R8SNorm = gl::R8_SNORM,

	// depth formats
	DepthComponent32F = gl::DEPTH_COMPONENT32F,
	DepthComponent24 = gl::DEPTH_COMPONENT24,
	DepthComponent16 = gl::DEPTH_COMPONENT16,

	// combined depth+stencil formats
	Depth32FStencil8 = gl::DEPTH32F_STENCIL8,
	Depth24Stencil8 = gl::DEPTH24_STENCIL8,

	// compressed formats
	CompressedRed = gl::COMPRESSED_RED,
	CompressedRG = gl::COMPRESSED_RG,
	CompressedRGB = gl::COMPRESSED_RGB,
	CompressedRGBA = gl::COMPRESSED_RGBA,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexImageDataFormat {
	Red = gl::RED,
	Green = gl::GREEN,
	Blue = gl::BLUE,
	RG = gl::RG,
	RGB = gl::RGB,
	RGBA = gl::RGBA,
	BGR = gl::BGR,
	BGRA = gl::BGRA,

	RedInteger = gl::RED_INTEGER,
	GreenInteger = gl::GREEN_INTEGER,
	BlueInteger = gl::BLUE_INTEGER,
	RGInteger = gl::RG_INTEGER,
	RGBInteger = gl::RGB_INTEGER,
	RGBAInteger = gl::RGBA_INTEGER,
	BGRInteger = gl::BGR_INTEGER,
	BGRAInteger = gl::BGRA_INTEGER,
	
	DepthComponent = gl::DEPTH_COMPONENT,
	DepthStencil = gl::DEPTH_STENCIL,
	StencilIndex = gl::STENCIL_INDEX,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TexImageDataType {
	UnsignedByte = gl::UNSIGNED_BYTE,
	Byte = gl::BYTE,

	UnsignedShort = gl::UNSIGNED_SHORT,
	Short = gl::SHORT,

	UnsignedInt = gl::UNSIGNED_INT,
	Int = gl::INT,

	HalfFloat = gl::HALF_FLOAT,
	Float = gl::FLOAT,

	UnsignedByte_3_3_2 = gl::UNSIGNED_BYTE_3_3_2,
	UnsignedByte_2_3_3_Rev = gl::UNSIGNED_BYTE_2_3_3_REV,
	
	UnsignedShort_5_6_5 = gl::UNSIGNED_SHORT_5_6_5,
	UnsignedShort_5_6_5_Rev = gl::UNSIGNED_SHORT_5_6_5_REV,
	UnsignedShort_4_4_4_4 = gl::UNSIGNED_SHORT_4_4_4_4,
	UnsignedShort_4_4_4_4_Rev = gl::UNSIGNED_SHORT_4_4_4_4_REV,
	UnsignedShort_5_5_5_1 = gl::UNSIGNED_SHORT_5_5_5_1,
	UnsignedShort_1_5_5_5_Rev = gl::UNSIGNED_SHORT_1_5_5_5_REV,
	
	UnsignedInt_8_8_8_8 = gl::UNSIGNED_INT_8_8_8_8,
	UnsignedInt_8_8_8_8_Rev = gl::UNSIGNED_INT_8_8_8_8_REV,
	UnsignedInt_10_10_10_2 = gl::UNSIGNED_INT_10_10_10_2,
	UnsignedInt_2_10_10_10_Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
}

// surely these can be shared across these functions, right?
fn util_datavalidation(
	inner_format: TexImageInnerFormat,
	data_format: TexImageDataFormat,
	data_type: TexImageDataType,
) -> bool {
	let a = match data_type {
		| TexImageDataType::UnsignedByte_3_3_2
		| TexImageDataType::UnsignedByte_2_3_3_Rev
		| TexImageDataType::UnsignedShort_5_6_5
		| TexImageDataType::UnsignedShort_5_6_5_Rev =>
			match data_format {
				TexImageDataFormat::RGB => true,
				_ => false,
			},
		| TexImageDataType::UnsignedShort_4_4_4_4
		| TexImageDataType::UnsignedShort_4_4_4_4_Rev
		| TexImageDataType::UnsignedShort_5_5_5_1
		| TexImageDataType::UnsignedShort_1_5_5_5_Rev
		| TexImageDataType::UnsignedInt_8_8_8_8
		| TexImageDataType::UnsignedInt_8_8_8_8_Rev
		| TexImageDataType::UnsignedInt_10_10_10_2
		| TexImageDataType::UnsignedInt_2_10_10_10_Rev =>
			match data_format {
				| TexImageDataFormat::RGBA
				| TexImageDataFormat::BGRA => true,
				_ => false,
			},
		_ => true,
	};

	let b = match inner_format {
		| TexImageInnerFormat::DepthComponent
		| TexImageInnerFormat::DepthComponent16
		| TexImageInnerFormat::DepthComponent24
		| TexImageInnerFormat::DepthComponent32F => 
			match data_format {
				| TexImageDataFormat::DepthComponent => true,
				_ => false,
			},
		_ => match data_format {
				| TexImageDataFormat::DepthComponent => false,
				_ => true,
			},
	};

	return a && b;
}

fn util_samplesize(
	data_format: TexImageDataFormat,
	data_type: TexImageDataType,
) -> Option<usize> {
	let stride = match data_format {
		TexImageDataFormat::Red => 1,
		TexImageDataFormat::RG => 2,
		TexImageDataFormat::RGB
		| TexImageDataFormat::BGR => 3,
		TexImageDataFormat::RGBA
		| TexImageDataFormat::BGRA => 4,
		// deal with it later
		TexImageDataFormat::DepthComponent => 1,
		// what the fuck ???
		_ => return None,
	};

	let size = match data_type {
		| TexImageDataType::Byte
		| TexImageDataType::UnsignedByte
			=> stride * 1,
		| TexImageDataType::UnsignedByte_3_3_2
		| TexImageDataType::UnsignedByte_2_3_3_Rev
			=> 1,
		| TexImageDataType::Short
		| TexImageDataType::UnsignedShort
			=> stride * 2,
		| TexImageDataType::UnsignedShort_5_6_5
		| TexImageDataType::UnsignedShort_5_6_5_Rev
		| TexImageDataType::UnsignedShort_4_4_4_4
		| TexImageDataType::UnsignedShort_4_4_4_4_Rev
		| TexImageDataType::UnsignedShort_5_5_5_1
		| TexImageDataType::UnsignedShort_1_5_5_5_Rev
			=> 2,
		| TexImageDataType::Int
		| TexImageDataType::UnsignedInt
		| TexImageDataType::Float
			=> stride * 4,
		| TexImageDataType::UnsignedInt_8_8_8_8
		| TexImageDataType::UnsignedInt_8_8_8_8_Rev
		| TexImageDataType::UnsignedInt_10_10_10_2
		| TexImageDataType::UnsignedInt_2_10_10_10_Rev
			=> 4,
		_ => return None,
	};

	return Some(size);
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexImage1DTarget {
	Texture1D = gl::TEXTURE_1D,
	ProxyTexture1D = gl::PROXY_TEXTURE_1D,
}


/// [`glTexImage1D()`](https://docs.gl/gl3/glTexImage1D)
pub fn tex_image_1d(
	target: TexImage1DTarget,
	level: u16,
	inner_format: TexImageInnerFormat,
	width: usize,
	data_format: TexImageDataFormat,
	data_type: TexImageDataType,
	data: Option<&[u8]>,
) {
	debug_assert!(util_datavalidation(inner_format, data_format, data_type), "invalid format");

	if let Some(bytes) = data {
		let size = util_samplesize(data_format, data_type);
		debug_assert!(
			size.map(|x| width * x <= bytes.len()).unwrap_or(false),
			"invalid width",
		);
	}

	/*
	safety:
	the main concern is if `width` is more than the provided `data` buffer... this is
	validated by calculating how many bytes each sample would be read as, then checking
	if that number * `width` is less than `data.len()`.
	it's okay if it is less, but this function panics if the number is larger. (hopefully -
	barring opengl bullshit i couldnt have seen coming.)
	*/ 
	unsafe {
		gl::TexImage1D(
			target as u32,
			level as i32,
			inner_format as i32,
			width as i32,
			0,
			data_format as u32,
			data_type as u32,
			match data {
				Some(v) => v.as_ptr() as *const c_void,
				None => std::ptr::null(),
			},
		);
	}
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexImage2DTarget {
	/// GL_TEXTURE_2D
	Texture2D = gl::TEXTURE_2D,
	// GL_TEXTURE_1D_ARRAY
	Texture1DArray = gl::TEXTURE_1D_ARRAY,
	// GL_TEXTURE_RECTANGLE
	TextureRectangle = gl::TEXTURE_RECTANGLE,
	TextureCubeMapPositiveX = gl::TEXTURE_CUBE_MAP_POSITIVE_X,
	TextureCubeMapNegativeX = gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
	TextureCubeMapPositiveY = gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
	TextureCubeMapNegativeY = gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
	TextureCubeMapPositiveZ = gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
	TextureCubeMapNegativeZ = gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
	ProxyTexture2D = gl::PROXY_TEXTURE_2D,
	ProxyTexture1DArray = gl::PROXY_TEXTURE_1D_ARRAY,
	ProxyTextureRectangle = gl::PROXY_TEXTURE_RECTANGLE,
	ProxyTextureCubeMap = gl::PROXY_TEXTURE_CUBE_MAP,
}

/// [`glTexImage2D()`](https://docs.gl/gl3/glTexImage2D)
pub fn tex_image_2d(
	target: TexImage2DTarget,
	level: u16,
	inner_format: TexImageInnerFormat,
	width: usize,
	height: usize,
	data_format: TexImageDataFormat,
	data_type: TexImageDataType, 
	data: Option<&[u8]>
) {
	// level must be zero for texturerectangles
	debug_assert!(match target {
		| TexImage2DTarget::TextureRectangle
		| TexImage2DTarget::ProxyTextureRectangle
		=> level == 0,
		_ => true,
	});

	// width and height must be equal in cubemap targets
	debug_assert!(match target {
		| TexImage2DTarget::TextureCubeMapNegativeX
		| TexImage2DTarget::TextureCubeMapNegativeY
		| TexImage2DTarget::TextureCubeMapNegativeZ
		| TexImage2DTarget::TextureCubeMapPositiveX
		| TexImage2DTarget::TextureCubeMapPositiveY
		| TexImage2DTarget::TextureCubeMapPositiveZ
			=> width == height,
		_ => true,
	});

	// todo: 0 <= width <= GL_MAX_TEXTURE_SIZE ?

	// todo: if target != texture1darrays, 0 <= height GL_MAX_TEXTURE_SIZE
	// else, 0 <= height <= GL_MAX_ARRAY_TEXTURE_LAYERS

	// todo: level > log_2(GL_MAX_TEXTURE_SIZE)

	// todo: format checking

	if let Some(bytes) = data {
		let size = util_samplesize(data_format, data_type);
		debug_assert!(
			size.map(|x| width * height * x <= bytes.len()).unwrap_or(false),
			"invalid width and height",
		);
	}
	
	unsafe {
		gl::TexImage2D(
			target as u32,
			level as i32,
			inner_format as i32,
			width as i32,
			height as i32,
			0,
			data_format as u32,
			data_type as u32,
			match data {
				Some(v) => v.as_ptr() as *const c_void,
				None => std::ptr::null(),
			},
		);
	}
}



#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexImage3DTarget {
	Texture3D = gl::TEXTURE_3D,
	Texture2DArray = gl::TEXTURE_2D_ARRAY,
	ProxyTexture3D = gl::PROXY_TEXTURE_3D,
	ProxyTexture2DArray = gl::PROXY_TEXTURE_2D_ARRAY,
}

/// [`glTexImage3D()`](https://docs.gl/gl3/glTexImage3D)
pub fn tex_image_3d(
	target: TexImage3DTarget,
	level: u16,
	inner_format: TexImageInnerFormat,
	width: usize,
	height: usize,
	depth: usize,
	data_format: TexImageDataFormat,
	data_type: TexImageDataType, 
	data: Option<&[u8]>
) {
	debug_assert!(data_format != TexImageDataFormat::StencilIndex);
	debug_assert!(match inner_format {
		| TexImageInnerFormat::DepthStencil
		| TexImageInnerFormat::Depth24Stencil8
		| TexImageInnerFormat::Depth32FStencil8
		=> data_format == TexImageDataFormat::DepthStencil,
		_ => true,
	});
	// todo: input validation

	if let Some(bytes) = data {
		let size = util_samplesize(data_format, data_type);
		debug_assert!(
			size.map(|x| width * height * depth * x <= bytes.len()).unwrap_or(false),
			"invalid width, height, and depth",
		);
	}

	unsafe {
		gl::TexImage3D(
			target as u32,
			level as i32,
			inner_format as i32,
			width as i32,
			height as i32,
			depth as i32,
			0,
			data_format as u32,
			data_type as u32,
			match data {
				Some(v) => v.as_ptr() as *const c_void,
				None => std::ptr::null(),
			}
		);
	}
}




