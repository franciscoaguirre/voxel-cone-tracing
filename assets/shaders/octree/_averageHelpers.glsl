vec4 averageHandlingEmpty(vec4 colors[2]) {
    vec4 result = vec4(0);
    vec4 resultSinVacios = vec4(0);

    int cantNoVacios = 0;
    int cantVacios = 0;

    for (int i = 0; i < colors.length(); i++) {
        if (colors[i] == vec4(0)) {
            // Habría que sumar el alpha pero es 0
            cantVacios += 1;
        } else {
            result += colors[i];
            cantNoVacios += 1;
        }
    }

    result.rgb /= float(cantNoVacios);
    result.a /= float(colors.length());

    return result;
}

vec4 averageHandlingEmpty(vec4 colors[4]) {
    vec4 result = vec4(0);
    vec4 resultSinVacios = vec4(0);

    int cantNoVacios = 0;
    int cantVacios = 0;

    for (int i = 0; i < colors.length(); i++) {
        if (colors[i] == vec4(0)) {
            // Habría que sumar el alpha pero es 0
            cantVacios += 1;
        } else {
            result += colors[i];
            cantNoVacios += 1;
        }
    }

    result.rgb /= float(cantNoVacios);
    result.a /= float(colors.length());

    return result;
}

vec4 averageHandlingEmpty(vec4 colors[8]) {
    vec4 result = vec4(0);
    vec4 resultSinVacios = vec4(0);

    int cantNoVacios = 0;
    int cantVacios = 0;

    for (int i = 0; i < colors.length(); i++) {
        if (colors[i] == vec4(0)) {
            // Habría que sumar el alpha pero es 0
            cantVacios += 1;
        } else {
            result += colors[i];
            cantNoVacios += 1;
        }
    }

    result.rgb /= float(cantNoVacios);
    result.a /= float(colors.length());

    return result;
}