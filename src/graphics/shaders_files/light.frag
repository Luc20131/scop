#version 330 core
out vec4 FragColor;

flat in float ourColor;
in vec2 TexCoord;
in vec3 Normal;

uniform sampler2D diffuseTexture;
uniform int has_alpha_tex;
uniform sampler2D alphaTexture;
uniform int has_normal_tex;
uniform sampler2D normalTexture;

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

struct Material {
    float d;
};

uniform Material material;

void main()
{
    vec3 ambient = light.ambientStrength * lightColor;
    vec3 norm = normalize(Normal);
    // diffuse
    if (has_normal_tex == 1) {
        vec4 normal_color = texture(normalTexture, TexCoord);
        norm = norm * (vec3(normal_color.r, normal_color.g, normal_color.b));
    }
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


    if (TexMode == 1) {

        FragColor = vec4(result, 1.0) * vec4(0.2, 0.2, 0.2, 1.0);
        vec4 color_tex = texture(diffuseTexture, TexCoord);
        vec4 alpha_tex;
        if (has_alpha_tex == 1) {
            alpha_tex = texture(alphaTexture, TexCoord);
            if (alpha_tex.r < 0.1)
                discard;
        }
        else {
            alpha_tex.r = material.d;
        }
        color_tex.a = alpha_tex.r;
        FragColor = color_tex * vec4(result, 1.0);
    }
    else
        FragColor = vec4(normalize(norm) * 0.5 + 0.5, 1.0);
    //     FragColor = vec4(result, 1.0);
}
