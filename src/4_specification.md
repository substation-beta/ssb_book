# Specification

# 1. Features
*	Human-friendly syntax
*	Small text, much effect
*	Tiny script information - few and short chunks, just needed stuff
*	Scene definition: how to interpret depth and screen resolution
*	Styles/Macros: classification of source blocks with beginning style+text
*	Flexible timestamps, source block notes and raw text insertion
*	Powerful animation, controlled by math expressions
*	2.5D
*	Much control over positioning and layout
*	Many effects: textures, individual transformations, blending modes, masking...
*	Fast rendering (=softsubs)

# 2. File
## Encoding
SSB files have to be saved in UTF-8.
We chose UTF-8 because:

* it is the gold standard for text encoding around the globe and most programs assume UTF-8 if nothing is specified
* it does not need BOM at beginning of the file
* no complications in allowing multiple encodings

Because SSB files are text files, except for horizontal tab (U+9), new line (U+A) and carriage return (U+D) non other characters below U+20 are allowed as content.
Anyway, carriage return will be ignored. New line alone starts a new line (see newline of text files).

## File Extension
Obviously, SSB file names end with .SSB.

## Matroska Codec ID
Muxed into a matroska file, they have codec id S_TEXT/SSB.

## Structure
The SSB structure depends on different sections to describe the file creation, source to render and destination frame. Every information chunk is one text line in a SSB file.
INI and CSV influenced the section structure, BBCode influenced the style tags structure.
Sections headers are single lines and starting with # (U+23) followed by the section name like:

\#SECTION_NAME

Invalid sections will be ignored, so nothing happens until a valid section starts.

## INFO section
The INFO section holds information about the script itself. Format of information chunks is...

Type: data

You can come up with any data yourself but here are some examples:

## Title
Title of script. For example: 

Title: XXX logo animation.

## Author
Author of script / script creator. For example:

Author: Youka

## Version
Script version. Can contain date, time, phase (alpha, beta, public, private), etc. For example:

Version: 1.0

## Description
Additional description of script. Something like the purpose, team or anything else that wasn’t written before. For Example:

Description: The thing to do the thing

## TARGET section
The TARGET section describes the 3 dimensional plane where geometries reside in. 
Coordinates, 2D or 3D, outside of the target aren’t visible. Format of information chunks is...

Type: data

Following types are available:

### Width
Width of the plane. Coordinates in range 0 until given width are visible. Default value is video width. If an value is defined different than the video width, rendered frame will be scaled to fit. For example:

Width: 1280

### Height
Height of the plane. Coordinates in range 0 until given height are visible. Default value is video height. If an value is defined different than the video height, rendered frame will be scaled to fit.  For example:

Height: 720

### Depth
Depth of the plane. Coordinates in range negative half of given depth until positive half of given depth are visible. Default value is 1000, that means, -500 until +500 are visible depths.  For example:

Depth: 1000

### View
Viewer perspective. Can be ‘orthogonal’ or ‘perspective’. Default is ‘perspective’. In case of ‘perspective’, geometries are nearing screen center with increasing negative depth and double size with increasing positive depth. Depth 0 means original size.  For example:

View: perspective

## MACROS section
The MACROS section defines styles for use in following source blocks. Macros can be understood as a collection of tags. Format of macros is...

name: content

Name can be a chain of characters except (obviously) : (U+3A).

Content just shouldn’t be empty, that’s all.

## EVENTS section
The EVENTS section describes what to render at all. Every source block tells the renderer: “when render what”.
Format of source blocks is...

```
start-end | macro | note | text
```

### start-end
Start and end are the times when to start and end rendering.
Times have one of the following formats...
#### Time Based
```
[[[hours:]minutes:]seconds.]milliseconds
```
#### Event Based
```
'event-id'
```

You will be able to pass an array of **event-id**'s to the frame render function of SSB which will allow you to control when lines should be shown from the outside.

### macro
macro is the name of a macro defined in the MACROS section. Content of it will be inserted to the beginning of source block text.
### note
Note is just an information beside, renderers ignore it.
### text
Text is a mix of style tags and geometries. Everything what should be rendered is written here.

## RESOURCE section
The RESOURCE section describes all resources that will be used within the subtitle format. This includes images and fonts.

### Texture
Texture: TEXTURE_ID,data|url,base_64_or_url 

TEXTURE_ID: can be any character but must not contain comma.

data|url: You need to tell if the data after this is actual data or an url

base_64_or_url: base64 encoded string or an url as an absolute path to the file on the file system.

### Fonts
Font: FONT_FAMILY,style,data|url,base_64_or_url

FONT_FAMILY: can be any character but must not contain comma.

style: regular, bold, italic, bold-italic

data|url: You need to tell if the data after this is actual data or an url

base_64_or_url: base64 encoded string or a url as an absolute path to the file on the file system.

