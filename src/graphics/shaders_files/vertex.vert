#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;
layout(location = 2) in vec3 aNormal;
layout(location = 3) in float aColor;

flat out float ourColor;
out vec2 TexCoord;
out vec3 Normal;
flat out int TexMode;

uniform int aTexMode;
uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * transform * vec4(aPos, 1.0f);
    ourColor = aColor;
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
    Normal = aNormal;
    TexMode = aTexMode;
}
