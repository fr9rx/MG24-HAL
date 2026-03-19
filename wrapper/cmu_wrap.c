#include "cmu_wrap.h"
#include "emlib/inc/em_cmu.h"


void cmu_wrap_enable_clock(unsigned int clock, unsigned int enable) {
    CMU_ClockEnable((CMU_Clock_TypeDef)clock, (bool)enable);
}

unsigned int cmu_wrap_clock_gpio() {
    return (unsigned int)cmuClock_GPIO;
}

unsigned int cmu_wrap_clock_hfper() {
    return (unsigned int)cmuClock_HFPER;
}

void cmu_wrap_enable_gpio() {
    CMU->CLKEN0_SET = CMU_CLKEN0_GPIO;
}
