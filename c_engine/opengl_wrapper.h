#ifndef OPENGL_WRAPPER_H
#define OPENGL_WRAPPER_H

typedef struct {
    float x, y, width, height;
} Sprite;

void clear_screen();
void draw_sprite(Sprite s);
int is_key_down(int key_code);

#endif
