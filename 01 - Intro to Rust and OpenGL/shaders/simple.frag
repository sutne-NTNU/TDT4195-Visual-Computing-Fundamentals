#version 450 core

out vec4 color;

// mod(a, b) = a % b
int mod(int a, int b){
    return int(a - (b * floor(a/b)));
}

void main()
{
    // Plain White
    color = vec4(1.0, 1.0, 1.0, 1.0);

    // normalize coordinates (between 0 and 1)
    float x = gl_FragCoord.x / 800; 
    float y = gl_FragCoord.y / 800;

    // Color based on position
    color = vec4(
        1.0 - x, // R
        1.0 - y, // G 
        1.0,     // B 
        1.0                                            
    );

    // Checkerboard pattern
    int checker_size = 75;
    int x_square = int(gl_FragCoord.x / checker_size);
    int y_square = int(gl_FragCoord.y / checker_size);
    
    if (mod(x_square + y_square, 2) == 0) {
        color = vec4(1.0 - x, 1.0 - y, 1.0, 1.0);
    } else {
        color = vec4(x, y, 1.0, 1.0);
    }
}