
extern "C" {

void draw_waves();

void init_rust(void *lcd_ptr, void (*print_serial)(const char*));

void sample_wave();

void tiny_init(void (*print_serial)(const char*));

void blinka(void (*on)(), void (*off)());


} // extern "C"
