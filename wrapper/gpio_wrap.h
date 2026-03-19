#ifndef GPIO_WRAP
#define GPIO_WRAP

void gpio_wrap_pin_cfg(unsigned int port, unsigned int pin, unsigned int mode, unsigned int out);
void gpio_wrap_pin_high(unsigned int port, unsigned int pin);
void gpio_wrap_pin_low(unsigned int port, unsigned int pin);
void gpio_wrap_pin_toggle(unsigned int port, unsigned int pin);

#endif
