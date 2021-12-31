# Text

Text in Rusty Engine is a lot like sprites. It has the same set of translation, rotation, scale, and layer fields. It is placed using the same world coordinate system as sprites. (Indeed, Rusty Engine went to great pains to use the same coordinate system for _everything_: rather than having separate coordinate systems for world and screen space, there is only world space.) Only instead of being based on a rectangular image file, text is based on a string value combined with a font and font size which are used to generate an image at runtime.