## Comment
Comments are source blocks which don’t display anything on render target.
They can contain line information, like time, style, note and text, so they can be used as backups or multiline notes.
Commented source blocks differ from others by two / (U+2F) before the first cell.

```
//0-2:0.0|||Nothing to see.
```

# 3. Styling
## General
Events can have some style properties like color or position. These will be set by style tags which have to be inserted before any given geometry definition e.g. text.
A style tag effects any geometry definition which comes after the tag. The default style, excluding user styles or style tags, is defined as...

```wrap
[font=Liberation Sans;size=20;bold=n;italic=n;underline=n;strikeout=n;position=0,0,0;direction=ltr;space=0;alignment=2;margin=10;mode=text;border=2;join=round;cap=round;color=FFFFFF;bordercolor=000000;alpha=FF;borderalpha=FF;texture=;texfill=0,0,1,0,clamp;blend=normal;target=mask;mask-mode=normal;blur=0]
```

On Windows the default Font is Arial.

Animations and transformations aren’t static properties, so they aren’t listed above.
Transformations are a special case, because they stack and have to be cleared by reset style tag. Animations interpolate transformations from its non-affecting identity, so an animation of a translation starts with 0/0/0.
In case of no use of position, direction or any transformation tag, geometry will be wrapped by margin rule to screen edges. Geometry rows nearer the related edge of alignment are wider than rows farther away.
Setting a new position resets current geometry position. Following text wouldn’t continue it’s position flow from text before.

## Escape characters
Characters |, [ and ] are identificators for source blocks, so they can’t be used in source cells directly. Add \ before these characters (and \ itself) to escape and use them as content f.e. for texts or notes.

## Auto-Wrapping
Auto-Wrapping describes the process in which text is broken down into multiple lines of text to not go outside the edge of the screen.

Text will only be auto-wrapped if no **position** tag is present in the current line.

The Auto-Wrapper will try to create wrapped lines where the last line has the most text, the second-last the second most text etc. culminating in a pleasing pyramid structure.

Wrapping is mostly influenced by the tags **margin**, **alignment** and **wrap-style**. It is also influenced by all tags which increase the size of the line or the characters.
The following describes how each of these tags effects auto-wrapping. Note however that these tags do not _only_ effect auto-wrapping.

### margin
**margin** adds a border to the auto-wrapping, effectively making the screen smaller and leading to text being pushed away from the edges.

### alignment
**alignment** puts the text at certain default positions on the screen, for example **alignment=7** would put the text on the top left corner of the screen and makes the text left-aligned
while **alignment=2** would put the text on the bottom of the screen in the middle and would effectively center-align the text.

### wrap-style
What each wrap-style does is defined in the **wrap-style** section in the style-tags section.

## Style-Tags
### Font
#### font
```
[font=Liberation Sans]
```

Name of font for text rendering. Defined in the resource section or coming from the operating system.

#### size
```
[size=20]
```

In text mode: size of font in pixel.

In point mode: point range in pixel.

In shape mode: no effect.

#### bold
```
[bold=n]
```

Font weight. ‘y’ for bold, ‘n’ for normal.

#### italic
```
[italic=n]
```

Font style. ‘y’ for setting italic, ‘n’ for normal.

#### underline
```
[underline=n]
```

Font decoration. ‘y’ for underlining, ‘n’ for normal.

#### strikeout
```
[strikeout=n]
```

Font decoration 2. ‘y’ for striking out, ‘n’ for normal.

### Position
#### position
```
[position=0,0,0]
[position=0,0]
```

Position on screen. 2D or 3D coordinate possible. 0,0 is in the top left corner of the screen.

#### alignment
```
[alignment=7]
[alignment=0,0]
```

Alignment of geometry at position point.

One value: see keyboard numpad for anchor point definition.

Two values: horizontal and vertical offset from anchor point as geometry width and height in percent.

#### margin
```
[margin=10]
[margin=10,10,10,10]
[margin-top=10]
[margin-right=10]
[margin-bottom=10]
[margin-left=10]
```

Margin to screen edges in pixel. Only affects line if no position is set.

### wrap-style
```
[wrap-style=space]
```

Only has any effect if auto-wrapping is enabled.

Will wrap text according to the specified style. Can be either **nowrap**, **space** or **character**;

With **space** the auto-wrapper will try to break lines at the " " character.

With **character** the auto-wrapper will try to break lines at characters.

With **hyphen** the auto-wrapper will try to intelligently use a dictionary to add hyphens to words in addition to breaking on spaces.

#### direction
```
[direction=ltr]
```

Draws text in different directions, usually depends on the writing system (think english vs. japanese vs. hebrew). Default is LTR.

Can be: ltr, rtl, ttb, btt

ltr = left-to-right

rtl = right-to-left

