precision mediump float;

attribute vec4 aUnitPos;

uniform mat4 uWorldMatrix;

varying highp vec2 vTexCoord;

void main() {
    gl_Position = uWorldMatrix * aUnitPos;
    vTexCoord = aUnitPos.xy;
}
