#version 330 core
out vec4 FragColor;

flat in float ourColor;
in vec2 TexCoord;
in vec3 Normal;
uniform sampler2D ourTexture;
flat in int TexMode;

// uniform vec3 objectColor;
uniform vec3 lightColor;

uniform vec3 lightPos;
uniform vec3 viewPos;
uniform float ambientStrength;
uniform float specularStrength;

in vec3 FragPos;

void main()
{
    vec3 ambient = ambientStrength * lightColor;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * vec3(1.0);
    if (TexMode == 1)
        FragColor = vec4(result, 1.0) * vec4(0.2, 0.2, 0.2, 1.0);
    else
        FragColor = vec4(normalize(Normal) * 0.5 + 0.5, 1.0);
    //     FragColor = vec4(result, 1.0);
}
