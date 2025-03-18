#version 330 core

layout (location = 0) in vec2 aPos; // position
layout (location = 1) in vec3 aColor; // color of vertex

// --NEW-- //
uniform vec2 cameraPos; // take in the camera position as a vec2 of floats

out vec3 color;

void main() {
    // --- NEW --- //
    gl_Position = vec4(
	aPos.x - cameraPos.x, // we SUBTRACT the camera pos from the vertex pos
	aPos.y - cameraPos.y, // as the camera moves opposite to the shape.
    0.0, 1.0); 
    // --- END NEW --- //
    
    color = aColor;
}