ttb = top-to-down

btt = bottom-to-top

#### space
```
[space=0]
[space=0,0]
[space-h=0]
[space-v=0]
```

Space between geometries. For text, horizontal space between characters and vertical space between lines are defined too.

### Transformation
#### rotate
```
[rotate=0,0,0]
[rotate-x=0]
[rotate-y=0]
[rotate-z=0]
```

Geometry rotation on plane axis in degree.

#### scale
```
[scale=1,1,1]
[scale-x=1]
[scale-y=1]
[scale-z=1]
```

Geometry scale on plane axis in percent. (1 = 100%)

#### translate
```
[translate=0,0,0]
[translate-x=0]
[translate-y=0]
[translate-z=0]
```

Geometry translation on plane in pixel.

#### shear
```
[shear=0,0]
[shear-x=0]
[shear-y=0]
```

Geometry shearing on plane as weight.

#### matrix
```
[matrix=1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]
```

You can also directly manipulate the matrix yourself, but beware that setting this may reset/overwrite translate, scale, rotate and shear. 

#### reset
```
[reset]
```

Resets transformations in source block or in other words, resets the matrix.

### Geometry
#### mode
Geometry type to draw. Every character in a line that is not within squared brackets will be interpreted and rendered as geometry according to this mode.

```
[mode=text]
```
Lorem ipsum dolor sit amet, consectetur adipiscing elit.

```
[mode=shape]
```

Can be one of the  following:

```
m - move (m 0 0)

l - line (l 0 0 1 1)

a - arcs (a 0 0 360)

b - cubic bezier (b 0 0 1 1 2 2 3 3)
```

Example of a circle: 
```html
m 0 -10 b 0 -10 -10 -10 -10 0 b -10 0 -10 10 0 10 b 0 10 10 10 10 0 b 10 0 10 -10 0 -10
```

![circle shape](assets/img/circle.png)

Example of a heart shape: 
```html
m -17 4 b -17 -4 -14 -9 -10 -9 b -4 -9 0 -2 0 -2 b 0 -2 4 -9 10 -9 b 14 -9 17 -4 17 4 b 17 13 0 22 0 22 b 0 22 -17 14 -17 4
```

![heart shape](assets/img/heart.png)

```
[mode=points]
```
0 0

TODO: Better example, add picture


#### border
```
[border=2]
[border=2,2]
[border-h=2]
[border-v=2]
```

Border width of geometry.

#### join
```
[join=round]
```

Border and line join style. Can be ‘round’, ‘bevel’ or ‘miter’.

#### cap
```
[cap=round]
```

Border and line cap style. Can be ‘round’, 'butt' or ‘square’.

### Textures

#### texture
```
[texture=RESOURCE_ID]
```

Texture on geometry. Texturing enabled by a valid RESOURCE_ID referencing an image. Disabled by an invalid value (like an empty value).

Can be set to **@** to use the current frame of the video as a texture.

#### texfill
```
[texfill=0,0,1,0]
[texfill=0,0,1,0,pad]
```

Texture position, size and wrapping.

Offset: The first two numbers describe where the texture starts on the geometry (as a value between 0 - 1 (0-100%)).

Range: The two numbers after that describe how far the texture stretches in respect to the geometry (as a value between 0 -1 (0-100%)).

Wrapping modes: **pad**, **clamp**, **repeat**, **mirror**. **pad** is default.

Wrapping modes describe what happens outside of the texture on the geometries (beyond the edges of the texture).

**pad** means everywhere there is no texture you will have the standard filling color of the geometry.

**clamp** means that the last pixel of the texture is repeated in the direction of the edge.

**repeat** means that the texture is repeated in the direction of the edge.

**mirror** means that the texture is repeated but flipped on the respective edge in the direction on the edge.

### Color
#### color
```
[color=FFFFFF]
[color=FFFFFF,FFFFFF]
[color=FFFFFF,FFFFFF,FFFFFF]
[color=FFFFFF,FFFFFF,FFFFFF,FFFFFF]
[color=FFFFFF,FFFFFF,FFFFFF,FFFFFF,FFFFFF]
[bordercolor=000000]
[bordercolor=000000,000000]
[bordercolor=000000,000000,000000]
[bordercolor=000000,000000,000000,000000]
[bordercolor=000000,000000,000000,000000,000000]
```

Color for geometry or his border. Can be mono, left-to-right gradient, left-to-mid-to-right gradient, 4-corners gradient or 4-corners + center gradient. Color definition in hexadecimal (RGB).

#### alpha
```
[alpha=FF]
[alpha=FF,FF]
[alpha=FF,FF,FF,FF]
[alpha=FF,FF,FF,FF,FF]
[borderalpha=FF]
[borderalpha=FF,FF]
[borderalpha=FF,FF,FF,FF]
[borderalpha=FF,FF,FF,FF,FF]
```

