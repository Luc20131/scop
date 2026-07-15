#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;
layout(location = 2) in vec3 aNormal;
layout(location = 3) in float aColor;

flat out float ourColor;
out vec2 TexCoord;
flat out int TexMode;

out vec3 Normal;
out vec3 FragPos;

uniform int aTexMode;
uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    ourColor = aColor;
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
    TexMode = aTexMode;
    FragPos = vec3(transform * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(transform))) * aNormal;
    gl_Position = projection * view * vec4(FragPos, 1.0);
}
