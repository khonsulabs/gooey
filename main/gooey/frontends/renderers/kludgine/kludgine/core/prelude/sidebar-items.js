initSidebarItems({"enum":[["AnimationMode","The animation mode of the sprite."],["PathEvent","An entry in a [`Path`]."],["Text","Text rendering functionality"]],"macro":[["include_aseprite_sprite","Includes an Aseprite sprite sheet and Json export. For more information, see [`Sprite::load_aseprite_json`]. This macro will append “.png” and “.json” to the path provided and include both files in your binary."],["include_font","Embeds a font into your executable."],["include_texture","Embeds a texture in the binary."]],"struct":[["Batch","A batch of shapes that can be rendered together."],["Color","A RGBA color with f32 components."],["Fill","Shape fill options."],["Font","Font provides TrueType Font rendering"],["FrameRenderer","Renders frames created by a `Scene`."],["Length","A one-dimensional distance, with value represented by `T` and unit of measurement `Unit`."],["Path","A geometric shape defined by a path."],["PathBuilder","Builds a [`Path`]."],["PreparedSpan","A formatted span of text that is ready to render. Cheap to clone."],["Raw","A unit representing physical pixels on a display."],["Scale","A scaling factor between two different units of measurement."],["Scaled","A unit representing Desktop publishing points/PostScript points. Measurements in this scale are equal to 1/72 of an imperial inch."],["Scene","The main rendering destination, usually interacted with through [`Target`]."],["Shape","A 2d shape."],["Sprite","A sprite is a renderable graphic with optional animations."],["SpriteAnimation","An animation of one or more [`SpriteFrame`]s."],["SpriteAnimations","A collection of [`SpriteAnimation`]s. This is an immutable object that shares data when cloned to minimize data copies."],["SpriteFrame","A single frame for a [`SpriteAnimation`]."],["SpriteMap","A collection of [`SpriteSource`]s."],["SpriteRotation","A rotation of a sprite."],["SpriteSheet","A collection of sprites from a single [`Texture`]."],["SpriteSource","A sprite’s source location and texture. Cheap to clone."],["SpriteSourceSublocation","A sub-location of a joined sprite."],["Stroke","A shape stroke (outline) options."],["Target","A render target"],["Texture","An image that can be used as a sprite. Cheap to clone."],["Unknown","A unit representing"]],"trait":[["PointExt","Extension trait for [`Point`]."],["ShutdownCallback","A callback that can be invoked when a [`FrameRenderer`] is fully shut down."],["SizeExt","Extension trait for [`Size`]."],["SpriteCollection","A collection of sprites."]],"type":[["Angle","A type representing an angle of measurement."],["ControlPoint","A control point used to create curves."],["Endpoint","A point on a [`Path`]."],["Pixels","A measurement of length using [`Raw`] as the unit."],["Point","A type representing an x and y coordinate."],["Points","A measurement of length using [`Scaled`] as the unit."],["Rect","A type representing a [`Point`] and [`Size`]."],["ScreenScale","The scale used to convert between [`Points`] ([`Scaled`]) and [`Pixels`] ([`Raw`])."],["Size","A type representing a width and height."],["Vector","A type representing a vector with magnitudes x and y."]]});