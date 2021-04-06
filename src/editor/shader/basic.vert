precision mediump float;

attribute vec2 aPos;
attribute vec2 aTexCoord;

uniform mat3 viewMatrix;
uniform mat3 projectionMatrix;

varying highp vec2 vTexCoord;

void main() {
    gl_Position = vec4(projectionMatrix * viewMatrix * vec3(aPos, 1.0), 1.0);
    vTexCoord = aTexCoord;
}
