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
You can come up with any data yourself but here aer some examples:

## Title
Title of script. For example: Video name - Typeset or *** logo animation.

## Author
Author of script / script creator.

## Version
Script version. Can contain date, time, phase (alpha, beta, public, private), etc.

## Description
Additional description of script. Something like the purpose, team or anything else what wasn’t written before.

## TARGET section
The TARGET section describes the room dimension interpretation. Coordinates, 2D or 3D, outside of the target aren’t visible. Format of information chunks is...
Type: data

Following types are available:

### Width
Width of the room. Coordinates in range 0 until given width are visible. Default value is video width. If an value is defined different than the video width, rendered frame will be scaled to fit.

### Height
Height of the room. Coordinates in range 0 until given height are visible. Default value is video height. If an value is defined different than the video height, rendered frame will be scaled to fit.

### Depth
Depth of the room. Coordinates in range negative half of given depth until positive half of given depth are visible. Default value is 1000, that means, -500 until +500 are visible depths.

### View
Viewer perspective. Can be ‘orthogonal’ or ‘perspective’. Default is ‘perspective’. In case of ‘orthogonal’, objects are nearing screen center with increasing negative depth and double size with increasing positive depth. Depth 0 means original size.

## MACROS section
The MACROS section defines styles for use in following source blocks. Styles can be understood as macros, because that's what they are: source content with an identify name. Format of styles is...
name: content
Name can be a chain of characters except (obviously) : (U+3A).
Content just shouldn’t be empty, that’s all.

## EVENTS section
The EVENTS section describes what to render at all. Every source block tells the renderer: “when render what”.
Format of source blocks is...

`start-end|template|note|text`

### start-end
Start and end are the times when to start and end rendering.
Times have one of the following formats...
#### Time Based
`[[[hours:]minutes:]seconds.]milliseconds`
#### Event Based
`'event-tag'`

You will be able to pass an array of `event-tag`s to the frame render function of SSB which will allow you to control when lines should be shown from the outside.

### template
Template is the name of a macros defined in the MACROS section. Content of it will be inserted to the beginning of source block text.
### note
Note is just an information beside, renderers ignore it.
### text
Text is a mix of style tags, text and shape descriptions. Everything what should be rendered is written here.

## RESOURCE section
The RESOURCE section describes all resources that will be used within the subtitle format. This includes images and fonts.

### Texture
Texture: TEXTURE_ID, data | url

TEXTURE_ID: can be any character but must not contain comma.
data: base64 encoded string or a url as an absolute path to the file on the file system

### Fonts
Font: font_family, style, data | url

font_family: can be any character but must not contain comma.
style: regular, bold, italic, bold-italic
data: base64 encoded string or a url as an absolute path to the file on the file system

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
Drawings have to be enabled by style tag mode. After this, text is interpreted as drawing command, consisting 2D points.
Different types of drawings are available.
The drawing syntax is almost a carbon copy of the SVG syntax for drawing shapes.

# 5. Styling
## General
Objects can have some style properties like color or position. These will be set by style tags which have to be inserted before object definition. This can happen in a style which assigns to objects source block or directly in text cell.
The default style, excluding user styles or style tags, is defined as...

`[font=Arial; size=20; bold=n; italic=n; underline=n; strikeout=n; encoding=1; position=0,0; direction=0; 
space=0; alignment=7; margin=10; mode=text; border=2; join=round; depth=0; color=FFFFFF; bordercolor=000000; 
alpha=FF; borderalpha=FF; texture=; texfill=0,0,1,0,clamp; blend=normal; target=mask; mask-mode=normal; blur=0;]`

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

Name of font for text rendering. Defined in the resource section or coming from the operating system.

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

Font style. ‘y’ for setting italic, ‘n’ for normal.

#### underline
`[underline=n]`

Font decoration. ‘y’ for underlining, ‘n’ for normal.

#### strikeout
`[strikeout=n]`

Font decoration 2. ‘y’ for striking out, ‘n’ for normal.

### Position
#### position
`[position=0,0,0]`

`[position=0,0]`

Position on screen. 2D or 3D coordinate possible. 0,0 is in the top left corner of the screen.

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

Margin to screen edges in pixel. Only affects line if no position is set.

#### direction
`[direction=LTR]`

Draws text in different directions, usually depends on the writing system (think english vs. japanese vs. hebrew). Default LTR.

Can be: LTR, RTL, TTB, BTT

LTR = left-to-right
RTL = right-to-left
TTB = top-to-down
BTT = bottom-to-top

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

Object scale on room axis in percent. (1 = 100%)

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

You can also directly manipulate the matrix yourself, but beware that setting this may reset/overwrite translate, scale, rotate and shear. 

#### reset
`[reset]`

Resets transformations in source block or in other words, resets the matrix.

### Geometry
#### mode
`[mode=text]`
abcdefghijklmnopqrstuvwxyz

`[mode=shape]`
m - move (m 0 0)

l - line (l 0 0 1 1)

a - arcs (a 0 0 360)

b - cubic bezier (b 0 0 1 1 2 2 3 3)

`[mode=point]`
0 0

Geometry type to draw. Form will be interpreted by non-style content of fourth cell in source block. Default mode is text.

