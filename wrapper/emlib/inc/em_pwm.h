/***************************************************************************//**
 * @file
 * @brief Simple PWM helper API for EFR32MG24 / Seeed Studio XIAO MG24.
 *******************************************************************************
 * SPDX-License-Identifier: Zlib
 ******************************************************************************/

#ifndef EM_PWM_H
#define EM_PWM_H

#include "em_device.h"
#if defined(TIMER_COUNT) && (TIMER_COUNT > 0)

#include <stdbool.h>
#include <stdint.h>
#include "em_gpio.h"
#include "em_timer.h"

#ifdef __cplusplus
extern "C" {
#endif

/***************************************************************************//**
 * @addtogroup pwm PWM - Simple PWM Helper
 * @{
 ******************************************************************************/

/***************************************************************************//**
 * @brief Big all-in-one configuration function for XIAO MG24 PWM.
 *
 * @param[in] timerNumber
 *   TIMER instance number (0..4 on MG24).
 *
 * @param[in] channel
 *   Compare channel (0..2).
 *
 * @param[in] prescale
 *   TIMER prescaler.
 *
 * @param[in] frequencyHz
 *   PWM frequency in Hz.
 *
 * @param[in] dutyPermille
 *   Duty in 0..1000 (500 = 50.0%).
 *
 * @param[in] port
 *   GPIO port for PWM output (XIAO MG24 exposed pins only).
 *
 * @param[in] pin
 *   GPIO pin for PWM output.
 *
 * @param[in] invertOutput
 *   true to invert PWM output polarity.
 *
 * @param[in] enableNow
 *   true to start timer immediately after setup.
 *
 * @return
 *   true on success, false on invalid timer/channel/pin/frequency.
 ******************************************************************************/
bool PWM_XiaoMg24Configure(uint8_t timerNumber,
                           uint8_t channel,
                           unsigned int prescale,
                           uint32_t frequencyHz,
                           uint16_t dutyPermille,
                           unsigned int port,
                           uint8_t pin,
                           bool invertOutput,
                           bool enableNow);

/***************************************************************************//**
 * @brief Write duty cycle for a configured PWM channel.
 *
 * @param[in] timerNumber
 *   TIMER instance number (0..4 on MG24).
 *
 * @param[in] channel
 *   Compare channel (0..2).
 *
 * @param[in] dutyPermille
 *   Duty in 0..1000 (500 = 50.0%).
 *
 * @return
 *   true on success, false on invalid parameters.
 ******************************************************************************/
bool PWM_XiaoMg24WriteDuty(uint8_t timerNumber,
                           uint8_t channel,
                           uint16_t dutyPermille);

/** @} (end addtogroup pwm) */

#ifdef __cplusplus
}
#endif

#endif /* defined(TIMER_COUNT) && (TIMER_COUNT > 0) */
#endif /* EM_PWM_H */
