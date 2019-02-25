# Specification

# 1. Preamble

# 2. Features
*	Human-friendly syntax
*	Small text, much effect
*	Tiny script information - few and short chunks, just needed stuff
*	Scene definition: how to interprete depth and screen resolution
*	Styles/macros: classification of source blocks with beginning style+text
*	Flexible timestamps, source block notes and raw text insertion
*	Powerful animation, controlled by math expressions
*	2D
*	Much influence in positioning
*	Many effects: textures, individual transformations, ...
*	Fast rendering (=softsubs)

# 3. File
## Encoding
SSB files have to be saved in following encoding:
*	UTF-8
*	UTF-16 LE/BE
For UTF-16, the BOM is needed for reading mode detection.
Because SSB files are text files, except for horizontal tab (U+9), new line (U+A) and carriage return (U+D) non other characters below U+20 are allowed as content.
Anyway, carriage return will be ignored. New line alone starts a new line (see newline of text files).

Obviously, SSB file names end with .SSB.
Muxed into a matroska file, they have codec id S_TEXT/SSB.

## Structure
The SSB structure depends on different sections to describe the file creation, source to render and destination frame. Every information chunk is one text line in an SSB file.
INI and CSV influenced the section structure, BBC influenced the style tags structure.
Sections headers are single lines and starting with # (U+23) followed by the section name.
\#SECTION_NAME
Invalid sections will be ignored, so nothing happens until a valid section starts.

## INFO section
The INFO section holds information about the script itself. Format of information chunks is...
Type: data
Following types are available:
Title
Title of script. For example: Video name - Typeset or *** logo animation.

## Author
Author of script / script creator.

## Version
Script version. Can contain date, time, phase (alpha, beta, public, private), etc.

## Description
Additional description of script. Something like the purpose, team or anything else what wasn’t written before.

## TARGET section
The TARGET section describes the room dimension interpretation. Coordinates, 2D or 3D, outside of the room aren’t visible. Format of information chunks is...
Type: data
Following types are available:
### Width
Width of the room. Coordinates in range 0 until given width are visible. Default value is video width. If an value is defined different than the video width, rendered frame will be scaled to fit.

### Height
Height of the room. Coordinates in range 0 until given height are visible. Default value is video height. If an value is defined different than the video height, rendered frame will be scaled to fit.

### Depth
Depth of the room. Coordinates in range negative half of given depth until positive half of given depth are visible. Default value is 1000, that means, -500 until +500 are visible depths.

### View
Viewer perspective. Can be ‘orthogonal’ or ‘perspective’. Default is ‘orthogonal’. In case of ‘perspective’, objects are nearing screen center with increasing negative depth and double size with increasing positive depth. Depth 0 means original size.

## MACROS section
The MACROS section defines styles for use in following source blocks. Styles can be understand as macros, because that they are: source content with an identify name. Format of styles is...
name: content
Name can be a chain of characters except (obviously) : (U+3A).
Content just shouldn’t be empty, that’s all.

## EVENT section
The EVENT section describes what to render at all. Every source block tells the renderer: “when render what”.
Format of source blocks is...

`start-end|template|note|text`

### start-end
Start and end are the times when to start and end rendering.
Times have one of the following formats...
#### Time Based
`[[[hours:]minutes:]seconds.]milliseconds`
#### Event Based
`'event-tag'`

### template
Template is the name of a macros defined in the MACROS section. Content of it will be inserted to the beginning of source block text.
### note
Note is just an information beside, renderers ignore it.
### text
Text is a mix of style tags, text and shape descriptions. Everything what should be rendered is written here.

TODO

# 4. Objects
## Comment
Comments are source blocks which don’t display anything on render target.
They can contain line information, like time, style, note and text, so they can be used as backups or multiline notes.
Commented source blocks differ from others by two / (U+2F) before the first cell.

`//0-2:0.0|||Nothing to see.`

## Text
Text is the default mode of geometry rendering. Everything written in fourth cell of a source block, which isn’t enclosed by [] (also not a bunch of style tags), will be rendered as text if there’s no other type defined  by style tag mode (explanation later).

