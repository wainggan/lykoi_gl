
# lykoi_gl

this crate provides a stupid simple, mostly type safe,
not-at-all-idiomatic, incomplete wrapper over some vague version of OpenGL.

importantly, this has an api that is as close to the C api as reasonable, though multiple
concessions were made in the name of "safety". namely:

- the `glGet` functions (and those similar in nature) are split into multiple functions
per output-requested. ie: `glGetIntegerv(GL_ACTIVE_TEXTURE, ...)` vs [`get_active_texture()`].
- otherwise, inputs of `glEnum` are represented with Rust enums.
- others?

all functions are themselves thin wrappers over bindings provided by the `gl` crate,
re-exported via [`raw`].

50% sure there is a vulnerability somewhere. I have no idea what I'm doing...

good luck.

