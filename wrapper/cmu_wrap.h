#ifndef CMU_WRAP
#define CMU_WRAP
#define GPIO_PRESENT

void cmu_wrap_enable_clock(unsigned int clock, unsigned int enable);
unsigned int cmu_wrap_clock_gpio(void);
unsigned int cmu_wrap_clock_hfper(void);
void cmu_wrap_enable_gpio(void);

#endif
