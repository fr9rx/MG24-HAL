#include "em_cmu.h"

void CMU_ClockEnable(CMU_Clock_TypeDef clock, bool enable)
{
    uint32_t bit = (clock >> CMU_EN_BIT_POS) & CMU_EN_BIT_MASK;
    (void)clock;

    if (enable)
    {
        CMU->CLKEN0_SET = 1UL << bit;
    }
    else
    {
        CMU->CLKEN0_CLR = 1UL << bit;
    }
}

