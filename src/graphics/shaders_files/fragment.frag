#version 330 core
out vec4 FragColor;

flat in float ourColor;
in vec2 TexCoord;
in vec3 Normal;
uniform sampler2D ourTexture;
flat in int TexMode;

void main()
{
    if (TexMode == 1)
        FragColor = texture(ourTexture, TexCoord);
    else
        FragColor = vec4(ourColor, ourColor, ourColor, 1.0);
}
