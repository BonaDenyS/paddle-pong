#include "opengl_wrapper.h"
#include <stdio.h>

void clear_screen() {
    // Stub: In real engine, you would call OpenGL clear functions
    printf("Screen cleared\n");
}

void draw_sprite(Sprite s) {
    // Stub: In real engine, you would draw the sprite
    printf("Drawing sprite at %.2f, %.2f\n", s.x, s.y);
}

int is_key_down(int key_code) {
    return 0; // Stub: Always return 0
}
