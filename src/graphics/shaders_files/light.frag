#version 330 core
out vec4 FragColor;

flat in float ourColor;
in vec2 TexCoord;
in vec3 Normal;
uniform sampler2D ourTexture;
flat in int TexMode;

struct Light {
    vec3 position;
    float ambientStrength;
    float specularStrength;

    float constant;
    float linear;
    float quadratic;
};

uniform Light light;
// uniform vec3 objectColor;
uniform vec3 lightColor;

uniform vec3 viewPos;

in vec3 FragPos;

float distance = length(light.position - FragPos);
float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

void main()
{
    vec3 ambient = light.ambientStrength * lightColor;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = light.specularStrength * spec * lightColor;

    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    vec3 result = (ambient + diffuse + specular) * vec3(1.0);
    if (TexMode == 1)
        // FragColor = vec4(result, 1.0) * vec4(0.2, 0.2, 0.2, 1.0);
        FragColor = vec4(result, 1.0) * texture(ourTexture, TexCoord);
    else
        FragColor = vec4(normalize(Normal) * 0.5 + 0.5, 1.0);
    //     FragColor = vec4(result, 1.0);
}
