/***************************************************************************//**
 * @file
 * @brief Simple PWM helper API for EFR32MG24 / Seeed Studio XIAO MG24.
 *******************************************************************************
 * SPDX-License-Identifier: Zlib
 ******************************************************************************/

#include "em_pwm.h"
#if defined(TIMER_COUNT) && (TIMER_COUNT > 0)

#include <stddef.h>
#include "em_cmu.h"

static bool pwmTimerFromNumber(uint8_t timerNumber,
                               TIMER_TypeDef **timer,
                               CMU_Clock_TypeDef *clock)
{
  if ((timer == NULL) || (clock == NULL)) {
    return false;
  }

  switch (timerNumber) {
#if defined(TIMER0)
    case 0:
      *timer = TIMER0;
      *clock = cmuClock_TIMER0;
      return true;
#endif
#if defined(TIMER1)
    case 1:
      *timer = TIMER1;
      *clock = cmuClock_TIMER1;
      return true;
#endif
#if defined(TIMER2)
    case 2:
      *timer = TIMER2;
      *clock = cmuClock_TIMER2;
      return true;
#endif
#if defined(TIMER3)
    case 3:
      *timer = TIMER3;
      *clock = cmuClock_TIMER3;
      return true;
#endif
#if defined(TIMER4)
    case 4:
      *timer = TIMER4;
      *clock = cmuClock_TIMER4;
      return true;
#endif
    default:
      return false;
  }
}

static uint32_t pwmPrescaleToDiv(TIMER_Prescale_TypeDef prescale)
{
  switch (prescale) {
    case timerPrescale1: return 1U;
    case timerPrescale2: return 2U;
    case timerPrescale4: return 4U;
    case timerPrescale8: return 8U;
    case timerPrescale16: return 16U;
    case timerPrescale32: return 32U;
    case timerPrescale64: return 64U;
    case timerPrescale128: return 128U;
    case timerPrescale256: return 256U;
    case timerPrescale512: return 512U;
    case timerPrescale1024: return 1024U;
    default: return 1U;
  }
}

/* Validates only XIAO MG24 exposed digital pins, and rejects board-internal pins. */
static bool pwmIsXiaoMg24PinAllowed(GPIO_Port_TypeDef port, uint8_t pin)
{
  switch (port) {
    case gpioPortA:
      return (pin == 0U) || (pin == 3U) || (pin == 4U) || (pin == 5U)
             || (pin == 7U) || (pin == 8U) || (pin == 9U);
    case gpioPortB:
      return (pin == 0U) || (pin == 1U) || (pin == 2U) || (pin == 3U);
    case gpioPortC:
      return (pin <= 7U);
    case gpioPortD:
      return (pin == 2U);
    default:
      return false;
  }
}

static bool pwmConfigureRoute(uint8_t timerNumber,
                              uint8_t channel,
                              GPIO_Port_TypeDef port,
                              uint8_t pin)
{
  if ((timerNumber >= 5U) || (channel > 2U)) {
    return false;
  }

  switch (channel) {
    case 0U:
      GPIO->TIMERROUTE[timerNumber].CC0ROUTE = ((uint32_t)port << _GPIO_TIMER_CC0ROUTE_PORT_SHIFT)
                                               | ((uint32_t)pin << _GPIO_TIMER_CC0ROUTE_PIN_SHIFT);
      GPIO->TIMERROUTE_SET[timerNumber].ROUTEEN = GPIO_TIMER_ROUTEEN_CC0PEN;
      break;
    case 1U:
      GPIO->TIMERROUTE[timerNumber].CC1ROUTE = ((uint32_t)port << _GPIO_TIMER_CC1ROUTE_PORT_SHIFT)
                                               | ((uint32_t)pin << _GPIO_TIMER_CC1ROUTE_PIN_SHIFT);
      GPIO->TIMERROUTE_SET[timerNumber].ROUTEEN = GPIO_TIMER_ROUTEEN_CC1PEN;
      break;
    case 2U:
      GPIO->TIMERROUTE[timerNumber].CC2ROUTE = ((uint32_t)port << _GPIO_TIMER_CC2ROUTE_PORT_SHIFT)
                                               | ((uint32_t)pin << _GPIO_TIMER_CC2ROUTE_PIN_SHIFT);
      GPIO->TIMERROUTE_SET[timerNumber].ROUTEEN = GPIO_TIMER_ROUTEEN_CC2PEN;
      break;
    default:
      return false;
  }

  return true;
}

