#version 330 core

// take in the color from the vertex shader:
in vec3 color;

out vec4 FragColor; // used to output the color of the triangle

void main() {
    // set the color to the color passed in, with alpha 1.0 (fully opaque)
    FragColor = vec4(color, 1.0);
}
