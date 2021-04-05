precision mediump float;

uniform sampler2D panelTexture;

varying highp vec2 vTexCoord;

void main() {
    gl_FragColor = texture2D(panelTexture, vTexCoord);
}