bool PWM_XiaoMg24Configure(uint8_t timerNumber,
                           uint8_t channel,
                           unsigned int prescale,
                           uint32_t frequencyHz,
                           uint16_t dutyPermille,
                           unsigned int port,
                           uint8_t pin,
                           bool invertOutput,
                           bool enableNow)
{
  TIMER_TypeDef *timer;
  CMU_Clock_TypeDef timerClock;
  TIMER_Prescale_TypeDef timerPrescale;
  GPIO_Port_TypeDef gpioPort;
  uint32_t timerFreq;
  uint32_t div;
  uint32_t top;

  timerPrescale = (TIMER_Prescale_TypeDef)prescale;
  gpioPort = (GPIO_Port_TypeDef)port;

  if ((channel > 2U)
      || (frequencyHz == 0U)
      || (dutyPermille > 1000U)
      || !pwmIsXiaoMg24PinAllowed(gpioPort, pin)) {
    return false;
  }

  if (!pwmTimerFromNumber(timerNumber, &timer, &timerClock)) {
    return false;
  }

  CMU_ClockEnable(cmuClock_GPIO, true);
  CMU_ClockEnable(timerClock, true);
  GPIO_PinModeSet(gpioPort, pin, gpioModePushPull, 0U);

  if (!pwmConfigureRoute(timerNumber, channel, gpioPort, pin)) {
    return false;
  }

  TIMER_Init_TypeDef timerInit = TIMER_INIT_DEFAULT;
  timerInit.enable = false;
  timerInit.prescale = timerPrescale;
  timerInit.mode = timerModeUp;
  TIMER_Init(timer, &timerInit);

  TIMER_InitCC_TypeDef ccInit = TIMER_INITCC_DEFAULT;
  ccInit.mode = timerCCModePWM;
  ccInit.outInvert = invertOutput;
  TIMER_InitCC(timer, channel, &ccInit);

  div = pwmPrescaleToDiv(timerPrescale);
  timerFreq = CMU_ClockFreqGet(timerClock) / div;
  if (timerFreq < frequencyHz) {
    return false;
  }

  top = (timerFreq / frequencyHz) - 1U;
  if (top > _TIMER_TOP_MASK) {
    top = _TIMER_TOP_MASK;
  }

  TIMER_TopSet(timer, top);
  (void)PWM_XiaoMg24WriteDuty(timerNumber, channel, dutyPermille);

  if (enableNow) {
    timer->CMD = TIMER_CMD_START;
  }

  return true;
}

bool PWM_XiaoMg24WriteDuty(uint8_t timerNumber,
                           uint8_t channel,
                           uint16_t dutyPermille)
{
  TIMER_TypeDef *timer;
  CMU_Clock_TypeDef timerClock;
  uint32_t top;
  uint32_t compare;

  if ((channel > 2U) || (dutyPermille > 1000U)) {
    return false;
  }

  if (!pwmTimerFromNumber(timerNumber, &timer, &timerClock)) {
    return false;
  }

  (void)timerClock;
  top = TIMER_TopGet(timer);
  compare = ((top + 1U) * (uint32_t)dutyPermille) / 1000U;
  if (compare > top) {
    compare = top;
  }

  TIMER_CompareSet(timer, channel, compare);
  return true;
}

#endif /* defined(TIMER_COUNT) && (TIMER_COUNT > 0) */
