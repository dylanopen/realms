#version 330 core // specify the version of GLSL we want to use

layout (location = 0) in vec2 aPos; // take in 2D coordinates for each vertex

void main() {
    // gl_Position is provided by opengl. Whatever we set it to will be the
    // *normal* coordinates of the vertex.
    // Here, we use the provided x and y position, but keep the last two
    // constant as we're currently only working in 2D.
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0); 
}

