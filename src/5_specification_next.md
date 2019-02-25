# Specification

### Geometry
#### mode
[mode=point]
[mode=line]
[mode=lines]
[mode=triangle]
[mode=triangles]
[mode=polygon]
[mode=ellipse]
[mode=shape]
Geometry type to draw. Form will be interpreted by non-style content of fourth cell in source block. Default mode is text.

#### deform
[deform=”x”,”y”,”z”]
[deform-x=”x”]
[deform-y=”y”]
[deform-z=”z”]
Deform filter for objects.
TODO: similar to baseline of AS5?

#### texture
[texture=invalid_image.png]
Texture on object. Texturing enabled by a valid image file name or ‘frame’ (current video frame), disabled by an invalid value (like an empty value).

#### texfill
[texfill=0,0,1,0,1,1,0,1]
[texfill=0,0,1,0,1,1,0,1,clamp]
Texture position and wrapping.
Position described by 4 corners of object. Origin is 0/0 at left-top corner, dimension is 1/1, edges are defined clockwise.
Wrapping modes are ‘clamp’, ‘repeat’ and ‘mirror’. ‘clamp’ is default.

### Rastering
#### stencil
[stencil=set]
[stencil=unset]
[stencil=in]
[stencil=out]
[stencil=clear]
Stenciling on screen. Defines overwritable pixels.
First version activates stencil drawing mode. All following objects are defining stencil areas.
Second version activates stencil removing mode. All following objects are removing stencil areas.
Third version reactivates normal drawing, but just on stencil areas.
Fourth version reactivates normal drawing, but just on non stencil areas.
Sixth version reactivates normal drawing and clears all stencil areas.

#### blend
[blend=normal]
Blending mode (see here). Following modes are available:
add: source color + destination color
subtract: source color - destination color
mask: source color * destination color
invert: ~destination color
overlay: source color
copy: destination color

### Animation
#### fade
[fade=0]
[fade=0,0]
[fade-in=0]
[fade-out=0]
In- & outfade of object in milliseconds.

#### animate
[animate=[color=000000;translate-x=20]]
[animate=t,[color=000000;translate-x=20]]
[animate=0,1000,[color=000000;translate-x=20]]
[animate=0,1000,sin(t*pi),[color=000000;translate-x=20]]
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
#### k
[k=100]
Description.

#### kstrip
[kstrip=1205]
Description.

#### kset
[kset=0]
TODO: how should the karaoke system works?