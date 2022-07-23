# Changelog

## v0.1.7
- Fix a crash that occurs when reducing the number of vertex attributes bound

## v0.1.6
- Add a function `borrow_texture` to `Surface`

## v0.1.5
- Fix a panic on the web when using the depth testing API

## v0.1.4 (YANKED)
- Add an API for depth testing, via `DepthTestMode` and `DepthTestFunction`

## v0.1.3
- Export the version of glow used
- Add the std feature
- impl Error for GolemError (if std is enabled)

## v0.1.2
- Fix a possible panic when setting subimage data

## v0.1.1
- FIX: Don't crash when creating non-pow-2 textures
- Add no_std support (though glow still requires std)
- Update the version of blinds in the dev-dependency

## v0.1.0
- [Breaking] Add mipmap texture filter variants, and error cases for when mipmaps are unavailable
- Non-power-of-2 textures are now supported, but they don't have mipmaps
- Add methods for the size of a surface

## v0.1.0-alpha6
- Indicate the default blend mode in the docs
- Rework the Surface API to prevent texture loops