#### border
`[border=2]`

`[border=2,2]`

`[border-h=2]`

`[border-v=2]`

Border width of objects.

#### join
`[join=round]`
Border and line join style. Can be ‘round’, ‘bevel’ or ‘miter’.

#### cap
`[cap=round]`
Border and line join style. Can be ‘round’, 'butt' or ‘square’.

### Textures

#### texture
`[texture=RESOURCE_ID]`

Texture on object. Texturing enabled by a valid RESOURCE_ID referencing an image. Disabled by an invalid value (like an empty value).

// TODO: Do we still want to add the possibility of using the current frame as a texture?

#### texfill
`[texfill=0,0,1,0]`

`[texfill=0,0,1,0,clamp]`

Texture position, size and wrapping.

Offset: The first two numbers describe where the texture starts on the object (as a value between 0 - 1 (0-100%))

Range: The two numbers after that describe how far the texture stretches in respect to the object (as a value between 0 -1 (0-100%))

Wrapping modes: ‘base’, ‘clamp’, ‘repeat’, ‘mirror’. ‘base’ is default.

Wrapping modes describe what happens outside of the texture on the objects (beyond the edges of the texture).

`base` means everywhere there is no texture you will have the standard filling color of the object.

`clamp` means that the last pixel of the texture is repeated in the direction of the edge.

`repeat` means that the texture is repeated in the direction of the edge.

`mirror` means that the texture is repeated but flipped on the respective edge in the direction on the edge.

### Color
#### color
`[color=FFFFFF]`

`[color=FFFFFF,FFFFFF]`

`[color=FFFFFF,FFFFFF,FFFFFF]`

`[color=FFFFFF,FFFFFF,FFFFFF,FFFFFF]`

`[color=FFFFFF,FFFFFF,FFFFFF,FFFFFF,FFFFFF]`

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

#### blur
`[blur=0]`

`[blur=0,0]`

`[blur-h=0]`

`[blur-v=0]`

Gaussian blur on object + border. Sigma value defines strength. Horizontal and vertical blur available.

### Rastering
#### mask
`[target=mask]`

`[target=frame]`

`[mask-mode=normal]`

`[mask-mode=invert]`

`[mask-clear]`

With this you can use one object to mask another, making clipping and holes inside texts etc. possible.

If you set target to `mask` then you will start to render on a different canvas which you are unable to see.
On this canvas only alpha-values exist for each pixel (0-255) so no colors. If you draw any shape on this hidden canvas
with an alpha value of 255, each pixel of said shape will be "masked out" on the original canvas, meaning you will
stance a hole into any shape at said position. If you use 125 alpha instead, the "hole" will be 50% transparent etc.

You can invert this logic by setting `mask-mode` to `invert`.
Beware that if you do this, nothing will show on screen as the hidden canvas will be all 0 per default and by
inverting the logic everything is masked out.

With using `mask-clear` you can reset the entire canvas to 0.

Example:

`[target=mask;mask-mode=invert;alpha=FF;blur=5;]KREIS_SMALL[target=frame;blur=0;]KREIS`

#### blend
`[blend=overlay]`

Blending mode. This will set how this layer is blended with the background. A Background might also be another layer that is drawn behind this layer.

Following modes are available:

overlay: source color (((source alpha * source color)) + ((1 -  source alpha) * destination color))

add: source color + destination color

subtract: source color - destination color

multiply: source color * destination color

invert: ~destination color

difference: abs(source color - destination color)

screen: 1 - (1 - source color) * (1 - destination color)


### Animation
#### animate
`[animate=[color=000000;translate-x=20]]`

`[animate=t,[color=000000;translate-x=20]]`

`[animate=0,1000,[color=000000;translate-x=20]]`

`[animate=500,1000,[color=000000;translate-x=-20]]`

`[animate=0,1000,sin(t*pi),[color=000000;translate-x=20]]`

Interpolates style properties over time.
First version goes over whole source block time to interpolate style tags.
Second version additionally sends interpolation factor of current frame through an equation.
Third version specifies a time zone to interpolate style tags.
Fourth version combines all versions. A description for his values:
First value:
Start time in milliseconds.
Positive number for offset to start time of source block.
Negativ number for offset to end time of source block.
Second value:
End time in milliseconds.
Positive number for offset to start time of source block.
Negativ number for offset to end time of source block.
Third value:
Equation for interpolation factor.
Fourth value:
Style tags to interpolate.

### Karaoke
The standard karaoke effect will be an interpolation from "color" to "kcolor" in the given k duration.

#### k
`[k=100]`

Karaoke duration

#### kset
`[kset=0]`

Reset ktime in ms to start-time of line.

#### kcolor
`[kcolor=FF00FF]`

This will create something akin to the following animation tag for each syllable:

`[animate=kStart,kEnd,t^0.5,[color=FF00FF]]`

Color to which color the karaoke will interpolate within the timespam of the duration. For color syntax see the color tag.

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

#MACROS
Default: [bold=y]
Mine: [bold=n;color=FF0000]
Another: [Mine;position=100,200,-1;rotate-z=50%]I'm a

#TARGET
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


