#version 330 core

in vec2 fragment_uv;                // Default
in vec4 fragment_color;             // Default

uniform sampler2D atlas;
uniform vec4 background_color;

float median(float a, float b, float c) {
    return max(min(a, b), min(max(a, b), c));
}

void main() {
    if(fragment_color.w == 0.0f) {
        discard;
    }
    float alpha = 0.0f;
    if(texture(atlas, fragment_uv).x > 0.5f){
        alpha = 1.0f;
    }
    // gl_FragColor = vec4(fragment_color.xyz, alpha * fragment_color.w);
    
    // Bilinear sampling of the distance field
    vec3 s = texture2D(atlas, fragment_uv).rgb;
    // Acquiring the signed distance
    float d = median(s.r, s.g, s.b) - 0.5;
    // The anti-aliased measure of how "inside" the fragment lies
    // float screenpxdistance = d/fwidth(d);
    // screenpxdistance = 64.0;
    float w = clamp(d/fwidth(d) + 0.5, 0.0, 1.0);
    // Combining the two colors
    gl_FragColor = mix(vec4(background_color.rgb, 0.0), fragment_color,  w);
    // gl_FragColor = vec4(s, 1.0);
    //Clamp function
    //A function that yields the closest value to x in a given range ha, bi:
    //clamp(x, a, b) = min(max(x, a), b)

    //Mix function:
        // x: Specify the start of the range in which to interpolate.
        // y: Specify the end of the range in which to interpolate.
        // a: Specify the value to use to interpolate between x and y.
}