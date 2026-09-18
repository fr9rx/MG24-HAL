

EFR32MG24 Wireless SoC Family Data
## Sheet
The EFR32MG24 Wireless SoCs are ideal for mesh IoT wireless
connectivity using Matter, OpenThread and Zigbee.
With key features like high performance 2.4 GHz RF, low current consumption, an AI/ML
hardware accelerator and Secure Vault, IoT device makers can create smart, robust, and
energy-efficient products that are secure from remote and local cyber-attacks. A Cor-
tex®-M33 running up to 78 MHz and up to 1536 kB of Flash and 256 kB of RAM pro-
vides resources for demanding applications while leaving room for future growth.
Target applications include:
## KEY FEATURES
- 32-bit ARM® Cortex®-M33 core with 78
MHz maximum operating frequency
- Up to 1536 kB of flash and 256 kB of RAM
- High performance radio with up to +19.5
dBm output power
- Energy efficient design with low active and
sleep currents
## • Secure Vault™
- AI/ML Hardware Accelerator
- Channel sounding
- AEC-Q100 Grade 1 (selected part
numbers only)
- Smart Home - Gateways and hubs, sen-
sors, switches, door locks, smart plugs
- Lighting - LED bulbs, luminaires
- Building Automation - Gateways, sen-
sors, switches, location services
- AI/ML - Predictive maintenance, glass
break detection, wake-word detection
## • Remote Controls
- Passive Keyless Entry (PKE)
- Passive Entry Passive Start (PEPS)
## • Tire Pressure Monitoring System
## (TPMS)
- Tire Monitoring Sensor (TMS)
## • Rearview Mirrors
## • Garage Door Openers
Timers and Triggers
32-bit bus
## Peripheral Reflex System
Serial InterfacesI/O Ports
Analog I/F
Lowest power mode with peripheral operational:
## USART
## EUSART
EM4—Shutoff
## Energy Management
Brown-Out
## Detector
## Voltage Regulator
Power-On Reset
SecurityClock Management
HF Crystal
## Oscillator
LF Crystal
## Oscillator
## LF
RC Oscillator
## HF
RC Oscillator
Ultra LF RC
## Oscillator
## Core / Memory / Acceleration
ARM Cortex
## TM
M33 processor
with DSP extensions,
FPU and TrustZone
ETMDebug InterfaceRAM MemoryLDMA Controller
## Flash Program
## Memory
## System Real Time
## Counter
Timer/Counter
## Low Energy Timer
## Watchdog Timer
## Protocol Timer
EM3—StopEM2—Deep SleepEM1—SleepEM0—Active
## Fast Startup
RC Oscillator
Back-Up Real Time
## Counter
## I
## 2
## C
## EUSART
DC-DC Converter
## Radio Subsystem
RX/TX Frontend with
Integrated 0dBm, +10dBm,
and +20dBm PA
## Frequency Synthesizer
ARM Cortex
## TM
## M0+
## Radio Controller
## CRC
## BUFC RAM
## FRC
## DEMOD
## AGC
## IFADC
## MOD
## External Interrupts
## General
Purpose I/O
## Pin Reset
## Pin Wakeup
## Keypad Scanner
## Pulse Counter
## IADC
## VDAC
## Temperature
## Sensor
## ACMP
## Secure Debug
## Authentication
## Crypto Acceleration
## Secure Engine
## True Random Number
## Generator
DPA Countermeasures
AI/ML Hardware
## Accelerator
## (MVP)
silabs.com | Building a more connected world.Copyright © 2024 by Silicon LaboratoriesRev. 1.2

