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
    
       vec3 norm = normalize(Normal);

    vec3 lightDir = normalize(lightPos - FragPos);

    vec3 ambient = ambientStrength * lightColor;

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular);
    if (TexMode == 1)
        FragColor = texture(ourTexture, TexCoord);
    else
        FragColor = vec4(result, 1.0) * vec4(ourColor, ourColor, ourColor, 1.0);
}
