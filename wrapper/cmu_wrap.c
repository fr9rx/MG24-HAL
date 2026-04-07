#include "cmu_wrap.h"
#include "em_cmu.h"


void cmu_wrap_enable_clock(unsigned int clock, unsigned int enable) {
    CMU_Unlock();
    CMU_ClockEnable((CMU_Clock_TypeDef)clock, (bool)enable);
}

unsigned int cmu_wrap_clock_gpio() {
    return (unsigned int)cmuClock_GPIO;
}

void cmu_wrap_enable_gpio() {
    cmu_wrap_enable_clock(cmu_wrap_clock_gpio(), 1);
}

unsigned int cmu_wrap_core_clock_hz() {
    return (unsigned int)CMU_ClockFreqGet(cmuClock_CORE);
}

unsigned int cmu_wrap_init_78mhz() {
    CMU_DPLLInit_TypeDef dpll = CMU_DPLLINIT_DEFAULT;

    // XIAO MG24 Sense uses a 39 MHz HFXO. Ratio 2.0 gives 78 MHz core clock.
    dpll.frequency = 78000000U;
    dpll.n = 3839U;
    dpll.m = 1919U;
    dpll.refClk = cmuSelect_HFXO;

    CMU_Unlock();

    if (!CMU_DPLLLock(&dpll)) {
        return 0U;
    }

    CMU_ClockSelectSet(cmuClock_SYSCLK, cmuSelect_HFRCODPLL);
    CMU_ClockDivSet(cmuClock_HCLK, 1U);
    return 1U;
}
