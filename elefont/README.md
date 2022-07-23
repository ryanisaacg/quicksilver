# elefont

A library that handles caching rendered glyphs on the GPU

Elefont is a simple caching layer for a font rendering stack, which abstracts over both the font provider and the output format. It's intended for use mostly in games, but could serve in other applications as well.

Scope of this library:
- DO support various font libraries / types of fonts (TTFs, bitmap fonts)
- DO support whatever backend (rendering to an image, GPU frameworks, etc.)
- DON'T handle complex tasks like shaping. The font stack should handle that elsewhere, and
provide this library the glyphs to render
- DON'T handle layout. This can be taken care of by the client
application when rendering.

Support is available out-of-the-box for software rendering via `image`, rendering via
`rusttype`, and performing automatic unicode normalization. All of these are optional features.
