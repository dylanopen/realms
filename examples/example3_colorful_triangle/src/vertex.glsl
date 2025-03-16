#version 330 core // specify the version of GLSL we want to use

// Notice that the locations specified in the shader are the same as we
// specified in the rust code!
layout (location = 0) in vec2 aPos; // take in 2D coordinates for each vertex
layout (location = 1) in vec3 aColor; // take in RGB color values for each vertex

// we need to pass out the color so the fragment shader can use it:
out vec3 color;

void main() {
    // gl_Position is provided by opengl. Whatever we set it to will be the
    // *normal* coordinates of the vertex.
    // Here, we use the provided x and y position, but keep the last two
    // constant as we're currently only working in 2D.
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0); 
    
    // just set the color to the inputted aColor so the frag shader can deal
    // with it...
    color = aColor;
}

