#version 450

layout(binding = 1) uniform sampler2D texSampler;

layout(push_constant) uniform PushConstants {
    uint colorMode;
} pcs;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;
layout(location = 2) in flat uint index;

layout(location = 0) out vec4 outColor;

const vec3 colors[4] = vec3[4](
    vec3(0.05, 0.05, 0.05),
    vec3(0.1, 0.1, 0.1),
    vec3(0.15, 0.15, 0.15),
    vec3(0.2, 0.2, 0.2)
);

void main() {
    if (pcs.colorMode == 0) {
        outColor = vec4(colors[index % 4], 1.0);
    } else if (pcs.colorMode == 1) {
        outColor = texture(texSampler, fragTexCoord) * vec4(fragColor, 1.0);
    }
}
