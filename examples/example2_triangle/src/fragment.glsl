#version 330 core

out vec4 FragColor; // used to output the color of the triangle

void main() {
    FragColor = vec4(1.0, 0.6, 0.2, 1.0); // set the color to orange.
    // currently, we use a hardcoded color value. in the next example, we will
    // specify the color of each vertex in the VertexBuffer.
}
