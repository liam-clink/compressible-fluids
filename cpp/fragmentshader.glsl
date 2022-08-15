// This means that when writing in the variable “color”,
// we will actually write in the Render Target 0,
// which happens to be our texture because DrawBuffers[0] is GL_COLOR_ATTACHMENTi,
// which is, in our case, renderedTexture.
layout(location = 0) out vec3 color;

uniform vec2 u_resolution;

void main()
{
    // gl_FragCoord is GLSL default input
    vec2 screen_coord = gl_FragCoord.xy/u_resolution;
    // gl_FragColor is GLSL default output
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    gl_FragData; //idk what this is
}