Transparency for geometry or his border. Can be mono, left-to-right gradient, left-to-mid-to-right gradient, 4-corners gradient or 4-corners + center gradient. Value in hexadecimal (00 = invisible, FF = opaque).

#### blur
```
[blur=0]
[blur=0,0]
[blur-h=0]
[blur-v=0]
```

Gaussian blur on geometry + border. Sigma value defines strength. Horizontal and vertical blur available.

### Rastering
#### mask
```
[target=mask]
[target=frame]
[mask-mode=normal]
[mask-mode=invert]
[mask-clear]
```

With this you can use one geometry to mask another, making clipping and holes inside texts etc. possible.

If you set target to **mask** then you will start to render on a different canvas which you are unable to see.
On this canvas (back buffer) only alpha values exist for each pixel (0-255) so no colors. If you draw any shape on this hidden canvas
with an alpha value of 255, each pixel of said shape will be "masked out" on the original canvas, meaning you will
stance a hole into any shape at said position. If you use 127 alpha instead, the "hole" will be 50% transparent etc.

You can invert this logic by setting **mask-mode** to **invert**.
Beware that if you do this, nothing will show on screen as the hidden canvas will be all 0 per default and by
inverting the logic everything is masked out.

With using **mask-clear** you can reset the entire canvas to 0.

Example:

```
[target=mask;mask-mode=invert;alpha=FF;blur=5;]CIRCLE_SMALL[target=frame;blur=0;]CIRCLE
```

#### blend
```
[blend=overlay]
```

Blending mode. This will set how this geometry is blended with the background.

Following modes are available:

overlay: (source alpha * source color) + ((1 -  source alpha) * destination color)

add: source color + destination color

subtract: source color - destination color

multiply: source color * destination color

invert: ~destination color

difference: abs(source color - destination color)

screen: 1 - (1 - source color) * (1 - destination color)


### Animation
#### animate
```
[animate=[color=000000;translate-x=20]]
[animate=t,[color=000000;translate-x=20]]
[animate=0,1000,[color=000000;translate-x=20]]
[animate=500,1000,[color=000000;translate-x=-20]]
[animate=0,1000,sin(t*pi),[color=000000;translate-x=20]]
```

Interpolates style properties over time.

First version goes over whole source block time to interpolate style tags.

Second version additionally sends interpolation factor of current frame through an equation.

Third version specifies a time range to interpolate style tags.

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
```
[k=100]
```

Karaoke duration in milliseconds.

#### kset
```
[kset=0]
```

Reset karaoke time in milliseconds to start time of event.

#### kcolor
```
[kcolor=FF00FF]
```

This will create something akin to the following animation tag for each syllable:

```
[animate=kStart,kEnd,t^0.5,[color=FF00FF]]
```

Color to which the karaoke will interpolate within the given duration of the karaoke timing. For color syntax see the color tag.

# 6. Examples

## Minimal
This is an absolute minimal example.
Only the EVENTS section is needed for an output.
```
#EVENTS
|1.0-5:0.0|||Boring line.|
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

#TARGET
Width: 1280
Height: 720
Depth: 1000
View: perspective

#MACROS
Default: [bold=y]
Mine: [bold=n;color=FF0000]
Another: [Mine;position=100,200,-1;rotate-z=180]I'm a

#EVENTS
//0-2.0|||This line is a comment over 2 seconds!
2.0-5:0.0|Another|Hello, i'm a note!|red,    rotated\ntext over multiple lines.
5:0.0-2:5:0.0|Mine|Draw sth.|[mode=shape;texture=RAMEN]m 0 0 l 50.5 0 50.5 20.125 0 20.125
10:0.0-10:50:0.0||${Another}Lets scale some text to double its size!|[animate=[500, 1000, [scale=2]]This text is getting huge
20.0.0-21.0.0|||[font=MaterialIcon]some_circle_ligature
'show-something'|Default||This will only be shown when the event id is given

#RESOURCE
Texture: RAMEN,url,../ramen.tga
// Will we support ligaturs? Pretty important for icon fonts
Font: MaterialIcon,regular,data,AAEAAAAKAIAAAwAgT1MvMnwMf9s...
```

# 7. Credits
References:
*	[ASS specification](http://docs.aegisub.org/3.2/Main_Page/)
*	[VSFilterMod - NewTags](https://code.google.com/archive/p/vsfiltermod/wikis/NewTags.wiki)
*	[SVG Paths](https://www.w3.org/TR/SVG/paths.html)

Thanks to...
*   Youka
*	McWhite
*	OutlawJones

# 8. License
This specification is licensed under a Creative Commons Attribution-NoDerivs 3.0 license.
You can share it in commercial or non-commercial way, but have to mention the author, keep it uncut and don’t change content.


