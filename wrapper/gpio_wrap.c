#include "gpio_wrap.h"
#include "emlib/inc/em_gpio.h"

void gpio_wrap_pin_cfg(unsigned int port, unsigned int pin, unsigned int mode, unsigned int out) {
    GPIO_Unlock();
    GPIO_PinModeSet((GPIO_Port_TypeDef)port, pin, (GPIO_Mode_TypeDef)mode, out);
}

void gpio_wrap_pin_high(unsigned int port, unsigned int pin) {
    GPIO_PinOutSet((GPIO_Port_TypeDef)port, pin);
}
void gpio_wrap_pin_low(unsigned int port, unsigned int pin) {
    GPIO_PinOutClear((GPIO_Port_TypeDef)port, pin);
}

void gpio_wrap_pin_toggle(unsigned int port, unsigned int pin) {
    GPIO_PinOutToggle((GPIO_Port_TypeDef)port, pin);
}

unsigned int gpio_wrap_pin_read(unsigned int port, unsigned int pin) {
    return (unsigned int)GPIO_PinInGet((GPIO_Port_TypeDef)port, pin);
}
