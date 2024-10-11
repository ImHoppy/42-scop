#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;
layout(location = 2) in vec2 inTexCoord;

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 fragTexCoord;

const vec3 LIGHT_DIRECTION = normalize(vec3(1.0, -3.0, -1.0));

void main() {
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 1.0);
    vec3 normal = normalize(mat3(transpose(inverse(ubo.model))) * inPosition);
    float intensity = dot(normal, -LIGHT_DIRECTION);
    fragColor = clamp(intensity, 0.2, 1.0) * inColor;
    fragTexCoord = inTexCoord;
}