## 1.  Feature List
The EFR32MG24 highlighted features are listed below.
•Low Power Wireless System-on-Chip
## •
High Performance 32-bit 78 MHz ARM Cortex
## ®
-M33 with
DSP instruction and floating-point unit for efficient signal
processing
- Up to 1536 kB flash program memory
- Up to 256 kB RAM data memory
- 2.4 GHz radio operation
- Matrix Vector Processor for AI/ML acceleration
•Radio Performance
- -105.4 dBm sensitivity @ 250 kbps O-QPSK DSSS
- -105.7 dBm sensitivity @ 125 kbps GFSK
- -97.6 dBm sensitivity @ 1 Mbps GFSK
- -94.8 dBm sensitivity @ 2 Mbps GFSK
- TX power up to 19.5 dBm
•Low System Energy Consumption
- 4.4 mA RX current (1 Mbps GFSK)
- 5.1 mA RX current (250 kbps O-QPSK DSSS)
- 5 mA TX current @ 0 dBm output power
- 19.1 mA TX current @ 10 dBm output power
- 156.8 mA TX current @ 19.5 dBm output power
- 33.4 μA/MHz in Active Mode (EM0) at 39.0 MHz
- 1.3 μA EM2 DeepSleep current (16 kB RAM retention and
RTC running from LFRCO)
•Supported Modulation Format
- 2 (G)FSK with fully configurable shaping
## • OQPSK DSSS
## • (G)MSK
•Protocol Support
## • Matter
- OpenThread
## • Zigbee
## • Bluetooth Low Energy
## • Bluetooth Mesh
- Proprietary 2.4 GHz
## • Multiprotocol
- Direction finding using Angle-of-Arrival (AoA) and Angle-of-
Departure (AoD)
- Channel sounding
- 40 MHz crystal required
- Maximum TX power for Channel Sounding is 10 dBm for
all part numbers
•Wide Operating Range
- 1.71 V to 3.8 V single power supply
- -40 °C to 125 °C
## •
## Secure Vault
- Hardware Cryptographic Acceleration for AES128/192/256,
ChaCha20-Poly1305, SHA-1, SHA-2/256/384/512, ECDSA
+ECDH(P-192, P-256, P-384, P-521), Ed25519 and
Curve25519, J-PAKE, PBKDF2
- True Random Number Generator (TRNG)
- ARM® TrustZone®
- Secure Boot (Root of Trust Secure Loader)
## • Secure Debug Unlock
- DPA Countermeasures
- Secure Key Management with PUF
- Anti-Tamper
## • Secure Attestation
•Wide selection of MCU peripherals
- Analog to Digital Converter (IADC)
- 12-bit @ 1 Msps or 16-bit @ 76.9 ksps
- Select OPNs support High Speed Mode (up to 2 Msps)
and High Accuracy Mode (up to 16 bits ENOB at 3.8
ksps)
- 2 × Analog Comparator (ACMP)
- 2 × Digital to Analog Converter (VDAC)
- Up to 32 General Purpose I/O pins with output state reten-
tion and asynchronous interrupts
- 8 Channel DMA Controller (LDMA)
## • 16 Asynchronous Channel, 4 Synchronous Channel Periph-
eral Reflex System (PRS)
- 3 × 16-bit Timer/Counter with 3 Compare/Capture/PWM
channels (TIMER2/3/4)
- 2 × 32-bit Timer/Counter with 3 Compare/Capture/PWM
channels (TIMER0/1)
- 2 × 32-bit Real Time Counter (SYSRTC/BURTC)
- 24-bit Low Energy Timer for waveform generation (LETIM-
## ER)
- 16-bit Pulse Counter with asynchronous operation (PCNT)
- 2 × Watchdog Timer (WDOG)
- 1 × Universal Synchronous/Asynchronous Receiver/Trans-
mitter (USART), supporting UART/SPI/SmartCard (ISO
7816)/IrDA/I
## 2
## S
- 2 × Enhanced Universal Synchronous/Asynchronous Re-
ceiver/Transmitter (EUSART) supporting UART/SPI/DALI/
IrDA
## •
## 2 × I
## 2
C interface with SMBus support
- Low-Frequency RC Oscillator with precision mode to re-
place 32 kHz sleep crystal (LFRCO)
- Keypad scanner supporting up to 6x8 matrix (KEYSCAN)
- Die temperature sensor with +/-1.5 °C accuracy after single-
point calibration
•Packages
•QFN40 5 mm × 5 mm × 0.85 mm
•QFN48 6 mm × 6 mm × 0.85 mm
EFR32MG24 Wireless SoC Family Data Sheet
## Feature List
silabs.com | Building a more connected world.Rev. 1.2  |  2

## 2.  Ordering Information
## Table 2.1.  Ordering Information
## Ordering Code
Max TX
## Power
## Flash
(kB)
## RAM
(kB)
## Secure
## Vault
IADC High-
## Speed /
High-Accu-
racy
## Multi
## Vector
## Pro-
cessor
GPIOPackage / Pinout
## EFR32MG24B310F1536IM48-B
## 1
10 dBm1536256HighYesYes28QFN48 / ADC
## EFR32MG24B220F1536IM48-B
## 1
19.5 dBm1536256HighNoYes32QFN48 / Standard
## EFR32MG24B210F1536IM48-B
## 1
10 dBm1536256HighNoYes32QFN48 / Standard
## EFR32MG24B120F1536IM48-B
## 1
19.5 dBm1536256HighYesNo28QFN48 / ADC
## EFR32MG24B110F1536IM48-B
## 1
10 dBm1536256HighYesNo28QFN48 / ADC
## EFR32MG24B020F1536IM48-B
## 1
19.5 dBm1536256HighNoNo32QFN48 / Standard
## EFR32MG24B020F1536IM40-B
## 1
19.5 dBm1536256HighNoNo26QFN40 / Standard
## EFR32MG24B020F1024IM48-B
## 1
19.5 dBm1024128HighNoNo32QFN48 / Standard
## EFR32MG24B010F1536IM48-B
## 1
10 dBm1536256HighNoNo32QFN48 / Standard
## EFR32MG24B010F1536IM40-B
## 1
10 dBm1536256HighNoNo26QFN40 / Standard
## EFR32MG24B010F1024IM48-B
## 1
10 dBm1024128HighNoNo32QFN48 / Standard
## EFR32MG24A420F1536IM48-B
## 1
19.5 dBm1536256MidNoNo32QFN48 / Standard
## EFR32MG24A420F1536IM40-B
## 1
19.5 dBm1536256MidNoNo26QFN40 / Standard
## EFR32MG24A410F1536IM48-B
## 1
10 dBm1536256MidNoNo32QFN48 / Standard
## EFR32MG24A410F1536IM40-B
## 1
10 dBm1536256MidNoNo26QFN40 / Standard
## EFR32MG24A110F1024IM48-B
## 1
10 dBm1024128MidYesNo28QFN48 / ADC
## EFR32MG24A021F1024IM40-B
## 1
19.5 dBm1024128MidNoNo25QFN40 / HFCLKOUT
## EFR32MG24A020F1536IM48-B
## 1
19.5 dBm1536192MidNoNo32QFN48 / Standard
## EFR32MG24A020F1536IM40-B
## 1
19.5 dBm1536192MidNoNo26QFN40 / Standard
## EFR32MG24A020F1024IM48-B
## 1
19.5 dBm1024128MidNoNo32QFN48 / Standard
## EFR32MG24A020F1024IM40-B
## 1
19.5 dBm1024128MidNoNo26QFN40 / Standard
## EFR32MG24A010F1536IM48-B
## 1
10 dBm1536192MidNoNo32QFN48 / Standard
## EFR32MG24A010F1536IM40-B
## 1
10 dBm1536192MidNoNo26QFN40 / Standard
## EFR32MG24A010F1024IM48-B
## 1
10 dBm1024128MidNoNo32QFN48 / Standard
## EFR32MG24A010F1024IM40-B
## 1
10 dBm1024128MidNoNo26QFN40 / Standard
## Note:
1.AEC-Q100 Grade 1 qualified parts are not Automotive Grade. Refer to AN1421: AEC Qualification vs Automotive Grade for de-
tails.

EFR32MG24 Wireless SoC Family Data Sheet
## Ordering Information
silabs.com | Building a more connected world.Rev. 1.2  |  3

## Product Family
## Security
## Features
## Memory
## Temperature Grade
## Size
## Package
## Pins
## Revision
## Tape & Reel
## EFR32MG24B020FIM48R
## -
## 1536B
## Figure 2.1.  Ordering Code Key
FieldOptions
Product Family•EFR32MG24: Mighty Gecko 24 Family
Security•A: Secure Vault Mid
•B: Secure Vault High
## Features [f1][f2][f3]
- f1
## •0: Base Configuration
•1: IADC High-Speed / High-Accuracy Available
•2: Matrix Vector Processor (MVP) Available
•3: IADC High-Speed / High-Accuracy and Matrix Vector Processor (MVP) Available
•4: 256K RAM and Secure Vault – Mid
- f2
•1: 10 dBm PA Transmit Power
•2: 19.5 dBm PA Transmit Power
- f3
•0: No feature enabled
•1: High Quality HFCLKOUT Pin Available
Memory•F: Flash
Size•Memory Size in kBytes
Temperature Grade•G: -40 to +85 °C
•I: -40 to +125 °C
Package•M: QFN
Pins•Number of Package Pins
Revision•B: Revision B
Tape & Reel•R: Tape & Reel (optional)
EFR32MG24 Wireless SoC Family Data Sheet
## Ordering Information
silabs.com | Building a more connected world.Rev. 1.2  |  4

Table of Contents
## 1.  Feature List................................2
## 2.  Ordering Information............................3
## 3.  System Overview..............................9
## 3.1  Introduction...............................9
## 3.2  Radio.................................9
## 3.2.1  Antenna Interface...........................9
3.2.2  Fractional-N Frequency Synthesizer.....................10
## 3.2.3  Receiver Architecture..........................10
## 3.2.4  Transmitter Architecture.........................10
3.2.5  Packet and State Trace.........................10
## 3.2.6  Data Buffering.............................10
3.2.7  Radio Controller (RAC)..........................10
3.2.8  RF Signal Identifier...........................11
3.3  General Purpose Input/Output (GPIO)......................11
3.4  Keypad Scanner (KEYSCAN).........................11
## 3.5  Clocking................................11
3.5.1  Clock Management Unit (CMU).......................11
3.5.2  Internal and External Oscillators.......................11
3.6  Counters/Timers and PWM..........................12
3.6.1  Timer/Counter (TIMER).........................12
3.6.2  Low Energy Timer (LETIMER).......................12
3.6.3  System Real Time Clock with Capture (SYSRTC).................12
3.6.4  Back-Up Real Time Counter (BURTC)....................12
3.6.5  Watchdog Timer (WDOG).........................12
3.7  Communications and Other Digital Peripherals...................12
3.7.1  Universal Synchronous/Asynchronous Receiver/Transmitter (USART)..........12
3.7.2  Enhanced Universal Synchronous/Asynchronous Receiver/Transmitter (EUSART).....12
3.7.3  Inter-Integrated Circuit Interface (I
## 2
## C) .....................13
3.7.4  Peripheral Reflex System (PRS)......................13
## 3.8  Secure Vault Features...........................13
3.8.1  Secure Boot with Root of Trust and Secure Loader (RTSL).............14
## 3.8.2  Cryptographic Accelerator.........................14
## 3.8.3  True Random Number Generator......................14
3.8.4  Secure Debug with Lock/Unlock.......................14
3.8.5  DPA Countermeasures..........................14
3.8.6  Secure Key Management with PUF.....................15
3.8.7  Anti-Tamper.............................15
## 3.8.8  Secure Attestation...........................15
## 3.9  Analog.................................15
3.9.1  Analog to Digital Converter (IADC)......................15
3.9.2  Analog Comparator (ACMP)........................16
3.9.3  Digital to Analog Converter (VDAC).....................16
silabs.com
| Building a more connected world.Rev. 1.2 |  5

## 3.10  Power................................17
3.10.1  Energy Management Unit (EMU)......................17
## 3.10.2  Voltage Scaling............................17
3.10.3  DC-DC Converter...........................17
## 3.10.4  Power Domains............................18
3.11  Reset Management Unit (RMU)........................18
3.12  Core, Memory, and Accelerators.......................19
## 3.12.1  Processor Core............................19
3.12.2  Memory System Controller (MSC).....................19
3.12.3  Linked Direct Memory Access Controller (LDMA)................19
3.12.4  Matrix Vector Processor (MVP)......................19
## 3.13  Memory Map..............................20
## 3.14  Configuration Summary..........................21
## 4.  Electrical Specifications..........................22
## 4.1  Electrical Characteristics..........................22
## 4.2  Absolute Maximum Ratings..........................23
## 4.3  General Operating Conditions.........................24
4.4  DC-DC Converter.............................26
## 4.5  Thermal Characteristics...........................28
## 4.6  Current Consumption............................29
4.6.1  MCU current consumption using DC-DC at 3.0 V input...............29
4.6.2  Radio current consumption at 3.0V using DCDC.................32
4.6.3  MCU current consumption at 3.0 V......................34
4.6.4  Radio current consumption at 3.0V......................37
4.6.5  MCU current consumption at 1.8 V......................39
4.6.6  Radio current consumption at 1.8V......................42
## 4.7  Flash Characteristics............................44
4.8  Energy Mode Wake-up and Entry Times.....................45
4.9  2.4 GHz RF Transceiver Characteristics.....................46
4.9.1  RF Transmitter Characteristics.......................46
4.9.2  RF Receiver Characteristics........................54
## 4.10  Oscillators...............................60
4.10.1  High Frequency Crystal Oscillator (HFXO)...................60
4.10.2  Low Frequency Crystal Oscillator (LFXO)...................61
4.10.3  High Frequency RC Oscillator (HFRCO)...................62
4.10.4  Fast Start-Up RC Oscillator (FSRCO)....................63
4.10.5  Precision Low Frequency RC Oscillator (LFRCO)................64
4.10.6  Ultra Low Frequency RC Oscillator (ULFRCO).................64
4.11  GPIO Pins (GPIO)............................65
4.12  Analog to Digital Converter (IADC).......................67
4.13  Analog Comparator (ACMP).........................73
4.14  Digital to Analog Converter (VDAC)......................75
silabs.com
| Building a more connected world.Rev. 1.2 |  6

## 4.15  Temperature Sensor...........................77
## 4.16  Brown Out Detectors...........................78
## 4.16.1  DVDD BOD.............................78
4.16.2  Low-Energy DVDD BOD.........................78
4.16.3  AVDD and IOVDD BODs........................79
4.17  Pulse Counter (PCNT)...........................79
4.18  USART SPI Main Timing..........................80
4.18.1  USART SPI Main Timing, Voltage Scaling = VSCALE2..............81
4.18.2  USART SPI Main Timing, Voltage Scaling = VSCALE1..............81
4.19  USART SPI Secondary Timing........................82
4.19.1  USART SPI Secondary Timing, Voltage Scaling = VSCALE2............83
4.19.2  USART SPI Secondary Timing, Voltage Scaling = VSCALE1............83
4.20  EUSART SPI Main Timing..........................84
4.20.1  EUSART SPI Main Timing, Voltage Scaling = VSCALE2..............84
4.20.2  EUSART SPI Main Timing, Voltage Scaling = VSCALE1..............85
4.21  EUSART SPI Secondary Timing.......................86
4.21.1  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE2............86
4.21.2  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE1............87
4.21.3  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE0............87
4.22  I2C Electrical Specifications.........................88
4.22.1  I2C Standard-mode (Sm)........................88
4.22.2  I2C Fast-mode (Fm)..........................89
4.22.3  I2C Fast-mode Plus (Fm+)........................90
## 4.23  Boot Timing..............................90
4.24  Crypto Operation Timing for SE Manager API...................92
4.25  Crypto Operation Average Current for SE Manager API................94
4.26  Matrix Vector Processor (MVP)........................96
## 4.27  Typical Performance Curves.........................97
## 4.27.1  Supply Current............................98
4.27.2  RF Characteristics..........................100
4.27.3  DC-DC Converter..........................102
## 4.27.4  IADC..............................103
## 4.27.5  GPIO..............................104
## 5.  Typical Connections...........................105
## 5.1  Power................................105
## 5.2  Other Connections............................106
## 6.  Pin Definitions..............................107
6.1  QFN48 / Standard Device Pinout.......................107
6.2  QFN48 / ADC Device Pinout........................109
6.3  QFN40 / Standard Device Pinout.......................111
6.4  QFN40 / HFCLKOUT Device Pinout......................113
## 6.5  Alternate Function Table..........................115
silabs.com
| Building a more connected world.Rev. 1.2 |  7

## 6.6  Analog Peripheral Connectivity.......................116
## 6.7  Digital Peripheral Connectivity........................117
-  QFN40 Package Specifications........................121
7.1  QFN40 Package Dimensions........................121
7.2  QFN40 PCB Land Pattern.........................123
7.3  QFN40 Package Marking.........................124
-  QFN48 Package Specifications........................125
8.1  QFN48 Package Dimensions........................125
8.2  QFN48 PCB Land Pattern.........................127
8.3  QFN48 Package Marking.........................128
## 9.  Revision History.............................129
silabs.com | Building a more connected world.Rev. 1.2 |  8

## 3.  System Overview
## 3.1  Introduction
The EFR32 product family combines an energy-friendly MCU with a high performance radio transceiver. The devices are well suited for
secure connected IoT multi-protocol devices requiring high performance and low energy consumption. This section gives a short intro-
duction to the full radio and MCU system. The detailed functional description can be found in the EFR32xG24 Reference Manual.
A block diagram of the EFR32MG24 family is shown in Figure 3.1 Detailed EFR32MG24 Block Diagram on page 9. The diagram
shows a superset of features available on the family, which vary by OPN. For more information about specific device features, consult
## 2. Ordering Information.
## Analog Peripherals
## Clock Management
## HFRCO
Core and Memory
Up to 1536 KB ISP Flash
## Program Memory
Up to 256 KB RAM
## A
## H
## B
## Watchdog
## Timer
RESETn
## Digital Peripherals
## Input Mux
## DBUS
## Port
## Mappers
Port I/O Configuration
12-20 bit
## ADC
## VDD
## Internal
## Reference
## IOVDD
## LFXO
## FSRCO
## HFXO
## Trust Zone
## LFRCO
## A
## P
## B
LDMA Controller
ABUS Multiplexers
## CRC
## I2C
## USART
## SYSRTC
## TIMER
## LETIMER
## Port D
## Drivers
PDn
## Port C
## Drivers
PCn
## Port B
## Drivers
PBn
## Port A
## Drivers
PAn
## HFXTAL_I
## HFXTAL_O
## LFXTAL_I
## LFXTAL_O
## Debug Signals
(shared w/GPIO)
## Reset Management Unit,
Brown Out and POR
Serial Wire and ETM
## Debug / Programming
with Debug Challenge I/F
## RF2G4_IO
## TRNG
## ULFRCO
## Temperature
## Sensor
## EUSART
## KEYSCAN
## Energy Management
## DVDD
## VREGVDD
## VREGSW
bypass
## AVDD
## PAVDD
## RFVDD
## DECOUPLE
## IOVDD
## Voltage
## Monitor
## Voltage
## Regulator
## DC-DC
## Converter
## CRYPTOACC
ARM Cortex-M33 Core
with Floating Point Unit
## Radio Subsystem
RX/TX Frontend
with Integrated PA
## Frequency
## Synthesizer
ARM Cortex
## TM
## M0+
## Radio Controller
## CRC
## BUFC RAM
## FRC
## DEMOD
## AGC
## IFADC
## MOD
## VDAC
## ACMP
## Security
## Crypto
## Acceleration
## True Random
## Number Generator
## DPA
## Countermeasures
## Secure Engine
## Secure Debug
## Authentication
## Accelerators
## Matrix Vector
## Processor
Figure 3.1.  Detailed EFR32MG24 Block Diagram
## 3.2  Radio
The EFR32MG24 Wireless SoC features a highly configurable radio transceiver supporting Zigbee, Bluetooth Low Energy and Blue-
tooth Mesh wireless protocols.
## 3.2.1  Antenna Interface
The 2.4 GHz antenna interface consists of a single-ended pin (RF2G4_IO). The external components for the antenna interface in typi-
cal applications are shown in the RF Matching Networks section.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  9

3.2.2  Fractional-N Frequency Synthesizer
The EFR32MG24 contains a high performance, low phase noise, fully integrated fractional-N frequency synthesizer. The synthesizer is
used in receive mode to generate the LO frequency for the down-conversion mixer. It is also used in transmit mode to directly generate
the modulated RF carrier.
The fractional-N architecture provides excellent phase noise performance, frequency resolution better than 100 Hz, and low energy
consumption. The synthesizer’s fast frequency settling allows for very short receiver and transmitter wake up times to reduce system
energy consumption.
## 3.2.3  Receiver Architecture
The EFR32MG24 uses a low-IF receiver architecture, consisting of a Low-Noise Amplifier (LNA) followed by an I/Q down-conversion
mixer. The I/Q signals are further filtered and amplified before being sampled by the IF analog-to-digital converter (IFADC).
The IF frequency is configurable from 150 kHz to 1371 kHz. The IF can further be configured for high-side or low-side injection, provid-
ing flexibility with respect to known interferers at the image frequency.
The Automatic Gain Control (AGC) module adjusts the receiver gain to optimize performance and avoid saturation for excellent selec-
tivity and blocking performance. The 2.4 GHz radio is calibrated at production to improve image rejection performance.
Demodulation is performed in the digital domain. The demodulator performs configurable decimation and channel filtering to allow re-
ceive bandwidths ranging from 0.1 to 2530 kHz. High carrier frequency and baud rate offsets are tolerated by active estimation and
compensation. Advanced features supporting high quality communication under adverse conditions include forward error correction by
block and convolutional coding as well as Direct Sequence Spread Spectrum (DSSS).
A Received Signal Strength Indicator (RSSI) is available for signal quality metrics, for level-based proximity detection, and for RF chan-
nel access by Collision Avoidance (CA) or Listen Before Talk (LBT) algorithms. An RSSI capture value is associated with each received
frame and the dynamic RSSI measurement can be monitored throughout reception.
## 3.2.4  Transmitter Architecture
The EFR32MG24 uses a direct-conversion transmitter architecture. For constant envelope modulation formats, the modulator controls
phase and frequency modulation in the frequency synthesizer. Transmit symbols or chips are optionally shaped by a digital shaping
filter. The shaping filter is fully configurable, including the BT product, and can be used to implement Gaussian or Raised Cosine shap-
ing.
Carrier Sense Multiple Access - Collision Avoidance (CSMA-CA) or Listen Before Talk (LBT) algorithms can be automatically timed by
the EFR32MG24. These algorithms are typically defined by regulatory standards to improve inter-operability in a given bandwidth be-
tween devices that otherwise lack synchronized RF channel access.
3.2.5  Packet and State Trace
The EFR32MG24 Frame Controller has a packet and state trace unit that provides valuable information during the development phase.
It features the following:
- Non-intrusive trace of transmit data, receive data and state information
- Data observability on a single-pin UART data output or on a two-pin SPI data output
- Configurable data output bitrate / baudrate
- Multiplexed transmitted data, received data and state / meta information in a single serial data stream
## 3.2.6  Data Buffering
The EFR32MG24 features an advanced Radio Buffer Controller (BUFC) capable of handling up to 4 buffers of adjustable size from 64
bytes to 4096 bytes. Each buffer can be used for RX, TX or both. The buffer data is located in RAM, enabling zero-copy operations.
3.2.7  Radio Controller (RAC)
The Radio Controller controls the top level state of the radio subsystem in the EFR32MG24. It performs the following tasks:
- Precisely-timed control of enabling and disabling of the receiver and transmitter circuitry
- Run-time calibration of receiver, transmitter and frequency synthesizer
- Detailed frame transmission timing, including optional LBT or CSMA-CA
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  10

3.2.8  RF Signal Identifier
When an IoT radio is placed next to a high duty-cycle co-located Wi-Fi radio transmission, IoT radios are blocked from receiving weak
signals. The RF Signal Identifier feature available on EFR32MG24 devices enables the IoT radio to detect partial 802.15.4 or BLE/BT
Mesh packets. When a partial packet is detected, the IoT radio can communicate this information to the corresponding Wi-Fi device
(through serial interface or GPIO asserts), which can consequently halt transmission while the IoT radio waits for a packet retry to be
received. This helps provide a higher success rate of receiving packets from other devices on the network, when co-located with an
interfering Wi-Fi radio.
3.3  General Purpose Input/Output (GPIO)
EFR32MG24 has up to 32 General Purpose Input/Output pins. Each GPIO pin can be individually configured as either an output or
input. More advanced configurations including open-drain, open-source, and glitch-filtering can be configured for each individual GPIO
pin. The GPIO pins can be overridden by peripheral connections, like SPI communication. Each peripheral connection can be routed to
several GPIO pins on the device. The input value of a GPIO pin can be routed through the Peripheral Reflex System to other peripher-
als. The GPIO subsystem supports asynchronous external pin interrupts.
All of the pins on ports A and port B are EM2 capable. These pins may be used by Low-Energy peripherals in EM2/3 and may also be
used as EM2/3 pin wake-ups. Pins on ports C and D are latched/retained in their current state when entering EM2 until EM2 exit upon
which internal peripherals could once again drive those pads.
A few GPIOs also have EM4 wake functionality. These pins are listed in the Alternate Function Table.
3.4  Keypad Scanner (KEYSCAN)
A low-energy keypad scanner (KEYSCAN) is included, which can scan up to a 6 x 8 matrix of keyboard switches. The KEYSCAN pe-
ripheral contains logic for debounce and settling time, allowing it to scan through the switch matrix autonomously in EM0 and EM1, and
interrupt the processor when a key press is detected. A wake-on-keypress feature is also supported, allowing for the detection of any
key press down to EM3.
## 3.5  Clocking
3.5.1  Clock Management Unit (CMU)
The Clock Management Unit controls oscillators and clocks in the EFR32MG24. Individual enabling and disabling of clocks to all periph-
eral modules is performed by the CMU. The CMU also controls enabling and configuration of the oscillators. A high degree of flexibility
allows software to optimize energy consumption in any specific application by minimizing power dissipation in unused peripherals and
oscillators.
3.5.2  Internal and External Oscillators
The EFR32MG24 supports two crystal oscillators and fully integrates four RC oscillators, listed below.
- A high frequency crystal oscillator (HFXO) with integrated load capacitors, tunable in small steps, provides a precise timing refer-
ence for the MCU. The HFXO provides excellent RF clocking performance using a 39.0 MHz crystal. The HFXO can also support an
external clock source such as a TCXO for applications that require an extremely accurate clock frequency over temperature.
- A 32.768 kHz crystal oscillator (LFXO) provides an accurate timing reference for low energy modes.
- An integrated high frequency RC oscillator (HFRCO) is available for the MCU system, when crystal accuracy is not required. The
HFRCO employs fast start-up at minimal energy consumption combined with a wide frequency range, from 1 MHz to 78 MHz.
- An integrated fast start-up RC oscillator (FSRCO) that runs at a fixed 20 MHz
- An integrated low frequency 32.768 kHz RC oscillator (LFRCO) for low power operation without an external crystal. Precision mode
enables periodic recalibration against the 39.0 MHz HFXO crystal to improve accuracy to +/- 500 ppm, suitable for BLE sleep inter-
val timing.
- An integrated ultra-low frequency 1 kHz RC oscillator (ULFRCO) is available to provide a timing reference at the lowest energy con-
sumption in low energy modes.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  11

3.6  Counters/Timers and PWM
3.6.1  Timer/Counter (TIMER)
TIMER peripherals keep track of timing, count events, generate PWM outputs and trigger timed actions in other peripherals through the
Peripheral Reflex System (PRS). The core of each TIMER is a 16-bit or 32-bit counter with up to 3 compare/capture channels. Each
channel is configurable in one of three modes. In capture mode, the counter state is stored in a buffer at a selected input event. In
compare mode, the channel output reflects the comparison of the counter to a programmed threshold value. In PWM mode, the TIMER
supports generation of pulse-width modulation (PWM) outputs of arbitrary waveforms defined by the sequence of values written to the
compare registers. In addition some timers offer dead-time insertion.
See 3.14 Configuration Summary for information on the feature set of each timer.
3.6.2  Low Energy Timer (LETIMER)
The unique LETIMER is a 24-bit timer that is available in energy mode EM0 Active, EM1 Sleep, EM2 Deep Sleep, and EM3 Stop. This
allows it to be used for timing and output generation when most of the device is powered down, allowing simple tasks to be performed
while the power consumption of the system is kept at an absolute minimum. The LETIMER can be used to output a variety of wave-
forms with minimal software intervention. The LETIMER is connected to the Peripheral Reflex System (PRS), and can be configured to
start counting on compare matches from other peripherals such as the Real Time Clock.
3.6.3  System Real Time Clock with Capture (SYSRTC)
The System Real Time Clock (SYSRTC) is a 32-bit counter providing timekeeping down to EM3. The SYSRTC can be clocked by any
of the on-board low-frequency oscillators, and it is capable of providing system wake-up at user defined intervals.
3.6.4  Back-Up Real Time Counter (BURTC)
The Back-Up Real Time Counter (BURTC) is a 32-bit counter providing timekeeping in all energy modes, including EM4. The BURTC
can be clocked by any of the on-board low-frequency oscillators, and it is capable of providing system wake-up at user-defined inter-
vals.
3.6.5  Watchdog Timer (WDOG)
The watchdog timer can act both as an independent watchdog or as a watchdog synchronous with the CPU clock. It has windowed
monitoring capabilities, and can generate a reset or different interrupts depending on the failure mode of the system. The watchdog can
also monitor autonomous systems driven by the Peripheral Reflex System (PRS).
3.7  Communications and Other Digital Peripherals
3.7.1  Universal Synchronous/Asynchronous Receiver/Transmitter (USART)
The Universal Synchronous/Asynchronous Receiver/Transmitter is a flexible serial I/O module. It supports full duplex asynchronous
UART communication with hardware flow control as well as RS-485, SPI, MicroWire and 3-wire. It can also interface with devices sup-
porting:
- ISO7816 SmartCards
- IrDA
## •
## I
## 2
## S
3.7.2  Enhanced Universal Synchronous/Asynchronous Receiver/Transmitter (EUSART)
The Enhanced Universal Synchronous/Asynchronous Receiver/Transmitter supports full duplex asynchronous UART communication
with hardware flow control, RS-485, and IrDA support. The EUSART also supports high-speed SPI. In EM0 and EM1 the EUSART pro-
vides a high-speed, buffered communication interface.
When routed to GPIO ports A or B, the EUSART0 may also be used in a low-energy mode and operate in EM2. A 32.768 kHz clock
source allows full duplex UART communication up to 9600 baud. EUSART0 can also act as a SPI secondary device in EM2 and EM3,
and wake the system when data is received from an external bus controller.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  12

3.7.3  Inter-Integrated Circuit Interface (I
## 2
## C)
## The I
## 2
C module provides an interface between the MCU and a serial I
## 2
C bus. It is capable of acting as a main or secondary interface
and supports multi-drop buses. Standard-mode, fast-mode and fast-mode plus speeds are supported, allowing transmission rates from
10 kbit/s up to 1 Mbit/s. Bus arbitration and timeouts are also available, allowing implementation of an SMBus-compliant system. The
interface provided to software by the I
## 2
C module allows precise timing control of the transmission process and highly automated trans-
fers. Automatic recognition of addresses is provided in active and low energy modes. Not all instances of I
## 2
C are available in all energy
modes.
3.7.4  Peripheral Reflex System (PRS)
The Peripheral Reflex System provides a communication network between different peripheral modules without software involvement.
Peripheral modules producing Reflex signals are called producers. The PRS routes Reflex signals from producers to consumer periph-
erals which in turn perform actions in response. Edge triggers and other functionality such as simple logic operations (AND, OR, NOT)
can be applied by the PRS to the signals. The PRS allows peripherals to act autonomously without waking the MCU core, saving pow-
er.
## 3.8  Secure Vault Features
A dedicated hardware secure engine containing its own CPU enables the Secure Vault functions. It isolates cryptographic functions and
data from the host Cortex-M33 core, and provides several additional security features. The EFR32MG24 family includes devices with
Secure Vault High and Secure Vault Mid capabilities, which are summarized in the table below.
## Table 3.1.  Secure Vault Features
FeatureSecure Vault MidSecure Vault High
True Random Number Generator (TRNG)YesYes
Secure Boot with Root of Trust and Secure
Loader (RTSL)
YesYes
Secure Debug with Lock/UnlockYesYes
DPA CountermeasuresYesYes
Anti-TamperYes
Secure AttestationYes
Secure Key ManagementYes
Symmetric Encryption• AES 128 / 192 / 256 bit
## • ECB, CTR, CBC, CFB, CCM, GCM,
CBC-MAC, and GMAC
- AES 128 / 192 / 256 bit
## • ECB, CTR, CBC, CFB, CCM, GCM,
CBC-MAC, and GMAC
- ChaCha20
Public Key Encryption - ECDSA / ECDH /
EdDSA
- p192 and p256
## •
Curve25519 (ECDH)
## 1
## •
Ed25519 (EdDSA)
## 1
- p192, p256, p384 and p521
- Curve25519 (ECDH)
- Ed25519 (EdDSA)
Key Derivation• ECJ-PAKE p192 and p256• ECJ-PAKE p192, p256, p384, and p521
## • PBKDF2
## • HKDF
Hashes• SHA-1
## • SHA-2/256
## • SHA-1
- SHA-2 256, 384, and 512
## • Poly1305
## Note:
1.These curves are supported in devices running SE v2.1.7 and higher

EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  13

3.8.1  Secure Boot with Root of Trust and Secure Loader (RTSL)
The Secure Boot with RTSL authenticates a chain of trusted firmware that begins from an immutable memory (ROM).
It prevents malware injection, prevents rollback, ensures that only authentic firmware is executed, and protects Over The Air updates.
For more information about this feature, see AN1218: Series 2 Secure Boot with RTSL.
## 3.8.2  Cryptographic Accelerator
The Cryptographic Accelerator is an autonomous hardware accelerator with Differential Power Analysis (DPA) countermeasures to pro-
tect keys.
It supports AES encryption and decryption with 128/192/256-bit keys, ChaCha20 encryption, and Elliptic Curve Cryptography (ECC) to
support public key operations, and hashes.
Supported block cipher modes of operation for AES include:
- ECB (Electronic Code Book)
- CTR (Counter Mode)
- CBC (Cipher Block Chaining)
- CFB (Cipher Feedback)
- GCM (Galois Counter Mode)
- CCM (Counter with CBC-MAC)
- CBC-MAC (Cipher Block Chaining Message Authentication Code)
- GMAC (Galois Message Authentication Code)
The Cryptographic Accelerator accelerates Elliptical Curve Cryptography and supports the NIST (National Institute of Standards and
Technology) recommended curves including P-192, P-256, P-384, and P-521 for ECDH (Elliptic Curve Diffie-Hellman) key derivation,
and ECDSA (Elliptic Curve Digital Signature Algorithm) sign and verify operations. Also supported is the non-NIST Curve25519 for
ECDH and Ed25519 for EdDSA (Edwards-curve Digital Signature Algorithm) sign and verify operations.
Secure Vault also supports ECJ-PAKE (Elliptic Curve variant of Password Authenticated Key Exchange by Juggling) and PBKDF2
(Password-Based Key Derivation Function 2).
Supported hashes include SHA-1, SHA-2/256/384/512 and Poly1305.
This implementation provides a fast and energy efficient solution to state of the art cryptographic needs.
## 3.8.3  True Random Number Generator
The True Random Number Generator module is a non-deterministic random number generator that harvests entropy from a thermal
energy source. It includes start-up health tests for the entropy source as required by NIST SP800-90B and AIS-31 as well as online
health tests required for NIST SP800-90C.
The TRNG is suitable for periodically generating entropy to seed an approved pseudo random number generator.
3.8.4  Secure Debug with Lock/Unlock
For obvious security reasons, it is critical for a product to have its debug interface locked before being released in the field.
Secure Vault also provides a secure debug unlock function that allows authenticated access based on public key cryptography. This
functionality is particularly useful for supporting failure analysis while maintaining confidentiality of IP and sensitive end-user data.
For more information about this feature, see AN1190: Series 2 Secure Debug.
3.8.5  DPA Countermeasures
The AES and ECC accelerators have Differential Power Analysis (DPA) countermeasures support. This makes it very expensive from a
time and effort standpoint to use DPA to recover secret keys.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  14

3.8.6  Secure Key Management with PUF
Key material in Secure Vault High products is protected by "key wrapping" with a standardized symmetric encryption mechanism. This
method has the advantage of protecting a virtually unlimited number of keys, limited only by the storage that is accessible by the Cor-
tex-M33, which includes off-chip storage as well. The symmetric key used for this wrapping and unwrapping must be highly secure
because it can expose all other key materials in the system. The Secure Vault Key Management system uses a Physically Unclonable
Function (PUF) to generate a persistent device-unique seed key on power up to dynamically generate this critical wrapping/unwrapping
key which is only visible to the AES encryption engine and is not retained when the device loses power.
3.8.7  Anti-Tamper
Secure Vault High devices provide internal tamper protection which monitors parameters such as voltage, temperature, and electro-
magnetic pulses as well as detecting tamper of the security sub-system itself. Additionally, 8 external configurable tamper pins support
external tamper sources, such as enclosure tamper switches.
For each tamper event, the user is able to select the severity of the tamper response ranging from an interrupt, to a reset, to destroying
the PUF reconstruction data which will make all protected key materials un-recoverable and effectively render the device inoperable.
The tamper system also has an internal resettable event counter with programmable trigger threshold and refresh periods to mitigate
false positive tamper events.
For more information about this feature, see AN1247: Anti-Tamper Protection Configuration and Use.
## 3.8.8  Secure Attestation
Secure Vault High products support Secure Attestation, which begins with a secure identity that is created during the Silicon Labs man-
ufacturing process. During device production, each device generates its own public/private keypair and securely stores the wrapped
private key into immutable OTP memory and this key never leaves the device. The corresponding public key is extracted from the de-
vice and inserted into a binary DER-encoded X.509 device certificate, which is signed into a Silicon Labs CA chain and then program-
med back into the chip into an immutable OTP memory.
The secure identity can be used to authenticate the chip at any time in the life of the product. The production certification chain can be
requested remotely from the product. This certification chain can be used to verify that the device was authentically produced by Silicon
Labs. The device unique public key is also bound to the device certificate in the certification chain. A challenge can be sent to the chip
at any point in time to be signed by the device private key. The public key in the device certificate can then be used to verify the chal-
lenge response, proving that the device has access to the securely-stored private key, which prevents counterfeit products or imperso-
nation attacks.
For more information about this feature, see AN1268: Authenticating Silicon Labs Devices Using Device Certificates.
## 3.9  Analog
3.9.1  Analog to Digital Converter (IADC)
The IADC is a hybrid architecture combining techniques from both SAR and Delta-Sigma style converters. Flexible controls allow fine-
tuned performance and speed to meet the needs of a wide variety of applications. Hardware oversampling reduces system-level noise
over multiple front-end samples. The IADC includes integrated voltage reference options. Inputs are selectable from a wide range of
sources, including pins configurable as either single-ended or differential.
The IADC supports three operational modes:
- Normal Mode (all devices): Flexible speed and performance, 12-16 bits output resolution
- 11.7 bits ENOB performance at 1 Msps (OSR = 2)
- 14.3 bits ENOB performance at 76.9 ksps (OSR = 32)
- High Speed Mode (select devices): Doubles output speed of Normal mode with similar performance, 12-16 bits output resolution
- 11.7 bits ENOB performance at 2 Msps (OSR = 2)
- 14.3 bits ENOB performance at 153.8 ksps (OSR = 32)
- High Accuracy Mode (select devices): Optimized for low-rate, high performance applications, with 20 bit output resolution
- 16 bits ENOB performance at 3.8 ksps (OSR = 256)
- 15 bits ENOB performance at 15.3 ksps (OSR = 64)
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  15

3.9.2  Analog Comparator (ACMP)
The Analog Comparator is used to compare the voltage of two analog inputs, with a digital output indicating which input voltage is high-
er. Inputs are selected from among internal references and external pins. The tradeoff between response time and current consumption
is configurable by software. Two 6-bit reference dividers allow for a wide range of internally-programmable reference sources. The
ACMP can also be used to monitor the supply voltage. An interrupt can be generated when the supply falls below or rises above the
programmable threshold.
3.9.3  Digital to Analog Converter (VDAC)
The Digital to Analog Converter (VDAC) can convert a digital value to an analog output voltage. The VDAC is a fully differential, 500
ksps, 12-bit converter. The VDAC may be used for a number of different applications such as sensor interfaces or sound output. The
VDAC can generate high-resolution analog signals while the MCU is operating at low frequencies and with low total power consump-
tion. Using DMA and a timer, the VDAC can be used to generate waveforms without any CPU intervention. The VDAC is available in all
energy modes down to and including EM3.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  16

## 3.10  Power
The EFR32MG24 has an Energy Management Unit (EMU) and efficient integrated regulators to generate internal supply voltages. Only
a single external supply voltage is required, from which all internal voltages are created. An optional integrated DC-DC buck regulator
can be utilized to further reduce the current consumption. The DC-DC regulator requires one external inductor and one external capaci-
tor.
The EFR32MG24 device family includes support for internal supply voltage scaling, as well as two different power domains groups for
peripherals. These enhancements allow for further supply current reductions and lower overall power consumption.
3.10.1  Energy Management Unit (EMU)
The Energy Management Unit manages transitions of energy modes in the device. Each energy mode defines which peripherals and
features are available and the amount of current the device consumes. The EMU can also be used to implement system-wide voltage
scaling and turn off the power to unused RAM blocks to optimize the energy consumption in the target application. The DC-DC regula-
tor operation is tightly integrated with the EMU.
## 3.10.2  Voltage Scaling
The EFR32MG24 supports supply voltage scaling for the LDO powering DECOUPLE, with independent selections for EM0 / EM1 and
EM2 / EM3. Voltage scaling helps to optimize the energy efficiency of the system by operating at lower voltages when possible. The
EM0 / EM1 voltage scaling level defaults to VSCALE2, which allows the core to operate in active mode at full speed. The intermediate
level, VSCALE1, allows operation in EM0 and EM1 at up to 40 MHz. The lowest level, VSCALE0, can be used to conserve power fur-
ther in EM2 and EM3. The EMU will automatically switch the target voltage scaling level when transitioning between energy modes.
3.10.3  DC-DC Converter
The DC-DC buck converter covers a wide range of load currents, providing high efficiency in energy modes EM0, EM1, EM2 and EM3.
RF noise mitigation allows operation of the DC-DC converter without significantly degrading sensitivity of radio components. An on-chip
supply-monitor signals when the supply voltage is low to allow bypass of the regulator via programmable software interrupt. It employs
soft switching at boot and DCDC regulating-to-bypass transitions to limit the max supply slew-rate and mitigate inrush current.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  17

## 3.10.4  Power Domains
Peripherals may exist on one of several independent power domains which are powered down to minimize supply current when not in
use. Power domains are managed automatically by the EMU.
The lowest-energy power domain is the "high-voltage" power domain (PDHV), which supports extremely low-energy infrastructure and
peripherals. Circuits powered from PDHV are always on and available in all energy modes down to EM4.
The next power domain is the low power domain (PD0), which is further divided to power subsets of peripherals. All PD0 power do-
mains are shut down in EM4. Circuits powered from PD0 power domains may be available in EM0, EM1, EM2, and EM3.
Low power domain A (PD0A) is the base power domain for EM2 and EM3 and will always remain on in EM0-EM3. It powers the most
commonly-used EM2 and EM3-capable peripherals and infrastructure required to operate in EM2 and EM3. Auxiliary PD0 power do-
mains (PD0B, PD0C, PD0D, PD0E) power additional EM2 and EM3-capable peripherals on demand. If any peripherals on one of the
auxiliary power domains is enabled, that power domain will be active in EM2 and EM3. Otherwise, the auxiliary PD0 power domains will
be shut down to reduce current.
Note: Power domain PD0E is also turned on when peripherals on PD0B, PD0C, or PD0D are used.

The active power domain (PD1) powers the rest of the device circuitry, including the CPU core and EM0 / EM1 peripherals. PD1 is
always powered on in EM0 and EM1. PD1 is always shut down in EM2, EM3, and EM4.
Table 3.2 Peripheral Power Subdomains on page 18 shows the peripherals on the PDHV and PD0x domains. Any peripheral not lis-
ted is on PD1.
## Table 3.2.  Peripheral Power Subdomains
Always On in EM2/EM3Selectively On in EM2/3
## PDHV
## 1
## PD0A
## PD0B
## 2
## PD0C
## 2
## PD0D
## 2
## PD0E
LFRCO (Non-preci-
sion Mode)
SYSRTCLETIMER0LFRCO (Precision
## Calibration Mode)
## DEBUGGPIO
## LFXOFSRCOIADC0HFRCOEM23WDOG0/1KEYSCAN
## BURTCPCNT0HFXOEUSART0PRS
## BURAMACMP0/1I2C0
## ULFRCOVDAC0/1
## Note:
1.Peripherals on PDHV are also available in EM4.
2.If any of PD0B, PD0C, or PD0D are enabled, PD0E will also be automatically enabled.

3.11  Reset Management Unit (RMU)
The RMU is responsible for handling reset of the EFR32MG24. A wide range of reset sources are available, including several power
supply monitors, pin reset, software controlled reset, core lockup reset, and watchdog reset.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  18

3.12  Core, Memory, and Accelerators
## 3.12.1  Processor Core
The ARM Cortex-M processor includes a 32-bit RISC processor integrating the following features and tasks in the system:
- ARM Cortex-M33 RISC processor achieving 1.50 Dhrystone MIPS/MHz
- ARM TrustZone security technology
- Embedded Trace Macrocell (ETM) for real-time trace and debug
- Up to 1536 KB flash program memory
- Up to 256 KB RAM data memory
- Configuration and event handling of all modules
- 2-pin Serial-Wire debug interface
3.12.2  Memory System Controller (MSC)
The Memory System Controller (MSC) is the program memory unit of the microcontroller. The flash memory is readable and writable
from both the Cortex-M33 and LDMA. In addition to the main flash array where Program code is normally written the MSC also provides
an Information block where additional information such as special user information or flash-lock bits are stored. There is also a read-
only page in the information block containing system and device calibration data. Read and write operations are supported in energy
modes EM0 Active and EM1 Sleep.
3.12.3  Linked Direct Memory Access Controller (LDMA)
The Linked Direct Memory Access (LDMA) controller allows the system to perform memory operations independently of software. This
reduces both energy consumption and software workload. The LDMA allows operations to be linked together and staged, enabling so-
phisticated operations to be implemented.
3.12.4  Matrix Vector Processor (MVP)
The Matrix Vector Processor (MVP) is designed to offload the major computationally intensive floating point operations, particularly ma-
trixed complex floating point multiplications and additions. The MVP supports the acceleration of the key Angle-of-Arrival (AoA) MUSIC
(MUltiple SIgnal Classification) algorithm computations, as well as other heavily floating-point computational problems such as Machine
Learning (ML) or linear algebra.
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  19

## 3.13  Memory Map
The EFR32MG24 memory map is shown in the figures below. RAM and flash sizes are for the largest memory configuration.
## M33 Peripherals
## 0xe00fffff
## 0xe0000000
## 0xfffffffe
## 0xe0100000
## 0xdfffffff
## 0xb0005000
FRCRAM (non-secure)
## 0xb0004fff
## 0xb0004000
SEQRAM (non-secure)
## 0xb0003fff
## 0xb0000000
## 0xafffffff
## 0xa0005000
FRCRAM (secure)
## 0xa0004fff
## 0xa0004000
SEQRAM (secure)
## 0xa0003fff
## 0xa0000000
## 0x9fffffff
## 0x60000000
## Peripherals (non-secure)
## 0x5fffffff
## 0x50000000
## Peripherals (secure)
## 0x4fffffff
## 0x40000000
## RAM (DMEM)
## 0x2003ffff
## 0x20000000
## Flash
## 0x1fffffff
## 0x08000000
## 0x3fffffff
## 0x20040000
0x07FFFFFF
## 0x00000000
## FLASH
## 0x08000000
## 0x08180000
## FLASH_USERDATA
## 0x0fe00000
## 0x0fe00400
## FLASH_DEVINFO
## 0x0fe08000
## 0x0fe08400
## FLASH_CHIPCONFIG
## 0x0fe08a00
Instrumentation Trace Macrocell (ITM)
## 0xe0000000
Data Watchpoint and Trace (DWT)
## 0xe0001000
Flash Patch and Breakpoint (FPB)
## 0xe0002000
## 0xe0003000
## System Control Space
## 0xe000e000
## 0xe000f000
Trace Port Interface Unit (TPIU)
## 0xe0040000
Embedded Trace Macrocell (ETM)
## 0xe0041000
## 0xe0042000
M33 ROM Table
## 0xe00ff000
## 0xe0100000
Figure 3.2.  EFR32MG24 Memory Map — Core Peripherals and Code Space
EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  20

## 3.14  Configuration Summary
The features of the EFR32MG24 are a subset of the feature set described in the device reference manual. The table below describes
device specific implementation of the features. Remaining modules support full configuration. Refer to the Energy Modes table in the
Reference Manual EMU Chapter for a more comprehensive list of energy mode support for all device peripherals.
## Table 3.3.  Configuration Summary
ModuleLowest Energy ModeConfiguration
I2C0EM1 - Full functionality
## EM2/3
## 1
- Functionality limited to receive address recog-
nition
I2C1EM1 - Full functionality
## LETIMER0
## EM2/3
## 1
## 24-bit, 2-channels
TIMER0EM132-bit, 3-channels, +DTI
TIMER1EM132-bit, 3-channels, +DTI
TIMER2EM116-bit, 3-channels, +DTI
TIMER3EM116-bit, 3-channels, +DTI
TIMER4EM116-bit, 3-channels, +DTI
EUSART0EM1 - Full high-speed operation, all modes
## EM2
## 1
- Low-energy UART operation, 9600 Baud
## EM2/3
## 1
- Low-energy SPI secondary receiver
UART, SPI, IrDA, DALI
EUSART1EM1UART, SPI, IrDA, DALI
USART0EM1UART, SPI, IrDA, I2S, SmartCard
## Note:
1.EM2 and EM3 operation is only supported for digital peripheral I/O on Port A and Port B. All GPIO ports support digital peripheral
operation in EM0 and EM1.

EFR32MG24 Wireless SoC Family Data Sheet
## System Overview
silabs.com | Building a more connected world.Rev. 1.2  |  21

## 4.  Electrical Specifications
## 4.1  Electrical Characteristics
All electrical parameters in all tables are specified under the following conditions, unless stated otherwise:
- Typical values are based on T
## A
=25 °C and all supplies at 3.0 V, by production test and/or technology characterization.
- Radio performance numbers are measured in conducted mode, based on Silicon Laboratories reference designs using output pow-
er-specific external RF impedance-matching networks for interfacing to a 50 Ω antenna.
- Minimum and maximum values represent the worst conditions across supply voltage, process variation, and operating temperature,
unless stated otherwise.
Due to on-chip circuitry (e.g., diodes), some EFR32MG24 power supply pins have a dependent relationship with one or more other
power supply pins. These internal relationships between the external voltages applied to the various EFR32MG24 supply pins are de-
fined below. Exceeding the below constraints can result in damage to the device and/or increased current draw.
- VREGVDD and DVDD
- In systems using the DCDC converter, DVDD (the buck converter output) should not be driven externally and VREGVDD (the
buck converter input) must be greater than DVDD (VREGVDD ≥ DVDD)
- In systems not using the DCDC converter, DVDD must be shorted to VREGVDD on the PCB (VREGVDD = DVDD)
- AVDD, IOVDD: No dependency with each other or any other supply pin. Additional leakage may occur if DVDD remains unpowered
with power applied to these supplies.
## • DVDD ≥ DECOUPLE
## • PAVDD ≥ RFVDD
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  22

## 4.2  Absolute Maximum Ratings
Stresses beyond those listed below may cause permanent damage to the device. This is a stress rating only and functional operation of
the devices at those or any other conditions beyond those indicated in the operation listings of this specification is not implied. Exposure
to maximum rating conditions for extended periods may affect device reliability. For more information on the available quality and relia-
bility data, see the Quality and Reliability Monitor Report at http://www.silabs.com/support/quality/pages/default.aspx.
## Table 4.1.  Absolute Maximum Ratings
ParameterSymbolTest ConditionMinTypMaxUnit
Storage temperature rangeT
## STG
## -50—+150°C
Voltage on any supply pin
## 1
## V
## DDMAX
## -0.3—3.8V
Junction temperatureT
## JMAX
-I grade——+125°C
Voltage ramp rate on any
supply pin
## V
## DDRAMPMAX
——1.0V / μs
Voltage on HFXO pinsV
## HFXOPIN
## -0.3—1.2V
DC voltage on any GPIO pinV
## DIGPIN
## -0.3—V
## IOVDD
## +
## 0.3
## V
DC voltage on RESETn pin
## 2
## V
RESETn
## -0.3—3.8V
DC voltage on RF pin
## RF2G4_IO
## V
## MAX2G4
## -0.3—1.2V
Total current into VDD power
lines
## I
## VDDMAX
Source——200mA
Total current into VSS
ground lines
## I
## VSSMAX
Sink——200mA
Current per I/O pinI
## IOMAX
Sink——50mA
Source——50mA
Current for all I/O pinsI
## IOALLMAX
Sink——200mA
Source——200mA
## Note:
1.The maximum supply voltage on VREGVDD is limited under certain conditions when using the DC-DC. See the DC-DC specifica-
tions for more details.
2.The RESETn pin has a pull-up device to the DVDD supply. For minimum leakage, RESETn should not exceed the voltage at
## DVDD.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  23

## 4.3  General Operating Conditions
## Table 4.2.  General Operating Conditions
ParameterSymbolTest ConditionMinTypMaxUnit
Operating ambient tempera-
ture range
## T
## A
-I temperature grade
## 1
## -40—+125° C
DVDD supply voltageV
## DVDD
## EM0/11.713.03.8V
## EM2/3/4
## 2
## 1.713.03.8V
AVDD supply voltageV
## AVDD
## AVDDBODEN=0
## 3
## 1.713.03.8V
IOVDDx operating supply
voltage (All IOVDD pins)
## V
IOVDDx
IOVDDxBODEN=0
## 3
## 1.713.03.8V
RFVDD operating supply
voltage
## V
## RFVDD
## 1.713.0V
## PAVDD
## V
VREGVDD operating supply
voltage
## V
## VREGVDD
DC-DC in regulation
## 4
## 2.23.03.8V
DC-DC in bypass 60 mA load1.83.03.8V
DC-DC in bypass 120 mA load1.83.03.8V
DC-DC not in use. DVDD exter-
nally shorted to VREGVDD
## 1.713.03.8V
PAVDD operating supply
voltage
## V
## PAVDD
## 1.713.03.8V
DECOUPLE output capaci-
tor
## 5
## C
## DECOUPLE
1.0 μF ± 10% X8L capacitor used
for performance characterization.
1.0—2.75μF
HCLK and SYSCLK frequen-
cy
f
## HCLK
VSCALE2, MODE = WS1——78MHz
VSCALE2, MODE = WS0——40MHz
VSCALE1, MODE = WS1——40MHz
VSCALE1, MODE = WS0——20MHz
PCLK frequencyf
## PCLK
VSCALE2 or VSCALE1——40MHz
EM01 Group A clock fre-
quency
f
## EM01GRPACLK
VSCALE2——78MHz
VSCALE1——40MHz
EM01 Group C clock fre-
quency
f
## EM01GRPCCLK
VSCALE2——78MHz
VSCALE1——40MHz
Radio HCLK frequency
## 6
f
## RHCLK
VSCALE2 or VSCALE1—39.0—MHz
## External Clock Inputf
## CLKIN
VSCALE2 or VSCALE1, IOVDD ≥
## 2.7 V
——40MHz
DPLL Reference Clockf
## DPLLREFCLK
VSCALE2 or VSCALE1——40MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  24

ParameterSymbolTest ConditionMinTypMaxUnit
## Note:
1.The device may operate continuously at the maximum allowable ambient T
## A
rating as long as the absolute maximum T
## JMAX
is not
exceeded. For an application with significant power dissipation, the allowable T
## A
may be lower than the maximum T
## A
rating. T
## A
## =
## T
## JMAX
## - (THETA
## JA
x PowerDissipation). Refer to the Absolute Maximum Ratings table and the Thermal Characteristics table for
## T
## JMAX
and THETA
## JA
## .
2.The DVDD supply is monitored by the DVDD BOD in EM0/1 and the LE DVDD BOD in EM2/3/4.
3.The AVDD and IOVDD enable bits are in the EMU_BOD3SENSE register. These BODs are disabled on reset.
4.The maximum supply voltage on VREGVDD is limited under certain conditions when using the DC-DC. See the DC-DC specifica-
tions for more details.
5.Murata GCM21BL81C105KA58L used for performance characterization. Actual capacitor values can be significantly de-rated
from their specified nominal value by the rated tolerance, as well as the application's AC voltage, DC bias, and temperature. The
minimum capacitance counting all error sources should be no less than 0.6 μF.
6.The recommended radio crystal frequency for the 2.4GHz radio is 39 MHz. The minimum and maximum RHCLK frequency in this
table represent the design timing limits, which are much wider than the typical crystal tolerance.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  25

4.4  DC-DC Converter
Test conditions: L
## DCDC
= 2.2 μH (Murata DFE2HCAH2R2MJ0), C
## DCDC
= 4.7 μF (TDK CGA5L3X8R1C475K160AB), V
## VREGVDD
## = 3.0 V,
## V
## OUT
= 1.8 V, IPKVAL in EM0/1 modes is set to 150 mA, and in EM2/3 modes is set to 90 mA, unless otherwise indicated.
Table 4.3.  DC-DC Converter
ParameterSymbolTest ConditionMinTypMaxUnit
Input voltage range at
VREGVDD pin
## V
## VREGVDD
DCDC in regulation, I
## LOAD
## = I
## LOAD
## MAX
## 1
, EM0/EM1 mode
## 2.2—3.8V
DCDC in regulation, I
## LOAD
## = 5
mA, EM0/EM1 or EM2/EM3 mode
## 1.8—3.8V
## Bypass Mode, I
## LOAD
≤ 60 mA1.8—3.8V
## Bypass Mode, I
## LOAD
≤ 120 mA1.9—3.8V
Regulated output voltageV
## OUT
## —1.8—V
Regulation DC accuracyACC
## DC
## V
## VREGVDD
≥ 2.2 V, Steady state in
EM0/EM1 mode or EM2/EM3
mode
## -2.5—4.0%
Regulation total accuracyACC
## TOT
All error sources (including DC er-
rors, overshoot, undershoot)
## -5—7%
Steady-state output rippleV
## R
## I
## LOAD
= 20 mA in EM0/EM1 mode—12—mVpp
DC line regulationV
## REG
## I
## LOAD
## = I
## LOAD
MAX in EM0/EM1
mode, V
## VREGVDD
## ≥ 2.2 V
—-2.6—mV/V
EfficiencyEFFLoad current between 100 μA and
60 mA in EM0/EM1 mode
## —90—%
Load current between 10 μA and
5 mA in EM2/EM3 mode
## —89—%
DC load regulationI
## REG
Load current between 100 μA and
## I
## LOAD
MAX in EM0/EM1 mode
—-0.08—mV/mA
Output load currentI
## LOAD
EM0/EM1 mode, DCDC in regula-
tion, DCDC_EM01CTRL0.IPKVAL
= 9, Radio not transmitting
——60mA
EM0/EM1 mode, DCDC in regula-
tion, Radio in receive mode
——36mA
EM0/EM1 mode, DCDC in regula-
tion, Radio transmitting
## 1
——120mA
EM2/EM3 mode, DCDC in regula-
tion
——5mA
Bypass mode, 1.8 V ≤ V
## VREGVDD
## ≤ 3.8 V
——60mA
Bypass mode, 1.9 V ≤ V
## VREGVDD
## ≤ 3.8 V
——120mA
Nominal output capacitorC
## DCDC
4.7 μF ± 10% X7R capacitor used
for performance characterization
## 2
—4.710μF
Nominal inductorL
## DCDC
± 20% tolerance—2.2—μH
Nominal input capacitorC
## IN
## C
## DCDC
——μF
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  26

ParameterSymbolTest ConditionMinTypMaxUnit
Resistance in bypass modeR
## BYP
Bypass switch from VREGVDD to
## DVDD, V
## VREGVDD
## = 1.8 V
## —0.450.69Ω
Powertrain PFET switch from
VREGVDD to VREGSW,
## V
## VREGVDD
## = 1.8 V
## —0.60.9Ω
Supply monitor threshold
programming range
## V
## CMP_RNG
Programmable in 0.1 V steps2—2.3V
Supply monitor threshold ac-
curacy
## V
## CMP_ACC
Supply falling edge trip point-5—5%
Supply monitor threshold
hysteresis
## V
## CMP_HYST
Positive hysteresis on the supply
rising edge referred to the falling
edge trip point
## —4—%
Supply monitor response
time
t
## CMP_DELAY
Supply falling edge at -100 mV /
μs
## —0.6—μs
## Note:
1.During radio transmit operations, the RAIL library will place the DCDC into a mode that increases the maximum load current, to
support higher TX output power supplied from the DCDC converter.
2.Actual capacitor values can be significantly de-rated from their specified nominal value by the rated tolerance, as well as the ap-
plication's AC voltage, DC bias, and temperature. The minimum capacitance counting all error sources should be no less than 3.6
μF.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  27

## 4.5  Thermal Characteristics
## Table 4.4.  Thermal Characteristics
PackageBoardParameterSymbolTest ConditionValueUnit
## 40QFN
## (5x5mm)
JEDEC - High
## Thermal Cond.
## (2s2p)
## 1
## Thermal Resistance, Junction
to Ambient
## Θ
## JA
Still Air29.2°C/W
## Thermal Resistance, Junction
to Board
## Θ
## JB
## 15.2°C/W
## Thermal Resistance, Junction
to Top Center
## Ѱ
## JT
## 0.3°C/W
## Thermal Resistance, Junction
to Board
## Ѱ
## JB
## 11.2°C/W
No BoardThermal Resistance, Junction
to Case
## Θ
## JC
Temperature controlled heat sink on
top of package, all other sides of
package insulated to prevent heat
flow.
## 24.6°C/W
## 48QFN
## (6x6mm)
JEDEC - High
## Thermal Cond.
## (2s2p)
## 1
## Thermal Resistance, Junction
to Ambient
## Θ
## JA
Still Air27.7°C/W
## Thermal Resistance, Junction
to Board
## Θ
## JB
## 14.6°C/W
## Thermal Resistance, Junction
to Top Center
## Ѱ
## JT
## 0.69°C/W
## Thermal Resistance, Junction
to Board
## Ѱ
## JB
## 11.85°C/W
No BoardThermal Resistance, Junction
to Case
## Θ
## JC
Temperature controlled heat sink on
top of package, all other sides of
package insulated to prevent heat
flow.
## 23.0°C/W
## Note:
1.Based on 4 layer PCB with dimension 3" x 4.5", PCB Thickness of 1.6 mm, per JEDEC. PCB Center Land with 9 Via to top inter-
nal plane of PCB.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  28

## 4.6  Current Consumption
4.6.1  MCU current consumption using DC-DC at 3.0 V input
Unless otherwise indicated, typical conditions are: VREGVDD = AVDD = IOVDD = 3.0 V. DVDD = RFVDD = PAVDD = 1.8 V from DC-
DC. Voltage scaling level = VSCALE1. T
## A
= 25 °C. Minimum and maximum values in this table represent the worst conditions across
process variation at T
## A
## = 25 °C.
Table 4.5.  MCU current consumption using DC-DC at 3.0 V input
ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in EM0
mode with all peripherals dis-
abled
## I
## ACTIVE
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running Prime from flash,
## VSCALE2
—33.3—μA/MHz
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running while loop from flash,
## VSCALE2
—32.8—μA/MHz
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running CoreMark loop from flash,
## VSCALE2
—49.1—μA/MHz
39 MHz crystal, CPU running
Prime from flash
—33.9—μA/MHz
39 MHz crystal, CPU running
while loop from flash
—33.4—μA/MHz
39 MHz crystal, CPU running
CoreMark loop from flash
—49.4—μA/MHz
38 MHz HFRCO, CPU running
while loop from flash
—28.1—μA/MHz
26 MHz HFRCO, CPU running
while loop from flash
—31.0—μA/MHz
16 MHz HFRCO, CPU running
while loop from flash
—37.6—μA/MHz
1 MHz HFRCO, CPU running
while loop from flash
—281.8—μA/MHz
Current consumption in EM1
mode with all peripherals dis-
abled
## I
## EM1
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal,
## VSCALE2
—22.6—μA/MHz
39 MHz crystal—24.4—μA/MHz
38 MHz HFRCO—19.0—μA/MHz
26 MHz HFRCO—22.0—μA/MHz
16 MHz HFRCO—28.5—μA/MHz
1 MHz HFRCO—272.1—μA/MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  29

ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in EM2
mode, VSCALE0
## I
## EM2_VS
256 kB RAM and full Radio RAM
retention, RTC running from
## LFXO
## 1
—2.9—μA
256 kB RAM and full Radio RAM
retention, RTC running from
## LFRCO
## 1
—2.9—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## LFXO
## 1
—1.3—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## LFRCO
## 1
—1.3—μA
16 kB RAM and full Radio RAM
retention, RTC running from
LFRCO in precision mode
## 1
—1.9—μA
Current consumption in EM3
mode, VSCALE0
## I
## EM3_VS
256 kB RAM and full Radio RAM
retention, RTC running from
## ULFRCO
## 1
—2.7—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## ULFRCO
## 1
—1.1—μA
Change in current consump-
tion if CPU cached unre-
tained in EM2 or EM3
## I
## EM23_CPUCACHE
—-0.06—μA
Change in current consump-
tion if EM0/1 peripheral
states unretained in EM2 or
## EM3
## I
## EM23_STATERET
—-0.01—μA
Change in current consump-
tion for retained RAM bank in
EM2 or EM3
## I
## EM23_RAM
Per 16 kB RAM bank—0.11—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0B is enabled
## 2
## I
## PD0B_VS
—0.93—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0C is enabled
## 2
## I
## PD0C_VS
—0.26—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0D is enabled
## 2
## I
## PD0D_VS
—1.1—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0E is enabled
## 2
## I
## PD0E_VS
—0.09—μA
Current consumption in EM4
mode
## I
## EM4
No BURTC, no LF oscillator—0.25—μA
BURTC with LFXO—0.64—μA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  30

ParameterSymbolTest ConditionMinTypMaxUnit
## Note:
1.CPU cache retained, EM0/1 peripheral states retained
2.Extra current consumed by power domain. Does not include current associated with the enabled peripherals. See 3.10.4 Power
Domains for a list of the peripherals in each power domain. Note that if the PD0B, PD0C, or PD0D domains are enabled, PD0E
will also automatically be enabled.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  31

4.6.2  Radio current consumption at 3.0V using DCDC
RF current consumption measured with MCU in EM1, HCLK = 39.0 MHz, and all MCU peripherals disabled. Unless otherwise indica-
ted, typical conditions are: VREGVDD = IOVDD = 3.0 V. AVDD = DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC. T
## A
## = 25 °C.
Minimum and maximum values in this table represent the worst conditions across process variation at T
## A
## = 25 °C.
Table 4.6.  Radio current consumption at 3.0V using DCDC
ParameterSymbolTest ConditionMinTypMaxUnit
System current consumption
in receive mode, active pack-
et reception
## I
## RX_ACTIVE
125 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—4.6—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—4.9—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—5.2—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—4.7—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—5—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—5.2—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—4.4—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—4.7—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—4.9—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—5.1—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—5.4—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—5.6—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE1, EM1P (Radio
clocks only)
—5.1—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE1
—5.4—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE2
—5.7—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  32

ParameterSymbolTest ConditionMinTypMaxUnit
System current consumption
in receive mode, listening for
packet
## I
## RX_LISTEN
125 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—4.7—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—4.9—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—5.2—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—4.7—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—5—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—5.2—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—4.3—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—4.6—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—4.9—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—5.1—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—5.4—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—5.7—mA
802.15.4, f = 2.4 GHz, VSCALE1,
EM1P (Radio clocks only)
—5—mA
802.15.4, f = 2.4 GHz, VSCALE1—5.3—mA
802.15.4, f = 2.4 GHz, VSCALE2—5.6—mA
System current consumption
in transmit mode
## I
## TX
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power, VSCALE1
—5—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power, VSCALE1
—19.1—mA
f = 2.4 GHz, CW, 20 dBm PA,
19.5 dBm output power,
## VSCALE1, VREGVDD = PAVDD
## = 3.3 V
—156.8—mA
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power, VSCALE2
—5.2—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power, VSCALE2
—19.2—mA
f = 2.4 GHz, CW, 20 dBm PA,
19.5 dBm output power,
## VSCALE2, VREGVDD = PAVDD
## = 3.3 V
—157.2—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  33

4.6.3  MCU current consumption at 3.0 V
Unless otherwise indicated, typical conditions are: AVDD = DVDD = RFVDD = PAVDD = VREGVDD = 3.0 V. DC-DC not used. Voltage
scaling level = VSCALE1. T
## A
= 25 °C. Minimum and maximum values in this table represent the worst conditions across process varia-
tion at T
## A
## = 25 °C.
Table 4.7.  MCU current consumption at 3.0 V
ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in EM0
mode with all peripherals dis-
abled
## I
## ACTIVE
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running Prime from flash,
## VSCALE2
—47.3—μA/MHz
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running while loop from flash,
## VSCALE2
—46.1—μA/MHz
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running CoreMark loop from flash,
## VSCALE2
—69.5—μA/MHz
39 MHz crystal, CPU running
Prime from flash
—48.4—μA/MHz
39 MHz crystal, CPU running
while loop from flash
—47.1—μA/MHz
39 MHz crystal, CPU running
CoreMark loop from flash
—69.6—μA/MHz
38 MHz HFRCO, CPU running
while loop from flash
—39.462μA/MHz
26 MHz HFRCO, CPU running
while loop from flash
—43.6—μA/MHz
16 MHz HFRCO, CPU running
while loop from flash
—52.7—μA/MHz
1 MHz HFRCO, CPU running
while loop from flash
—392.41170μA/MHz
Current consumption in EM1
mode with all peripherals dis-
abled
## I
## EM1
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal,
## VSCALE2
—32.2—μA/MHz
39 MHz crystal—34.5—μA/MHz
38 MHz HFRCO—26.849μA/MHz
26 MHz HFRCO—30.9—μA/MHz
16 MHz HFRCO—40.0—μA/MHz
1 MHz HFRCO—380.01160μA/MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  34

ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in EM2
mode, VSCALE0
## I
## EM2_VS
256 kB RAM and full Radio RAM
retention, RTC running from
## LFXO
## 1
—4.2—μA
256 kB RAM and full Radio RAM
retention, RTC running from
## LFRCO
## 1
—4.29.2μA
16 kB RAM and full Radio RAM
retention, RTC running from
## LFXO
## 1
—1.8—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## LFRCO
## 1
—1.9—μA
16 kB RAM and full Radio RAM
retention, RTC running from
LFRCO in precision mode
## 1
—2.8—μA
Current consumption in EM3
mode, VSCALE0
## I
## EM3_VS
256 kB RAM and full Radio RAM
retention, RTC running from
## ULFRCO
## 1
—3.9—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## ULFRCO
## 1
—1.52.5μA
Change in current consump-
tion if CPU cached unre-
tained in EM2 or EM3
## I
## EM23_CPUCACHE
—-0.07—μA
Change in current consump-
tion if EM0/1 peripheral
states unretained in EM2 or
## EM3
## I
## EM23_STATERET
—-0.01—μA
Change in current consump-
tion for retained RAM bank in
EM2 or EM3
## I
## EM23_RAM
Per 16 kB RAM bank—0.16—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0B is enabled
## 2
## I
## PD0B_VS
—1.4—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0C is enabled
## 2
## I
## PD0C_VS
—0.39—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0D is enabled
## 2
## I
## PD0D_VS
—1.6—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0E is enabled
## 2
## I
## PD0E_VS
—0.11—μA
Current consumption in EM4
mode
## I
## EM4
No BURTC, no LF oscillator—0.260.65μA
BURTC with LFXO—0.64—μA
Current consumption during
reset
## I
## RST
Hard pin reset held—457—μA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  35

ParameterSymbolTest ConditionMinTypMaxUnit
## Note:
1.CPU cache retained, EM0/1 peripheral states retained
2.Extra current consumed by power domain. Does not include current associated with the enabled peripherals. See 3.10.4 Power
Domains for a list of the peripherals in each power domain. Note that if the PD0B, PD0C, or PD0D domains are enabled, PD0E
will also automatically be enabled.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  36

4.6.4  Radio current consumption at 3.0V
RF current consumption measured with MCU in EM1, HCLK = 39.0 MHz, and all MCU peripherals disabled. Unless otherwise indica-
ted, typical conditions are: AVDD = DVDD = IOVDD = RFVDD = PAVDD = 3.0 V. DCDC disabled. T
## A
= 25 °C. Minimum and maximum
values in this table represent the worst conditions across process variation at T
## A
## = 25 °C.
Table 4.8.  Radio current consumption at 3.0V
ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in re-
ceive mode, active packet
reception
## I
## RX_ACTIVE
125 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.1—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.5—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.9—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.2—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.6—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—8—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—6.7—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.1—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.4—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.7—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—8.1—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—8.6—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE1, EM1P (Radio
clocks only)
—7.8—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE1
—8.2—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE2
—8.6—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  37

ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in re-
ceive mode, listening for
packet
## I
## RX_LISTEN
125 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.1—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.5—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.9—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.1—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.5—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.9—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—6.6—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.4—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.7—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—8.2—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—8.6—mA
802.15.4, f = 2.4 GHz, VSCALE1,
EM1P (Radio clocks only)
—7.6—mA
802.15.4, f = 2.4 GHz, VSCALE1—8—mA
802.15.4, f = 2.4 GHz, VSCALE2—8.5—mA
Current consumption in
transmit mode
## I
## TX
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power, VSCALE2
—8—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power, VSCALE2
—28.7—mA
f = 2.4 GHz, CW, 20 dBm PA,
19.5 dBm output power,
## VSCALE2, PAVDD = 3.3 V
—159.3—mA
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power, VSCALE1
—7.8—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power, VSCALE1
—28.4—mA
f = 2.4 GHz, CW, 20 dBm PA,
19.5 dBm output power,
## VSCALE1, PAVDD = 3.3 V
—160—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  38

4.6.5  MCU current consumption at 1.8 V
Unless otherwise indicated, typical conditions are: AVDD = DVDD = RFVDD = PAVDD = VREGVDD = 1.8 V. DC-DC not used. Voltage
scaling level = VSCALE1. T
## A
= 25 °C. Minimum and maximum values in this table represent the worst conditions across process varia-
tion at T
## A
## = 25 °C.
Table 4.9.  MCU current consumption at 1.8 V
ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in EM0
mode with all peripherals dis-
abled
## I
## ACTIVE
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running Prime from flash,
## VSCALE2
—47.8—μA/MHz
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running while loop from flash,
## VSCALE2
—46.1—μA/MHz
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal, CPU
running CoreMark loop from flash,
## VSCALE2
—69.4—μA/MHz
39 MHz crystal, CPU running
Prime from flash
—48.1—μA/MHz
39 MHz crystal, CPU running
while loop from flash
—47.1—μA/MHz
39 MHz crystal, CPU running
CoreMark loop from flash
—69.8—μA/MHz
38 MHz HFRCO, CPU running
while loop from flash
—39.4—μA/MHz
26 MHz HFRCO, CPU running
while loop from flash
—43.5—μA/MHz
16 MHz HFRCO, CPU running
while loop from flash
—52.5—μA/MHz
1 MHz HFRCO, CPU running
while loop from flash
—390.0—μA/MHz
Current consumption in EM1
mode with all peripherals dis-
abled
## I
## EM1
78 MHz HFRCO w/ DPLL refer-
enced to 39 MHz crystal,
## VSCALE2
—32.2—μA/MHz
39 MHz crystal—34.5—μA/MHz
38 MHz HFRCO—26.7—μA/MHz
26 MHz HFRCO—30.8—μA/MHz
16 MHz HFRCO—39.8—μA/MHz
1 MHz HFRCO—377.3—μA/MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  39

ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in EM2
mode, VSCALE0
## I
## EM2_VS
256 kB RAM and full Radio RAM
retention, RTC running from
## LFXO
## 1
—4.1—μA
256 kB RAM and full Radio RAM
retention, RTC running from
## LFRCO
## 1
—4.1—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## LFXO
## 1
—1.8—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## LFRCO
## 1
—1.8—μA
16 kB RAM and full Radio RAM
retention, RTC running from
LFRCO in precision mode
## 1
—2.7—μA
Current consumption in EM3
mode, VSCALE0
## I
## EM3_VS
256 kB RAM and full Radio RAM
retention, RTC running from
## ULFRCO
## 1
—3.7—μA
16 kB RAM and full Radio RAM
retention, RTC running from
## ULFRCO
## 1
—1.4—μA
Change in current consump-
tion if CPU cached unre-
tained in EM2 or EM3
## I
## EM23_CPUCACHE
—-0.07—μA
Change in current consump-
tion if EM0/1 peripheral
states unretained in EM2 or
## EM3
## I
## EM23_STATERET
—-0.01—μA
Change in current consump-
tion for retained RAM bank in
EM2 or EM3
## I
## EM23_RAM
Per 16 kB RAM bank—0.16—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0B is enabled
## 2
## I
## PD0B_VS
—1.4—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0C is enabled
## 2
## I
## PD0C_VS
—0.38—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0D is enabled
## 2
## I
## PD0D_VS
—1.6—μA
Additional current in EM2 or
EM3 when any peripheral in
PD0E is enabled
## 2
## I
## PD0E_VS
—0.12—μA
Current consumption in EM4
mode
## I
## EM4
No BURTC, no LF oscillator—0.18—μA
BURTC with LFXO—0.53—μA
Current consumption during
reset
## I
## RST
Hard pin reset held—391—μA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  40

ParameterSymbolTest ConditionMinTypMaxUnit
## Note:
1.CPU cache retained, EM0/1 peripheral states retained
2.Extra current consumed by power domain. Does not include current associated with the enabled peripherals. See 3.10.4 Power
Domains for a list of the peripherals in each power domain. Note that if the PD0B, PD0C, or PD0D domains are enabled, PD0E
will also automatically be enabled.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  41

4.6.6  Radio current consumption at 1.8V
RF current consumption measured with MCU in EM1, HCLK = 39.0 MHz, and all MCU peripherals disabled. Unless otherwise indica-
ted, typical conditions are: AVDD = DVDD = IOVDD = RFVDD = PAVDD = 1.8 V. DCDC disabled. T
## A
= 25 °C. Minimum and maximum
values in this table represent the worst conditions across process variation at T
## A
## = 25 °C.
Table 4.10.  Radio current consumption at 1.8V
ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in re-
ceive mode, active packet
reception
## I
## RX_ACTIVE
125 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.5—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.9—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.1—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.6—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—8—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—6.6—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—7.1—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.4—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.7—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE1
—8.1—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—8.6—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE1, EM1P (Radio
clocks only)
—7.8—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE1
—8.2—mA
802.15.4 receiving frame, f = 2.4
GHz, VSCALE2
—8.6—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  42

ParameterSymbolTest ConditionMinTypMaxUnit
Current consumption in re-
ceive mode, listening for
packet
## I
## RX_LISTEN
125 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.1—mA
125 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.9—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.1—mA
500 kbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.9—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—6.6—mA
1 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—7.4—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
VSCALE1, EM1P (Radio clocks
only)
—7.7—mA
2 Mbit/s, 2GFSK, f = 2.4 GHz,
## VSCALE2
—8.6—mA
802.15.4, f = 2.4 GHz, VSCALE1,
EM1P (Radio clocks only)
—7.6—mA
802.15.4, f = 2.4 GHz, VSCALE2—8.4—mA
Current consumption in
transmit mode
## I
## TX
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power, VSCALE2
—7.8—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power, VSCALE2
—28.5—mA
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power, VSCALE1
—7.5—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power, VSCALE1
—28.2—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  43

## 4.7  Flash Characteristics
## Table 4.11.  Flash Characteristics
ParameterSymbolTest ConditionMinTypMaxUnit
Flash Supply voltage during
write or erase
## V
## FLASH
## 1.71—3.8V
Flash data retention
## 1
## RET
## FLASH
## T
## A
≤ 125 °C10——years
Flash erase cycles before
failure
## 1
## EC
## FLASH
## T
## A
≤ 125 °C10,000——cycles
## Program Timet
## PROG
## T
## A
= 25 °C, one word (32-bits)41.943.445.0μs
## T
## A
= 25 °C, average per word
over 128 words
## 10.610.911.3μs
## Page Erase Time
## 2
t
## PERASE
## T
## A
= 25 °C11.612.914.0ms
## Mass Erase Time
## 3

## 4
t
## MERASE
## T
## A
= 25 °C, 1536kB144.3150.5156.8ms
Program CurrentI
## WRITE
——2.8mA
Page Erase CurrentI
## PERASE
Page Erase——1.9mA
Mass Erase CurrentI
## MERASE
Mass Erase——2.0mA
## Note:
1.Flash data retention information is published in the Quarterly Quality and Reliability Report.
2.Page Erase time is measured from setting the ERASEPAGE bit in the MSC_WRITECMD register until the BUSY bit in the MSC-
STATUS register is cleared to 0. Internal set-up and hold times are included.
3.Mass Erase is issued by the CPU and erases all of User space.
4.Mass Erase time is measured from setting the ERASEMAIN0 bit in the MSC_WRITECMD register until the BUSY bit in the MSC-
STATUS register is cleared to 0. Internal set-up and hold times are included.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  44

4.8  Energy Mode Wake-up and Entry Times
Unless otherwise specified, these times are measured using the HFRCO at 19 MHz.
Table 4.12.  Energy Mode Wake-up and Entry Times
ParameterSymbolTest ConditionMinTypMaxUnit
Wake-up Time from EM1t
## EM1_WU
Code execution from flash—3—HCLKs
Code execution from RAM—1.4—μs
Wake-up Time from EM2t
## EM2_WU
Code execution from flash, No
## Voltage Scaling
## —13.7—μs
Code execution from RAM, No
## Voltage Scaling
## —5.1—μs
Voltage scaling up one level
## 1
## —37.7—μs
Voltage scaling up two levels
## 2
## —50.7—μs
Wake-up Time from EM3t
## EM3_WU
Code execution from flash, No
## Voltage Scaling
## —13.7—μs
Code execution from RAM, No
## Voltage Scaling
## —5.1—μs
Voltage scaling up one level
## 1
## —37.7—μs
Voltage scaling up two levels
## 2
## —50.7—μs
Wake-up Time from EM4t
## EM4_WU
Code execution from flash—21.7—ms
Entry time to EM1t
## EM1_ENT
Code execution from flash—1.5—μs
Entry time to EM2t
## EM2_ENT
Code execution from flash—6.1—μs
Entry time to EM3t
## EM3_ENT
Code execution from flash—6.0—μs
Entry time to EM4t
## EM4_ENT
Code execution from flash—11.2—μs
Voltage scaling time in EM0
## 3
t
## SCALE
Up from VSCALE1 to VSCALE2—32—μs
Down from VSCALE2 to
## VSCALE1
## —172—μs
## Note:
1.Voltage scaling one level is between VSCALE0 and VSCALE1 or between VSCALE1 and VSCALE2.
2.Voltage scaling two levels is between VSCALE0 and VSCALE2.
3.During voltage scaling in EM0, RAM is inaccessible and processor will be halted until complete.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  45

4.9  2.4 GHz RF Transceiver Characteristics
4.9.1  RF Transmitter Characteristics
4.9.1.1  RF Transmitter General Characteristics for the 2.4 GHz Band
Unless otherwise indicated, typical conditions are: T
## A
= 25 °C, Crystal frequency=39.0 MHz, RF center frequency = 2.45 GHz.
- For 0 dBm / 10 dBm PA: VREGVDD = IOVDD = AVDD = 3.0 V, DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC
- For 20 dBm PA: VREGVDD = IOVDD = AVDD = PAVDD = 3.3 V, DVDD = RFVDD = 1.8 V powered from DCDC
Table 4.13.  RF Transmitter General Characteristics for the 2.4 GHz Band
ParameterSymbolTest ConditionMinTypMaxUnit
RF tuning frequency rangeF
## RANGE
2400—2483.5MHz
Radio-only current consump-
tion while transmitting
## 1
## I
## TX_RADIO
f = 2.4 GHz, CW, 0 dBm PA, 0
dBm output power
—3.5—mA
f = 2.4 GHz, CW, 10 dBm PA, 10
dBm output power
—17.6—mA
Maximum TX power
## 2
## POUT
## MAX
20 dBm PA, PAVDD = 3.3 V—19.5—dBm
10 dBm PA
## 3
—10—dBm
0 dBm PA—-0.7—dBm
Minimum active TX powerPOUT
## MIN
20 dBm PA, PAVDD = 3.3 V—-34—dBm
10 dBm PA—-29.8—dBm
0 dBm PA—-25.2—dBm
Output power variation vs
supply voltage variation, fre-
quency = 2450 MHz
## POUT
## VAR_V
20 dBm PA P
out
## = POUT
## MAX
out-
put power with PAVDD voltage
swept from 3.0 V to 3.8 V
—0.75—dB
10 dbm PA output power with
PAVDD voltage swept from 1.8 V
to 3.0 V
—0.03—dB
0 dBm PA output power with
PAVDD voltage swept from 1.8 V
to 3.0 V
—0.02—dB
Output power variation vs
temperature, Frequency =
2450 MHz
## POUT
## VAR_T
PAVDD = 3.3 V supply, 20 dBm
PA at POUT
## MAX
, (-40 to +125 °C)
—0.7—dB
10 dBm PA at 10 dBm, (-40 to
## +125 °C)
—0.2—dB
0 dBm PA at 0 dBm, (-40 to +125
## °C)
—1.23—dB
Output power variation vs RF
frequency
## POUT
## VAR_F
20 dBm PA, POUT
## MAX
## , PAVDD =
## 3.3 V
—0.17—dB
10 dBm PA, 10 dBm—0.11—dB
0 dBm PA, 0 dBm—0.16—dB
Spurious emissions of har-
monics in restricted bands
per FCC Part 15.205/15.209
## SPUR
## HRM_FCC_
## R
Continuous transmission of CW
carrier, P
out
## = POUT
## MAX
## , Test
Frequency = 2450 MHz.
—-47—dBm
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  46

ParameterSymbolTest ConditionMinTypMaxUnit
Spurious emissions of har-
monics in non-restricted
bands per FCC Part
## 15.247/15.35
## SPUR
## HRM_FCC_
## NRR
Continuous transmission of CW
carrier. P
out
## = POUT
## MAX
## . Test
Frequency = 2450 MHz.
—-26—dBc
Spurious emissions out-of-
band (above 2.483 GHz or
below 2.4 GHz) in restricted
bands, per FCC part
## 15.205/15.209
## SPUR
## OOB_FCC_
## R
Restricted bands 30-88 MHz,
Continuous transmission of CW
carrier, P
out
## = POUT
## MAX
## , Test
Frequency = 2450 MHz
—-61—dBm
Restricted bands 88 - 216 MHz,
Continuous transmission of CW
carrier, P
out
## = POUT
## MAX
## , Test
Frequency = 2450 MHz
—-58—dBm
Restricted bands 216 - 960 MHz,
Continuous transmission of CW
carrier, P
out
## = POUT
## MAX
## , Test
Frequency = 2450 MHz
—-55—dBm
Restricted bands > 960 MHz,
Continuous transmission of CW
carrier, P
out
## = POUT
## MAX
## , Test
Frequency = 2450 MHz
—-47—dBm
Spurious emissions out-of-
band in non-restricted bands
per FCC Part 15.247
## SPUR
## OOB_FCC_
## NR
Frequencies above 2.483 GHz or
below 2.4 GHz, continuous trans-
mission CW carrier, P
out
## =
## POUT
## MAX
## , Test Frequency =
2450 MHz
—-26—dBc
Spurious emissions per ETSI
## EN300.440
## SPUR
## ETSI440
47-74 MHz,87.5-108 MHz,
174-230 MHz, 470-862 MHz, P
out
= 10 dBm, Test Frequency = 2450
MHz
—-60—dBm
25-1000 MHz, excluding above
frequencies. P
out
= 10 dBm, Test
Frequency = 2450 MHz
—-42—dBm
## 1G-14G, P
out
= 10 dBm, Test Fre-
quency = 2450 MHz
—-36—dBm
Spurious emissions out-of-
band, per ETSI 300.328
## SPUR
## ETSI328
[2400-2BW to 2400-BW],
[2483.5+BW to 2483.5+2BW],
## P
out
= 10 dBm, Test Frequency =
2450 MHz
—-26—dBm
47-74 MHz, 87.5-118 MHz,
174-230 MHz, 470-862 MHz, P
out
= 10 dBm, Test Frequency = 2450
MHz
—-60—dBm
30-47 MHz, 74-87.5 MHz,
118-174 MHz, 230-470 MHz,
862-1000 MHz , P
out
= 10 dBm,
Test Frequency = 2450 MHz
—-42—dBm
1G-12.75 GHz, excluding bands
listed above, P
out
= 10 dBm, Test
Frequency = 2450 MHz
—-36—dBm
[2400-BW to 2400], [2483.5 to
## 2483.5+BW] P
out
= 10 dBm, Test
Frequency = 2450 MHz
—-16—dBm
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  47

ParameterSymbolTest ConditionMinTypMaxUnit
## Note:
1.Supply current to radio, supplied by DC-DC with 3.0 V, measured at VREGVDD.
2.Supported transmit power levels are determined by the ordering part number (OPN). Transmit power ratings for all devices cov-
ered in this data sheet can be found in the Max TX Power column of the Ordering Information Table.
3.The PA is capable of delivering higher than 10 dBm output power (refer to Output Power plots in 4.27.2 RF Characteristics). How-
ever, all transmitter characteristics and recommended application circuits are specified at 10 dBm output. If used with the recom-
mended application circuits above 10 dBm, harmonics may be higher than regulatory limits.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  48

4.9.1.2  RF Transmitter Characteristics for 802.15.4 DSSS-OQPSK in the 2.4 GHz Band
Unless otherwise indicated, typical conditions are: T
## A
= 25 °C, Crystal frequency=39.0 MHz, RF center frequency = 2.45 GHz.
- For 0 dBm / 10 dBm PA: VREGVDD = IOVDD = AVDD = 3.0 V, DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC
- For 20 dBm PA: VREGVDD = IOVDD = AVDD = PAVDD = 3.3 V, DVDD = RFVDD = 1.8 V powered from DCDC
Table 4.14.  RF Transmitter Characteristics for 802.15.4 DSSS-OQPSK in the 2.4 GHz Band
ParameterSymbolTest ConditionMinTypMaxUnit
Error vector magnitude per
## 802.15.4-2011
EVMAverage across frequency, signal
is DSSS-OQPSK reference pack-
et, PAVDD = 3.3 V, P
out
## =
## POUT
## MAX
—3—% rms
Average across frequency, signal
is DSSS-OQPSK reference pack-
et, P
out
= 10 dBm
—2.9—% rms
Average across frequency, signal
is DSSS-OQPSK reference pack-
et, P
out
= 0 dBm
—2.9—% rms
Power spectral density limitPSD
## LIMIT
Relative, at carrier ± 3.5 MHz,
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
—-50.2—dBc/
100kHz
Relative, at carrier ± 3.5 MHz,
## P
out
= 10 dBm
—-50.1—dBc/
100kHz
Relative, at carrier ± 3.5 MHz,
## P
out
= 0 dBm
—-50.7—dBc/
100kHz
Absolute, at carrier ± 3.5 MHz,
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
—-38.3—dBm/
100kHz
Absolute, at carrier ± 3.5 MHz,
## P
out
= 10 dBm
—-48.7—dBm/
100kHz
Absolute, at carrier ± 3.5 MHz,
## P
out
= 0 dBm
—-59.2—dBm/
100kHz
Per FCC part 15.247, PAVDD =
## 3.3 V, P
out
## = POUT
## MAX
—0.5—dBm/
3kHz
Per FCC part 15.247, P
out
## = 10
dBm
—-9.2—dBm/
3kHz
Per FCC part 15.247, P
out
## = 0
dBm
—-19.9—dBm/
3kHz
## ETSI 300.328 P
out
= 10 dBm—8—dBm
## ETSI 300.328 P
out
= 0 dbm—-2.8—dBm
Occupied channel bandwidth
per ETSI EN300.328
## OCP
## ETSI328
99% BW at highest and lowest
channels in band, P
out
= 10 dBm
—2.2—MHz
99% BW at highest and lowest
channels in band, P
out
= 0 dBm
—2.2—MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  49

4.9.1.3  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 1 Mbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
= 25 °C, Crystal frequency=39.0 MHz, RF center frequency = 2.45 GHz.
- For 0 dBm / 10 dBm PA: VREGVDD = IOVDD = AVDD = 3.0 V, DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC
- For 20 dBm PA: VREGVDD = IOVDD = AVDD = PAVDD = 3.3 V, DVDD = RFVDD = 1.8 V powered from DCDC
Table 4.15.  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 1 Mbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
Transmit 6 dB bandwidthTXBWPAVDD = 3.3 V, P
out
## = POUT
## MAX
—718—kHz
## P
out
= 10 dBm—714—kHz
## P
out
= 0 dBm—715—kHz
Power spectral density limitPSD
## LIMIT
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Per FCC part 15.247
—-0.5—dBm/
3kHz
## P
out
= 10 dBm, Per FCC part
15.247 at 10 dBm
—-10.4—dBm/
3kHz
## P
out
= 0 dBm, Per FCC part
15.247 at 0 dBm
—-21.2—dBm/
3kHz
Per ETSI 300.328 at 10 dBm/1
MHz
—9.7—dBm
Occupied channel bandwidth
per ETSI EN300.328
## OCP
## ETSI328
## P
out
= 10 dBm 99% BW at highest
and lowest channels in band
—1—MHz
## P
out
= 0 dBm 99% BW at highest
and lowest channels in band
—1—MHz
In-band spurious emissions,
with allowed exceptions
## 1
## SPUR
## INB
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Inband spurs at ± 2 MHz
—-26.9—dBm
## P
out
= 10 dBm, Inband spurs at ±
2 MHz
—-38.8—dBm
## P
out
= 0 dBm, Inband spurs at ± 2
MHz
—-49.8—dBm
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
Inband spurs at ± 3 MHz
—-33.2—dBm
## P
out
= 10 dBm Inband spurs at ± 3
MHz
—-43.8—dBm
## P
out
= 0 dBm Inband spurs at ± 3
MHz
—-54.6—dBm
## Note:
1.Per Bluetooth Core_5.1, Vol.6 Part A, Section 3.2.2, exceptions are allowed in up to three bands of 1 MHz width, centered on a
frequency which is an integer multiple of 1 MHz. These exceptions shall have an absolute value of -20 dBm or less.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  50

4.9.1.4  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 2 Mbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
= 25 °C, Crystal frequency=39.0 MHz, RF center frequency = 2.45 GHz.
- For 0 dBm / 10 dBm PA: VREGVDD = IOVDD = AVDD = 3.0 V, DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC
- For 20 dBm PA: VREGVDD = IOVDD = AVDD = PAVDD = 3.3 V, DVDD = RFVDD = 1.8 V powered from DCDC
Table 4.16.  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 2 Mbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
Transmit 6 dB bandwidthTXBWPAVDD = 3.3 V, P
out
## = POUT
## MAX
—1307—kHz
## P
out
= 10 dBm—1308—kHz
## P
out
= 0 dBm—1306—kHz
Power spectral density limitPSD
## LIMIT
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Per FCC part 15.247
—1.5—dBm/
3kHz
## P
out
= 10 dBm, Per FCC part
15.247 at 10 dBm
—-8.5—dBm/
3kHz
## P
out
= 0 dBm, Per FCC part
15.247 at 0 dBm
—-19.3—dBm/
3kHz
Per ETSI 300.328 at 10 dBm/1
MHz
—8.7—dBm
Occupied channel bandwidth
per ETSI EN300.328
## OCP
## ETSI328
## P
out
= 10 dBm 99% BW at highest
and lowest channels in band
—2.1—MHz
## P
out
= 0 dBm 99% BW at highest
and lowest channels in band
—2.1—MHz
In-band spurious emissions,
with allowed exceptions
## 1
## SPUR
## INB
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Inband spurs at ± 4 MHz
—-33.7—dBm
## P
out
= 10 dBm, Inband spurs at ±
4 MHz
—-43.7—dBm
## P
out
= 0 dBm, Inband spurs at ± 4
MHz
—-54.5—dBm
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
Inband spurs at ± 6 MHz
—-38.9—dBm
## P
out
= 10 dBm Inband spurs at ± 6
MHz
—-48.8—dBm
## P
out
= 0 dBm Inband spurs at ± 6
MHz
—-59.5—dBm
## Note:
1.Per Bluetooth Core_5.1, Vol.6 Part A, Section 3.2.2, exceptions are allowed in up to three bands of 1 MHz width, centered on a
frequency which is an integer multiple of 1 MHz. These exceptions shall have an absolute value of -20 dBm or less.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  51

4.9.1.5  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 500 kbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
= 25 °C, Crystal frequency=39.0 MHz, RF center frequency = 2.45 GHz.
- For 0 dBm / 10 dBm PA: VREGVDD = IOVDD = AVDD = 3.0 V, DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC
- For 20 dBm PA: VREGVDD = IOVDD = AVDD = PAVDD = 3.3 V, DVDD = RFVDD = 1.8 V powered from DCDC
Table 4.17.  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 500 kbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
Transmit 6 dB bandwidthTXBWPAVDD = 3.3 V, P
out
## = POUT
## MAX
—717—kHz
## P
out
= 10 dBm—718—kHz
## P
out
= 0 dBm—717—kHz
Power spectral density limitPSD
## LIMIT
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Per FCC part 15.247
—-0.5—dBm/
3kHz
## P
out
= 10 dBm, Per FCC part
15.247 at 10 dBm
—-10.4—dBm/
3kHz
## P
out
= 0 dBm, Per FCC part
15.247 at 0 dBm
—-21.2—dBm/
3kHz
Per ETSI 300.328 at 10 dBm/1
MHz
—9.7—dBm
Occupied channel bandwidth
per ETSI EN300.328
## OCP
## ETSI328
## P
out
= 10 dBm 99% BW at highest
and lowest channels in band
—1—MHz
## P
out
= 0 dBm 99% BW at highest
and lowest channels in band
—1—MHz
In-band spurious emissions,
with allowed exceptions
## 1
## SPUR
## INB
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Inband spurs at ± 2 MHz
—-26.9—dBm
## P
out
= 10 dBm, Inband spurs at ±
2 MHz
—-38.9—dBm
## P
out
= 0 dBm, Inband spurs at ± 2
MHz
—-49.8—dBm
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
Inband spurs at ± 3 MHz
—-33.2—dBm
## P
out
= 10 dBm Inband spurs at ± 3
MHz
—-43.8—dBm
## P
out
= 0 dBm Inband spurs at ± 3
MHz
—-54.6—dBm
## Note:
1.Per Bluetooth Core_5.1, Vol.6 Part A, Section 3.2.2, exceptions are allowed in up to three bands of 1 MHz width, centered on a
frequency which is an integer multiple of 1 MHz. These exceptions shall have an absolute value of -20 dBm or less.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  52

4.9.1.6  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 125 kbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
= 25 °C, Crystal frequency=39.0 MHz, RF center frequency = 2.45 GHz.
- For 0 dBm / 10 dBm PA: VREGVDD = IOVDD = AVDD = 3.0 V, DVDD = RFVDD = PAVDD = 1.8 V powered from DCDC
- For 20 dBm PA: VREGVDD = IOVDD = AVDD = PAVDD = 3.3 V, DVDD = RFVDD = 1.8 V powered from DCDC
Table 4.18.  RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 125 kbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
Transmit 6 dB bandwidthTXBWPAVDD = 3.3 V, P
out
## = POUT
## MAX
—651—kHz
## P
out
= 10 dBm—651—kHz
## P
out
= 0 dBm—651—kHz
Power spectral density limitPSD
## LIMIT
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Per FCC part 15.247
—13.7—dBm/
3kHz
## P
out
= 10 dBm, Per FCC part
15.247 at 10 dBm
—3.8—dBm/
3kHz
## P
out
= 0 dBm, Per FCC part
15.247 at 0 dBm
—-7—dBm/
3kHz
Per ETSI 300.328 at 10 dBm/1
MHz
—9.7—dBm
Occupied channel bandwidth
per ETSI EN300.328
## OCP
## ETSI328
## P
out
= 10 dBm 99% BW at highest
and lowest channels in band
—1—MHz
## P
out
= 0 dBm 99% BW at highest
and lowest channels in band
—1—MHz
In-band spurious emissions,
with allowed exceptions
## 1
## SPUR
## INB
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
## ,
Inband spurs at ± 2 MHz
—-26.9—dBm
## P
out
= 10 dBm, Inband spurs at ±
2 MHz
—-39—dBm
## P
out
= 0 dBm, Inband spurs at ± 2
MHz
—-49.7—dBm
## PAVDD = 3.3 V, P
out
## = POUT
## MAX
Inband spurs at ± 3 MHz
—-33.1—dBm
## P
out
= 10 dBm Inband spurs at ± 3
MHz
—-43.7—dBm
## P
out
= 0 dBm Inband spurs at ± 3
MHz
—-54.5—dBm
## Note:
1.Per Bluetooth Core_5.1, Vol.6 Part A, Section 3.2.2, exceptions are allowed in up to three bands of 1 MHz width, centered on a
frequency which is an integer multiple of 1 MHz. These exceptions shall have an absolute value of -20 dBm or less.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  53

4.9.2  RF Receiver Characteristics
4.9.2.1  RF Receiver General Characteristics for the 2.4 GHz Band
Unless otherwise indicated, typical conditions are: T
## A
## = 25 °C, VREGVDD = IOVDD = AVDD = PAVDD = 3.0V, RFVDD = DVDD = 1.8
V powered from DCDC. Crystal frequency = 39.0 MHz, RF center frequency = 2.45 GHz.
Table 4.19.  RF Receiver General Characteristics for the 2.4 GHz Band
ParameterSymbolTest ConditionMinTypMaxUnit
RF tuning frequency rangeF
## RANGE
2400—2483.5MHz
Radio-only current consump-
tion in receive mode
## 1
## I
## RX_RADIO
—2.8—mA
Receive mode maximum
spurious emission
## SPUR
## RX
30 MHz to 1 GHz—-63—dBm
1 GHz to 12 GHz—-53—dBm
Max spurious emissions dur-
ing active receive mode, per
FCC Part 15.109(a)
## SPUR
## RX_FCC
216 MHz to 960 MHz, conducted
measurement
—-55—dBm
Above 960 MHz, conducted
measurement.
—-47—dBm
2GFSK SensitivitySENS
## 2GFSK
2 Mbps 2GFSK signal, 1% PER—-92.5—dBm
250 kbps 2GFSK signal, 0.1%
## BER
—-102.9—dBm
## Note:
1.Supply current to radio, supplied by DC-DC with 3.0 V, measured at VREGVDD.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  54

4.9.2.2  RF Receiver Characteristics for 802.15.4 DSSS-OQPSK in the 2.4 GHz Band
Unless otherwise indicated, typical conditions are: T
## A
## = 25 °C, VREGVDD = IOVDD = AVDD = PAVDD = 3.0V, RFVDD = DVDD = 1.8
V powered from DCDC. Crystal frequency = 39.0 MHz, RF center frequency = 2.45 GHz.
Table 4.20.  RF Receiver Characteristics for 802.15.4 DSSS-OQPSK in the 2.4 GHz Band
ParameterSymbolTest ConditionMinTypMaxUnit
## Rx Max Strong Signal Input
Level for 1% PER
## RX
## SAT
Signal is reference signal
## 1
, packet
length is 20 octets
—10—dBm
Sensitivity, 1% PERSENSSignal is reference signal, packet
length is 20 octets
—-105.4—dBm
Co-channel interferer rejec-
tion, 1% PER
CCRDesired signal 3 dB above sensi-
tivity limit
—-0.7—dB
Adjacent channel rejection,
Interferer is reference signal,
1% PER, desired is refer-
ence signal at 3 dB above
reference sensitivity level
## 2
## ACR
## REF1
Interferer is reference signal at +1
channel spacing
—36.8—dB
Interferer is reference signal at -1
channel spacing
—37.5—dB
Alternate channel rejection,
interferer is reference signal,
1% PER, desired is refer-
ence signal at 3 dB above
reference sensitivity level
## 2
## ACR
## REF2
Interferer is reference signal at +2
channel spacing
—48.9—dB
Interferer is reference signal at -2
channel spacing
—49.4—dB
Image rejection, 1% PER,
desired is reference signal at
3 dB above reference sensi-
tivity level
## 2
## IR
Interferer is CW in image band
## 3
—53.5—dB
Blocking rejection of all other
channels, 1% PER, desired
is reference signal at 3 dB
above reference sensitivity
level
## 2
, interferer is reference
signal
BLOCKInterferer frequency < desired fre-
quency -3 channel spacing
—55.3—dB
Interferer frequency > desired fre-
quency +3 channel spacing
—55.1—dB
RSSI resolutionRSSI
## RES
-100 dBm to +5 dBm—0.25—dB
RSSI accuracy in the linear
region as defined by
## 802.15.4-2003
## RSSI
## LIN
—+/-6—dB
## Note:
1.Reference signal is defined as O-QPSK DSSS per 802.15.4, Frequency range = 2400-2483.5 MHz, Symbol rate = 62.5 ksym-
bols/s.
2.Reference sensitivity level is -85 dBm.
3.Due to low-IF frequency, there is some overlap of adjacent channel and image channel bands. Adjacent channel CW blocker
tests place the Interferer center frequency at the Desired frequency ± 5 MHz on the channel raster, whereas the image rejection
test places the CW interferer near the image frequency of the Desired signal carrier, regardless of the channel raster.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  55

4.9.2.3  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 1 Mbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
## = 25 °C, VREGVDD = IOVDD = AVDD = PAVDD = 3.0V, RFVDD = DVDD = 1.8
V powered from DCDC. Crystal frequency = 39.0 MHz, RF center frequency = 2.45 GHz, Packet length is 255 bytes.
Table 4.21.  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 1 Mbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
## Rx Max Strong Signal Input
Level for 0.1% BER
## RX
## SAT
Signal is reference signal
## 1
—10—dBm
SensitivitySENSSignal is reference signal, 37 byte
payload
## 2
—-97.6—dBm
Signal is reference signal, 255
byte payload
## 1
—-96—dBm
With non-ideal signals
## 3

## 1
—-95.7—dBm
Signal to co-channel interfer-
er
## C/I
## CC
(see notes)
## 1

## 4
—8.7—dB
N ± 1 Adjacent channel se-
lectivity
## C/I
## 1
Interferer is reference signal at +1
MHz offset
## 1

## 5

## 4

## 6
—-5.4—dB
Interferer is reference signal at -1
MHz offset
## 1

## 5

## 4

## 6
—-5.3—dB
N ± 2 Alternate channel se-
lectivity
## C/I
## 2
Interferer is reference signal at +2
MHz offset
## 1

## 5

## 4

## 6
—-40.9—dB
Interferer is reference signal at -2
MHz offset
## 1

## 5

## 4

## 6
—-39.7—dB
N ± 3 Alternate channel se-
lectivity
## C/I
## 3
Interferer is reference signal at +3
MHz offset
## 1

## 5

## 4

## 6
—-45.5—dB
Interferer is reference signal at -3
MHz offset
## 1

## 5

## 4

## 6
—-45.7—dB
Selectivity to image frequen-
cy
## C/I
## IM
Interferer is reference signal at im-
age frequency with 1 MHz preci-
sion
## 1

## 6
—-23.3—dB
Selectivity to image frequen-
cy ± 1 MHz
## C/I
## IM_1
Interferer is reference signal at im-
age frequency +1 MHz with 1
MHz precision
## 1

## 6
—-40.9—dB
Interferer is reference signal at im-
age frequency -1 MHz with 1 MHz
precision
## 1

## 6
—-5.4—dB
Intermodulation performanceIM
n = 3 (see note
## 7
## )
—-17.3—dBm
## Note:
## 1.0.017% Bit Error Rate.
## 2.0.1% Bit Error Rate.
3.With non-ideal signals as specified in Bluetooth Test Specification RF-PHY.TS.5.0.1 section 4.7.1
4.Desired signal -67 dBm.
5.Desired frequency 2402 MHz ≤ Fc ≤ 2480 MHz.
6.With allowed exceptions.
7.As specified in Bluetooth Core specification version 5.1, Vol 6, Part A, Section 4.4

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  56

4.9.2.4  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 2 Mbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
## = 25 °C, VREGVDD = IOVDD = AVDD = PAVDD = 3.0V, RFVDD = DVDD = 1.8
V powered from DCDC. Crystal frequency = 39.0 MHz, RF center frequency = 2.45 GHz, Packet length is 255 bytes.
Table 4.22.  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 2 Mbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
## Rx Max Strong Signal Input
Level for 0.1% BER
## RX
## SAT
Signal is reference signal
## 1
—10—dBm
SensitivitySENSSignal is reference signal, 37 byte
payload
## 2
—-94.8—dBm
Signal is reference signal, 255
byte payload
## 1
—-93.3—dBm
With non-ideal signals
## 3

## 1
—-93.1—dBm
Signal to co-channel interfer-
er
## C/I
## CC
(see notes)
## 1

## 4
—8.6—dB
N ± 1 Adjacent channel se-
lectivity
## C/I
## 1
Interferer is reference signal at +2
MHz offset
## 1

## 5

## 4

## 6
—-5.3—dB
Interferer is reference signal at -2
MHz offset
## 1

## 5

## 4

## 6
—-5.8—dB
N ± 2 Alternate channel se-
lectivity
## C/I
## 2
Interferer is reference signal at +4
MHz offset
## 1

## 5

## 4

## 6
—-42.2—dB
Interferer is reference signal at -4
MHz offset
## 1

## 5

## 4

## 6
—-44.2—dB
N ± 3 Alternate channel se-
lectivity
## C/I
## 3
Interferer is reference signal at +6
MHz offset
## 1

## 5

## 4

## 6
—-48.1—dB
Interferer is reference signal at -6
MHz offset
## 1

## 5

## 4

## 6
—-50.2—dB
Selectivity to image frequen-
cy
## C/I
## IM
Interferer is reference signal at im-
age frequency with 1 MHz preci-
sion
## 1

## 6
—-22.8—dB
Selectivity to image frequen-
cy ± 2 MHz
## C/I
## IM_1
Interferer is reference signal at im-
age frequency +2 MHz with 1
MHz precision
## 1

## 6
—-42.2—dB
Interferer is reference signal at im-
age frequency -2 MHz with 1 MHz
precision
## 1

## 6
—-5.3—dB
Intermodulation performanceIM
n = 3 (see note
## 7
## )
—-18.3—dBm
## Note:
## 1.0.017% Bit Error Rate.
## 2.0.1% Bit Error Rate.
3.With non-ideal signals as specified in Bluetooth Test Specification RF-PHY.TS.5.0.1 section 4.7.1
4.Desired signal -64 dBm.
5.Desired frequency 2402 MHz ≤ Fc ≤ 2480 MHz.
6.With allowed exceptions.
7.As specified in Bluetooth Core specification version 5.1, Vol 6, Part A, Section 4.4

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  57

4.9.2.5  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 500 kbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
## = 25 °C, VREGVDD = IOVDD = AVDD = PAVDD = 3.0V, RFVDD = DVDD = 1.8
V powered from DCDC. Crystal frequency = 39.0 MHz, RF center frequency = 2.45 GHz, Packet length is 255 bytes.
Table 4.23.  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 500 kbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
## Rx Max Strong Signal Input
Level for 0.1% BER
## RX
## SAT
Signal is reference signal
## 1
—10—dBm
SensitivitySENSSignal is reference signal, 37 byte
payload
## 2
—-101.4—dBm
Signal is reference signal, 255
byte payload
## 1
—-100.1—dBm
With non-ideal signals
## 3

## 1
—-99.1—dBm
Signal to co-channel interfer-
er
## C/I
## CC
(see notes)
## 1

## 4
—2.7—dB
N ± 1 Adjacent channel se-
lectivity
## C/I
## 1
Interferer is reference signal at +1
MHz offset
## 1

## 5

## 4

## 6
—-7.1—dB
Interferer is reference signal at -1
MHz offset
## 1

## 5

## 4

## 6
—-7.4—dB
N ± 2 Alternate channel se-
lectivity
## C/I
## 2
Interferer is reference signal at +2
MHz offset
## 1

## 5

## 4

## 6
—-46.8—dB
Interferer is reference signal at -2
MHz offset
## 1

## 5

## 4

## 6
—-49.7—dB
N ± 3 Alternate channel se-
lectivity
## C/I
## 3
Interferer is reference signal at +3
MHz offset
## 1

## 5

## 4

## 6
—-49.4—dB
Interferer is reference signal at -3
MHz offset
## 1

## 5

## 4

## 6
—-54.5—dB
Selectivity to image frequen-
cy
## C/I
## IM
Interferer is reference signal at im-
age frequency with 1 MHz preci-
sion
## 1

## 6
—-49—dB
Selectivity to image frequen-
cy ± 1 MHz
## C/I
## IM_1
Interferer is reference signal at im-
age frequency +1 MHz with 1
MHz precision
## 1

## 6
—-49.4—dB
Interferer is reference signal at im-
age frequency -1 MHz with 1 MHz
precision
## 1

## 6
—-46.8—dB
## Note:
## 1.0.017% Bit Error Rate.
## 2.0.1% Bit Error Rate.
3.With non-ideal signals as specified in Bluetooth Test Specification RF-PHY.TS.5.0.1 section 4.7.1
4.Desired signal -72 dBm.
5.Desired frequency 2402 MHz ≤ Fc ≤ 2480 MHz.
6.With allowed exceptions.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  58

4.9.2.6  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 125 kbps Data Rate
Unless otherwise indicated, typical conditions are: T
## A
## = 25 °C, VREGVDD = IOVDD = AVDD = PAVDD = 3.0V, RFVDD = DVDD = 1.8
V powered from DCDC. Crystal frequency = 39.0 MHz, RF center frequency = 2.45 GHz, Packet length is 255 bytes.
Table 4.24.  RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 125 kbps Data Rate
ParameterSymbolTest ConditionMinTypMaxUnit
## Rx Max Strong Signal Input
Level for 0.1% BER
## RX
## SAT
Signal is reference signal
## 1
—10—dBm
SensitivitySENSSignal is reference signal, 37 byte
payload
## 2
—-105.7—dBm
Signal is reference signal, 255
byte payload
## 1
—-105.3—dBm
With non-ideal signals
## 3

## 1
—-104.8—dBm
Signal to co-channel interfer-
er
## C/I
## CC
(see notes)
## 1

## 4
—0.9—dB
N ± 1 Adjacent channel se-
lectivity
## C/I
## 1
Interferer is reference signal at +1
MHz offset
## 1

## 5

## 4

## 6
—-12.4—dB
Interferer is reference signal at -1
MHz offset
## 1

## 5

## 4

## 6
—-12.8—dB
N ± 2 Alternate channel se-
lectivity
## C/I
## 2
Interferer is reference signal at +2
MHz offset
## 1

## 5

## 4

## 6
—-52.6—dB
Interferer is reference signal at -2
MHz offset
## 1

## 5

## 4

## 6
—-55.5—dB
N ± 3 Alternate channel se-
lectivity
## C/I
## 3
Interferer is reference signal at +3
MHz offset
## 1

## 5

## 4

## 6
—-53.8—dB
Interferer is reference signal at -3
MHz offset
## 1

## 5

## 4

## 6
—-60—dB
Selectivity to image frequen-
cy
## C/I
## IM
Interferer is reference signal at im-
age frequency with 1 MHz preci-
sion
## 1

## 6
—-53—dB
Selectivity to image frequen-
cy ± 1 MHz
## C/I
## IM_1
Interferer is reference signal at im-
age frequency +1 MHz with 1
MHz precision
## 1

## 6
—-53.8—dB
Interferer is reference signal at im-
age frequency -1 MHz with 1 MHz
precision
## 1

## 6
—-52.6—dB
## Note:
## 1.0.017% Bit Error Rate.
## 2.0.1% Bit Error Rate.
3.With non-ideal signals as specified in Bluetooth Test Specification RF-PHY.TS.5.0.1 section 4.7.1
4.Desired signal -79 dBm.
5.Desired frequency 2402 MHz ≤ Fc ≤ 2480 MHz.
6.With allowed exceptions.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  59

## 4.10  Oscillators
4.10.1  High Frequency Crystal Oscillator (HFXO)
Unless otherwise indicated, typical conditions are: AVDD = DVDD = 3.0 V. T
## A
= 25 °C. Minimum and maximum values in this table
represent the worst conditions across process variation, operating supply voltage range, and operating temperature range.
Table 4.25.  High Frequency Crystal Oscillator (HFXO)
ParameterSymbolTest ConditionMinTypMaxUnit
Crystal FrequencyF
## HFXO
see note
## 1

## 2

## 3
38.039.040.0MHz
Supported crystal maximum
equivalent series resistance
## (ESR)
## ESR
## HFXO
Crystal Frequency = 39.0 MHz——60Ω
Supported range of crystal
load capacitance
## 4
## C
## L_HFXO
39.0 MHz, ESR = 40 Ω
## 5
—10—pF
Supply CurrentI
## HFXO
39.0 MHz—565—μA
## Startup Time
## 6
## T
## STARTUP
39.0 MHz, ESR = 40 Ω, C
## L
## = 10
pF
## —188—μs
On-chip tuning cap step
size
## 7
## SS
## HFXO
—0.04—pF
HFCLKOUT load capaci-
tance
## C
## HFCLKOUT
—2030pF
HFCLKOUT output voltageV
## HFCLKOUT
## 0—1.2V
HFCLKOUT AC output am-
plitude, XOUTBIASANA = 5,
## C
## HFCLKOUT
= 20 pF
## VAC
## HFCLKOUT
XOUTCFANA = 0—470—mVpp
XOUTCFANA = 1—510—mVpp
XOUTCFANA = 2—560—mVpp
XOUTCFANA = 3—615—mVpp
HFCLKOUT current con-
sumption
## I
## HFCLKOUT
## C
## HFCLKOUT
≤ 10 pF, XOUTBIA-
## SANA = 3
—3.1—mA
10 pF < C
## HFCLKOUT
≤ 20 pF,
## XOUTBIASANA = 5
—3.9—mA
20 pF < C
## HFCLKOUT
≤ 30 pF,
## XOUTBIASANA = 15
—6.3—mA
Frequency shift of HFXTAL
when HFCLKOUT is active
## FS
## HFCLKOUT
Assuming crystal pullability of 13
ppm/pF
## -1.5—4.5ppm
HFCLKOUT shorting output
resistance
## RS
## HFCLKOUT
When output is disabled—150—Ω
Total harmonic distortion,
## XOUTBIASANA = 5,
## C
## HFCLKOUT
= 20 pF
## THD
## HFCLKOUT
## XOUTCFANA = 0—8.53—%
## XOUTCFANA = 1—10.05—%
## XOUTCFANA = 2—11.98—%
## XOUTCFANA = 3—14.39—%
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  60

ParameterSymbolTest ConditionMinTypMaxUnit
## Note:
1.The BLE radio requires a 38.4 Mhz or 39.0 MHz crystal with a tolerance of ± 50 ppm over temperature and aging. Please use a
crystal with the recommended frequency and tolerance (refer to AN0016.2 for recommended crystals).
2.The ZigBee radio requires a 38.4 Mhz or 39.0 MHz crystal with a tolerance of ± 40 ppm over temperature and aging. Please use
a crystal with the recommended frequency and tolerance (refer to AN0016.2 for recommended crystals).
3.The radio requires additional software configuration based on crystal frequency. Refer to the Simplicity Studio component "RAIL
Utility, Built-in PHYs Across HFXO Frequencies".
4.Total load capacitance as seen by the crystal.
5.RF performance characteristics have been determined using crystals with an ESR of 40 Ω and C
## L
of 10 pF.
6.Startup time does not include time implemented by programmable TIMEOUTSTEADY delay.
7.The tuning step size is the effective step size when incrementing both of the tuning capacitors by one count. The step size for the
each of the individual tuning capacitors is twice this value.

4.10.2  Low Frequency Crystal Oscillator (LFXO)
Table 4.26.  Low Frequency Crystal Oscillator (LFXO)
ParameterSymbolTest ConditionMinTypMaxUnit
Crystal FrequencyF
## LFXO
—32.768—kHz
Supported Crystal equivalent
series resistance (ESR)
## ESR
## LFXO
GAIN = 0——80kΩ
GAIN = 1 to 3——100kΩ
Supported range of crystal
load capacitance
## 1
## C
## L_LFXO
GAIN = 04—6pF
GAIN = 16—10pF
GAIN = 2 (see note
## 2
## )
10—12.5pF
GAIN = 3 (see note
## 2
## )
12.5—18pF
Current consumptionI
CL12p5
ESR = 70 kΩ, C
## L
= 12.5 pF,
## GAIN
## 3
## = 2, AGC
## 4
## = 1
—294—nA
Startup TimeT
## STARTUP
ESR = 70 kΩ, C
## L
= 7 pF, GAIN
## 3
## =
## 1, AGC
## 4
## = 1
## —52—ms
On-chip tuning cap step sizeSS
## LFXO
—0.26—pF
On-chip tuning capacitor val-
ue at minimum setting
## 5
## C
## LFXO_MIN
CAPTUNE = 0—5.2—pF
On-chip tuning capacitor val-
ue at maximum setting
## 5
## C
## LFXO_MAX
CAPTUNE = 0x4F—26.2—pF
## Note:
1.Total load capacitance seen by the crystal
2.Crystals with a load capacitance of greater than 12 pF require external load capacitors.
3.In LFXO_CAL Register
4.In LFXO_CFG Register
5.The effective load capacitance seen by the crystal will be C
## LFXO
/2. This is because each XTAL pin has a tuning cap and the two
caps will be seen in series by the crystal

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  61

4.10.3  High Frequency RC Oscillator (HFRCO)
Unless otherwise indicated, typical conditions are: AVDD = DVDD = 3.0 V. T
## A
= 25 °C. Minimum and maximum values in this table
represent the worst conditions across process variation, operating supply voltage range, and operating temperature range.
Table 4.27.  High Frequency RC Oscillator (HFRCO)
ParameterSymbolTest ConditionMinTypMaxUnit
Frequency AccuracyF
## HFRCO_ACC
For all production calibrated fre-
quencies
## -3—3%
Current consumption on all
supplies
## 1
## I
## HFRCO
## F
## HFRCO
= 4 MHz—28—μA
## F
## HFRCO
= 5 MHz
## 2
—29—μA
## F
## HFRCO
= 7 MHz—59—μA
## F
## HFRCO
= 10 MHz
## 2
—63—μA
## F
## HFRCO
= 13 MHz—77—μA
## F
## HFRCO
= 16 MHz—87—μA
## F
## HFRCO
= 19 MHz—90—μA
## F
## HFRCO
= 20 MHz
## 2
—107—μA
## F
## HFRCO
= 26 MHz—116—μA
## F
## HFRCO
= 32 MHz—139—μA
## F
## HFRCO
= 38 MHz
## 3
—170—μA
## F
## HFRCO
= 40 MHz
## 2
—172—μA
## F
## HFRCO
= 48 MHz
## 3
—207—μA
## F
## HFRCO
= 56 MHz
## 3
—228—μA
## F
## HFRCO
= 64 MHz
## 3
—269—μA
## F
## HFRCO
= 80 MHz
## 3
—285—μA
Clock Out current for
## HFRCODPLL
## 4
## I
## CLKOUT_HFRCOD
## PLL
FORCEEN bit of HFRCO0_CTRL
## = 1
—4.5—μA/MHz
Clock Out current for
## HFRCOEM23
## 4
## I
## CLKOUT_HFRCOE
## M23
FORCEEN bit of
## HFRCOEM23_CTRL = 1
—2.0—μA/MHz
## Startup Time
## 5
## T
## STARTUP
FREQRANGE = 0 to 7—1.2—μs
FREQRANGE = 8 to 15—0.6—μs
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  62

ParameterSymbolTest ConditionMinTypMaxUnit
## Band Frequency Limits
## 6
f
## HFRCO_BAND
FREQRANGE = 03.71—5.24MHz
FREQRANGE = 14.39—6.26MHz
FREQRANGE = 25.25—7.55MHz
FREQRANGE = 36.22—9.01MHz
FREQRANGE = 47.88—11.6MHz
FREQRANGE = 59.9—14.6MHz
FREQRANGE = 611.5—17.0MHz
FREQRANGE = 714.1—20.9MHz
FREQRANGE = 816.4—24.7MHz
FREQRANGE = 919.8—30.4MHz
FREQRANGE = 1022.7—34.9MHz
FREQRANGE = 1128.6—44.4MHz
FREQRANGE = 1233.0—51.0MHz
FREQRANGE = 1342.2—64.6MHz
FREQRANGE = 1448.8—74.8MHz
FREQRANGE = 1557.6—87.4MHz
## Note:
1.Does not include additional clock tree current. See specifications for additional current when selected as a clock source for a par-
ticular clock multiplexer.
2.This frequency is calibrated for the HFRCOEM23 only.
3.This frequency is calibrated for the HFRCODPLL (HFRCO0) only.
4.When the HFRCO is enabled for characterization using the FORCEEN bit, the total current will be the HFRCO core current plus
the specified CLKOUT current. When the HFRCO is enabled on demand, the clock current may be different.
5.Hardware delay ensures settling to within ± 0.5%. Hardware also enforces this delay on a band change.
6.The frequency band limits represent the lowest and highest frequency which each band can achieve over the operating range.

4.10.4  Fast Start-Up RC Oscillator (FSRCO)
Table 4.28.  Fast Start-Up RC Oscillator (FSRCO)
ParameterSymbolTest ConditionMinTypMaxUnit
FSRCO frequencyF
## FSRCO
17.22021.2MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  63

4.10.5  Precision Low Frequency RC Oscillator (LFRCO)
Table 4.29.  Precision Low Frequency RC Oscillator (LFRCO)
ParameterSymbolTest ConditionMinTypMaxUnit
Nominal oscillation frequen-
cy
## F
## LFRCO
—32.768—kHz
Frequency accuracyF
## LFRCO_ACC
Normal mode-3—3%
Precision mode
## 1
, across operat-
ing temperature range
## 2
## -500—500ppm
Startup timet
## STARTUP
Normal mode—204—μs
Precision mode
## 1
## —11.7—ms
Current consumptionI
## LFRCO
Normal mode—189.9—nA
Precision mode
## 1
, T = stable at 25
## °C
## 3
—649.8—nA
## Note:
1.The LFRCO operates in high-precision mode when CFG_HIGHPRECEN is set to 1. High-precision mode is not available in EM4.
2.Includes ± 40 ppm frequency tolerance of the HFXO crystal.
3.Includes periodic re-calibration against HFXO crystal oscillator.

4.10.6  Ultra Low Frequency RC Oscillator (ULFRCO)
Table 4.30.  Ultra Low Frequency RC Oscillator (ULFRCO)
ParameterSymbolTest ConditionMinTypMaxUnit
Oscillation FrequencyF
## ULFRCO
0.9441.01.095kHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  64

4.11  GPIO Pins (GPIO)
Table 4.31.  GPIO Pins (GPIO)
ParameterSymbolTest ConditionMinTypMaxUnit
Leakage currentI
## LEAK_IO
MODEx = DISABLED, IOVDD =
## 1.71 V
—1.9—nA
MODEx = DISABLED, IOVDD =
## 3.0 V
—2.5—nA
MODEx = DISABLED, IOVDD =
## 3.8 V T
## A
## = 125 °C, PB00-PB03,
## PC06-PC09, PA00
——250nA
MODEx = DISABLED, IOVDD =
## 3.8 V T
## A
= 125 °C, All Other Pins
——200nA
Input low voltage
## 1
## V
## IL
Any GPIO pin——0.3 *
## IOVDD
## V
RESETn——0.3 * DVDDV
Input high voltage
## 1
## V
## IH
Any GPIO pin0.7 *
## IOVDD
## ——V
RESETn0.7 * DVDD——V
Hysteresis of input voltageV
## HYS
Any GPIO pin0.05 *
## IOVDD
## ——V
RESETn0.05 *
## DVDD
## ——V
Output high voltageV
## OH
Sourcing 20mA, IOVDD = 3.0 V0.8 *
## IOVDD
## ——V
Sourcing 8mA, IOVDD = 1.71 V0.6 *
## IOVDD
## ——V
Output low voltageV
## OL
Sinking 20mA, IOVDD = 3.0 V——0.2 *
## IOVDD
## V
Sinking 8mA, IOVDD = 1.71 V——0.4 *
## IOVDD
## V
GPIO rise timeT
## GPIO_RISE
## IOVDD = 3.0 V, C
load
= 50pF,
SLEWRATE = 4, 10% to 90%
## —8.4—ns
## IOVDD = 1.7 V, C
load
= 50pF,
SLEWRATE = 4, 10% to 90%
## —13—ns
GPIO fall timeT
## GPIO_FALL
## IOVDD = 3.0 V, C
load
= 50pF,
SLEWRATE = 4, 90% to 10%
## —7.1—ns
## IOVDD = 1.7 V, C
load
= 50pF,
SLEWRATE = 4, 90% to 10%
## —11.9—ns
Pull up/down resistance
## 2
## R
## PULL
Any GPIO pin. Pull-up to IOVDD:
MODEn = DISABLE DOUT=1.
Pull-down to VSS: MODEn =
## WIREDORPULLDOWN DOUT =
## 0.
354455kΩ
RESETn pin. Pull-up to DVDD354455kΩ
Maximum filtered glitch widthT
## GF
MODE = INPUT, DOUT = 1—27—ns
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  65

ParameterSymbolTest ConditionMinTypMaxUnit
RESETn low time to ensure
pin reset
## T
## RESET
## 100——ns
## Note:
1.GPIO input thresholds are proportional to the IOVDD pin. RESETn input thresholds are proportional to DVDD.
2.GPIO pull-ups connect to IOVDD supply, pull-downs connect to VSS. RESETn pull-up connects to DVDD.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  66

4.12  Analog to Digital Converter (IADC)
Specified at 1 Msps, ADCCLK = 10 MHz, OSR=2, unless otherwise indicated.
Table 4.32.  Analog to Digital Converter (IADC)
ParameterSymbolTest ConditionMinTypMaxUnit
Main analog supplyV
## AVDD
Normal mode1.71—3.8V
High-Speed mode1.71—3.8V
High-Accuracy mode1.71—3.8V
## Maximum Input Range
## 1
## V
## IN_MAX
Maximum allowable input voltage0—AVDDV
Full-Scale VoltageV
## FS
Voltage required for Full-Scale
measurement
## —V
## REF
/ Gain—V
Input Measurement RangeV
## IN
Differential Mode - Plus and Mi-
nus inputs
## -V
## FS
## —+V
## FS
## V
Single Ended Mode - One input
tied to ground
## 0—V
## FS
## V
Input Sampling CapacitanceCsAnalog Gain = 1x—1.8—pF
Analog Gain = 2x—3.6—pF
Analog Gain = 3x—5.4—pF
Analog Gain = 4x—7.2—pF
Analog Gain = 0.5x—0.9—pF
ADC clock frequencyf
## ADC_CLK
Normal mode, Gain = 1x or 0.5x——10MHz
Normal mode, Gain = 2x——5MHz
Normal mode, Gain = 3x or 4x——2.5MHz
High-Speed mode, Gain = 1x or
## 0.5x
——20MHz
High-Speed mode, Gain = 2x——10MHz
High-Speed mode, Gain = 3x or
## 4x
——5MHz
High-Accuracy mode——5MHz
Input sampling frequencyf
## S
## Normal Mode—f
## ADC_CLK
/4—MHz
High-Speed Mode—f
## ADC_CLK
/4—MHz
High-Accuracy Mode—f
## ADC_CLK
/5—MHz
Throughput ratef
## SAMPLE
Normal mode, f
## ADC_CLK
## = 10
MHz, OSR = 2
——1Msps
Normal mode, f
## ADC_CLK
## = 10
MHz, OSR = 32
## ——76.9ksps
High-Speed mode, f
## ADC_CLK
## = 20
MHz, OSR = 2
——2Msps
High-Accuracy mode, f
## ADC_CLK
## =
5 MHz, OSR = 92
## ——10.7ksps
High-Accuracy mode, f
## ADC_CLK
## =
5 MHz, OSR = 256
## ——3.88ksps
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  67

ParameterSymbolTest ConditionMinTypMaxUnit
Current from all supplies,
Continuous operation
## I
## ADC_CONT
Normal Mode, 1 Msps, OSR = 2,
f
## ADC_CLK
= 10 MHz
—305385μA
High-Speed Mode, 2 Msps, OSR
= 2, f
## ADC_CLK
= 20 MHz
—550655μA
High-Accuracy Mode, 10.7 ksps,
OSR = 92, f
## ADC_CLK
= 5 MHz
—108170μA
Current in Standby mode.
ADC is not functional but can
wake up in 1us.
## I
## STBY
Normal mode—17—μA
High-Speed mode—21—μA
High-Accuracy mode—10—μA
ADC Startup Timet
startup
From power down state—5—μs
From standby state—1—μs
Normal Mode ADC Resolu-
tion
## 2
ResolutionOSR = 2—12—bits
OSR = 32—16—bits
High-Speed Mode ADC Res-
olution
## 2
## Resolution
## HS
OSR = 2—12—bits
OSR = 32—16—bits
High-Accuracy Mode ADC
## Resolution
## Resolution
## HA
High Accuracy mode. Typical val-
ue is for default OSR = 92 for 10.7
ksps, max value is limited by code
length.
## —1620bits
Differential NonlinearityDNLNormal mode. Differential Input.
OSR = 2 (No missing codes)
## -1+/- 0.251.5LSB12
High Speed mode. Differential In-
put. OSR = 2
## -1+/- 0.251.5LSB12
High-Accuracy mode
## 3
## . Differential
Input. 10.7 ksps with OSR = 92
## -1—1LSB16
Integral NonlinearityINLNormal mode. Differential Input,
## OSR = 2
## -2.5+/- 0.652.5LSB12
High-Speed mode. Differential In-
put.
## -2.5+/- 0.652.5LSB12
High-Accuracy mode
## 3
## . Differential
Input. External VREF = 1.25 V.
10.7 ksps with OSR = 92
## —+/- 0.25—LSB16
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  68

ParameterSymbolTest ConditionMinTypMaxUnit
Effective number of bits
## 4
ENOBNormal Mode, Differential Input.
Gain = 1x, OSR = 2, f
## IN
= 10 kHz,
Internal VREF = 1.21V
## 10.711.7—bits
## Normal Mode, Differential Input.
Gain = 1x, OSR = 32, f
## IN
## = 2.5
kHz, Internal VREF = 1.21 V.
## —13.5—bits
## Normal Mode, Differential Input.
Gain = 1x, OSR = 32, f
## IN
## = 2.5
kHz, External VREF = 1.25 V.
## —14.3—bits
High Speed mode. Differential In-
put. Gain = 1x, OSR = 2, f
## IN
## = 10
kHz, Internal VREF = 1.21 V
## 10.711.5—bits
High-Accuracy mode
## 3
## . Differential
Input. Gain = 1x, f
## IN
## = 100 Hz, Ex-
ternal VREF = 1.25 V. 10.7 ksps
with OSR = 92
## 14.015.3—bits
High-Accuracy mode
## 3
## . Differential
Input. Gain = 1x, f
## IN
## = 100 Hz, Ex-
ternal VREF = 1.25 V. 3.88 ksps
with OSR = 256
## —16.1—bits
Signal to Noise + Distortion
## Ratio Normal Mode
## 4
SNDRDifferential Input. Gain=1x, OSR =
2, f
## IN
= 10 kHz, Internal VREF =
## 1.21V
6672.3—dB
Differential Input. Gain=2x, OSR =
2, f
## IN
= 10 kHz, Internal VREF =
## 1.21V
—72.3—dB
Differential Input. Gain=4x, OSR =
2, f
## IN
= 10 kHz, Internal VREF =
## 1.21V
—68.8—dB
Differential Input. Gain=0.5x, OSR
= 2, f
## IN
= 10 kHz, Internal VREF =
## 1.21V
—72.5—dB
Differential Input. Gain = 1x, OSR
= 64, f
## IN
= 1.25 kHz, Internal
## VREF = 1.21 V
—83.9—dB
Signal to Noise + Distortion
Ratio High-Speed mode
## SNDR
## HS
High Speed mode. Differential In-
put. Gain = 1x, OSR = 2, f
## IN
## = 10
kHz, Internal VREF = 1.21 V
6672.3—dB
High Speed mode. Differential In-
put. Gain = 2x, OSR = 2, f
## IN
## = 10
kHz, Internal VREF = 1.21 V
—72.3—dB
High Speed mode. Differential In-
put. Gain = 4x, OSR = 2, f
## IN
## = 10
kHz, Internal VREF = 1.21 V
—68.8—dB
High Speed mode. Differential In-
put. Gain = 0.5x, OSR = 2, f
## IN
## =
10 kHz, Internal VREF = 1.21 V
—72.5—dB
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  69

ParameterSymbolTest ConditionMinTypMaxUnit
Signal to Noise + Distortion
Ratio High-Accuracy mode
## 3
## SNDR
## HA
High-Accuracy. Differential Input.
Gain = 1x, f
## IN
## = 100 Hz, External
VREF = 1.25 V. 3.88 ksps with
## OSR = 256
—98.7—dB
High-Accuracy. Differential Input.
Gain = 1x, f
## IN
## = 100 Hz, External
VREF = 1.25 V. 10.7 ksps with
## OSR = 92
8693.8—dB
High-Accuracy. Differential Input.
Gain = 2x, f
## IN
## = 100 Hz, External
VREF = 1.25 V. 10.7 ksps with
## OSR = 92
—93.5—dB
High-Accuracy. Differential Input.
Gain = 4x, f
## IN
## = 100 Hz, External
VREF = 1.25 V. 10.7 ksps with
## OSR = 92
—91.0—dB
High-Accuracy. Differential Input.
Gain = 0.5x, f
## IN
## = 100 Hz, Exter-
nal VREF = 1.25 V. 10.7 ksps with
## OSR = 92
—94.7—dB
Total Harmonic DistortionTHDNormal mode, Differential Input.
Gain = 1x, OSR = 2, f
## IN
= 10 kHz,
Internal VREF = 1.21 V
—-80.8-70dB
High Speed mode, Differential In-
put. Gain = 1x, OSR = 2, f
## IN
## = 10
kHz, Internal VREF = 1.21 V
—-84.3-70dB
High-Accuracy mode
## 3
## , Differential
Input. f
## IN
## = 100 Hz, External
VREF = 1.25 V. 10.7 ksps with
## OSR = 92
—-101-80dB
Spurious-Free Dynamic
## Range
SFDRNormal mode, Differential Input.
Gain = 1x, OSR = 2, f
## IN
= 10 kHz,
Internal VREF = 1.21 V
7286.5—dB
High Speed mode, Differential In-
put. Gain = 1x, f
## IN
= 10 kHz, Inter-
nal VREF = 1.21 V
7284.3—dB
High-Accuracy mode
## 3
## , Differential
Input. f
## IN
## = 100 Hz, External
VREF = 1.25 V. 10.7 ksps with
## OSR = 92
100118.1—dB
## Common Mode Rejection
## Ratio
CMRRNormal mode. DC to 100 Hz—87.0—dB
Normal mode. AC high frequency.—68.6—dB
High-Speed mode. DC to 100 Hz—86.3—dB
High-Speed mode. AC high fre-
quency.
—59.0—dB
High-Accuracy mode
## 3
. DC to 100
## Hz
—93.8—dB
High Accuracy mode
## 3
. AC high
frequency.
—87.0—dB
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  70

ParameterSymbolTest ConditionMinTypMaxUnit
## Power Supply Rejection Ra-
tio
PSRRNormal mode. DC to 100 Hz—80.4—dB
Normal mode. AC high frequency,
using internal VBGR
—33.4—dB
Normal mode. AC high frequency,
using VREF pad
—65.2—dB
High Speed modes. DC to 100 Hz—79.8—dB
High-Speed mode. AC high fre-
quency, using internal VBGR
—31.0—dB
High Speed mode. AC high fre-
quency, using VREF pad
—65.0—dB
High Accuracy mode
## 3
. DC to 100
## Hz
—124—dB
High-Accuracy mode
## 3
. AC high
frequency, using external VREF
pin.
—85.0—dB
External reference voltage
range
## 1
## V
## EVREF
## 1.0—AVDDV
Offset Error, Normal modeOFFSETGAIN = 1 and 0.5, Differential In-
put
## -30.273LSB12
GAIN = 2, Differential Input-40.274LSB12
GAIN = 3, Differential Input-40.254LSB12
GAIN = 4, Differential Input-40.294LSB12
## Offset Error, High-speed
mode
## OFFSET
## HS
GAIN = 1 and 0.5, Differential In-
put
## -30.273LSB12
GAIN = 2, Differential Input-40.274LSB12
GAIN = 3, Differential Input-40.254LSB12
GAIN = 4, Differential Input-40.294LSB12
## Offset Error, High-accuracy
mode
## 3
## OFFSET
## HA
All GAIN settings, Differential In-
put
## -7-0.0117LSB16
Gain Error, Normal modeGEGAIN = 1 and 0.5, using external
VREF, direct mode, f
## ADC_CLK
## = 10
MHz
## -0.6-0.1550.6%
GAIN = 2, using external VREF,
direct mode, f
## ADC_CLK
= 5 MHz
## -0.6-0.1550.6%
GAIN = 3, using external VREF,
direct mode, f
## ADC_CLK
= 2.5 MHz
## -0.70.1860.7%
GAIN = 4, using external VREF,
direct mode, f
## ADC_CLK
= 2.5 MHz
## -1.10.2271.1%
Internal VREF
## 5
, all GAIN settings
## -1.50.0231.5%
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  71

ParameterSymbolTest ConditionMinTypMaxUnit
## Gain Error, High-speed
mode
## GE
## HS
GAIN = 1 and 0.5, using external
VREF, direct mode, f
## ADC_CLK
## = 20
MHz
## -0.6-0.1550.6%
GAIN = 2, using external VREF,
direct mode, f
## ADC_CLK
= 10 MHz
## -0.6-0.0550.6%
GAIN = 3, using external VREF,
direct mode, f
## ADC_CLK
= 5 MHz
## -0.70.1860.7%
GAIN = 4, using external VREF,
direct mode, f
## ADC_CLK
= 5 MHz
## -1.10.2271.1%
Internal VREF
## 5
, all GAIN settings
## -1.50.0231.5%
## Gain Error, High-accuracy
mode
## 3
## GE
## HA
GAIN = 1 and 0.5, using external
VREF, direct mode.
## -0.50.0060.5%
GAIN = 2, using external VREF,
direct mode.
## -0.5-0.0670.5%
GAIN = 3, using external VREF,
direct mode.
## -0.5-0.0700.5%
GAIN = 4, using external VREF,
direct mode.
## -0.5-0.0980.5%
Internal Reference voltageV
## IVREF
## —1.21—V
## Note:
1.When inputs are routed to external GPIO pins, the maximum pin voltage is limited to the lower of the IOVDD and AVDD supplies.
2.ADC output resolution depends on the OSR and digital averaging settings. With no digital averaging, ADC output resolution is 12
bits at OSR = 2, 13 bits at OSR = 4, 14 bits at OSR = 8, 15 bits at OSR = 16, 16 bits at OSR = 32 and 17 bits at OSR = 64. Digital
averaging has a similar impact on ADC output resolution. See the product reference manual for additional details.
3.High-Accuracy mode performance specifications are tested with inputs applied to the dedicated AIN pins.
4.The relationship between ENOB and SNDR is specified according to the equation: ENOB = (SNDR - 1.76) / 6.02.
5.Includes error from internal VREF drift.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  72

4.13  Analog Comparator (ACMP)
Table 4.33.  Analog Comparator (ACMP)
ParameterSymbolTest ConditionMinTypMaxUnit
ACMP Supply currentI
## ACMP
## BIAS = 0
## 1
## , HYST = DISABLED
(100 °C max)
—71—nA
## BIAS = 1
## 1
## , HYST = DISABLED
—270—nA
## BIAS = 2
## 1
## , HYST = DISABLED
—668—nA
## BIAS = 3
## 1
## , HYST = DISABLED
—2.5—μA
BIAS = 4, HYST = DISABLED—5.4—μA
BIAS = 5, HYST = DISABLED—10.6—μA
BIAS = 6, HYST = DISABLED—27—μA
BIAS = 7, HYST = DISABLED—50100μA
ACMP Supply current with
## Hysteresis
## 2
## I
## ACMP_WHYS
## BIAS = 3
## 1
## , HYST = SYM30MV
—3.4—μA
BIAS = 4, HYST = SYM30MV—7.3—μA
BIAS = 5, HYST = SYM30MV—15—μA
BIAS = 6, HYST = SYM30MV—38—μA
BIAS = 7, HYST = SYM30MV—71—μA
Current consumption from
VREFDIV in continuous
mode
## I
## VREFDIV
NEGSEL = VREFDIVAVDD—3.2—μA
NEGSEL = VREFDIV1V25—4.3—μA
NEGSEL = VREFDIV2V5—7.1—μA
Current consumption from
VREFDIV in sample/hold
mode
## I
## VREFDIV_SH
NEGSEL = VREFDIV2V5LP—81—nA
NEGSEL = VREFDIV1V25LP—74—nA
NEGSEL = VREFDIVAVDDLP—76—nA
Current consumption from
VSENSEDIV in continuous
mode
## I
## VSENSEDIV
NEGSEL = VSENSE01DIV4—1.7—μA
Current consumption from
VSENSEDIV in sample/hold
mode
## I
## VSENSEDIV_SH
NEGSEL = VSENSE01DIV4LP—59.1—nA
Hysteresis (BIAS = 4)
## 2
## V
## HYST
## HYST = SYM10MV
## 3
—18—mV
## HYST = SYM20MV
## 3
—33—mV
## HYST = SYM30MV
## 3
—47—mV
Reference VoltageV
## ACMPREF
Internal 1.25 V Reference1.191.251.31V
Internal 2.5 V Reference2.342.52.75V
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  73

ParameterSymbolTest ConditionMinTypMaxUnit
Input offset voltageV
## OFFSET
BIAS = 0, VCM = 0.15 to AVDD -
## 0.15 V
-25—25mV
BIAS = 3, VCM = 0.15 to AVDD -
## 0.15 V
-25—25mV
BIAS = 4, VCM = 0.15 to AVDD -
## 0.15 V
-25—25mV
BIAS = 7, VCM = 0.15 to AVDD -
## 0.15 V
-30—30mV
Input RangeV
## IN
Input Voltage Range0—AVDDV
Comparator delay with 100
mV overdrive
## T
## DELAY
BIAS = 0, (100 °C max)—10—μs
BIAS = 1—2.7—μs
BIAS = 2—1.4—μs
BIAS = 3—0.58—μs
BIAS = 4—224—ns
BIAS = 5—133—ns
BIAS = 6—80—ns
BIAS = 7—63—ns
## Capacitive Sense Oscillator
## Resistance
## R
## CSRESSEL
CSRESSEL = 0—15.9—kΩ
CSRESSEL = 1—25.3—kΩ
CSRESSEL = 2—43.6—kΩ
CSRESSEL = 3—61.9—kΩ
CSRESSEL = 4—80.2—kΩ
CSRESSEL = 5—98.6—kΩ
CSRESSEL = 6—117—kΩ
## Note:
1.When using the 1.25 V or 2.5 V VREF in continuous mode (VREFDIV1V25 or VREFDIV2V5) and BIAS < 4, an additional 1 μA of
supply current is required.
2.Hysteresis is not supported for BIAS=0/1/2. Software should set HYST=DISABLED if using BIAS=0/1/2.
## 3.V
## CM
## = 1.25 V

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  74

4.14  Digital to Analog Converter (VDAC)
Table 4.34.  Digital to Analog Converter (VDAC)
ParameterSymbolTest ConditionMinTypMaxUnit
Output voltageV
## DACOUT
## 0—VREFV
Output CurrentI
## DACOUT
-10—10mA
DAC clock frequencyf
## DAC
——1MHz
Sample rateSR
## DAC
f
## DAC
= f
DAC(max)
## ——500ksps
ResolutionN
## RESOLUTION
## —12—bits
## Load Capacitance
## 1
## C
## LOAD
High Power and Lower Power
## Modes
——50pF
High Capacitance Load Mode25——nF
Load ResistanceR
## LOAD
5——kΩ
Current consumption, Dy-
namic, 500 ksps, 1 channel
active
## 2
## I
## DAC_1_500
High Power Mode—281—μA
Low Power Mode—179—μA
Current consumption, Dy-
namic, 500 ksps, 2 channels
active
## 2
## I
## DAC_2_500
High Power Mode—445—μA
Low Power Mode—242—μA
Current consumption, Static,
1 channel active
## 3
## I
## DAC_1_STAT
High Power Mode—135—μA
Low Power Mode—31—μA
High Capacitance Mode—43—μA
Current consumption, Static,
2 channels active
## 3
## I
## DAC_2_STAT
High Power Mode—262—μA
Low Power Mode—53—μA
High Capacitance Mode—78—μA
Startup timet
## DACSTARTUP
Enable to 90% full scale output,
settling to 10 LSB
## —4.54.9μs
Settling timet
## DACSETTLE
High Power Mode, 25% to 75% of
full scale, settling to 10 LSB
## —1.11.6μs
Low Power Mode, 25% to 75% of
full scale, settling to 1%
## —2.7—μs
Output impedanceR
## OUT
Main Output, High Power Mode—2.1—Ω
Main Output, Low Power Mode—3.4—Ω
Power supply rejection ratio
## 4
PSRRVout = 50% full scale, DC output—88.6—dB
Signal to noise and distortion
ratio
## SNDR
## DAC
High Power mode, 500 ksps, in-
ternal 2.5 V reference, 1 kHz sine
wave input, BW limited to 250 kHz
65.867.2—dB
High Power mode, 500 ksps, in-
ternal 2.5 V reference, 1 kHz sine
wave input, BW limited to 22 kHz
68.070.6—dB
Total Harmonic DistortionTHDHigh Power Mode, internal 2.5 V
reference, 1 kHz sine wave input
—-72.5-68.7dB
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  75

ParameterSymbolTest ConditionMinTypMaxUnit
Integral Non-LinearityINL
## DAC
High Power Mode, Across full
temperature range
## -5—5LSB
Differential Non-Linearity
## 5
## DNL
## DAC
High Power Mode, Across full
temperature range
## -1—1.3LSB
Offset error
## 6
## V
## OFFSET
High Power mode-15—15mV
Low Power Mode-25—25mV
High Capacitance Load mode-35—35mV
Gain error
## 6
## V
## GAIN
1.25 V internal reference-1.5—1.5%
2.5 V internal reference-2—2%
## External Reference-0.6—0.6%
## External Reference Voltage
## 7
## V
## EXTREF
## 1.1—V_AVDDV
## Note:
1.Main outputs only.
2.Dynamic current specifications are for VDAC circuitry operating at max clock frequency with the output updated at the specified
sampling rate using DMA transfers. Output is a 1 kHz sine wave from 10% to 90% full scale. Specified current does not include
current required to drive the external load. Measurement includes all current from AVDD and DVDD supplies.
3.Static current specifications are for VDAC circuitry operating after a one-time update to a static output at 50% full scale, with the
VDAC APB clock disabled. Specified current does not include current required to drive the external load. Measurement includes
all current from AVDD and DVDD supplies.
4.PSRR calculated as 20 * log
## 10
## (ΔVDD / ΔV
## OUT
## ).
5.Entire range is monotonic and has no missing codes.
6.Gain is calculated by measuring the slope from 10% to 90% of full scale. Offset is calculated by comparing actual VDAC output at
10% of full scale to ideal VDAC output at 10% of full scale with the measured gain.
7.External reference voltage on VREFP pin or PA00 when used for VREFP

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  76

## 4.15  Temperature Sensor
## Table 4.35.  Temperature Sensor
ParameterSymbolTest ConditionMinTypMaxUnit
Temperature sensor range
## 1
## T
## RANGE
## -40—125°C
Temperature sensor resolu-
tion
## T
## RESOLUTION
## —0.25—°C
Measurement noise (RMS)T
## NOISE
Single measurement—0.6—°C
16-sample average (TEMPAVG-
## NUM = 0)
## —0.17—°C
64-sample average (TEMPAVG-
## NUM = 1)
## —0.12—°C
Temperature offsetT
## OFF
Mean error of uncorrected output
across full temperature range
## —3.2—°C
Temperature sensor accura-
cy
## 2

## 3
## T
## ACC
Direct output accuracy after mean
error (T
## OFF
) removed
## —+/-3—°C
After linearization in software, no
calibration
## —+/-2—°C
After linearization in software, with
single-temperature calibration at
## 25 °C
## 4
## —+/-1.5—°C
Measurement intervalt
## MEAS
## —250—ms
## Note:
1.The sensor reports absolute die temperature in Kelvin (K). All specifications are in °C to match the units of the specified product
temperature range.
2.Error is measured as the deviation of the mean temperature reading from the expected die temperature. Accuracy numbers rep-
resent statistical minimum and maximum using ± 4 standard deviations of measured error.
3.The raw output of the temperature sensor is a predictable curve. It can be linearized with a polynomial function for additional ac-
curacy.
4.Assuming calibration accuracy of ± 0.25 °C.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  77

## 4.16  Brown Out Detectors
## 4.16.1  DVDD BOD
BOD thresholds on DVDD in EM0 and EM1 only, unless otherwise noted. Typical conditions are at T
## A
= 25 °C. Minimum and maximum
values in this table represent the worst conditions across process variation, operating supply voltage range, and operating temperature
range.
Table 4.36.  DVDD BOD
ParameterSymbolTest ConditionMinTypMaxUnit
BOD thresholdV
## DVDD_BOD
Supply Rising—1.671.71V
Supply Falling1.621.65—V
BOD response timet
## DVDD_BOD_DE-
## LAY
Supply dropping at 100 mV/μs
slew rate
## 1
## —0.95—μs
BOD hysteresisV
## DVDD_BOD_HYS
## T
—25—mV
## Note:
1.If the supply slew rate exceeds the specified slew rate, the BOD may trip later than expected (at a threshold below the minimum
specified threshold), or the BOD may not trip at all (e.g., if the supply ramps down and then back up at a very fast rate)

4.16.2  Low-Energy DVDD BOD
BOD thresholds on DVDD pin for low energy modes EM2 to EM4, unless otherwise noted.
Table 4.37.  Low-Energy DVDD BOD
ParameterSymbolTest ConditionMinTypMaxUnit
BOD thresholdV
## DVDD_LE_BOD
Supply Falling1.5—1.71V
BOD response timet
## DVDD_LE_BOD_D
## ELAY
Supply dropping at 2 mV/μs slew
rate
## 1
## —50—μs
BOD hysteresisV
## DVDD_LE_BOD_
## HYST
—20—mV
## Note:
1.If the supply slew rate exceeds the specified slew rate, the BOD may trip later than expected (at a threshold below the minimum
specified threshold), or the BOD may not trip at all (e.g., if the supply ramps down and then back up at a very fast rate)

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  78

4.16.3  AVDD and IOVDD BODs
BOD thresholds for AVDD BOD and IOVDD BOD. Available in all energy modes.
Table 4.38.  AVDD and IOVDD BODs
ParameterSymbolTest ConditionMinTypMaxUnit
BOD thresholdV
## BOD
Supply falling1.45—1.71V
BOD response timet
## BOD_DELAY
Supply dropping at 2 mV/μs slew
rate
## 1
## —50—μs
BOD hysteresisV
## BOD_HYST
—24—mV
## Note:
1.If the supply slew rate exceeds the specified slew rate, the BOD may trip later than expected (at a threshold below the minimum
specified threshold), or the BOD may not trip at all (e.g., if the supply ramps down and then back up at a very fast rate)

4.17  Pulse Counter (PCNT)
Table 4.39.  Pulse Counter (PCNT)
ParameterSymbolTest ConditionMinTypMaxUnit
Input frequencyF
## IN
Asynchronous Single and Quad-
rature Modes
——1.0MHz
Sampled Modes with Debounce
filter set to 0.
——8kHz
Setup time in asynchronous
external clock mode
t
## SU_S1N_S0N
S1N (data) to S0N (clock)53——ns
Hold time in asynchronous
external clock mode
t
## HD_S0N_S1N
S0N (clock) to S1N (data)47——ns
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  79

4.18  USART SPI Main Timing
## CS
## SCLK
## CLKPOL = 0
## MOSI
## MISO
t
## CS_MO
t
## H_MI
t
## SU_MI
t
## SCLK_MO
t
## SCLK
## SCLK
## CLKPOL = 1
Figure 4.1.  SPI Main Timing (SMSDELAY = 0)
## CS
## SCLK
## CLKPOL = 0
## MOSI
## MISO
t
## H_MI
t
## SU_MI
t
## SCLK_MO
t
## SCLK
## SCLK
## CLKPOL = 1
t
## CS_MO
Figure 4.2.  SPI Main Timing (SMSDELAY = 1)
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  80

4.18.1  USART SPI Main Timing, Voltage Scaling = VSCALE2
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD). All GPIO set to slew rate = 6.
Table 4.40.  USART SPI Main Timing, Voltage Scaling = VSCALE2
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK period
## 1

## 2

## 3
t
## SCLK
## 2*t
## PCLK
## ——ns
CS to MOSI
## 1

## 2
t
## CS_MO
## -15—15ns
SCLK to MOSI
## 1

## 2
t
## SCLK_MO
## -6—13ns
MISO setup time
## 1

## 2
t
## SU_MI
IOVDD = 1.62 V40——ns
IOVDD = 3.0 V31——ns
MISO hold time
## 1

## 2
t
## H_MI
## -9——ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1.
2.Measurement done with 8 pF output loading at 10% and 90% of the I/O supply.
## 3.t
## PCLK
is one period of the selected PCLK.

4.18.2  USART SPI Main Timing, Voltage Scaling = VSCALE1
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD). All GPIO set to slew rate = 6.
Table 4.41.  USART SPI Main Timing, Voltage Scaling = VSCALE1
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK period
## 1

## 2

## 3
t
## SCLK
## 2*t
## PCLK
## ——ns
CS to MOSI
## 1

## 2
t
## CS_MO
## -26—25ns
SCLK to MOSI
## 1

## 2
t
## SCLK_MO
## -7—24ns
MISO setup time
## 1

## 2
t
## SU_MI
IOVDD = 1.62 V50——ns
IOVDD = 3.0 V42——ns
MISO hold time
## 1

## 2
t
## H_MI
## -9——ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1.
2.Measurement done with 8 pF output loading at 10% and 90% of the I/O supply.
## 3.t
## PCLK
is one period of the selected PCLK.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  81

4.19  USART SPI Secondary Timing
## CS
## SCLK
## CLKPOL = 0
## MOSI
## MISO
t
## CS_ACT_MI
t
## SCLK_HI
t
## SCLK
t
## SU_MO
t
## H_MO
t
## SCLK_MI
t
## CS_DIS_MI
t
## SCLK_LO
## SCLK
## CLKPOL = 1
Figure 4.3.  SPI Secondary Timing (SSSEARLY = 0)
## CS
## SCLK
## CLKPOL = 0
## MOSI
## MISO
t
## CS_ACT_MI
t
## SCLK_HI
t
## SCLK
t
## SU_MO
t
## H_MO
t
## SCLK_MI
t
## CS_DIS_MI
t
## SCLK_LO
## SCLK
## CLKPOL = 1
Figure 4.4.  SPI Secondary Timing (SSSEARLY = 1)
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  82

4.19.1  USART SPI Secondary Timing, Voltage Scaling = VSCALE2
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD). All GPIO set to slew rate = 6.
Table 4.42.  USART SPI Secondary Timing, Voltage Scaling = VSCALE2
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK period
## 1

## 2

## 3
t
## SCLK
## 6*t
## PCLK
## ——ns
SCLK high time
## 1

## 2

## 3
t
## SCLK_HI
## 2.5*t
## PCLK
## ——ns
SCLK low time
## 1

## 2

## 3
t
## SCLK_LO
## 2.5*t
## PCLK
## ——ns
CS active to MISO
## 1

## 2
t
## CS_ACT_MI
## 19—67ns
CS disable to MISO
## 1

## 2
t
## CS_DIS_MI
## 24—89ns
MOSI setup time
## 1

## 2
t
## SU_MO
## 12——ns
MOSI hold time
## 1

## 2

## 3
t
## H_MO
## 13——ns
SCLK to MISO
## 1

## 2

## 3
t
## SCLK_MI
## 14 +
## 1.5*t
## PCLK
## —24 +
## 2.5*t
## PCLK
ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1 (figure only shows CLKPHA = 0).
2.Measurement done with 8 pF output loading at 10% and 90% of the I/O supply (figure shows 50%).
## 3.t
## PCLK
is one period of the selected PCLK.

4.19.2  USART SPI Secondary Timing, Voltage Scaling = VSCALE1
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD). All GPIO set to slew rate = 6.
Table 4.43.  USART SPI Secondary Timing, Voltage Scaling = VSCALE1
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK period
## 1

## 2

## 3
t
## SCLK
## 6*t
## PCLK
## ——ns
SCLK high time
## 1

## 2

## 3
t
## SCLK_HI
## 2.5*t
## PCLK
## ——ns
SCLK low time
## 1

## 2

## 3
t
## SCLK_LO
## 2.5*t
## PCLK
## ——ns
CS active to MISO
## 1

## 2
t
## CS_ACT_MI
## 25—96ns
CS disable to MISO
## 1

## 2
t
## CS_DIS_MI
## 24—87ns
MOSI setup time
## 1

## 2
t
## SU_MO
## 13——ns
MOSI hold time
## 1

## 2

## 3
t
## H_MO
## 14——ns
SCLK to MISO
## 1

## 2

## 3
t
## SCLK_MI
## 17 +
## 1.5*t
## PCLK
## —33 +
## 2.5*t
## PCLK
ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1 (figure only shows CLKPHA = 0).
2.Measurement done with 8 pF output loading at 10% and 90% of the I/O supply (figure shows 50%).
## 3.t
## PCLK
is one period of the selected PCLK.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  83

4.20  EUSART SPI Main Timing
## CS
## SCLK
## CLKPOL = 0
## MOSI
## MISO
t
## CS_MO
t
## H_MI
t
## SU_MI
t
## SCLK_MO
t
## SCLK
## SCLK
## CLKPOL = 1
Figure 4.5.  SPI Main Timing
4.20.1  EUSART SPI Main Timing, Voltage Scaling = VSCALE2
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD) on consecutive pins. All GPIO set to slew
rate = 6.
Table 4.44.  EUSART SPI Main Timing, Voltage Scaling = VSCALE2
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK period
## 1

## 2

## 3
t
## SCLK
t_CLK——ns
CS to MOSI
## 1

## 2
t
## CS_MO
## -10—9ns
SCLK to MOSI
## 1

## 2
t
## SCLK_MO
## -3—8ns
MISO setup time
## 1

## 2
t
## SU_MI
## 6——ns
MISO hold time
## 1

## 2
t
## H_MI
## -21——ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1.
2.Measurement done with 15 pF output loading at 10% and 90% of V
## DD
## .
## 3.t
## CLK
is one period of the selected peripheral clock: EM01GRPCCLK for EUSART1/2, EUSART0CLK for EUSART0.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  84

4.20.2  EUSART SPI Main Timing, Voltage Scaling = VSCALE1
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD) on consecutive pins. All GPIO set to slew
rate = 6.
Table 4.45.  EUSART SPI Main Timing, Voltage Scaling = VSCALE1
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK period
## 1

## 2

## 3
t
## SCLK
t_CLK——ns
CS to MOSI
## 1

## 2
t
## CS_MO
## -19—15ns
SCLK to MOSI
## 1

## 2
t
## SCLK_MO
## -6—13ns
MISO setup time
## 1

## 2
t
## SU_MI
## 10——ns
MISO hold time
## 1

## 2
t
## H_MI
## -13——ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1.
2.Measurement done with 15 pF output loading at 10% and 90% of V
## DD
## .
## 3.t
## CLK
is one period of the selected peripheral clock: EM01GRPCCLK for EUSART1/2, EUSART0CLK for EUSART0.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  85

4.21  EUSART SPI Secondary Timing
## CS
## SCLK
## CLKPOL = 0
## MOSI
## MISO
t
## CS_ACT_MI
t
## SCLK_HI
t
## SCLK
t
## SU_MO
t
## H_MO
t
## SCLK_MI
t
## CS_DIS_MI
t
## SCLK_LO
## SCLK
## CLKPOL = 1
Figure 4.6.  SPI Secondary Timing
4.21.1  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE2
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD) on consecutive pins. All GPIO set to slew
rate = 6.
Table 4.46.  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE2
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK high time
## 1

## 2
t
## SCLK_HI
## 50——ns
SCLK low time
## 1

## 2
t
## SCLK_LO
## 50——ns
CS active to MISO
## 1

## 2
t
## CS_ACT_MI
## 4—49ns
CS disable to MISO
## 1

## 2
t
## CS_DIS_MI
## 5—34ns
MOSI setup time
## 1

## 2
t
## SU_MO
## 5——ns
MOSI hold time
## 1

## 2
t
## H_MO
## 6——ns
SCLK to MISO
## 1

## 2
t
## SCLK_MI
IOVDD = 1.8 V8—40ns
IOVDD = 3.0 V8—30ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1 (figure only shows CLKPHA = 0).
2.Measurement done with 15 pF output loading at 10% and 90% of V
## DD
(figure shows 50% of V
## DD
## ).

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  86

4.21.2  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE1
Timing specifications are for all SPI signals routed to the same DBUS (DBUSAB or DBUSCD) on consecutive pins. All GPIO set to slew
rate = 6.
Table 4.47.  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE1
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK high time
## 1

## 2
t
## SCLK_HI
## 50——ns
SCLK low time
## 1

## 2
t
## SCLK_LO
## 50——ns
CS active to MISO
## 1

## 2
t
## CS_ACT_MI
## 6—75ns
CS disable to MISO
## 1

## 2
t
## CS_DIS_MI
## 5—56ns
MOSI setup time
## 1

## 2
t
## SU_MO
## 4——ns
MOSI hold time
## 1

## 2
t
## H_MO
## 6——ns
SCLK to MISO
## 1

## 2
t
## SCLK_MI
IOVDD = 1.8 V9—49ns
IOVDD = 3.0 V9—41ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1 (figure only shows CLKPHA = 0).
2.Measurement done with 15 pF output loading at 10% and 90% of V
## DD
(figure shows 50% of V
## DD
## ).

4.21.3  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE0
Timing specifications at VSCALE0 apply to EUSART0 only, routed to DBUSAB on consecutive pins. All GPIO set to slew rate = 6.
Table 4.48.  EUSART SPI Secondary Timing, Voltage Scaling = VSCALE0
ParameterSymbolTest ConditionMinTypMaxUnit
SCLK high time
## 1

## 2
t
## SCLK_HI
## 100——ns
SCLK low time
## 1

## 2
t
## SCLK_LO
## 100——ns
CS active to MISO
## 1

## 2
t
## CS_ACT_MI
## 8—100ns
CS disable to MISO
## 1

## 2
t
## CS_DIS_MI
## 7—70ns
MOSI setup time
## 1

## 2
t
## SU_MO
## 9——ns
MOSI hold time
## 1

## 2
t
## H_MO
## 32——ns
SCLK to MISO
## 1

## 2
t
## SCLK_MI
IOVDD = 1.8 V11—86ns
IOVDD = 3.0 V11—78ns
## Note:
1.Applies for both CLKPHA = 0 and CLKPHA = 1 (figure only shows CLKPHA = 0).
2.Measurement done with 15 pF output loading at 10% and 90% of V
## DD
(figure shows 50% of V
## DD
## ).

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  87

4.22  I2C Electrical Specifications
4.22.1  I2C Standard-mode (Sm)
CLHR set to 0 in the I2Cn_CTRL register.
Table 4.49.  I2C Standard-mode (Sm)
ParameterSymbolTest ConditionMinTypMaxUnit
SCL clock frequency
## 1
f
## SCL
0—100kHz
SCL clock low timet
## LOW
## 4.7——μs
SCL clock high timet
## HIGH
## 4——μs
SDA set-up timet
## SU_DAT
## 250——ns
SDA hold timet
## HD_DAT
## 0——ns
Repeated START condition
set-up time
t
## SU_STA
## 4.7——μs
Repeated START condition
hold time
t
## HD_STA
## 4.0——μs
STOP condition set-up timet
## SU_STO
## 4.0——μs
Bus free time between a
STOP and START condition
t
## BUF
## 4.7——μs
## Note:
1.The maximum SCL clock frequency listed is assuming that an arbitrary clock frequency is available. The maximum attainable
SCL clock frequency may be slightly less using the HFXO or HFRCO due to the limited frequencies available. The CLKDIV
should be set to a value that keeps the SCL clock frequency below the max value listed.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  88

4.22.2  I2C Fast-mode (Fm)
CLHR set to 1 in the I2Cn_CTRL register.
Table 4.50.  I2C Fast-mode (Fm)
ParameterSymbolTest ConditionMinTypMaxUnit
SCL clock frequency
## 1
f
## SCL
0—400kHz
SCL clock low timet
## LOW
## 1.3——μs
SCL clock high timet
## HIGH
## 0.6——μs
SDA set-up timet
## SU_DAT
## 100——ns
SDA hold timet
## HD_DAT
## 0——ns
Repeated START condition
set-up time
t
## SU_STA
## 0.6——μs
Repeated START condition
hold time
t
## HD_STA
## 0.6——μs
STOP condition set-up timet
## SU_STO
## 0.6——μs
Bus free time between a
STOP and START condition
t
## BUF
## 1.3——μs
## Note:
1.The maximum SCL clock frequency listed is assuming that an arbitrary clock frequency is available. The maximum attainable
SCL clock frequency may be slightly less using the HFXO or HFRCO due to the limited frequencies available. The CLKDIV
should be set to a value that keeps the SCL clock frequency below the max value listed.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  89

4.22.3  I2C Fast-mode Plus (Fm+)
CLHR set to 1 in the I2Cn_CTRL register.
Table 4.51.  I2C Fast-mode Plus (Fm+)
ParameterSymbolTest ConditionMinTypMaxUnit
SCL clock frequency
## 1
f
## SCL
0—1000kHz
SCL clock low timet
## LOW
## 0.5——μs
SCL clock high timet
## HIGH
## 0.26——μs
SDA set-up timet
## SU_DAT
## 50——ns
SDA hold timet
## HD_DAT
## 0——ns
Repeated START condition
set-up time
t
## SU_STA
## 0.26——μs
Repeated START condition
hold time
t
## HD_STA
## 0.26——μs
STOP condition set-up timet
## SU_STO
## 0.26——μs
Bus free time between a
STOP and START condition
t
## BUF
## 0.5——μs
## Note:
1.The maximum SCL clock frequency listed is assuming that an arbitrary clock frequency is available. The maximum attainable
SCL clock frequency may be slightly less using the HFXO or HFRCO due to the limited frequencies available. The CLKDIV
should be set to a value that keeps the SCL clock frequency below the max value listed.

## 4.23  Boot Timing
Secure boot impacts the recovery time from all sources of device reset. In addition to the root code authentication process, which can-
not be disabled or bypassed, the root code can authenticate a bootloader, and the bootloader can authenticate the application. In
projects that include only an application and no bootloader, the root code can authenticate the application directly. The duration of each
authentication operation depends on two factors: the computation of the associated image hash, which is proportional to the size of the
image, and the verification of the image signature, which is independent of image size.
The duration for the root code to authenticate the bootloader will depend on the SE firmware version as well as on the size of the boot-
loader.
The duration for the bootloader to authenticate the application can depend on the size of the application.
The configurations below assume that the associated bootloader and application code images do not contain a bootloader certificate or
an application certificate. Authenticating a bootloader certificate or an application certificate will extend the boot time by an additional 6
to 7 ms.
The table below provides the durations from the termination of reset until the completion of the secure boot process (start of main()
function in the application image) under various conditions.
## Conditions:
- SE firmware version 2.1.5
- Gecko Bootloader size 10.2 KB
Timing is expected to be similar for subsequent SE firmware versions. Refer to SE firmware release notes for any significant changes.
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  90

## Table 4.52.  Boot Timing
ParameterSymbolTest ConditionMinTypMaxUnit
Boot timet
## BOOT
Secure boot application check dis-
abled, no bootloader
## —30.5—ms
Secure boot application check dis-
abled, second stage bootloader
check enabled, 50 kB application
size
## —37.7—ms
Secure boot application check en-
abled, second stage bootloader
check enabled, 50 kB application
size
## —48.3—ms
Secure boot application check en-
abled, second stage bootloader
check enabled, 150 kB application
size
## —51.0—ms
Secure boot application check en-
abled, second stage bootloader
check enabled, 350 kB application
size
## —56.4—ms
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  91

4.24  Crypto Operation Timing for SE Manager API
Values in this table represent timing from SE Manager API call to return. The Cortex-M33 HCLK frequency is 39.0 MHz. The timing
specifications below are measured at the SE Manager function call API. Each duration in the table contains some portion that is influ-
enced by SE Manager build compilation and Cortex-M33 operating frequency and some portion that is influenced by the Hardware Se-
cure Engine's firmware version and its operating speed (typically 80 MHz). The contributions of the Cortex-M33 properties to the overall
specification timing are most pronounced for the shorter operations such as AES and hash when operating on small payloads. The
overhead of command processing at the mailbox interface can also dominate the timing for shorter operations.
## Conditions:
- SE firmware version 2.1.5
- GSDK version 3.2
Timing is expected to be similar for subsequent SE firmware versions. Refer to SE firmware release notes for any significant changes.
Table 4.53.  Crypto Operation Timing for SE Manager API
ParameterSymbolTest ConditionMinTypMaxUnit
AES-128 timingt
## AES128
AES-128 CCM encryption, PT 1
kB
## —571—μs
AES-128 CCM encryption, PT 32
kB
## —1751—μs
AES-128 CTR encryption, PT 1
kB
## —474—μs
AES-128 CTR encryption, PT 32
kB
## —1043—μs
AES-128 GCM encryption, PT 1
kB
## —522—μs
AES-128 GCM encryption, PT 32
kB
## —1087—μs
AES-256 timingt
## AES256
AES-256 CCM encryption, PT 1
kB
## —585—μs
AES-256 CCM encryption, PT 32
kB
## —2184—μs
AES-256 CTR encryption, PT 1
kB
## —482—μs
AES-256 CTR encryption, PT 32
kB
## —1255—μs
AES-256 GCM encryption, PT 1
kB
## —529—μs
AES-256 GCM encryption, PT 32
kB
## —1306—μs
ECC P-256 timingt
## ECC_P256
ECC key generation, P-256—5.5—ms
ECC signing, P-256—5.9—ms
ECC verification, P-256—6.2—ms
ECC P-521 timing
## 1
t
## ECC_P521
ECC key generation, P-521—30.2—ms
ECC signing, P-521—31.0—ms
ECC verification, P-521—36.2—ms
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  92

ParameterSymbolTest ConditionMinTypMaxUnit
ECC P-25519 timing
## 2
t
## ECC_P25519
ECC key generation, P-25519—4.5—ms
ECC signing, P-25519—8.9—ms
ECC verification, P-25519—6.3—ms
ECDH compute secret timingt
## ECDH
ECDH compute secret, P-521
## 1
## —30.5—ms
ECDH compute secret, P-25519
## 2
## —4.5—ms
ECDH compute secret, P-256—5.6—ms
ECJPAKE client timingt
## ECJPAKE_C
ECJPAKE client write round one—21.4—ms
ECJPAKE client read round one—11.7—ms
ECJPAKE client write round two—15.2—ms
ECJPAKE client read round two—6.3—ms
ECJPAKE client derive secret—8.8—ms
ECJPAKE server timingt
## ECJPAKE_S
ECJPAKE server write round one—21.4—ms
ECJPAKE server read round one—11.7—ms
ECJPAKE server write round two—15.3—ms
ECJPAKE server read round two—6.4—ms
ECJPAKE server derive secret—8.8—ms
POLY-1305 timing
## 1
t
## POLY1305
POLY-1305, PT 1 kB—514—μs
POLY-1305, PT 32 kB—1177—μs
SHA-256 timingt
## SHA256
SHA-256, PT 1 kB—308—μs
SHA-256, PT 32 kB—737—μs
SHA-512 timing
## 1
t
## SHA512
SHA-512, PT 1 kB—305—μs
SHA-512, PT 32 kB—620—μs
## Note:
1.Option is only available on OPNs with Secure Vault High feature set.
2.Option is not available on Secure Vault Mid devices with SE firmware earlier than v2.1.7.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  93

4.25  Crypto Operation Average Current for SE Manager API
Values in this table represent current consumed by security core during the operation, and represent additions to the current consumed
by the Cortex-M33 application CPU due to the Hardware Secure Engine CPU and its associated crypto accelerators. The current meas-
urements below represent the average value of the current for the duration of the crypto operation. Instantaneous peak currents may be
higher.
## Conditions:
- SE firmware version 2.1.5
- GSDK version 3.2
Current consumption is expected to be similar for subsequent SE firmware versions. Refer to SE firmware release notes for any signifi-
cant changes.
Table 4.54.  Crypto Operation Average Current for SE Manager API
ParameterSymbolTest ConditionMinTypMaxUnit
AES-128 currentI
## AES128
AES-128 CCM encryption, PT 1
kB
—0.9—mA
AES-128 CCM encryption, PT 32
kB
—3.9—mA
AES-128 CTR encryption, PT 1
kB
—0.8—mA
AES-128 CTR encryption, PT 32
kB
—3.8—mA
AES-128 GCM encryption, PT 1
kB
—0.8—mA
AES-128 GCM encryption, PT 32
kB
—3.8—mA
AES-256 currentI
## AES256
AES-256 CCM encryption, PT 1
kB
—1.0—mA
AES-256 CCM encryption, PT 32
kB
—4.0—mA
AES-256 CTR encryption, PT 1
kB
—0.8—mA
AES-256 CTR encryption, PT 32
kB
—4.0—mA
AES-256 GCM encryption, PT 1
kB
—0.8—mA
AES-256 GCM encryption, PT 32
kB
—3.9—mA
ECC P-256 currentI
## ECCP256
ECC key generation, P-256—1.7—mA
ECC signing, P-256—1.6—mA
ECC verification, P-256—1.7—mA
ECC P-521 current
## 1
## I
## ECCP521
ECC key generation, P-521—1.8—mA
ECC signing, P-521—1.8—mA
ECC verification, P-521—1.8—mA
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  94

ParameterSymbolTest ConditionMinTypMaxUnit
ECC P-25519 current
## 2
## I
## ECCP25519
ECC key generation, P-25519—1.6—mA
ECC signing, P-25519—1.6—mA
ECC verification, P-25519—1.6—mA
ECDH compute secret cur-
rent
## I
## ECDH
ECDH compute secret, P-521
## 1
—1.8—mA
ECDH compute secret, P-25519
## 2
—1.5—mA
ECDH compute secret, P-256—1.6—mA
ECJPAKE client currentI
## ECJPAKE_C
ECJPAKE client write round one—1.7—mA
ECJPAKE client read round one—1.7—mA
ECJPAKE client write round two—1.7—mA
ECJPAKE client read round two—1.7—mA
ECJPAKE client derive secret—1.7—mA
ECJPAKE server currentI
## ECJPAKE_S
ECJPAKE server write round one—1.7—mA
ECJPAKE server read round one—1.7—mA
ECJPAKE server write round two—1.7—mA
ECJPAKE server read round two—1.6—mA
ECJPAKE server derive secret—1.7—mA
POLY-1305 current
## 1
## I
## POLY1305
POLY-1305, PT 1 kB—0.6—mA
POLY-1305, PT 32 kB—1.6—mA
SHA-256 currentI
## SHA256
SHA-256, PT 1 kB—0.7—mA
SHA-256, PT 32 kB—2.3—mA
SHA-512 current
## 1
## I
## SHA512
SHA-512, PT 1 kB—0.7—mA
SHA-512, PT 32 kB—1.9—mA
## Note:
1.Option is only available on OPNs with Secure Vault High feature set.
2.Option is not available on Secure Vault Mid devices with SE firmware earlier than v2.1.7.

EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  95

4.26  Matrix Vector Processor (MVP)
All measurements are in comparison to EM1 baseline current at given VSCALE and Clock settings. Matrix dimensions are X = 24 x 32,
Y = 32 x 24 and Z = 24 x 24.
Table 4.55.  Matrix Vector Processor (MVP)
ParameterSymbolTest ConditionMinTypMaxUnit
MVP Enable CurrentI
## EN
VSCALE1, HFXO @ 39 MHz—16.8—μA
VSCALE2, HFRCO w/ DPLL @
78 MHz
—41.2—μA
## Matrix Multiplication Duration
using MVP Hardware
## T
## MVP_MULTIPLY
16-bit complex numbers, fully
banked memory, VSCALE1,
HFXO @ 39 MHz
## —504.0—μs
16-bit complex numbers, fully
banked memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
## —252.0—μs
16-bit complex numbers, inter-
leaved memory, VSCALE1, HFXO
@ 39 MHz
## —596.2—μs
16-bit complex numbers, inter-
leaved memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
## —298.2—μs
## Matrix Multiplication Duration
using Software
## T
## SW_MULTIPLY
16-bit complex numbers, inter-
leaved memory, VSCALE1, HFXO
@ 39 MHz
## —41.0—ms
16-bit complex numbers, inter-
leaved memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
## —20.5—ms
32-bit complex numbers, inter-
leaved memory, VSCALE1, HFXO
@ 39 MHz
## —19.7—ms
32-bit complex numbers, inter-
leaved memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
## —9.9—ms
## Matrix Multiplication Current
using MVP Hardware
## I
## MVP_MULTIPLY
16-bit complex numbers, fully
banked memory, VSCALE1,
HFXO @ 39 MHz
—61.3—μA/MHz
16-bit complex numbers, fully
banked memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
—62.4—μA/MHz
16-bit complex numbers, inter-
leaved memory, VSCALE1, HFXO
@ 39 MHz
—53.2—μA/MHz
16-bit complex numbers, inter-
leaved memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
—53.8—μA/MHz
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  96

ParameterSymbolTest ConditionMinTypMaxUnit
## Matrix Multiplication Current
using Software
## I
## SW_MULTIPLY
16-bit complex numbers, inter-
leaved memory, VSCALE1, HFXO
@ 39 MHz
—41.7—μA/MHz
16-bit complex numbers, inter-
leaved memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
—44.7—μA/MHz
32-bit complex numbers, inter-
leaved memory, VSCALE1, HFXO
@ 39 MHz
—31.9—μA/MHz
32-bit complex numbers, inter-
leaved memory, VSCALE2,
HFRCO w/ DPLL @ 78 MHz
—33.8—μA/MHz
## 4.27  Typical Performance Curves
Typical performance curves indicate typical characterized performance under the stated conditions.
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  97

## 4.27.1  Supply Current
Figure 4.7.  EM0 and EM1 Typical Supply Current vs. Temperature
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  98

Figure 4.8.  EM2 and EM4 Typical Supply Current vs. Temperature
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  99

4.27.2  RF Characteristics
Figure 4.9.   2.4 GHz 20 dBm PA RF Transmitter Output Power (Pout=19.5 dBm)
Figure 4.10.  2.4 GHz 10 dBm PA RF Transmitter Output Power (Pout=10 dBm)
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  100

Figure 4.11.  2.4 GHz 0 dBm PA RF Transmitter Output Power (Pout=0 dBm)
Figure 4.12.  2.4 GHz 802.15.4 RF Receiver Sensitivity
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  101

Figure 4.13.  2.4 GHz BLE RF Receiver Sensitivity
4.27.3  DC-DC Converter
Performance characterized with Murata DFE2HCAH2R2MJ0 (L
## DCDC
= 2.2 uH ) and TDK CGA5L3X8R1C475K160AB (C
## DCDC
= 4.7 uF)
Figure 4.14.  DC-DC Efficiency
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  102

## 4.27.4  IADC
Typical performance is shown using 10 MHz ADC clock for fastest sampling speed and adjusting oversampling ratio (OSR).
Figure 4.15.  Typical ENOB vs. Oversampling Ratio
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  103

## 4.27.5  GPIO
Figure 4.16.  VOH and VOL vs. Load Current
EFR32MG24 Wireless SoC Family Data Sheet
## Electrical Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  104

## 5.  Typical Connections
## 5.1  Power
Typical power supply connections are shown in the following figures.
Note: PAVDD, RFVDD, AVDD, and IOVDD supply connections are flexible. They may be connected in other configurations or to exter-
nal supplies as long as the supply limits described in 4.1 Electrical Characteristics are met.

## Main
## Supply
## V
## DD
## VREGVDDAVDDIOVDD
## VREGSW
## VREGVSS
## DVDD
## DECOUPLE
## RFVDDPAVDD
## HFXTAL_I
## HFXTAL_O
## LFXTAL_I
## LFXTAL_O
## +
## –
## V
## DD
39.0 MHz or
40 MHz*
32.768 kHz
## (optional)
## C
## DECOUPLE
*40 MHz required for applications using Channel Sounding
Figure 5.1.   EFR32MG24 Typical Application Circuit: Direct Supply Configuration without DCDC
## Main
## Supply
## V
## DCDC
## V
## DD
## VREGVDDAVDDIOVDD
## VREGSW
## VREGVSS
## DVDD
## DECOUPLE
## RFVDDPAVDD
## HFXTAL_I
## HFXTAL_O
## LFXTAL_I
## LFXTAL_O
## +
## –
## L
## DCDC
## C
## DCDC
## C
## IN
## C
## DECOUPLE
39.0 MHz or
40 MHz*
*40 MHz required for applications using Channel Sounding
32.768 kHz
## (optional)
Figure 5.2.   EFR32MG24 Typical Application Circuit: DCDC Configuration, PAVDD and RFVDD from DCDC output, AVDD and
IOVDD from main supply
EFR32MG24 Wireless SoC Family Data Sheet
## Typical Connections
silabs.com | Building a more connected world.Rev. 1.2  |  105

## Main
## Supply
## V
## DCDC
## V
## DD
## VREGVDD
## VREGSW
## VREGVSS
## DVDD
## DECOUPLE
## RFVDDPAVDD
## HFXTAL_I
## HFXTAL_O
## LFXTAL_I
## LFXTAL_O
## +
## –
## L
## DCDC
## C
## DCDC
## C
## IN
## C
## DECOUPLE
39.0 MHz or
40 MHz*
32.768 kHz
## (optional)
## AVDDIOVDD
## V
## DCDC
*40 MHz required for applications using Channel Sounding
Figure 5.3.   EFR32MG24 Typical Application Circuit: DCDC Configuration with PAVDD, RFVDD, AVDD, and IOVDD from DCDC
output
## 5.2  Other Connections
Other components or connections may be required to meet the system-level requirements. Application Note AN0002.2: "EFR32 Wire-
less Gecko Series 2 Hardware Design Considerations" contains detailed information on these connections. Application Notes can be
accessed on the Silicon Labs website (www.silabs.com/32bit-appnotes).
EFR32MG24 Wireless SoC Family Data Sheet
## Typical Connections
silabs.com | Building a more connected world.Rev. 1.2  |  106

## 6.  Pin Definitions
6.1  QFN48 / Standard Device Pinout
Figure 6.1.  QFN48 / Standard Device Pinout
The following table provides package pin connections and general descriptions of pin functionality. For detailed information on the sup-
ported features for each GPIO pin, see 6.5 Alternate Function Table, 6.6 Analog Peripheral Connectivity, and 6.7 Digital Peripheral
## Connectivity.
Table 6.1.  QFN48 / Standard Device Pinout
Pin NamePin(s)DescriptionPin NamePin(s)Description
## PC001GPIOPC012GPIO
## PC023GPIOPC034GPIO
## PC045GPIOPC056GPIO
## PC067GPIOPC078GPIO
## PC089GPIOPC0910GPIO
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  107

Pin NamePin(s)DescriptionPin NamePin(s)Description
HFXTAL_I11High Frequency Crystal InputHFXTAL_O12High Frequency Crystal Output
RESETn13
Reset Pin. The RESETn pin is internally
pulled up to DVDD.
RFVDD14Radio power supply
RFVSS15Radio GroundRF2G4_IO162.4 GHz RF input/output
PAVDD17Power Amplifier (PA) power supplyPB0518GPIO
## PB0419GPIOPB0320GPIO
## PB0221GPIOPB0122GPIO
PB0023GPIOVREFN24Dedicated ADC VREF Negative Input
VREFP25Dedicated ADC VREF Positive InputPA0026GPIO
## PA0127GPIOPA0228GPIO
## PA0329GPIOPA0430GPIO
## PA0531GPIOPA0632GPIO
## PA0733GPIOPA0834GPIO
## PA0935GPIODECOUPLE36
Decouple output for on-chip voltage
regulator. An external decoupling ca-
pacitor is required at this pin.
VREGSW37DCDC regulator switching nodeVREGVDD38DCDC regulator input supply
VREGVSS39DCDC groundDVDD40Digital power supply
AVDD41Analog power supplyIOVDD42I/O power supply
## PD0543GPIOPD0444GPIO
## PD0345GPIOPD0246GPIO
## PD0147GPIOPD0048GPIO
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  108

6.2  QFN48 / ADC Device Pinout
Figure 6.2.  QFN48 / ADC Device Pinout
The following table provides package pin connections and general descriptions of pin functionality. For detailed information on the sup-
ported features for each GPIO pin, see 6.5 Alternate Function Table, 6.6 Analog Peripheral Connectivity, and 6.7 Digital Peripheral
## Connectivity.
Table 6.2.  QFN48 / ADC Device Pinout
Pin NamePin(s)DescriptionPin NamePin(s)Description
## PC001GPIOPC012GPIO
## PC023GPIOPC034GPIO
## PC045GPIOPC056GPIO
## PC067GPIOPC078GPIO
## PC089GPIOPC0910GPIO
HFXTAL_I11High Frequency Crystal InputHFXTAL_O12High Frequency Crystal Output
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  109

Pin NamePin(s)DescriptionPin NamePin(s)Description
RESETn13
Reset Pin. The RESETn pin is internally
pulled up to DVDD.
RFVDD14Radio power supply
RFVSS15Radio GroundRF2G4_IO162.4 GHz RF input/output
PAVDD17Power Amplifier (PA) power supplyPB0318GPIO
## PB0219GPIOPB0120GPIO
PB0021GPIOVREFN22Dedicated ADC VREF Negative Input
AIN323Dedicated ADC Input 3AIN224Dedicated ADC Input 2
AIN125Dedicated ADC Input 1AIN026Dedicated ADC Input 0
VREFP27Dedicated ADC VREF Positive InputPA0028GPIO
## PA0129GPIOPA0230GPIO
## PA0331GPIOPA0432GPIO
## PA0533GPIOPA0634GPIO
## PA0735GPIODECOUPLE36
Decouple output for on-chip voltage
regulator. An external decoupling ca-
pacitor is required at this pin.
VREGSW37DCDC regulator switching nodeVREGVDD38DCDC regulator input supply
VREGVSS39DCDC groundDVDD40Digital power supply
AVDD41Analog power supplyIOVDD42I/O power supply
## PD0543GPIOPD0444GPIO
## PD0345GPIOPD0246GPIO
## PD0147GPIOPD0048GPIO
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  110

6.3  QFN40 / Standard Device Pinout
Figure 6.3.  QFN40 / Standard Device Pinout
The following table provides package pin connections and general descriptions of pin functionality. For detailed information on the sup-
ported features for each GPIO pin, see 6.5 Alternate Function Table, 6.6 Analog Peripheral Connectivity, and 6.7 Digital Peripheral
## Connectivity.
Table 6.3.  QFN40 / Standard Device Pinout
Pin NamePin(s)DescriptionPin NamePin(s)Description
## PC001GPIOPC012GPIO
## PC023GPIOPC034GPIO
## PC045GPIOPC056GPIO
## PC067GPIOPC078GPIO
HFXTAL_I9High Frequency Crystal InputHFXTAL_O10High Frequency Crystal Output
RESETn11
Reset Pin. The RESETn pin is internally
pulled up to DVDD.
RFVDD12Radio power supply
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  111

Pin NamePin(s)DescriptionPin NamePin(s)Description
RFVSS13Radio GroundRF2G4_IO142.4 GHz RF input/output
PAVDD15Power Amplifier (PA) power supplyPB0416GPIO
## PB0317GPIOPB0218GPIO
## PB0119GPIOPB0020GPIO
## PA0021GPIOPA0122GPIO
## PA0223GPIOPA0324GPIO
## PA0425GPIOPA0526GPIO
## PA0627GPIOPA0728GPIO
## PA0829GPIODECOUPLE30
Decouple output for on-chip voltage
regulator. An external decoupling ca-
pacitor is required at this pin.
VREGSW31DCDC regulator switching nodeVREGVDD32DCDC regulator input supply
VREGVSS33DCDC groundDVDD34Digital power supply
AVDD35Analog power supplyIOVDD36I/O power supply
## PD0337GPIOPD0238GPIO
## PD0139GPIOPD0040GPIO
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  112

6.4  QFN40 / HFCLKOUT Device Pinout
Figure 6.4.  QFN40 / HFCLKOUT Device Pinout
The following table provides package pin connections and general descriptions of pin functionality. For detailed information on the sup-
ported features for each GPIO pin, see 6.5 Alternate Function Table, 6.6 Analog Peripheral Connectivity, and 6.7 Digital Peripheral
## Connectivity.
Table 6.4.  QFN40 / HFCLKOUT Device Pinout
Pin NamePin(s)DescriptionPin NamePin(s)Description
## PC001GPIOPC012GPIO
## PC023GPIOPC034GPIO
## PC045GPIOPC056GPIO
PC067GPIOHFCLKOUT8High Frequency Clock Output
HFXTAL_I9High Frequency Crystal InputHFXTAL_O10High Frequency Crystal Output
RESETn11
Reset Pin. The RESETn pin is internally
pulled up to DVDD.
RFVDD12Radio power supply
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  113

Pin NamePin(s)DescriptionPin NamePin(s)Description
RFVSS13Radio GroundRF2G4_IO142.4 GHz RF input/output
PAVDD15Power Amplifier (PA) power supplyPB0416GPIO
## PB0317GPIOPB0218GPIO
## PB0119GPIOPB0020GPIO
## PA0021GPIOPA0122GPIO
## PA0223GPIOPA0324GPIO
## PA0425GPIOPA0526GPIO
## PA0627GPIOPA0728GPIO
## PA0829GPIODECOUPLE30
Decouple output for on-chip voltage
regulator. An external decoupling ca-
pacitor is required at this pin.
VREGSW31DCDC regulator switching nodeVREGVDD32DCDC regulator input supply
VREGVSS33DCDC groundDVDD34Digital power supply
AVDD35Analog power supplyIOVDD36I/O power supply
## PD0337GPIOPD0238GPIO
## PD0139GPIOPD0040GPIO
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  114

## 6.5  Alternate Function Table
A wide selection of alternate functionality is available for multiplexing to various pins. The following table shows GPIO pins with support
for dedicated functions across the different package options.
Table 6.5.  GPIO Alternate Function Table
GPIOAlternate Functions
## QFN48 /
## Standard
## Package
## 1
## QFN48 / ADC
## Package
## 2
## QFN40 /
## Standard
## Package
## 3
## QFN40 /
## HFCLKOUT
## Package
## 4
PA00IADC0.VREFPYesYes
PA01GPIO.SWCLKYesYesYesYes
PA02GPIO.SWDIOYesYesYesYes
## PA03
GPIO.SWVYesYesYesYes
GPIO.TDOYesYesYesYes
GPIO.TRACEDATA0YesYesYesYes
## PA04
GPIO.TDIYesYesYesYes
GPIO.TRACECLKYesYesYesYes
## PA05
GPIO.TRACEDATA1YesYesYesYes
GPIO.EM4WU0YesYesYesYes
PA06GPIO.TRACEDATA2YesYesYesYes
PA07GPIO.TRACEDATA3YesYesYesYes
PB00VDAC0.VDAC_CH0_MAIN_OUTPUTYesYesYesYes
## PB01
GPIO.EM4WU3YesYesYesYes
VDAC0.VDAC_CH1_MAIN_OUTPUTYesYesYesYes
PB02VDAC1.VDAC_CH0_MAIN_OUTPUTYesYesYesYes
## PB03
GPIO.EM4WU4YesYesYesYes
VDAC1.VDAC_CH1_MAIN_OUTPUTYesYesYesYes
PC00GPIO.EM4WU6YesYesYesYes
PC01GPIO.EFP_TX_SDAYesYesYesYes
PC02GPIO.EFP_TX_SCLYesYesYesYes
## PC05
GPIO.EFP_INTYesYesYesYes
GPIO.EM4WU7YesYesYesYes
## PC06
GPIO.THMSW_ENYes
GPIO.THMSW_HALFSWITCHYes
## PC07
GPIO.EM4WU8YesYesYes
GPIO.THMSW_ENYes
GPIO.THMSW_HALFSWITCHYes
## PC09
GPIO.THMSW_ENYesYes
GPIO.THMSW_HALFSWITCHYesYes
PD00LFXO.LFXTAL_OYesYesYesYes
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  115

GPIOAlternate Functions
## QFN48 /
## Standard
## Package
## 1
## QFN48 / ADC
## Package
## 2
## QFN40 /
## Standard
## Package
## 3
## QFN40 /
## HFCLKOUT
## Package
## 4
## PD01
LFXO.LFXTAL_IYesYesYesYes
LFXO.LF_EXTCLKYesYesYesYes
PD02GPIO.EM4WU9YesYesYesYes
PD05GPIO.EM4WU10YesYes
## Note:
1.QFN48 / Standard Package includes OPNs EFR32MG24A010F1024IM48-B, EFR32MG24A010F1536IM48-B,
## EFR32MG24A020F1024IM48-B, EFR32MG24A020F1536IM48-B, EFR32MG24A410F1536IM48-B,
## EFR32MG24A420F1536IM48-B, EFR32MG24B010F1024IM48-B, EFR32MG24B010F1536IM48-B,
EFR32MG24B020F1024IM48-B, EFR32MG24B020F1536IM48-B, EFR32MG24B210F1536IM48-B, and
## EFR32MG24B220F1536IM48-B
2.QFN48 / ADC Package includes OPNs EFR32MG24A110F1024IM48-B, EFR32MG24B110F1536IM48-B,
EFR32MG24B120F1536IM48-B, and EFR32MG24B310F1536IM48-B
3.QFN40 / Standard Package includes OPNs EFR32MG24A010F1024IM40-B, EFR32MG24A010F1536IM40-B,
## EFR32MG24A020F1024IM40-B, EFR32MG24A020F1536IM40-B, EFR32MG24A410F1536IM40-B,
EFR32MG24A420F1536IM40-B, EFR32MG24B010F1536IM40-B, and EFR32MG24B020F1536IM40-B
4.QFN40 / HFCLKOUT Package includes OPN EFR32MG24A021F1024IM40-B

## 6.6  Analog Peripheral Connectivity
Many analog resources are routable and can be connected to numerous GPIO's. The table below indicates which peripherals are avail-
able on each GPIO port. When a differential connection is being used Positive inputs are restricted to the EVEN pins and Negative
inputs are restricted to the ODD pins. When a single ended connection is being used positive input is available on all pins. See the
device Reference Manual for more details on the ABUS and analog peripherals. Note that some functions may not be available on all
device variants.
Table 6.6.  ABUS Routing Table
PeripheralSignalPAPBPCPD
## EVENODDEVENODDEVENODDEVENODD
ACMP0ANA_NEGYesYesYesYesYesYesYesYes
ANA_POSYesYesYesYesYesYesYesYes
ACMP1ANA_NEGYesYesYesYesYesYesYesYes
ANA_POSYesYesYesYesYesYesYesYes
IADC0ANA_NEGYesYesYesYesYesYesYesYes
ANA_POSYesYesYesYesYesYesYesYes
## VDAC0VDAC_CH0_ABUS_OUT-
## PUT
YesYesYesYesYesYesYesYes
VDAC_CH1_ABUS_OUTYesYesYesYesYesYesYesYes
## VDAC1VDAC_CH0_ABUS_OUT-
## PUT
YesYesYesYesYesYesYesYes
VDAC_CH1_ABUS_OUTYesYesYesYesYesYesYesYes
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  116

## 6.7  Digital Peripheral Connectivity
Many digital resources are routable and can be connected to numerous GPIO's. The table below indicates which peripherals are availa-
ble on each GPIO port. Note that some functions may not be available on all device variants.
Table 6.7.  DBUS Routing Table
Peripheral.ResourcePORT
## PAPBPCPD
ACMP0.DIGOUTAvailableAvailableAvailableAvailable
ACMP1.DIGOUTAvailableAvailableAvailableAvailable
CMU.CLKIN0AvailableAvailable
CMU.CLKOUT0AvailableAvailable
CMU.CLKOUT1AvailableAvailable
CMU.CLKOUT2AvailableAvailable
EUSART0.CSAvailableAvailable
EUSART0.CTSAvailableAvailable
EUSART0.RTSAvailableAvailable
EUSART0.RXAvailableAvailable
EUSART0.SCLKAvailableAvailable
EUSART0.TXAvailableAvailable
EUSART1.CSAvailableAvailableAvailableAvailable
EUSART1.CTSAvailableAvailableAvailableAvailable
EUSART1.RTSAvailableAvailableAvailableAvailable
EUSART1.RXAvailableAvailableAvailableAvailable
EUSART1.SCLKAvailableAvailableAvailableAvailable
EUSART1.TXAvailableAvailableAvailableAvailable
FRC.DCLKAvailableAvailable
FRC.DFRAMEAvailableAvailable
FRC.DOUTAvailableAvailable
HFXO0.BUFOUT_REQ_IN_ASYNCAvailableAvailable
I2C0.SCLAvailableAvailableAvailableAvailable
I2C0.SDAAvailableAvailableAvailableAvailable
I2C1.SCLAvailableAvailable
I2C1.SDAAvailableAvailable
KEYSCAN.COL_OUT_0AvailableAvailableAvailableAvailable
KEYSCAN.COL_OUT_1AvailableAvailableAvailableAvailable
KEYSCAN.COL_OUT_2AvailableAvailableAvailableAvailable
KEYSCAN.COL_OUT_3AvailableAvailableAvailableAvailable
KEYSCAN.COL_OUT_4AvailableAvailableAvailableAvailable
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  117

Peripheral.ResourcePORT
## PAPBPCPD
KEYSCAN.COL_OUT_5AvailableAvailableAvailableAvailable
KEYSCAN.COL_OUT_6AvailableAvailableAvailableAvailable
KEYSCAN.COL_OUT_7AvailableAvailableAvailableAvailable
KEYSCAN.ROW_SENSE_0AvailableAvailable
KEYSCAN.ROW_SENSE_1AvailableAvailable
KEYSCAN.ROW_SENSE_2AvailableAvailable
KEYSCAN.ROW_SENSE_3AvailableAvailable
KEYSCAN.ROW_SENSE_4AvailableAvailable
KEYSCAN.ROW_SENSE_5AvailableAvailable
LETIMER0.OUT0AvailableAvailable
LETIMER0.OUT1AvailableAvailable
MODEM.ANT0AvailableAvailableAvailableAvailable
MODEM.ANT1AvailableAvailableAvailableAvailable
MODEM.ANT_ROLL_OVERAvailableAvailable
MODEM.ANT_RR0AvailableAvailable
MODEM.ANT_RR1AvailableAvailable
MODEM.ANT_RR2AvailableAvailable
MODEM.ANT_RR3AvailableAvailable
MODEM.ANT_RR4AvailableAvailable
MODEM.ANT_RR5AvailableAvailable
MODEM.ANT_SW_ENAvailableAvailable
MODEM.ANT_SW_USAvailableAvailable
MODEM.ANT_TRIGAvailableAvailable
MODEM.ANT_TRIG_STOPAvailableAvailable
MODEM.DCLKAvailableAvailable
MODEM.DINAvailableAvailable
MODEM.DOUTAvailableAvailable
PCNT0.S0INAvailableAvailable
PCNT0.S1INAvailableAvailable
PRS.ASYNCH0AvailableAvailable
PRS.ASYNCH1AvailableAvailable
PRS.ASYNCH2AvailableAvailable
PRS.ASYNCH3AvailableAvailable
PRS.ASYNCH4AvailableAvailable
PRS.ASYNCH5AvailableAvailable
PRS.ASYNCH6AvailableAvailable
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  118

Peripheral.ResourcePORT
## PAPBPCPD
PRS.ASYNCH7AvailableAvailable
PRS.ASYNCH8AvailableAvailable
PRS.ASYNCH9AvailableAvailable
PRS.ASYNCH10AvailableAvailable
PRS.ASYNCH11AvailableAvailable
PRS.ASYNCH12AvailableAvailable
PRS.ASYNCH13AvailableAvailable
PRS.ASYNCH14AvailableAvailable
PRS.ASYNCH15AvailableAvailable
PRS.SYNCH0AvailableAvailableAvailableAvailable
PRS.SYNCH1AvailableAvailableAvailableAvailable
PRS.SYNCH2AvailableAvailableAvailableAvailable
PRS.SYNCH3AvailableAvailableAvailableAvailable
RAC.LNAENAvailableAvailableAvailableAvailable
RAC.PAENAvailableAvailableAvailableAvailable
TIMER0.CC0AvailableAvailableAvailableAvailable
TIMER0.CC1AvailableAvailableAvailableAvailable
TIMER0.CC2AvailableAvailableAvailableAvailable
TIMER0.CDTI0AvailableAvailableAvailableAvailable
TIMER0.CDTI1AvailableAvailableAvailableAvailable
TIMER0.CDTI2AvailableAvailableAvailableAvailable
TIMER1.CC0AvailableAvailableAvailableAvailable
TIMER1.CC1AvailableAvailableAvailableAvailable
TIMER1.CC2AvailableAvailableAvailableAvailable
TIMER1.CDTI0AvailableAvailableAvailableAvailable
TIMER1.CDTI1AvailableAvailableAvailableAvailable
TIMER1.CDTI2AvailableAvailableAvailableAvailable
TIMER2.CC0AvailableAvailable
TIMER2.CC1AvailableAvailable
TIMER2.CC2AvailableAvailable
TIMER2.CDTI0AvailableAvailable
TIMER2.CDTI1AvailableAvailable
TIMER2.CDTI2AvailableAvailable
TIMER3.CC0AvailableAvailable
TIMER3.CC1AvailableAvailable
TIMER3.CC2AvailableAvailable
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  119

Peripheral.ResourcePORT
## PAPBPCPD
TIMER3.CDTI0AvailableAvailable
TIMER3.CDTI1AvailableAvailable
TIMER3.CDTI2AvailableAvailable
TIMER4.CC0AvailableAvailable
TIMER4.CC1AvailableAvailable
TIMER4.CC2AvailableAvailable
TIMER4.CDTI0AvailableAvailable
TIMER4.CDTI1AvailableAvailable
TIMER4.CDTI2AvailableAvailable
USART0.CLKAvailableAvailableAvailableAvailable
USART0.CSAvailableAvailableAvailableAvailable
USART0.CTSAvailableAvailableAvailableAvailable
USART0.RTSAvailableAvailableAvailableAvailable
USART0.RXAvailableAvailableAvailableAvailable
USART0.TXAvailableAvailableAvailableAvailable
EFR32MG24 Wireless SoC Family Data Sheet
## Pin Definitions
silabs.com | Building a more connected world.Rev. 1.2  |  120

-  QFN40 Package Specifications
7.1  QFN40 Package Dimensions
Figure 7.1.   QFN40 Package Drawing
EFR32MG24 Wireless SoC Family Data Sheet
QFN40 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  121

Table 7.1.   QFN40 Package Dimensions
DimensionMinTypMax
## A0.800.850.90
## A10.000.020.05
## A30.20 REF
b0.150.200.25
## D4.905.005.10
## E4.905.005.10
## D23.553.703.85
## E23.553.703.85
e0.40 BSC
## L0.300.400.50
## K0.20——
## R0.075——
aaa0.10
bbb0.07
ccc0.10
ddd0.05
eee0.08
fff0.10
## Note:
1.All dimensions shown are in millimeters (mm) unless otherwise noted.
2.Dimensioning and Tolerancing per ANSI Y14.5M-1994.
3.This drawing conforms to the JEDEC Solid State Outline MO-220, Variation VKKD-4.
4.Recommended card reflow profile is per the JEDEC/IPC J-STD-020 specification for Small Body Components.
5.Package external pad (epad) may have pin one chamfer.

EFR32MG24 Wireless SoC Family Data Sheet
QFN40 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  122

7.2  QFN40 PCB Land Pattern
Figure 7.2.   QFN40 PCB Land Pattern Drawing
Table 7.2.   QFN40 PCB Land Pattern Dimensions
DimensionTyp
## S14.25
## S4.25
## L13.85
## W13.85
e0.40
## W0.22
## L0.74
## R0.11
EFR32MG24 Wireless SoC Family Data Sheet
QFN40 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  123

DimensionTyp
## Note:
1.All dimensions shown are in millimeters (mm) unless otherwise noted.
2.This Land Pattern Design is based on the IPC-7351 guidelines.
3.A stainless steel, laser-cut and electro-polished stencil with trapezoidal walls should be used to assure good solder paste release.
4.The stencil thickness should be 0.101 mm (4 mils).
5.The ratio of stencil aperture to land pad size can be 1:1 for all perimeter pads.
6.A 3x3 array of 0.90 mm square openings on a 1.20 mm pitch can be used for the center ground pad.
7.A No-Clean, Type-3 solder paste is recommended.
8.The recommended card reflow profile is per the JEDEC/IPC J-STD-020 specification for Small Body Components.
9.Above notes and stencil design are shared as recommendations only. A customer or user may find it necessary to use
different parameters and fine tune their SMT process as required for their application and tooling.

7.3  QFN40 Package Marking
## PPPP
## PPPPPP
## TTTTTT
## YYWW
Figure 7.3.  QFN40 Package Marking
The package marking consists of:
- Line 1: PPPP – The product family codes (BG24 | MG24)
- Line 2: PPPPPP – The product option codes:
## • 1) Security ( A = Secure Vault Mid | B = Secure Vault High )
## • 2-4) Product Feature Codes
## • 5) Flash ( S = 768k | J = 1024k | V = 1536k )
- 6) Temperature grade (G = -40 to 85 °C | I = -40 to 125 °C )
- TTTTTT – A trace or manufacturing code. The first letter is the device revision.
- YY – The last 2 digits of the assembly year.
- WW – The 2-digit workweek when the device was assembled.
EFR32MG24 Wireless SoC Family Data Sheet
QFN40 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  124

-  QFN48 Package Specifications
8.1  QFN48 Package Dimensions
Figure 8.1.   QFN48 Package Drawing
EFR32MG24 Wireless SoC Family Data Sheet
QFN48 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  125

Table 8.1.   QFN48 Package Dimensions
DimensionMinTypMax
## A0.800.850.90
## A10.000.020.05
## A30.20 REF
b0.150.20.25
## D5.906.006.10
## E5.906.006.10
e0.40 BSC
## D24.154.304.45
## E24.154.304.45
## L0.300.40.50
## K0.20——
## R0.075——
aaa0.10
bbb0.07
ccc0.10
ddd0.05
eee0.08
fff0.10
## Note:
1.All dimensions shown are in millimeters (mm) unless otherwise noted.
2.Dimensioning and Tolerancing per ANSI Y14.5M-1994.
3.This drawing conforms to the JEDEC Outline MS-013, Variation AA.
4.Recommended reflow profile per JEDEC J-STD-020C specification for small body, lead-free components.

EFR32MG24 Wireless SoC Family Data Sheet
QFN48 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  126

8.2  QFN48 PCB Land Pattern
Figure 8.2.   QFN48 PCB Land Pattern Drawing
Table 8.2.   QFN48 PCB Land Pattern Dimensions
DimensionTyp
## L0.86
## W0.22
e0.40
## S5.01
## S15.01
## L14.45
## W14.45
## R0.11
EFR32MG24 Wireless SoC Family Data Sheet
QFN48 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  127

DimensionTyp
## Note:
1.All dimensions shown are in millimeters (mm) unless otherwise noted.
2.This Land Pattern Design is based on the IPC-7351 guidelines.
3.All metal pads are to be non-solder mask defined (NSMD). Clearance between the solder mask and the metal pad is to be 60 μm
minimum, all the way around the pad.
4.A stainless steel, laser-cut and electro-polished stencil with trapezoidal walls should be used to assure good solder paste release.
5.The stencil thickness should be 0.101 mm (4 mils).
6.The ratio of stencil aperture to land pad size can be 1:1 for all perimeter pads.
7.A 3x3 array of 1.10mm x 1.10mm openings on 1.30mm pitch should be used for the center ground pad.
8.A No-Clean, Type-3 solder paste is recommended.
9.The recommended card reflow profile is per the JEDEC/IPC J-STD-020 specification for Small Body Components.
10.Above notes and stencil design are shared as recommendations only. A customer or user may find it necessary to use
different parameters and fine tune their SMT process as required for their application and tooling.

8.3  QFN48 Package Marking
## PPPP
## PPPPPP
## TTTTTT
## YYWW
Figure 8.3.  QFN48 Package Marking
The package marking consists of:
- Line 1: PPPP – The product family codes (BG24 | MG24)
- Line 2: PPPPPP – The product option codes:
## • 1) Security ( A = Secure Vault Mid | B = Secure Vault High )
## • 2-4) Product Feature Codes
## • 5) Flash ( S = 768k | J = 1024k | V = 1536k )
- 6) Temperature grade (G = -40 to 85 °C | I = -40 to 125 °C )
- TTTTTT – A trace or manufacturing code. The first letter is the device revision.
- YY – The last 2 digits of the assembly year.
- WW – The 2-digit workweek when the device was assembled.
EFR32MG24 Wireless SoC Family Data Sheet
QFN48 Package Specifications
silabs.com | Building a more connected world.Rev. 1.2  |  128

## 9.  Revision History
## Revision 1.2
## October, 2024
- Added information to front page and key features.
- Added Channel Sounding and updated PRS channels in Features list.
- Added note for AEC-Q100 parts in Table 2.1 Ordering Information on page 3.
- Updated Typical Connections diagrams in the 5.1 Power section to add Channel Sounding note.
## Revision 1.1
## March, 2023
- Updated front page diagram colors to indicate LFRCO works down to EM4
- Corrected diagram in 3.13 Memory Map
- Clarified that Secure Debug is supported for both Secure Vault Mid and High in 3.8.4 Secure Debug with Lock/Unlock
- Updated Table 3.1 Secure Vault Features on page 13 comparison table to show that "Curve25519 (ECDH)" and "Ed25519 (EdD-
SA)" are available for Public Key Encryption for Secure Vault Mid if using SE firmware v2.1.7 or higher.
- Added BURAM to Table 3.2 Peripheral Power Subdomains on page 18 table
- 4.3 General Operating Conditions table changes:
- Added VREGVDD operating supply range specifications for "Bypass, 120mA load" case
- Added HCLK and SYSCLK frequency specifications for "VSCALE1, WS0" and "VSCALE1, WS1"
- Corrected condition for External Clock input specifications to add "IOVDD >= 2.7V"
- Table 4.12 Energy Mode Wake-up and Entry Times on page 45 table changes:
- Corrected EM4 Wakeup time TYP value from "21.7 μs" to "21.7 ms"
- 4.9.2.2 RF Receiver Characteristics for 802.15.4 DSSS-OQPSK in the 2.4 GHz Band table changes
- Changed condition for "Max usable receiver input level, 1% PER" to "Rx Max Strong Signal Input Level for 1% PER"
- 4.9.2.3 RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 1 Mbps Data Rate table changes
- Changed condition for "Max usable receiver input level" to "Rx Max Strong Signal Input Level for 0.1% BER"
- 4.9.1.4 RF Transmitter Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 2 Mbps Data Rate table changes
- Changed condition for "Max usable receiver input level" to "Rx Max Strong Signal Input Level for 0.1% BER"
- Corrected condition for In-band spurious emissions specification for Pout
## MAX
from "+/- 2 MHz" to " +/- 4 MHz"
- 4.9.2.5 RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 500 kbps Data Rate table changes
- Changed condition for "Max usable receiver input level" to "Rx Max Strong Signal Input Level for 0.1% BER"
- 4.9.2.6 RF Receiver Characteristics for Bluetooth Low Energy in the 2.4 GHz Band 125 kbps Data Rate table changes
- Changed condition for "Max usable receiver input level" to "Rx Max Strong Signal Input Level for 0.1% BER"
- 4.10.1 High Frequency Crystal Oscillator (HFXO) table changes:
- Updated footnotes to clarify that BLE and Zigbee can use either a 38.4 MHz or 39.0 MHz crystal.
- Added new footnote to indicate that a Simplicity Studio component may need to be updated based on the crystal frequency
- 4.13 Analog Comparator (ACMP) table changes:
- Added specification for "Current consumption from VSENSEDIV in sample/hold mode"
- 4.12 Analog to Digital Converter (IADC) table changes:
- Corrected Input Sampling Frequency, High-Accuracy Mode Typical value from "f
## ADC_CLK
/ 4" to "f
## ADC_CLK
## / 5"
- Added Gain Error, High-Accuracy Mode specs for all gain settings
- Added Offset Error, High-Accuracy Mode specs for all gain settings
- Added Spurious-Free Dynamic Range, High Accuracy Mode spec MIN value
- Added Current from all supplies, Continuous operation, High Accuracy Mode TYP and MAX values
- 4.23 Boot Timing table changes:
- Added condition "Second stage bootloader check enabled" for all specifications
- Added specification for "Secure boot application check disable, no bootloader"
EFR32MG24 Wireless SoC Family Data Sheet
## Revision History
silabs.com | Building a more connected world.Rev. 1.2  |  129

## Revision 1.0
## August, 2022
- Updated "TBD" values in 4.1 Electrical Characteristics tables
- Updates to System Overview Table 3.3 Configuration Summary on page 21 table
## • Added 4.27 Typical Performance Curves
- Corrected DCDC output capacitor part number in 4.4 DC-DC Converter
- Modified ACMP Electrical Specifications to show that hysteresis is only supported when BIAS >= 3 in 4.13 Analog Comparator
## (ACMP)
- Added "T=25°C" condition to all timing specs in 4.7 Flash Characteristics
## Revision 0.6
## April, 2022
- Updated values in 4.1 Electrical Characteristics tables
## Revision 0.5
## March, 2022
- Updated front page block diagram
- Updated typical power consumption and performance numbers and Protocol Support in 1. Feature List
- Updated values in 4.1 Electrical Characteristics tables
## • Updated 5. Typical Connections Diagrams
- Updated Package Marking diagrams for 40QFN and 48QFN packages
## Revision 0.4
## Jan 2022
- Updated front page content
- Updated typical power consumption numbers and Protocol Support in 1. Feature List
- Updated list of OPNs in table
- Removed -G temperature grade information from Electrical Specification tables
- Added typical EM2/3/4 current data to 4.6 Current Consumption tables
- Revised EM2/3 spec format in 4.6 Current Consumption tables
- Added Startup Time and Current Consumption data to 4.10.5 Precision Low Frequency RC Oscillator (LFRCO) table
## Revision 0.3
## Oct 2021
- Updated typical power consumption numbers in 1. Feature List
- Added IADC High Speed / High Accuracy information and Package Pinout information to table
- Updated IADC information in System Overview
- Corrected "RTCC" to "SYSRTC" in 1. Feature List
- Added 20-bit ADC resolution and sampling rate to 1. Feature List
- Added DALI function to EUSART in 1. Feature List
- In   Table 3.2 Peripheral Power Subdomains on page 18 table
- Removed "Tamper"
- Corrected "LFRCO" cells
- Removed "EUSART1"
- Removed footnote from FSRCO suggesting it operates in EM4
- Revised table to show PD0A and PDHV columns
- Revised section text content
- Removed AVDD/DVDD power mux from Figure 3.1 Detailed EFR32MG24 Block Diagram on page 9
EFR32MG24 Wireless SoC Family Data Sheet
## Revision History
silabs.com | Building a more connected world.Rev. 1.2  |  130

## Revision 0.2
## June 2021
In Electrical Specifications section:
- Added EUSART timing and specifications
- Renamed USART specifications
- Added RX Sensitivity Specifications
- Removed Wi-Fi Notch Filter from Electrical Specifications
## Revision 0.1
## March 2021
Initial release.
EFR32MG24 Wireless SoC Family Data Sheet
## Revision History
silabs.com | Building a more connected world.Rev. 1.2  |  131

## Silicon Laboratories Inc.
## 400 West Cesar Chavez
Austin, TX 78701
## USA
www.silabs.com
IoT Portfolio
www.silabs.com/IoT
## SW/HW
www.silabs.com/simplicity
## Quality
www.silabs.com/quality
## Support & Community
www.silabs.com/community
## Simplicity Studio
One-click access to MCU and wireless
tools, documentation, software,
source code libraries & more. Available
for Windows, Mac and Linux!
## Disclaimer
Silicon Labs intends to provide customers with the latest, accurate, and in-depth documentation of all peripherals and modules available for system and software imple-
menters using or intending to use the Silicon Labs products. Characterization data, available modules and peripherals, memory sizes and memory addresses refer to each
specific device, and “Typical” parameters provided can and do vary in different applications. Application examples described herein are for illustrative purposes only. Silicon
Labs reserves the right to make changes without further notice to the product information, specifications, and descriptions herein, and does not give warranties as to the
accuracy or completeness of the included information. Without prior notification, Silicon Labs may update product firmware during the manufacturing process for security or
reliabilit y reasons. Such changes will not alter the specifications or the per formance of the product. Silicon Labs shall have no liabilit y for the consequences of use of the infor-
mation supplied in this document. This document does not imply or expressly grant any license to design or fabricate any integrated circuits. The products are not designed or
authorized to be used within any FDA Class III devices, applications for which FDA premarket approval is required or Life Support Systems without the specific written consent
of Silicon Labs. A “Life Support System” is any product or system intended to support or sustain life and/or health, which, if it fails, can be reasonably expected to result in
significant personal injury or death. Silicon Labs products are not designed or authorized for military applications. Silicon Labs products shall under no circumstances be used
in weapons of mass destruction including (but not limited to) nuclear, biological or chemical weapons, or missiles capable of delivering such weapons. Silicon Labs disclaims
all express and implied warranties and shall not be responsible or liable for any injuries or damages related to use of a Silicon Labs product in such unauthorized applications.
## Trademark Information
## Silicon  Laboratories  Inc.
## ®
## ,  Silicon  Laboratories
## ®
## ,  Silicon  Labs
## ®
## ,
SiLabs
## ®
and  the  Silicon  Labs  logo
## ®
## ,  Bluegiga
## ®
## ,  Bluegiga  Logo
## ®
## ,  EFM
## ®
## ,  EFM32
## ®
,  EFR,  Ember
## ®
## ,  Energy  Micro,  Energy
Micro  logo  and  combinations  thereof,  “the  world’s  most  energy  friendly  microcontrollers”,  Redpine  Signals
## ®
,  WiSeConnect  ,  n-Link,  EZLink
## ®
,  EZRadio
## ®
,  EZRadioPRO
## ®
## , Gecko
## ®
## ,
Gecko  OS,  Gecko  OS  Studio,  Precision32
## ®
## ,  Simplicity  Studio
## ®
,  Telegesis,  the  Telegesis  Logo
## ®
## ,  USB
## Xpress
## ®
,  Zentri,  the  Zentri  logo  and  Zentri  DMS,  Z-Wave
## ®
, and  others are
trademarks  or  registered  trademarks  of  Silicon  Labs.  ARM,  CORTEX,  Cortex-M3  and  THUMB  are  trademarks  or  registered  trademarks  of  ARM  Holdings.  Keil  is  a  registered
trademark of ARM Limited. Wi-Fi is a registered trademark of the Wi-Fi Alliance. All other products or brand names mentioned herein are trademarks of their respective holders.