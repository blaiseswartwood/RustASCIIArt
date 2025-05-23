#version 330 core

in int cmdline;         // DEBUG

in int tid;             // Default
in int corner;          // Default
// 0 [00] - bottom left
// 1 [01] - top left
// 2 [10] - bottom right
// 3 [11] - top right

in vec3 color;          // Default
in float random_seed;   // Default
in int start_time;         // Default
in int end_time;           // Default
in float font_size;     // Default

// Center oriented
in vec2 start_pos;      // Translate
in vec2 end_pos;        // Translate

in float fade_time;   // Fade out

in vec3 new_color;      // Color shift

in int color_delay_time;   // Color Randomization

in int char_delay_time;    // Character Randomization

in float accel_degree;


// --------------------- FRAGMENT OUTPUTS ---------------------
out vec2 fragment_uv;
out vec4 fragment_color;

// ------------------------- UNIFORMS -------------------------
uniform int time;
uniform vec2 windowSize;
uniform atlas_buffer { 
    vec4 glyph_data[512];
};
uniform int glyph_count;

// ------------------------- VARIABLES ------------------------
vec2 current_pos;
int duration_time;

float getRandom(float rand, int n) { // return value between 0 and 1
    float firstDigit = mod(floor(start_time / n), 10.0);
    float secondDigit = mod(floor(start_time / (n + 1)), 10.0);
    float combinedDigits = float((firstDigit/ 10.0) + (secondDigit / 100.0));
    return (sin(rand * n + combinedDigits) + 1.0f) / 2.0f;
}

vec3 getRandomColor() {
    int start_index = int(floor(float(time - start_time) / float(color_delay_time))) + 1;
    float color_1 = getRandom(random_seed, start_index);
    float color_2 = getRandom(random_seed, start_index * 11);
    float color_3 = getRandom(random_seed, start_index * 17);
    return vec3(color_1, color_2, color_3);
}

// converts pixel(sort of) values to gl coords: [0, windowSize] -> [-1, 1]
vec2 screenspaceToGLCoords(vec2 screenspace_coords) {
    return 2.0*(screenspace_coords/windowSize) - 1.0;
}

// convert corner (int) to binary and use each bit to determine an axis (vec2)
vec2 getNormalUV(){
    if(corner == 0) return vec2(0.0, 0.0);
    if(corner == 1) return vec2(0.0, 1.0);
    if(corner == 2) return vec2(1.0, 0.0);
    if(corner == 3) return vec2(1.0, 1.0);
}

// x0 -> x
// y0 -> y
// x1 -> z
// y1 -> w
// flipped since texture uvs read with origin top left
vec2 getTextureCoord(int glyphID){
    vec4 glyph = glyph_data[glyphID];
    vec2 coord = vec2(0.0, 0.0);
    if(corner == 0) coord = vec2(glyph.xw);                 // bottom left 
    if(corner == 1) coord = vec2(glyph.xy);                 // top left
    if(corner == 2) coord = vec2(glyph.zw);                 // bottom right
    if(corner == 3) coord = vec2(glyph.zy);                 // top right
    coord.y = 1 - coord.y;
    return coord;
}

vec2 getVertexOffset(int glyphID){
    vec4 glyph = glyph_data[glyphID];
    float w = (glyph.z - glyph.x) / 2.0;
    float h = (glyph.w - glyph.y) / 2.0;
    if(corner == 0) return vec2(-w, -h);
    if(corner == 1) return vec2(-w, h);
    if(corner == 2) return vec2(w, -h);
    if(corner == 3) return vec2(w, h);
}

float lerp(float from, float to, float factor){
    return from + (to - from) * factor;
}

vec2 vec2Lerp(vec2 from, vec2 to, float factor){
    return from + (to - from) * factor;
}

vec3 vec3Lerp(vec3 from, vec3 to, float factor) {
    return from + (to - from) * factor;
}

vec4 vec4Lerp(vec4 from, vec4 to, float factor) {
    return from + (to - from) * factor;
}

// vec2 position_from_accelerate(vec2 start_pos, vec2 end_pos, float accel_degree) {
//     vec2 C = (end_pos - start_pos) / pow(float(duration_time), accel_degree);
//     float td = pow(float(time - start_time), accel_degree);
//     return vec2(start_pos.x + (C.x * td), (start_pos.y + C.y * td));
// }

// converts a value to a normalized vector within bounds
// ex:
// normalize(0, 10, 3) => 0.3
float map(float start, float end, float factor){
    return (factor - start) / (end - start);
}

void main() {
    // --------------------- TIME FACTOR -------------------
    float time_factor = map(0.0f, float(end_time - start_time), float(time - start_time));
    
    // --------------------- DISCARD ----------------------
    if(time < start_time){
        fragment_color = vec4(1.0, 1.0, 1.0, 0.0);
        return;
    } else if (time > end_time){ 
        fragment_color = vec4(1.0, 1.0, 1.0, 0.0);
        return;
    }

    // --------------------- FADE OUT ---------------------
    float fragment_alpha = 1.0;
    if(time > end_time - fade_time){
        fragment_alpha = map(fade_time, 0.0f, time - (end_time - fade_time));
    }

    // ---------------------- COLOR -----------------------
    vec3 frag_color;
    if(color_delay_time == -1) { // NO COLOR RANDOMIZE
        if(color == new_color){ // NO COLOR SHIFT
            frag_color = color;   
        } else { // COLOR SHIFT
            frag_color = vec3Lerp(color.xyz, new_color.xyz, min(time_factor, 1.0));
        }
    } else { // COLOR RANDOMIZE
        frag_color = getRandomColor();
    }

    // ------------------------ UV ------------------------
    int glyphID = tid;
    if(char_delay_time != -1) {
        int rand_index = int(float(time - start_time) / float(char_delay_time));
        glyphID = int(getRandom(random_seed, rand_index) * glyph_count);
    }
    vec2 textureCoord = getTextureCoord(glyphID);
    vec2 vertexOffset = getVertexOffset(glyphID) * font_size;

    // -------------------- TRANSLATE ---------------------
    // TODO: move start_time back to first arg after we move off of unix time
    vec2 current_pos = vec2Lerp(start_pos, end_pos, time_factor);
    // if(accel_degree != 0.0) {
    //     current_pos = position_from_accelerate(start_pos, end_pos, accel_degree);
    // }

    vec2 position = current_pos + vertexOffset;
    // vec2 position = vec2Lerp(start_pos, end_pos, time_factor) + vertexOffset;

    // ------------------- RESULT -------------------------
    fragment_color = vec4(frag_color.xyz, fragment_alpha);
    gl_Position = vec4(screenspaceToGLCoords(position), 0.0, 1.0);
    fragment_uv = textureCoord;
}