`0-2:0.0|||Hello World!`

## Drawing
Drawings have to be enabled by style tag mode. After this, text is interpreted as drawing command, consisting 2D or 3D points.
Different types of drawings are available: from pixels over 3D bodies of triangles to simple 2D shapes.
The drawing syntax will be likely a carbon copy of the SVG syntax for drawing shapes.

# 5. Styling
## General
Objects can have some style properties like color or position. These will be set by style tags which have to be inserted before object definition. This can happen in a style which assigns to objects source block or directly in text cell.
The default style, excluding user styles or style tags, is defined as...

`[font=Arial; size=20; bold=n; italic=n; underline=n; strikeout=n; encoding=1; position=0,0; direction=0; space=0; alignment=7; margin=10; mode=text; border=2; join=round; depth=0; color=FFFFFF; bordercolor=000000; alpha=FF; borderalpha=FF; texture=; texfill=0,0,1,0,1,1,0,1; blend=normal; stencil=clear; blur=0;]`

Animations, transformations and deformation aren’t static properties, so they aren’t listed above.
Transformations are a special case, because they stack and have to be cleared by reset style tag. Animations interpolate transformations from its non-affecting identity, so an animation of a translation starts with 0/0/0.
In case of no use of position, direction or any transformation tag, objects will be wrapped by margin rule to screen edges. Object rows nearer the related edge of alignment are wider then rows farther away.
Setting a new position inside a text cell resets current object position. Following text wouldn’t continue it’s position flow from text before.

## Escape characters
Characters |, [ and ] are identificators for source blocks, so they can’t be used in source cells directly. Add \ before these characters (and \ itself) to escape and use them as content f.e. for texts or notes.

## Style-Tags
### Font
#### font
`[font=Arial]`

Name of font for text rendering.

#### size
`[size=20]`

In text mode: size of font in pixel.
In point mode: point range in pixel.
In line/s mode: line width in pixel.

#### bold
`[bold=n]`

Font weight. ‘y’ for bold, ‘n’ for normal.

#### italic
`[italic=n]`

Font style. ‘y’ for italic, ‘n’ for normal.

#### underline
`[underline=n]`

Font decoration. ‘y’ for underlined, ‘n’ for normal.

#### strikeout
`[strikeout=n]`

Font decoration 2. ‘y’ for strikeouted, ‘n’ for normal.

### Position
#### position
`[position=0,0]`

`[position=0,0,0]`

Position on screen. 2D or 3D coordinate possible.

#### alignment
`[alignment=7]`

`[alignment=0,0]`

Alignment of object at position point.
One value: see keyboard numpad for anchor point definition.
Two values: horizontal and vertical offset from anchor point as object width and height in percent.

#### margin
`[margin=10]`

`[margin=10,10,10,10]`

`[margin-top=10]`

`[margin-right=10]`

`[margin-bottom=10]`

`[margin-left=10]`
Margin to screen edges.

#### direction
// TODO: Think about how much this makes sense (japanese, hibru)
`[direction=0]`

Direction of object position flow as angle (f.e. ‘-90’ would be vertical down).

#### space
`[space=0]`

`[space=0,0]`

`[space-h=0]`

`[space-v=0]`

Space between objects. For text, horizontal space between characters and vertical space between lines are defined too.

### Transformation
#### rotate
`[rotate=0,0,0]`

`[rotate-x=0]`

`[rotate-y=0]`

`[rotate-z=0]`

Object rotation on room axis in degree.

#### scale
`[scale=1,1,1]`

`[scale-x=1]`

`[scale-y=1]`

`[scale-z=1]`

Object scale on room axis in percent.

#### translate
`[translate=0,0,0]`

`[translate-x=0]`

`[translate-y=0]`

`[translate-z=0]`

Object translation in room in pixel.

#### shear
`[shear=0,0]`

`[shear-x=0]`

`[shear-y=0]`
Object shearing in room as weight.

#### matrix
`[matrix=1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]`

