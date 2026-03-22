#include <stdint.h>

/*
 * Minimal clock service symbols required by em_cmu.c in this project setup.
 * These stubs avoid pulling full system_efr32mg24.c (TrustZone/CMSE startup paths).
 */

static uint32_t s_hfrcodpll_hz = 19000000U;
static uint32_t s_hclk_hz = 19000000U;

uint32_t SystemHFRCODPLLClockGet(void)
{
  return s_hfrcodpll_hz;
}

void SystemHFRCODPLLClockSet(uint32_t freq)
{
  if (freq > 0U) {
    s_hfrcodpll_hz = freq;
    s_hclk_hz = freq;
  }
}

uint32_t SystemSYSCLKGet(void)
{
  return s_hfrcodpll_hz;
}

uint32_t SystemHCLKGet(void)
{
  return s_hclk_hz;
}

uint32_t SystemMaxCoreClockGet(void)
{
  return 78000000U;
}

uint32_t SystemHFXOClockGet(void)
{
  return 39000000U;
}

uint32_t SystemCLKIN0Get(void)
{
  return SystemHFXOClockGet();
}

uint32_t SystemFSRCOClockGet(void)
{
  return 20000000U;
}

uint32_t SystemHFRCOEM23ClockGet(void)
{
  return 4000000U;
}

uint32_t SystemLFRCOClockGet(void)
{
  return 32768U;
}

uint32_t SystemULFRCOClockGet(void)
{
  return 1000U;
}

uint32_t SystemLFXOClockGet(void)
{
  return 32768U;
}

