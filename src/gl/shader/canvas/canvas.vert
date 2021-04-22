precision mediump float;

attribute vec4 aUnitPos;

uniform mat4 uWorldMatrix;
uniform mat4 uTextureMatrix;

varying highp vec2 vTexCoord;

void main() {
    gl_Position = uWorldMatrix * aUnitPos;
    vTexCoord = (uTextureMatrix * aUnitPos).xy;
}