Additional transformation matrix for object.

#### reset
`[reset]`

Resets transformations in source block.

### Geometry
#### mode
`[mode=text]`

#### border
`[border=2]`

`[border=2,2]`

`[border-h=2]`

`[border-v=2]`

Borderwidth of objects. No effect on objects with depth.

#### join
`[join=round]`
Border and line join style. Can be ‘round’, ‘bevel’ or ‘miter’.

#### cap
`[cap=round]`
Border and line join style. Can be ‘round’, 'butt' or ‘square’.

### Color
#### color
// TODO: color as one word for everything is confusing
`[color=FFFFFF]`

`[color-gradient=FFFFFF,FFFFFF]`

`[color-gradient=FFFFFF,FFFFFF,FFFFFF]`

`[color-corners=FFFFFF,FFFFFF,FFFFFF,FFFFFF]`

`[color-corners-mid=FFFFFF,FFFFFF,FFFFFF,FFFFFF,FFFFFF]`

`[bordercolor=000000]`

`[bordercolor=000000,000000]`

`[bordercolor=000000,000000,000000]`

`[bordercolor=000000,000000,000000,000000]`

`[bordercolor=000000,000000,000000,000000,000000]`

Color for object or his border. Can be mono, left-to-right gradient, left-to-mid-to-right gradient, 4-corners gradient or 4-corners + center gradient. Color definition in hexadecimal (RGB).

#### alpha
`[alpha=FF]`

`[alpha=FF,FF]`

`[alpha=FF,FF,FF,FF]`

`[alpha=FF,FF,FF,FF,FF]`

`[borderalpha=FF]`

`[borderalpha=FF,FF]`

`[borderalpha=FF,FF,FF,FF]`

`[borderalpha=FF,FF,FF,FF,FF]`

Transparency for object or his border. Can be mono, left-to-right gradient, left-to-mid-to-right gradient, 4-corners gradient or 4-corners + center gradient. Value in hexadecimal (00 = invisible, FF = opaque).

#### blend
`[blend=normal]`

normal: source color * source alpha + destination color * (1 - source alpha)

#### blur
`[blur=0]`

`[blur=0,0]`

`[blur-h=0]`

`[blur-v=0]`

Gaussian blur on object + border. Sigma value defines strength. Horizontal and vertical blur available.

# 6. Examples
## Minimal
This is an absolute minimal example.
Informations don’t influence the render result. The destination takes video and default values. Styles are just macros, optional. Only the source is needed for an output.
```
#SOURCE
|1.0-5:0.0|||Boring line.|
```

## Event based example
This is an example that shows how event based times would look like
Informations don’t influence the render result. The destination takes video and default values. Styles are just macros, optional. Only the source is needed for an output.
```
#SOURCE
|'EVENT'|||Boring line.|
```

## Extended
This is an extended example which shows a lot of SSB properties.
Script informations, destination values or styles are very useful if a script have to be shared with others or edited in future again.
```
#INFO
Title: My new project
Author: Youka
Version: 16.06.2012
Description: First concept of a new render format.

#DESTINATION
Width: 1280
Height: 720
Depth: 1000
View: perspective

#STYLES
Default: [bold=y]
Mine: [bold=n;color=FF0000]
Another: [Mine;position=100,200,-1;rotate-z=50%]I'm a

#SOURCE
//0-2.0|||This line is a comment over 2 seconds!
2.0-5:0.0|Another|Hello, i'm a note!|red,    rotated\ntext over multiple lines.
5:0.0-2:5:0.0|Mine|Draw sth.|[mode=shape; texture=../ramen.tga]m 0 0 l 50.5 0 50.5 20.125 0 20.125
```

# 7. Credits
References:
*	ASS specification [2]
*	VSFilterMod - NewTags
*	SVG

Thanks to...
*   Youka
*	McWhite
*	OutlawJones

# 8. License
This specification is licensed under a Creative Commons Attribution-NoDerivs 3.0 license.
You can share it in commercial or non-commercial way, but have to mention the author, keep it uncut and don’t change content.


