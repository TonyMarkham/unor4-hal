Datasheet
RA4M1 Group R01DS0355EJ0110
Rev.1.10
Datasheet Sep 29, 2023
High efficiency 48-MHz Arm® Cortex®-M4 core, 256-KB code flash memory, 32-KB SRAM, Segment LCD Controller,
Capacitive Touch Sensing Unit, USB 2.0 Full-Speed Module, 14-bit A/D Converter, 12-bit D/A Converter, security and
safety features
FFeeaattuurreess
■ Arm Cortex-M4 Core with Floating Point Unit (FPU) ■ System and Power Management
 Armv7E-M architecture with DSP instruction set  Low power modes
 Maximum operating frequency: 48 MHz  Realtime Clock (RTC) with calendar and Battery Backup support
 Support for 4-GB address space  Event Link Controller (ELC)
 Arm Memory Protection Unit (Arm MPU) with 8 regions  DMA Controller (DMAC) × 4
 Debug and Trace: ITM, DWT, FPB, TPIU, ETB  Data Transfer Controller (DTC)
 CoreSight™ Debug Port: JTAG-DP and SW-DP  Key Interrupt Function (KINT)
 Power-on reset
■ Memory
 Low Voltage Detection (LVD) with voltage settings
 256-KB code flash memory
 8-KB data flash memory (100,000 program/erase (P/E) cycles) ■ Security and Encryption
 32-KB SRAM  AES128/256
 Flash Cache (FCACHE)  GHASH
 Memory Protection Unit (MPU)  True Random Number Generator (TRNG)
 128-bit unique ID
■ Human Machine Interface (HMI)
■ Connectivity  Segment LCD Controller (SLCDC)
 USB 2.0 Full-Speed Module (USBFS) - Up to 38 segments × 4 commons
- On-chip transceiver with voltage regulator - Up to 34 segments × 8 commons
- Compliant with USB Battery Charging Specification 1.2  Capacitive Touch Sensing Unit (CTSU)
 Serial Communications Interface (SCI) × 4
■ Multiple Clock Sources
- UART
 Main clock oscillator (MOSC)
- Simple IIC
(1 to 20 MHz when VCC = 2.4 to 5.5 V)
- Simple SPI
(1 to 8 MHz when VCC = 1.8 to 2.4 V)
 Serial Peripheral Interface (SPI) × 2
(1 to 4 MHz when VCC = 1.6 to 1.8 V)
 I2C bus interface (IIC) × 2
 Sub-clock oscillator (SOSC) (32.768 kHz)
 Controller Area Network (CAN) module
 High-speed on-chip oscillator (HOCO)
 Serial Sound Interface Enhanced (SSIE)
(24, 32, 48, 64 MHz when VCC = 2.4 to 5.5 V)
■ Analog (24, 32, 48 MHz when VCC = 1.8 to 5.5 V)
 14-bit A/D Converter (ADC14) (24, 32 MHz when VCC = 1.6 to 5.5 V)
 12-bit D/A Converter (DAC12)  Middle-speed on-chip oscillator (MOCO) (8 MHz)
 8-bit D/A Converter (DAC8) ×2 (for ACMPLP)  Low-speed on-chip oscillator (LOCO) (32.768 kHz)
 Low-Power Analog Comparator (ACMPLP) × 2  IWDT-dedicated on-chip oscillator (15 kHz)
 Operational Amplifier (OPAMP) × 4  Clock trim function for HOCO/MOCO/LOCO
 Temperature Sensor (TSN)  Clock out support
■ Timers ■ General Purpose I/O Ports
 General PWM Timer 32-Bit (GPT32) × 2  Up to 84 input/output pins
 General PWM Timer 16-Bit (GPT16) × 6 - Up to 3 CMOS input
 Low Power Asynchronous General-Purpose Timer (AGT) × 2 - Up to 81 CMOS input/output
 Watchdog Timer (WDT) - Up to 9 input/output 5-V tolerant
- Up to 2 high current (20 mA)
■ Safety
 Error Correction Code (ECC) in SRAM ■ Operating Voltage
 SRAM parity error check  VCC: 1.6 to 5.5 V
 Flash area protection ■ Operating Temperature and Packages
 ADC self-diagnosis function
 Ta = -40°C to +85°C
 Clock Frequency Accuracy Measurement Circuit (CAC)
- 100-pin LGA (7 mm × 7 mm, 0.65 mm pitch)
 Cyclic Redundancy Check (CRC) calculator
 Ta = -40°C to +105°C
 Data Operation Circuit (DOC)
- 100-pin LQFP (14 mm × 14 mm, 0.5 mm pitch)
 Port Output Enable for GPT (POEG)
- 64-pin LQFP (10 mm × 10 mm, 0.5 mm pitch)
 Independent Watchdog Timer (IWDT)
- 64-pin QFN (8 mm × 8 mm, 0.4 mm pitch)
 GPIO readback level detection
- 48-pin LQFP (7 mm × 7 mm, 0.5 mm pitch)
 Register write protection
- 48-pin QFN (7 mm × 7 mm, 0.5 mm pitch)
 Main oscillator stop detection
- 40-pin QFN (6 mm × 6 mm, 0.5 mm pitch)
 Illegal memory access
R01DS0355EJ0110 Rev.1.10 Page 1 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
1. Overview
The MCU integrates multiple series of software- and pin-compatible Arm®-based 32-bit cores that share a common set
of Renesas peripherals to facilitate design scalability and efficient platform-based product development.
The MCU provides an optimal combination of low-power, high-performance Arm Cortex®-M4 core running up to
48MHz with the following features:
 256-KB code flash memory
 32-KB SRAM
 Segment LCD Controller (SLCDC)
 Capacitive Touch Sensing Unit (CTSU)
 USB 2.0 Full-Speed Module (USBFS)
 14-bit A/D Converter (ADC14)
 12-bit D/A Converter (DAC12)
 Security features.
1.1 Function Outline
Table 1.1 Arm core
Feature Functional description
Arm Cortex-M4 core  Maximum operating frequency: up to 48 MHz
 Arm Cortex-M4 core
- Revision: r0p1-01rel0
- Armv7E-M architecture profile
- Single precision floating-point unit compliant with the ANSI/IEEE Std 754-2008.
 Arm Memory Protection Unit (Arm MPU)
- Armv7 Protected Memory System Architecture
- 8 protected regions.
 SysTick timer
- Driven by SYSTICCLK (LOCO) or ICLK.
Table 1.2 Memory
Feature Functional description
Code flash memory Maximum 256-KB code flash memory. See section 44, Flash Memory in Userʼs Manual.
Data flash memory 8-KB data flash memory. See section 44, Flash Memory in Userʼs Manual.
Option-setting memory The option-setting memory determines the state of the MCU after a reset. See section 6,
Option-Setting Memory in Userʼs Manual.
SRAM On-chip high-speed SRAM with either parity bit or Error Correction Code (ECC). An area in
SRAM0 provides error correction capability using ECC. See section 43, SRAM in Userʼs
Manual.
R01DS0355EJ0110 Rev.1.10 Page 2 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
Table 1.3 System (1 of 2)
Feature Functional description
Operating modes Two operating modes:
 Single-chip mode
 SCI/USB boot mode.
See section 3, Operating Modes in Userʼs Manual.
Resets 14 resets:
 RES pin reset
 Power-on reset
 VBATT-selected voltage power-on reset
 Independent watchdog timer reset
 Watchdog timer reset
 Voltage monitor 0 reset
 Voltage monitor 1 reset
 Voltage monitor 2 reset
 SRAM parity error reset
 SRAM ECC error reset
 Bus master MPU error reset
 Bus slave MPU error reset
 CPU stack pointer error reset
 Software reset.
See section 5, Resets in Userʼs Manual.
Low Voltage Detection (LVD) Low Voltage Detection (LVD) function monitors the voltage level input to the VCC pin, and the
detection level can be selected using a software program. See section 7, Low Voltage
Detection (LVD) in Userʼs Manual.
Clocks  Main clock oscillator (MOSC)
 Sub-clock oscillator (SOSC)
 High-speed on-chip oscillator (HOCO)
 Middle-speed on-chip oscillator (MOCO)
 Low-speed on-chip oscillator (LOCO)
 PLL frequency synthesizer
 IWDT-dedicated on-chip oscillator
 Clock out support.
See section 8, Clock Generation Circuit in Userʼs Manual.
Clock Frequency Accuracy The Clock Frequency Accuracy Measurement Circuit (CAC) counts pulses of the clock to be
Measurement Circuit (CAC) measured (measurement target clock) within the time generated by the clock to be used as a
measurement reference (measurement reference clock), and determines the accuracy
depending on whether the number of pulses is within the allowable range.
When measurement is complete or the number of pulses within the time generated by the
measurement reference clock is not within the allowable range, an interrupt request is
generated.
See section 9, Clock Frequency Accuracy Measurement Circuit (CAC) in Userʼs Manual.
Interrupt Controller Unit (ICU) The Interrupt Controller Unit (ICU) controls which event signals are linked to the NVIC/DTC
module and DMAC module. The ICU also controls NMI interrupts. See section 13, Interrupt
Controller Unit (ICU) in Userʼs Manual.
Key Interrupt Function (KINT) A key interrupt can be generated by setting the Key Return Mode Register (KRM) and inputting
a rising or falling edge to the key interrupt input pins. See section 20, Key Interrupt Function
(KINT) in Userʼs Manual.
Low power modes Power consumption can be reduced in multiple ways, such as by setting clock dividers,
stopping modules, selecting power control mode in normal operation, and transitioning to low
power modes. See section 10, Low Power Modes in Userʼs Manual.
Battery backup function A battery backup function is provided for partial powering by a battery. The battery powered
area includes RTC, SOSC, LOCO, wakeup control, backup memory, VBATT_R low voltage
detection, and switches between VCC and VBATT.
During normal operation, the battery powered area is powered by the main power supply,
which is the VCC pin. When a VCC voltage drop is detected, the power source is switched to
the dedicated battery backup power pin, the VBATT pin.
When the voltage rises again, the power source is switched from the VBATT pin to the VCC
pin. See section 11, Battery Backup Function in Userʼs Manual.
Register write protection The register write protection function protects important registers from being overwritten
because of software errors. See section 12, Register Write Protection in Userʼs Manual.
R01DS0355EJ0110 Rev.1.10 Page 3 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
Table 1.3 System (2 of 2)
Feature Functional description
Memory Protection Unit (MPU) Four Memory Protection Units (MPUs) and a CPU stack pointer monitor function are provided
for memory protection. See section 15, Memory Protection Unit (MPU) in Userʼs Manual.
Watchdog Timer (WDT) The Watchdog Timer (WDT) is a 14-bit down-counter. It can be used to reset the MCU when
the counter underflows because the system has run out of control and is unable to refresh the
WDT. In addition, a non-maskable interrupt or interrupt can be generated by an underflow. A
refresh-permitted period can be set to refresh the counter and used as the condition to detect
when the system runs out of control. See section 25, Watchdog Timer (WDT) in Userʼs
Manual.
Independent Watchdog Timer (IWDT) The Independent Watchdog Timer (IWDT) consists of a 14-bit down-counter that must be
serviced periodically to prevent counter underflow. It can be used to reset the MCU or to
generate a non-maskable interrupt/interrupt for a timer underflow. Because the timer operates
with an independent, dedicated clock source, it is particularly useful in returning the MCU to a
known state as a fail-safe mechanism when the system runs out of control. The IWDT can be
triggered automatically on a reset, underflow, refresh error, or by a refresh of the count value in
the registers. See section 26, Independent Watchdog Timer (IWDT) in Userʼs Manual.
Table 1.4 Event link
Feature Functional description
Event Link Controller (ELC) The Event Link Controller (ELC) uses the interrupt requests generated by various peripheral
modules as event signals to connect them to different modules, enabling direct interaction
between the modules without CPU intervention. See section 18, Event Link Controller (ELC) in
Userʼs Manual.
Table 1.5 Direct memory access
Feature Functional description
Data Transfer Controller (DTC) A Data Transfer Controller (DTC) module is provided for transferring data when activated by an
interrupt request. See section 17, Data Transfer Controller (DTC) in Userʼs Manual.
DMA Controller (DMAC) A 4-channel DMA Controller (DMAC) module is provided for transferring data without the CPU.
When a DMA transfer request is generated, the DMAC transfers data stored at the transfer
source address to the transfer destination address. See section 16, DMA Controller (DMAC) in
Userʼs Manual.
R01DS0355EJ0110 Rev.1.10 Page 4 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
Table 1.6 Timers
Feature Functional description
General PWM Timer (GPT) The General PWM Timer (GPT) is a 32-bit timer with 2 channels and a 16-bit timer with 6
channels. PWM waveforms can be generated by controlling the up-counter, down-counter, or
the up- and down-counter. In addition, PWM waveforms can be generated for controlling
brushless DC motors. The GPT can also be used as a general-purpose timer. See section 22,
General PWM Timer (GPT) in Userʼs Manual.
Port Output Enable for GPT (POEG) Use the Port Output Enable for GPT (POEG) function to place the General PWM Timer (GPT)
output pins in the output disable state. See section 21, Port Output Enable for GPT (POEG) in
Userʼs Manual.
Low Power Asynchronous General The Low Power Asynchronous General Purpose Timer (AGT) is a 16-bit timer that can be used
Purpose Timer (AGT) for pulse output, external pulse width or period measurement, and counting of external events.
This 16-bit timer consists of a reload register and a down-counter. The reload register and the
down-counter are allocated to the same address, and they can be accessed with the AGT
register. See section 23, Low Power Asynchronous General Purpose Timer (AGT) in Userʼs
Manual.
Realtime Clock (RTC) The Realtime Clock (RTC) has two counting modes, calendar count mode and binary count
mode, that are controlled by the register settings.
For calendar count mode, the RTC has a 100-year calendar from 2000 to 2099 and
automatically adjusts dates for leap years.
For binary count mode, the RTC counts seconds and retains the information as a serial value.
Binary count mode can be used for calendars other than the Gregorian (Western) calendar.
See section 24, Realtime Clock (RTC) in Userʼs Manual.
Table 1.7 Communication interfaces (1 of 2)
Feature Functional description
Serial Communications Interface The Serial Communications Interface (SCI) is configurable to five asynchronous and
(SCI) synchronous serial interfaces:
 Asynchronous interfaces (UART and asynchronous communications interface adapter
(ACIA))
 8-bit clock synchronous interface
 Simple IIC (master-only)
 Simple SPI
 Smart card interface.
The smart card interface complies with the ISO/IEC 7816-3 standard for electronic signals and
transmission protocol.
SCI0 and SCI1 have FIFO buffers to enable continuous and full-duplex communication, and
the data transfer speed can be configured independently using an on-chip baud rate generator.
See section 28, Serial Communications Interface (SCI) in Userʼs Manual.
I2C Bus Interface (IIC) The 3-channel I2C Bus Interface (IIC) module conforms with and provides a subset of the NXP
I2C bus (Inter-Integrated Circuit bus) interface functions. See section 29, I2C Bus Interface
(IIC) in Userʼs Manual.
Serial Peripheral Interface (SPI) Two independent Serial Peripheral Interface (SPI) channels are capable of high-speed, full-
duplex synchronous serial communications with multiple processors and peripheral devices.
See section 31, Serial Peripheral Interface (SPI) in Userʼs Manual.
Serial Sound Interface Enhanced The Serial Sound Interface Enhanced (SSIE) peripheral provides functionality to interface with
(SSIE) digital audio devices for transmitting PCM audio data over a serial bus with the MCU. The
SSIE supports an audio clock frequency of up to 50 MHz, and can be operated as a slave or
master receiver, transmitter, or transceiver to suit various applications. The SSIE includes 8-
stage FIFO buffers in the receiver and transmitter, and supports interrupts and DMA-driven
data reception and transmission. See section 33, Serial Sound Interface Enhanced (SSIE) in
Userʼs Manual.
Controller Area Network (CAN) The Controller Area Network (CAN) module provides functionality to receive and transmit data
module using a message-based protocol between multiple slaves and masters in electromagnetically
noisy applications.
The CAN module complies with the ISO 11898-1 (CAN 2.0A/CAN 2.0B) standard and supports
up to 32 mailboxes, which can be configured for transmission or reception in normal mailbox
and FIFO modes. Both standard (11-bit) and extended (29-bit) messaging formats are
supported. See section 30, Controller Area Network (CAN) Module in Userʼs Manual.
R01DS0355EJ0110 Rev.1.10 Page 5 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
Table 1.7 Communication interfaces (2 of 2)
Feature Functional description
USB 2.0 Full-Speed Module (USBFS) The USB 2.0 Full-Speed Module (USBFS) can operate as a host controller or device controller.
The module supports full-speed and low-speed (only for the host controller) transfer as defined
in the Universal Serial Bus Specification 2.0. The module has an internal USB transceiver and
supports all of the transfer types defined in the Universal Serial Bus Specification 2.0. The USB
has buffer memory for data transfer, providing a maximum of 10 pipes. Pipes 1 to 9 can be
assigned any endpoint number based on the peripheral devices used for communication or
based on the user system. The MCU supports revision 1.2 of the Battery Charging
specification. Because the MCU can be powered at 5 V, the USB LDO regulator provides the
internal USB transceiver power supply at 3.3 V. See section 27, USB 2.0 Full-Speed Module
(USBFS) in Userʼs Manual.
Table 1.8 Analog
Feature Functional description
14-bit A/D Converter (ADC14) A 14-bit successive approximation A/D converter is provided. Up to 25 analog input channels
are selectable. Temperature sensor output and internal reference voltage are selectable for
conversion. The A/D conversion accuracy is selectable from 12-bit and 14-bit conversion
making it possible to optimize the tradeoff between speed and resolution in generating a digital
value. See section 35, 14-Bit A/D Converter (ADC14) in Userʼs Manual.
12-Bit D/A Converter (DAC12) The 12-Bit D/A Converter (DAC12) converts data and includes an output amplifier. See section
36, 12-Bit D/A Converter (DAC12) in Userʼs Manual.
8-Bit D/A Converter (DAC8) The 8-Bit D/A Converter (DAC8) converts data and does not include an output amplifier
for ACMPLP (DAC8). The DAC8 is used only as the reference voltage for ACMPLP. See section 40, 8-Bit D/
A Converter (DAC8) in Userʼs Manual.
Temperature Sensor (TSN) The on-chip Temperature Sensor (TSN) determines and monitors the die temperature for
reliable operation of the device. The sensor outputs a voltage directly proportional to the die
temperature, and the relationship between the die temperature and the output voltage is linear.
The output voltage is provided to the ADC14 for conversion and can be further used by the end
application. See section 37, Temperature Sensor (TSN) in Userʼs Manual.
Low-Power Analog Comparator The Low-Power Analog Comparator (ACMPLP) compares the reference input voltage and
(ACMPLP) analog input voltage. The comparison result can be read through software and also be output
externally. The reference voltage can be selected from an input to the CMPREFi(i = 0,1) pin,
an internal 8-bit D/A converter output, or the internal reference voltage (Vref) generated
internally in the MCU.
The ACMPLP response speed can be set before starting an operation. Setting the high-speed
mode decreases the response delay time, but increases current consumption. Setting the low-
speed mode increases the response delay time, but decreases current consumption. See
section 39, Low-Power Analog Comparator (ACMPLP) in Userʼs Manual.
Operational Amplifier (OPAMP) The Operational Amplifier (OPAMP) amplifies small analog input voltages and outputs the
amplified voltages. A total of four differential operational amplifier units with two input pins and
one output pin are provided. See section 38, Operational Amplifier (OPAMP) in Userʼs Manual.
Table 1.9 Human machine interfaces
Feature Functional description
Segment LCD Controller (SLCDC) The Segment LCD Controller (SLCDC) provides the following functions:
 Waveform A or B selectable
 The LCD driver voltage generator can switch between an internal voltage boosting method,
a capacitor split method, and an external resistance division method
 Automatic output of segment and common signals based on automatic display data register
read
 The reference voltage generated when operating the voltage boost circuit can be selected in
16 steps (contrast adjustment)
 The LCD can be made to blink.
See section 45, Segment LCD Controller (SLCDC) in Userʼs Manual.
Capacitive Touch Sensing Unit The Capacitive Touch Sensing Unit (CTSU) measures the electrostatic capacitance of the
(CTSU) touch sensor. Changes in the electrostatic capacitance are determined by software, which
enables the CTSU to detect whether a finger is in contact with the touch sensor. The electrode
surface of the touch sensor is usually enclosed within an electrical insulator so that fingers do
not come into direct contact with the electrode. See section 41, Capacitive Touch Sensing Unit
(CTSU) in Userʼs Manual.
R01DS0355EJ0110 Rev.1.10 Page 6 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
Table 1.10 Data processing
Feature Functional description
Cyclic Redundancy Check (CRC) The Cyclic Redundancy Check (CRC) calculator generates CRC codes to detect errors in the
calculator data. The bit order of CRC calculation results can be switched for LSB-first or MSB-first
communication. Additionally, various CRC generation polynomials are available. The snoop
function allows monitoring reads from and writes to specific addresses. This function is useful
in applications that require CRC code to be generated automatically in certain events, such as
monitoring writes to the serial transmit buffer and reads from the serial receive buffer. See
section 32, Cyclic Redundancy Check (CRC) Calculator in Userʼs Manual.
Data Operation Circuit (DOC) The Data Operation Circuit (DOC) compares, adds, and subtracts 16-bit data. See section 42,
Data Operation Circuit (DOC) in Userʼs Manual.
Table 1.11 Security
Feature Functional description
Secure Crypto Engine 5 (SCE5)  Security algorithm
- Symmetric algorithm: AES.
 Other support features
- TRNG (True Random Number Generator)
- Hash-value generation: GHASH.
Table 1.12 I/O ports
Feature Functional description
I/O ports  I/O ports for the 100-pin LQFP, 100-pin LGA
- I/O pins: 81
- Input pins: 3
- Pull-up resistors: 79
- N-ch open-drain outputs: 64
- 5-V tolerance: 9
 I/O ports for the 64-pin LQFP, 64-pin QFN
- I/O pins: 49
- Input pins: 3
- Pull-up resistors: 47
- N-ch open-drain outputs: 36
- 5-V tolerance: 7
 I/O ports for the 48-pin LQFP, 48-pin QFN
- I/O pins: 33
- Input pins: 3
- Pull-up resistors: 31
- N-ch open-drain outputs: 22
- 5-V tolerance: 4
 I/O ports for the 40-pin QFN
- I/O pins: 25
- Input pins: 3
- Pull-up resistors: 23
- N-ch open-drain outputs: 15
- 5-V tolerance: 2
R01DS0355EJ0110 Rev.1.10 Page 7 of 131
Sep 29, 2023

| RA4M1 Group |               |     |     |     |     |     |     | 1. Overview |
| ----------- | ------------- | --- | --- | --- | --- | --- | --- | ----------- |
| 1.2         | Block Diagram |     |     |     |     |     |     |             |
Figure 1.1 shows a block diagram of the MCU superset. Some individual devices within the group have a subset of the
features.
|     |     | Memory | Bus | Arm Cortex-M4 |     |     | System |     |
| --- | --- | ------ | --- | ------------- | --- | --- | ------ | --- |
Clocks
|     | 256 KB Code Flash |     | MPU | DSP | FPU | POR/LVD |     |     |
| --- | ----------------- | --- | --- | --- | --- | ------- | --- | --- |
MOSC/SOSC
8 KB Data Flash
|     |     |     |     | MPU |     | Reset |     |     |
| --- | --- | --- | --- | --- | --- | ----- | --- | --- |
(HOCO/
MOCO/
32 KB SRAM
LOCO)
|     |     |     |     | NVIC |     | Mode Control |     |     |
| --- | --- | --- | --- | ---- | --- | ------------ | --- | --- |
PLL
|     |     |     |     | System Timer |     | Power Control |     |     |
| --- | --- | --- | --- | ------------ | --- | ------------- | --- | --- |
DMA
CAC
|     |     | DTC |     | Test and DBG Interface |     | ICU |     |     |
| --- | --- | --- | --- | ---------------------- | --- | --- | --- | --- |
Battery Backup
Register Write
|     | DMAC × 4 |     |     |     |     | KINT |     |     |
| --- | -------- | --- | --- | --- | --- | ---- | --- | --- |
Protection
|     | Timers |     | Communication interfaces |     |     | Human machine interfaces |     |     |
| --- | ------ | --- | ------------------------ | --- | --- | ------------------------ | --- | --- |
GPT32 × 2
|     |     |     | SCI × 4 | CAN × 1 |     | CTSU | SLCDC |     |
| --- | --- | --- | ------- | ------- | --- | ---- | ----- | --- |
GPT16 × 6
USBFS
|     |         |     | IIC × 2 | with Battery  |     |     |     |     |
| --- | ------- | --- | ------- | ------------- | --- | --- | --- | --- |
|     | AGT × 2 |     |         | Charging      |     |     |     |     |
revision 1.2
SPI × 2
RTC
|     | WDT/IWDT   |     | SSIE × 1        |     |     |        |     |     |
| --- | ---------- | --- | --------------- | --- | --- | ------ | --- | --- |
|     | Event Link |     | Data processing |     |     | Analog |     |     |
ELC
|     |     |     | CRC |     | ADC14 |     | TSN OPAMP × 4   |     |
| --- | --- | --- | --- | --- | ----- | --- | --------------- | --- |
|     |     |     | DOC |     | DAC12 |     | DAC8 ACMPLP × 2 |     |
Security
SCE5
| Figure 1.1      |     | Block diagram |     |     |     |     |     |               |
| --------------- | --- | ------------- | --- | --- | --- | --- | --- | ------------- |
| R01DS0355EJ0110 |     | Rev.1.10      |     |     |     |     |     | Page 8 of 131 |
Sep 29, 2023

RA4M1 Group 1. Overview
1.3 Part Numbering
Figure 1.2 shows the product part number information, including memory capacity, and package type. Table 1.13 shows
a product list.
R 7 F A 4 M 1 A B 3 C F P #A A 0
Production identification code
Terminal material (Pb-free)
A: Sn (Tin) only
C: Others
Packing
A: Tray
B: Tray (Full carton)
H: Tape and reel
Package type
FP: LQFP 100 pins
FM: LQFP 64 pins
FL: LQFP 48 pins
LJ: LGA 100 pins
NB: QFN 64 pins
NE: QFN 48 pins
NF: QFN 40 pins
Quality Grade
Operating temperature
2: -40°C to 85°C
3: -40°C to 105°C
Code flash memory size
B: 256 KB
Feature set
Group name
Series name
RA family
Flash memory
Renesas microcontroller
Note: Check the order screen for each product on the Renesas website for valid symbols after the #.
Figure 1.2 Part numbering scheme
R01DS0355EJ0110 Rev.1.10 Page 9 of 131
Sep 29, 2023

RA4M1 Group 1. Overview
| Table 1.13 Product list |     |     |     |     |     |
| ----------------------- | --- | --- | --- | --- | --- |
Product part number Package code Code flash Data flash SRAM Operating temperature
| R7FA4M1AB3CFP | PLQP0100KB-B | 256 KB | 8 KB | 32 KB | -40 to +105°C |
| ------------- | ------------ | ------ | ---- | ----- | ------------- |
| R7FA4M1AB2CLJ | PTLG0100JA-A |        |      |       | -40 to +85°C  |
| R7FA4M1AB3CFM | PLQP0064KB-C |        |      |       | -40 to +105°C |
| R7FA4M1AB3CNB | PWQN0064LA-A |        |      |       | -40 to +105°C |
PWQN0064LB-B
| R7FA4M1AB3CFL | PLQP0048KB-B |     |     |     | -40 to +105°C |
| ------------- | ------------ | --- | --- | --- | ------------- |
| R7FA4M1AB3CNE | PWQN0048KB-A |     |     |     | -40 to +105°C |
PWQN0048KC-A
| R7FA4M1AB3CNF | PWQN0040KC-A |     |     |     | -40 to +105°C |
| ------------- | ------------ | --- | --- | --- | ------------- |
PWQN0040KD-A
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 10 of 131 |
| --------------- | -------- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |                     |     |     |     |                |     |                |     | 1. Overview |
| ----------- | ------------------- | --- | --- | --- | -------------- | --- | -------------- | --- | ----------- |
| 1.4         | Function Comparison |     |     |     |                |     |                |     |             |
| Table 1.14  | Function comparison |     |     |     |                |     |                |     |             |
|             |                     |     |     |     | R7FA4M1AB3CFM/ |     | R7FA4M1AB3CFL/ |     |             |
Part numbers R7FA4M1AB3CFP R7FA4M1AB2CLJ R7FA4M1AB3CNB R7FA4M1AB3CNE R7FA4M1AB3CNF
| Pin count         |           | 100  |     | 100 |     | 64        | 48       |     | 40  |
| ----------------- | --------- | ---- | --- | --- | --- | --------- | -------- | --- | --- |
| Package           |           | LQFP |     | LGA |     | LQFP/QFN  | LQFP/QFN |     | QFN |
| Code flash memory |           |      |     |     |     | 256 KB    |          |     |     |
| Data flash memory |           |      |     |     |     | 8 KB      |          |     |     |
| SRAM              |           |      |     |     |     | 32 KB     |          |     |     |
|                   | Parity    |      |     |     |     | 16 KB     |          |     |     |
|                   | ECC       |      |     |     |     | 16 KB     |          |     |     |
| System            | CPU clock |      |     |     |     | 48 MHz    |          |     |     |
|                   | Backup    |      |     |     |     | 512 bytes |          |     |     |
registers
|               | ICU          |     |     |     |     | Yes |     |     |     |
| ------------- | ------------ | --- | --- | --- | --- | --- | --- | --- | --- |
|               | KINT         |     |     | 8   |     |     | 5   |     | 3   |
| Event control | ELC          |     |     |     |     | Yes |     |     |     |
| DMA           | DTC          |     |     |     |     | Yes |     |     |     |
|               | DMAC         |     |     |     |     | 4   |     |     |     |
| Bus           | External bus |     |     |     |     | No  |     |     |     |
| Timers        | GPT32        |     |     |     |     | 2   |     |     |     |
|               | GPT16        |     |     | 6   |     |     | 4   |     | 2   |
|               | AGT          |     |     | 2   |     |     |     | No  |     |
|               | RTC          |     |     |     |     | Yes |     |     |     |
|               | WDT/IWDT     |     |     |     |     | Yes |     |     |     |
| Communication | SCI          |     |     |     |     | 4   |     |     |     |
|               | IIC          |     |     |     |     | 2   |     |     |     |
|               | SPI          |     |     |     | 2   |     |     |     | 1   |
|               | SSIE         |     | 1   |     |     |     | No  |     |     |
|               | QSPI         |     |     |     |     | No  |     |     |     |
|               | SDHI         |     |     |     |     | No  |     |     |     |
|               | CAN          |     |     |     |     | 1   |     |     |     |
|               | USBFS        |     |     |     |     | Yes |     |     |     |
| Analog        | ADC14        |     | 25  |     |     | 18  | 14  |     | 11  |
|               | DAC12        |     |     |     |     | 1   |     |     |     |
|               | DAC8         |     |     |     |     | 2   |     |     |     |
|               | ACMPLP       |     |     |     | 2   |     |     |     | 1   |
|               | OPAMP        | 4   |     | 4   |     | 3   | 1   |     | No  |
|               | TSN          |     |     |     |     | Yes |     |     |     |
HMI SLCDC 4 com × 38 seg or 8 com × 34 seg 4 com × 21 seg or  No
8 com × 17 seg
|       | CTSU |     | 27  |     |     | 24  | 15  |     | 10  |
| ----- | ---- | --- | --- | --- | --- | --- | --- | --- | --- |
| Data  | CRC  |     |     |     |     | Yes |     |     |     |
processing
|           | DOC        |     |     |     |     | Yes  |     |     |     |
| --------- | ---------- | --- | --- | --- | --- | ---- | --- | --- | --- |
| Security  |            |     |     |     |     | SCE5 |     |     |     |
| I/O ports | I/O pins   |     | 81  |     |     | 49   | 33  |     | 25  |
|           | Input pins |     | 3   |     |     | 3    | 3   |     | 3   |
|           | Pull-up    |     | 79  |     |     | 47   | 31  |     | 23  |
resistors
|     | N-ch open- |     | 64  |     |     | 36  | 22  |     | 15  |
| --- | ---------- | --- | --- | --- | --- | --- | --- | --- | --- |
drain outputs
|                 | 5-V tolerance |     | 9   |     |     | 7   | 4   |     | 2              |
| --------------- | ------------- | --- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10      |     |     |     |     |     |     |     | Page 11 of 131 |
Sep 29, 2023

| RA4M1 Group       |                        |                 | 1. Overview |
| ----------------- | ---------------------- | --------------- | ----------- |
| 1.5 Pin Functions |                        |                 |             |
| Table 1.15        | Pin functions (1 of 4) |                 |             |
| Function          | Signal                 | I/O Description |             |
Power supply VCC Input Power supply pin. Connect this pin to the system power supply. Connect it to
VSS through a 0.1-μF capacitor. The capacitor should be placed close to the
pin.
VCL I/O Connect this pin to the VSS pin through the smoothing capacitor used to
stabilize the internal power supply. Place the capacitor close to the pin.
|     | VSS   | Input Ground pin. Connect to the system power supply (0 V). |     |
| --- | ----- | ----------------------------------------------------------- | --- |
|     | VBATT | Input Backup power supply pin                               |     |
Clock XTAL Output Pins for a crystal resonator. An external clock signal can be input through the
EXTAL pin.
|     | EXTAL | Input |     |
| --- | ----- | ----- | --- |
XCIN Input Input/output pins for the sub-clock oscillator. Connect a crystal resonator
between XCOUT and XCIN.
|     | XCOUT  | Output                  |     |
| --- | ------ | ----------------------- | --- |
|     | CLKOUT | Output Clock output pin |     |
Operating mode  MD Input Pins for setting the operating mode. The signal levels on these pins must not
control be changed during operation mode transition on release from the reset state.
System control RES Input Reset signal input pin. The MCU enters the reset state when this signal goes
low.
| CAC       | CACREF          | Input Measurement reference clock input pin |     |
| --------- | --------------- | ------------------------------------------- | --- |
| Interrupt | NMI             | Input Non-maskable interrupt request pin    |     |
|           | IRQ0 to IRQ12,  | Input Maskable interrupt request pins       |     |
IRQ14, IRQ15
| KINT | KR00 to KR07 | Input Key interrupt input pins. |     |
| ---- | ------------ | ------------------------------- | --- |
A key interrupt (KINT) can be generated by inputting a falling edge to the key
interrupt input pins.
| On-chip debug | TMS   | I/O On-chip emulator or boundary scan pins  |     |
| ------------- | ----- | ------------------------------------------- | --- |
|               | TDI   | Input                                       |     |
|               | TCK   | Input                                       |     |
|               | TDO   | Output                                      |     |
|               | SWDIO | I/O Serial wire debug data input/output pin |     |
|               | SWCLK | Input Serial wire clock pin                 |     |
|               | SWO   | Output Serial wire trace output pin         |     |
Battery Backup VBATWIO0 to  I/O Output wakeup signal for the VBATT wakeup control function.
VBATWIO2 External event input for the VBATT wakeup control function.
| GPT | GTETRGA,  | Input External trigger input pin |     |
| --- | --------- | -------------------------------- | --- |
GTETRGB
|     | GTIOC0A to  | I/O Input capture, output capture, or PWM output pin |     |
| --- | ----------- | ---------------------------------------------------- | --- |
GTIOC7A,
GTIOC0B to
GTIOC7B
|     | GTIU | Input Hall sensor input pin U |     |
| --- | ---- | ----------------------------- | --- |
|     | GTIV | Input Hall sensor input pin V |     |
|     | GTIW | Input Hall sensor input pin W |     |
GTOUUP Output 3-phase PWM output for BLDC motor control (positive U phase)
GTOULO Output 3-phase PWM output for BLDC motor control (negative U phase)
GTOVUP Output 3-phase PWM output for BLDC motor control (positive V phase)
GTOVLO Output 3-phase PWM output for BLDC motor control (negative V phase)
GTOWUP Output 3-phase PWM output for BLDC motor control (positive W phase)
GTOWLO Output 3-phase PWM output for BLDC motor control (negative W phase)
| R01DS0355EJ0110 | Rev.1.10 |     | Page 12 of 131 |
| --------------- | -------- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |                        |                                                | 1. Overview |
| ----------- | ---------------------- | ---------------------------------------------- | ----------- |
| Table 1.15  | Pin functions (2 of 4) |                                                |             |
| Function    | Signal                 | I/O Description                                |             |
| AGT         | AGTEE0, AGTEE1         | Input External event input enable signals      |             |
|             | AGTIO0, AGTIO1         | I/O External event input and pulse output pins |             |
|             | AGTO0, AGTO1           | Output Pulse output pins                       |             |
|             | AGTOA0, AGTOA1         | Output Output compare match A output pins      |             |
|             | AGTOB0, AGTOB1         | Output Output compare match B output pins      |             |
| RTC         | RTCOUT                 | Output Output pin for 1-Hz/64-Hz clock         |             |
|             | RTCIC0 to RTCIC2       | Input Time capture event input pins            |             |
SCI SCK0 to SCK2,  I/O Clock (clock synchronous mode) input/output pins
SCK9
RXD0 to RXD2,  Input Received data (asynchronous mode/clock synchronous mode) input pins
RXD9
TXD0 to TXD2,  Output Transmitted data (asynchronous mode/clock synchronous mode) output pins
TXD9
CTS0_RTS0 to  I/O Input/output pins for controlling the start of transmission and reception
CTS2_RTS2,  (asynchronous mode/clock synchronous mode), active-low
CTS9_RTS9
|     | SCL0 to SCL2,  | I/O I2C clock (simple IIC) input/output pins |     |
| --- | -------------- | -------------------------------------------- | --- |
SCL9
|     | SDA0 to SDA2,  | I/O I2C data (simple IIC) input/output pins |     |
| --- | -------------- | ------------------------------------------- | --- |
SDA9
|     | SCK0 to SCK2,  | I/O Clock (simple SPI) input/output pins |     |
| --- | -------------- | ---------------------------------------- | --- |
SCK9
MISO0 to MISO2,  I/O Slave transmission of data (simple SPI) input/output pins
MISO9
MOSI0 to MOSI2,  I/O Master transmission of data (simple SPI) input/output pins
MOSI9
|      | SS0 to SS2, SS9 | Input Slave-select input pins (simple SPI), active-low        |     |
| ---- | --------------- | ------------------------------------------------------------- | --- |
| IIC  | SCL0, SCL1      | I/O Clock input/output pins                                   |     |
|      | SDA0, SDA1      | I/O Data input/output pins                                    |     |
| SSIE | SSIBCK0         | I/O SSIE serial bit clock pin                                 |     |
|      | SSILRCK0/SSIFS0 | I/O Word select pins                                          |     |
|      | SSITXD0         | Output Serial data output pin                                 |     |
|      | SSIRXD0         | Input Serial data input pin                                   |     |
|      | AUDIO_CLK       | Input External clock pin for audio (input oversampling clock) |     |
| SPI  | RSPCKA, RSPCKB  | I/O Clock input/output pin                                    |     |
|      | MOSIA, MOSIB    | I/O Input/output pins for data output from the master         |     |
|      | MISOA, MISOB    | I/O Input/output pins for data output from the slave          |     |
|      | SSLA0, SSLB0    | I/O Input/output pins for slave selection                     |     |
|      | SSLA1, SSLA2,   | Output Output pins for slave selection                        |     |
SSLA3, SSLB1,
SSLB2, SSLB3
| CAN             | CRX0     | Input Receive data   |                |
| --------------- | -------- | -------------------- | -------------- |
|                 | CTX0     | Output Transmit data |                |
| R01DS0355EJ0110 | Rev.1.10 |                      | Page 13 of 131 |
Sep 29, 2023

| RA4M1 Group |                        |                                              | 1. Overview |
| ----------- | ---------------------- | -------------------------------------------- | ----------- |
| Table 1.15  | Pin functions (3 of 4) |                                              |             |
| Function    | Signal                 | I/O Description                              |             |
| USBFS       | VSS_USB                | Input Ground pin                             |             |
|             | VCC_USB_LDO            | Input Power supply pin for USB LDO regulator |             |
|             | VCC_USB                | I/O Input: USB transceiver power supply pin. |             |
Output: USB LDO regulator output pin. This pin should be connected to an
external capacitor.
USB_DP I/O D+ I/O pin of the USB on-chip transceiver. This pin should be connected to the
D+ pin of the USB bus.
USB_DM I/O D- I/O pin of the USB on-chip transceiver. This pin should be connected to the
D- pin of the USB bus.
USB_VBUS Input USB cable connection monitor pin. This pin should be connected to VBUS of
the USB bus. The VBUS pin status (connected or disconnected) can be
detected when the USB module is operating as a device controller.
USB_EXICEN Output Low power control signal for external power supply (OTG) chip
USB_VBUSEN Output VBUS (5 V) supply enable signal for external power supply chip
USB_OVRCURA,  Input Connect the external overcurrent detection signals to these pins. Connect the
USB_OVRCURB VBUS comparator signals to these pins when the OTG power supply chip is
connected.
USB_ID Input Connect the MicroAB connector ID input signal to this pin during operation in
OTG mode
| Analog power  | AVCC0 | Input Analog voltage supply pin |     |
| ------------- | ----- | ------------------------------- | --- |
supply
|     | AVSS0  | Input Analog voltage supply ground pin                      |     |
| --- | ------ | ----------------------------------------------------------- | --- |
|     | VREFH0 | Input Analog reference voltage supply pin                   |     |
|     | VREFL0 | Input Reference power supply ground pin                     |     |
|     | VREFH  | Input Analog reference voltage supply pin for D/A converter |     |
|     | VREFL  | Input Analog reference ground pin for D/A converter         |     |
ADC14 AN000 to AN014,  Input Input pins for the analog signals to be processed by the A/D converter
AN016 to AN025
ADTRG0 Input Input pins for the external trigger signals that start the A/D conversion, active-
low
DAC12 DA0 Output Output pins for the analog signals to be processed by the D/A converter
| Comparator  | VCOUT | Output Comparator output pin |     |
| ----------- | ----- | ---------------------------- | --- |
output
| ACMPLP | CMPREF0,  | Input Reference voltage input pin |     |
| ------ | --------- | --------------------------------- | --- |
CMPREF1
|       | CMPIN0, CMPIN1 | Input Analog voltage input pins   |     |
| ----- | -------------- | --------------------------------- | --- |
| OPAMP | AMP0+ to AMP3+ | Input Analog voltage input pins   |     |
|       | AMP0- to AMP3- | Input Analog voltage input pins   |     |
|       | AMP0O to AMP3O | Output Analog voltage output pins |     |
CTSU TS00 to TS13, Input Capacitive touch detection pins (touch pins)
TS17 to TS22,
TS27 to TS31,
TS34, TS35
|                 | TSCAP    | - Secondary power supply pin for the touch driver |                |
| --------------- | -------- | ------------------------------------------------- | -------------- |
| R01DS0355EJ0110 | Rev.1.10 |                                                   | Page 14 of 131 |
Sep 29, 2023

| RA4M1 Group |                        |                                       | 1. Overview |
| ----------- | ---------------------- | ------------------------------------- | ----------- |
| Table 1.15  | Pin functions (4 of 4) |                                       |             |
| Function    | Signal                 | I/O Description                       |             |
| I/O ports   | P000 to P008,          | I/O General-purpose input/output pins |             |
P010 to P015
|     | P100 to P115   | I/O General-purpose input/output pins |     |
| --- | -------------- | ------------------------------------- | --- |
|     | P200           | Input General-purpose input pin       |     |
|     | P201 to P206,  | I/O General-purpose input/output pins |     |
P212, P213
|     | P214, P215    | Input General-purpose input pins      |     |
| --- | ------------- | ------------------------------------- | --- |
|     | P300 to P307  | I/O General-purpose input/output pins |     |
|     | P400 to P415  | I/O General-purpose input/output pins |     |
|     | P500 to P505  | I/O General-purpose input/output pins |     |
|     | P600 to P603, | I/O General-purpose input/output pins |     |
P608 to P610
|                 | P708               | I/O General-purpose input/output pins                           |                |
| --------------- | ------------------ | --------------------------------------------------------------- | -------------- |
|                 | P808, P809         | I/O General-purpose input/output pins                           |                |
|                 | P914, P915         | I/O General-purpose input/output pins                           |                |
| SLCDC           | VL1, VL2, VL3, VL4 | I/O Voltage pin for driving the LCD                             |                |
|                 | CAPH, CAPL         | I/O Capacitor connection pin for the LCD controller/driver      |                |
|                 | COM0 to COM7       | Output Common signal output pins for the LCD controller/driver  |                |
|                 | SEG00 to SEG37     | Output Segment signal output pins for the LCD controller/driver |                |
| R01DS0355EJ0110 | Rev.1.10           |                                                                 | Page 15 of 131 |
Sep 29, 2023

| RA4M1 Group |                 |     |     |     |     |     |     | 1. Overview |
| ----------- | --------------- | --- | --- | --- | --- | --- | --- | ----------- |
| 1.6         | Pin Assignments |     |     |     |     |     |     |             |
Figure 1.3 to Figure 1.6 show the pin assignments.
OIDWS/SMT/801P
OWS/ODT/901P
IDT/011P
|     |      | 001P 101P 201P 301P | 401P 501P 601P 701P | 006P 106P 206P 306P | 016P 906P 806P 511P | 411P 311P 211P 111P |                   |     |
| --- | ---- | ------------------- | ------------------- | ------------------- | ------------------- | ------------------- | ----------------- | --- |
|     |      |                     |                     | SSV                 | CCV                 |                     |                   |     |
|     |      | 57 47 37 27         | 17 07 96 86         | 76 66 56 46 36      | 26 16 06 95 85      | 75 65 55 45 35      | 25 15             |     |
|     | P500 | 76                  |                     |                     |                     |                     | 50 P300/TCK/SWCLK |     |
|     | P501 | 77                  |                     |                     |                     |                     | P301              |     |
49
|     | P502 | 78  |     |     |     |     | P302 |     |
| --- | ---- | --- | --- | --- | --- | --- | ---- | --- |
48
|     | P503 | 79  |     |     |     |     | P303 |     |
| --- | ---- | --- | --- | --- | --- | --- | ---- | --- |
47
|     | P504        | 80  |               |     |     |     | 46 P809    |     |
| --- | ----------- | --- | ------------- | --- | --- | --- | ---------- | --- |
|     | P505        | 81  |               |     |     |     | 45 P808    |     |
|     | VCC         | 82  |               |     |     |     | 44 P304    |     |
|     | VSS         | 83  |               |     |     |     | 43 P305    |     |
|     | P015        | 84  |               |     |     |     | 42 P306    |     |
|     | P014        | 85  |               |     |     |     | 41 P307    |     |
|     | P013/VREFL  | 86  |               |     |     |     | 40 P200    |     |
|     | P012/VREFH  | 87  | R7FA4M1AB3CFP |     |     |     | 39 P201/MD |     |
|     | AVCC0       | 88  |               |     |     |     | 38 RES     |     |
|     | AVSS0       | 89  |               |     |     |     | 37 VCC     |     |
|     | P011/VREFL0 | 90  |               |     |     |     | VSS        |     |
36
|     | P010/VREFH0 | 91  |     |     |     |     | P202 |     |
| --- | ----------- | --- | --- | --- | --- | --- | ---- | --- |
35
|     | P008 | 92  |     |     |     |     | P203 |     |
| --- | ---- | --- | --- | --- | --- | --- | ---- | --- |
34
|                 | P007                                       | 93                  |                      |                                        |                               |                          | 33 P204        |                |
| --------------- | ------------------------------------------ | ------------------- | -------------------- | -------------------------------------- | ----------------------------- | ------------------------ | -------------- | -------------- |
|                 | P006                                       | 94                  |                      |                                        |                               |                          | 32 P205        |                |
|                 | P005                                       | 95                  |                      |                                        |                               |                          | 31 P206        |                |
|                 | P004                                       | 96                  |                      |                                        |                               |                          | 30 VCC_USB_LDO |                |
|                 | P003                                       | 97                  |                      |                                        |                               |                          | 29 VCC_USB     |                |
|                 | P002                                       | 98                  |                      |                                        |                               |                          | 28 P914/USB_DP |                |
|                 | P001                                       | 99                  |                      |                                        |                               |                          | 27 P915/USB_DM |                |
|                 | P000                                       | 100                 |                      |                                        |                               |                          | 26 VSS_USB     |                |
|                 |                                            |                     |                      | 01 11 21 31                            | 41 51 61 71 81                | 91 02 12 22 32           | 42 52          |                |
|                 |                                            | 1 2 3 4             | 5 6 7 8              | 9                                      |                               |                          |                |                |
|                 |                                            | 004P 104P 204P 304P | 404P 504P 604P TTABV | LCV NICX/512P TUOCX/412P SSV LATX/312P | LATXE/212P CCV 807P 514P 414P | 314P 214P 114P 014P 904P | 804P 704P      |                |
| Figure 1.3      | Pin assignment for 100-pin LQFP (top view) |                     |                      |                                        |                               |                          |                |                |
| R01DS0355EJ0110 |                                            | Rev.1.10            |                      |                                        |                               |                          |                | Page 16 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------- |
R7FA4M1AB2CLJ
|     | A       | B       | C        | D    | E     | F     | G     | H     | J      | K      |     |
| --- | ------- | ------- | -------- | ---- | ----- | ----- | ----- | ----- | ------ | ------ | --- |
|     |         |         |          |      | P212/ | P215/ |       |       |        |        |     |
|     | 10 P407 | P409    | P412     | VCC  |       |       | VCL   | P403  | P400   | P000   | 10  |
|     |         |         |          |      | EXTAL | XCIN  |       |       |        |        |     |
|     | P915/   | P914/   |          |      | P213/ | P214/ |       |       |        |        |     |
|     | 9       |         | P413     | VSS  |       |       | VBATT | P405  | P401   | P001   | 9   |
|     | USB_DM  | USB_DP  |          |      | XTAL  | XCOUT |       |       |        |        |     |
|     | VCC_    | VSS_    | VCC_US   |      |       |       |       |       |        |        |     |
|     | 8       |         |          | P411 | P415  | P708  | P404  | P003  | P004   | P002   | 8   |
|     | USB     | USB     | B_LDO    |      |       |       |       |       |        |        |     |
|     | 7 P205  | P204    | P206     | P408 | P414  | P406  | P006  | P007  | P008   | P005   | 7   |
|     |         |         |          |      |       |       |       |       | P011/  | P010/  |     |
|     | 6 VSS   | VCC     | P202     | P203 | P410  | P402  | P505  | AVSS0 |        |        | 6   |
|     |         |         |          |      |       |       |       |       | VREFL0 | VREFH0 |     |
|     |         |         |          |      |       |       |       |       | P013/  | P012/  |     |
|     | 5 P200  | P201/MD | P307     | RES  | P113  | P600  | P504  | AVCC0 |        |        | 5   |
|     |         |         |          |      |       |       |       |       | VREFL  | VREFH  |     |
|     | 4 P305  | P304    | P808     | P306 | P115  | P601  | P503  | P100  | P015   | P014   | 4   |
|     | 3 P809  | P303    | P110/TDI | P111 | P609  | P602  | P107  | P103  | VSS    | VCC    | 3   |
P300/
|     | 2 TCK/ | P302 | P301 | P114 | P610 | P603 | P106 | P101 | P501 | P502 | 2   |
| --- | ------ | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | --- |
SWCLK
|                 | P108/                                     | P109/ |      |      |     |     |      |      |      |      |                |
| --------------- | ----------------------------------------- | ----- | ---- | ---- | --- | --- | ---- | ---- | ---- | ---- | -------------- |
|                 | 1 TMS/                                    | TDO/  | P112 | P608 | VCC | VSS | P105 | P104 | P102 | P500 | 1              |
|                 | SWDIO                                     | SWO   |      |      |     |     |      |      |      |      |                |
|                 | A                                         | B     | C    | D    | E   | F   | G    | H    | J    | K    |                |
| Figure 1.4      | Pin assignment for 100-pin LGA (top view) |       |      |      |     |     |      |      |      |      |                |
| R01DS0355EJ0110 | Rev.1.10                                  |       |      |      |     |     |      |      |      |      | Page 17 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | --- | --- | ----------- |
OIDWS/SMT/801P
OWS/ODT/901P
IDT/011P
|     |      | 001P 101P 201P | 301P 401P 501P 601P | 701P SSV CCV 311P | 211P 111P   |                |     |
| --- | ---- | -------------- | ------------------- | ----------------- | ----------- | -------------- | --- |
|     |      | 84 74 64       | 54 44 34 24         | 14 04 93 83       | 73 63 53 43 | 33             |     |
|     | P500 | 49             |                     |                   |             | P300/TCK/SWCLK |     |
32
|     | P501       | 50               |     |     |     | 31 P301    |     |
| --- | ---------- | ---------------- | --- | --- | --- | ---------- | --- |
|     | P502       | 51               |     |     |     | 30 P302    |     |
|     | P015       | 52               |     |     |     | 29 P303    |     |
|     | P014       | 53               |     |     |     | 28 P304    |     |
|     | P013/VREFL |                  |     |     |     | P200       |     |
|     |            | 54               |     |     |     | 27         |     |
|     | P012/VREFH | 55               |     |     |     | 26 P201/MD |     |
|     | AVCC0      | 56 R7FA4M1AB3CFM |     |     |     | 25 RES     |     |
|     | AVSS0      | 57               |     |     |     | P204       |     |
24
|     | P011/VREFL0 | 58  |     |     |     | 23 P205        |     |
| --- | ----------- | --- | --- | --- | --- | -------------- | --- |
|     | P010/VREFH0 | 59  |     |     |     | 22 P206        |     |
|     | P004        | 60  |     |     |     | 21 VCC_USB_LDO |     |
|     | P003        | 61  |     |     |     | 20 VCC_USB     |     |
|     | P002        | 62  |     |     |     | P914/USB_DP    |     |
19
|                 | P001                                      | 63             |                                |                              |                     | 18 P915/USB_DM |                |
| --------------- | ----------------------------------------- | -------------- | ------------------------------ | ---------------------------- | ------------------- | -------------- | -------------- |
|                 | P000                                      | 64             |                                |                              |                     | 17 VSS_USB     |                |
|                 |                                           |                |                                | 01 11                        | 21 31 41 51         | 61             |                |
|                 |                                           | 1 2 3          | 4 5 6 7                        | 8 9                          |                     |                |                |
|                 |                                           | 004P 104P 204P | TTABV LCV NICX/512P TUOCX/412P | SSV LATX/312P LATXE/212P CCV | 114P 014P 904P 804P | 704P           |                |
| Figure 1.5      | Pin assignment for 64-pin LQFP (top view) |                |                                |                              |                     |                |                |
| R01DS0355EJ0110 | Rev.1.10                                  |                |                                |                              |                     |                | Page 18 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | --- | ----------- |
OIDWS/SMT/801P
OWS/ODT/901P
IDT/011P
|     |      | 001P 101P 201P 301P | 401P 501P 601P 701P SSV | CCV 311P 211P 111P |                   |     |
| --- | ---- | ------------------- | ----------------------- | ------------------ | ----------------- | --- |
|     |      | 84 74 64 54         | 44 34 24 14 04          | 93 83 73 63 53     | 43 33             |     |
|     | P500 | 49                  |                         |                    | 32 P300/TCK/SWCLK |     |
Exposed die pad
|                 | P501                                              | 50                   |                                        |                               | 31 P301        |                |
| --------------- | ------------------------------------------------- | -------------------- | -------------------------------------- | ----------------------------- | -------------- | -------------- |
|                 | P502                                              | 51                   |                                        |                               | 30 P302        |                |
|                 | P015                                              | 52                   |                                        |                               | 29 P303        |                |
|                 | P014                                              | 53                   |                                        |                               | 28 P304        |                |
|                 | P013/VREFL                                        | 54                   |                                        |                               | 27 P200        |                |
|                 | P012/VREFH                                        | 55                   |                                        |                               | 26 P201/MD     |                |
|                 | AVCC0                                             | 56 R7FA4M1AB3CNB     |                                        |                               | 25 RES         |                |
|                 | AVSS0                                             | 57                   |                                        |                               | 24 P204        |                |
|                 | P011/VREFL0                                       | 58                   |                                        |                               | 23 P205        |                |
|                 | P010/VREFH0                                       | 59                   |                                        |                               | 22 P206        |                |
|                 | P004                                              | 60                   |                                        |                               | 21 VCC_USB_LDO |                |
|                 | P003                                              | 61                   |                                        |                               | 20 VCC_USB     |                |
|                 | P002                                              |                      |                                        |                               | P914/USB_DP    |                |
|                 |                                                   | 62                   |                                        |                               | 19             |                |
|                 | P001                                              | 63                   |                                        |                               | 18 P915/USB_DM |                |
|                 | P000                                              | 64                   |                                        |                               | 17 VSS_USB     |                |
|                 |                                                   | 1 2 3 4              | 5 6 7 8 9                              | 01 11 21 31 41                | 51 61          |                |
|                 |                                                   | 004P 104P 204P TTABV | LCV NICX/512P TUOCX/412P SSV LATX/312P | LATXE/212P CCV 114P 014P 904P | 804P 704P      |                |
| Note:           | Exposed die pad is recommended to connect to VSS. |                      |                                        |                               |                |                |
| Figure 1.6      | Pin assignment for 64-pin QFN (top view)          |                      |                                        |                               |                |                |
| R01DS0355EJ0110 | Rev.1.10                                          |                      |                                        |                               |                | Page 19 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | ----------- |
OIDWS/SMT/801P
OWS/ODT/901P
IDT/011P
|     |     | 001P 101P 201P 301P | 401P 211P 111P |     |     |
| --- | --- | ------------------- | -------------- | --- | --- |
SSV CCV
|     |             | 63 53 43 33      | 23 13 03 92 82 | 72 62 52          |     |
| --- | ----------- | ---------------- | -------------- | ----------------- | --- |
|     | P500        | 37               |                | 24 P300/TCK/SWCLK |     |
|     | P015        | 38               |                | 23 P301           |     |
|     | P014        | 39               |                | 22 P302           |     |
|     | P013/VREFL  | 40               |                | 21 P200           |     |
|     | P012/VREFH  | 41               |                | 20 P201/MD        |     |
|     | AVCC0       | 42 R7FA4M1AB3CFL |                | 19 RES            |     |
|     | AVSS0       | 43               |                | 18 P206           |     |
|     | P011/VREFL0 | 44               |                | 17 VCC_USB_LDO    |     |
P010/VREFH0 VCC_USB
|     |      | 45  |     | 16             |     |
| --- | ---- | --- | --- | -------------- | --- |
|     | P002 | 46  |     | 15 P914/USB_DP |     |
|     | P001 | 47  |     | P915/USB_DM    |     |
14
|                 | P000                                      | 48                       |                                         | 13 VSS_USB     |                |
| --------------- | ----------------------------------------- | ------------------------ | --------------------------------------- | -------------- | -------------- |
|                 |                                           | 1 2 3 4                  | 5 6 7 8 9                               | 01 11 21       |                |
|                 |                                           | 004P TTABV LCV NICX/512P | TUOCX/412P SSV LATX/312P LATXE/212P CCV | 904P 804P 704P |                |
| Figure 1.7      | Pin assignment for 48-pin LQFP (top view) |                          |                                         |                |                |
| R01DS0355EJ0110 | Rev.1.10                                  |                          |                                         |                | Page 20 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | ----------- |
OIDWS/SMT/801P
OWS/ODT/901P
IDT/011P
|     |     | 001P 101P | 201P 301P 401P 211P 111P |     |     |
| --- | --- | --------- | ------------------------ | --- | --- |
SSV CCV
|     |      | 63 53 | 43 33 23 13 03 92 82 | 72 62 52          |     |
| --- | ---- | ----- | -------------------- | ----------------- | --- |
|     | P500 | 37    |                      | 24 P300/TCK/SWCLK |     |
|     | P015 | 38    | Exposed die pad      | 23 P301           |     |
P014 P302
|     |            | 39  |     | 22         |     |
| --- | ---------- | --- | --- | ---------- | --- |
|     | P013/VREFL | 40  |     | 21 P200    |     |
|     | P012/VREFH | 41  |     | 20 P201/MD |     |
|     | AVCC0      | 42  |     | 19 RES     |     |
R7FA4M1AB3CNE
|     | AVSS0       | 43  |     | 18 P206        |     |
| --- | ----------- | --- | --- | -------------- | --- |
|     | P011/VREFL0 | 44  |     | 17 VCC_USB_LDO |     |
|     | P010/VREFH0 | 45  |     | 16 VCC_USB     |     |
|     | P002        | 46  |     | 15 P914/USB_DP |     |
P001 P915/USB_DM
|                 |                                                   | 47         |                                                       | 14             |                |
| --------------- | ------------------------------------------------- | ---------- | ----------------------------------------------------- | -------------- | -------------- |
|                 | P000                                              | 48         |                                                       | 13 VSS_USB     |                |
|                 |                                                   | 1 2        | 3 4 5 6 7 8 9                                         | 01 11 21       |                |
|                 |                                                   | 004P TTABV | LCV NICX/512P TUOCX/412P SSV LATX/312P LATXE/212P CCV | 904P 804P 704P |                |
| Note:           | Exposed die pad is recommended to connect to VSS. |            |                                                       |                |                |
| Figure 1.8      | Pin assignment for 48-pin QFN (top view)          |            |                                                       |                |                |
| R01DS0355EJ0110 | Rev.1.10                                          |            |                                                       |                | Page 21 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | ----------- |
OIDWS/SMT/801P
OWS/ODT/901P
IDT/011P
001P 101P 201P SSV CCV 211P 111P
|     |      | 03 92 82 72 62 52 42 32 22 12 |                   |     |
| --- | ---- | ----------------------------- | ----------------- | --- |
|     | P015 | 31                            | 20 P300/TCK/SWCLK |     |
Exposed die pad
|     | P014       | 32  | 19 P301    |     |
| --- | ---------- | --- | ---------- | --- |
|     | P013/VREFL | 33  | 18 P200    |     |
|     | P012/VREFH | 34  | 17 P201/MD |     |
|     | AVCC0      | 35  | 16 RES     |     |
R7FA4M1AB3CNF
|     | AVSS0       | 36  | 15 VCC_USB_LDO |     |
| --- | ----------- | --- | -------------- | --- |
|     | P011/VREFL0 | 37  | 14 VCC_USB     |     |
|     | P010/VREFH0 | 38  | 13 P914/USB_DP |     |
|     | P001        | 39  | 12 P915/USB_DM |     |
|     | P000        | 40  | 11 VSS_USB     |     |
01
|                                                         |                                          | 1 2 3 4 5 6 7 8 9                                                     |     |                |
| ------------------------------------------------------- | ---------------------------------------- | --------------------------------------------------------------------- | --- | -------------- |
|                                                         |                                          | TTABV LCV NICX/512P TUOCX/412P SSV LATX/312P LATXE/212P CCV 804P 704P |     |                |
| Note: Exposed die pad is recommended to connect to VSS. |                                          |                                                                       |     |                |
| Figure 1.9                                              | Pin assignment for 40-pin QFN (top view) |                                                                       |     |                |
| R01DS0355EJ0110                                         | Rev.1.10                                 |                                                                       |     | Page 22 of 131 |
Sep 29, 2023

| RA4M1 Group   |     |     |     |     |     |     |     |     |     | 1. Overview |
| ------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------- |
| 1.7 Pin Lists |     |     |     |     |     |     |     |     |     |             |
Pin number ,kcolC ,metsyS ,rewoP Timers Communication interfaces Analogs HMI
TTABV ,CAC ,gubeD
GEOP ,SPO_TPG
PMAPO ,21CAD
NAC,SFBSU
| 001PFQL       |                          | tpurretnI strop O/I |     |     |     |     |      |       | PLPMCA |       |
| ------------- | ------------------------ | ------------------- | --- | --- | --- | --- | ---- | ----- | ------ | ----- |
| 001AGL 46PFQL | 46NFQ 84PFQL 84NFQ 04NFQ |                     |     |     |     |     |      | 41CDA |        | CDCLS |
|               |                          |                     |     |     |     |     | EISS |       |        | USTC  |
|               |                          |                     | TGA | TPG | CTR | ICS | IPS  |       |        |       |
CII
1 J10 1 1 1 1 CACREF IRQ0 P400 AGTIO1 GTIOC6A SCK0 SCL0 AUDIO_CL SEG04 TS20
|     |     |     |     |     |     | SCK1 | K   |     |     |     |
| --- | --- | --- | --- | --- | --- | ---- | --- | --- | --- | --- |
2 J9 2 2 IRQ5 P401 GTETRGA GTIOC6B CTX0 CTS0_ SDA0 SEG05 TS19
RTS0/
SS0
TXD1/
MOSI1/
SDA1
3 F6 3 3 VBATWIO0 IRQ4 P402 AGTIO0/ RTCIC0 CRX0 RXD1/ SEG06 TS18
|     |     |     | AGTIO1 |     |     | MISO1/ |     |     |     |     |
| --- | --- | --- | ------ | --- | --- | ------ | --- | --- | --- | --- |
SCL1
4 H10 VBATWIO1 P403 AGTIO0/ GTIOC3A RTCIC1 CTS1_ SSIBCK0 TS17
|     |     |     | AGTIO1 |     |     | RTS1/ |     |     |     |     |
| --- | --- | --- | ------ | --- | --- | ----- | --- | --- | --- | --- |
SS1
| 5 G8 | VBATWIO2 | P404 |     | GTIOC3B RTCIC2 |     |     | SSILRCK0/ |     |     |     |
| ---- | -------- | ---- | --- | -------------- | --- | --- | --------- | --- | --- | --- |
SSIFS0
| 6 H9       |             | P405      |         | GTIOC1A |     |       | SSITXD0 |     |     |     |
| ---------- | ----------- | --------- | ------- | ------- | --- | ----- | ------- | --- | --- | --- |
| 7 F7       |             | P406      |         | GTIOC1B |     |       | SSIRXD0 |     |     |     |
| 8 G9 4 4   | 2 2 1 VBATT |           |         |         |     |       |         |     |     |     |
| 9 G10 5 5  | 3 3 2 VCL   |           |         |         |     |       |         |     |     |     |
| 10 F10 6 6 | 4 4 3 XCIN  | P215      |         |         |     |       |         |     |     |     |
| 11 F9 7 7  | 5 5 4 XCOUT | P214      |         |         |     |       |         |     |     |     |
| 12 D9 8 8  | 6 6 5 VSS   |           |         |         |     |       |         |     |     |     |
| 13 E9 9 9  | 7 7 6 XTAL  | IRQ2 P213 | GTETRGA | GTIOC0A |     | TXD1/ |         |     |     |     |
MOSI1/
SDA1
| 14 E10 10 10 | 8 8 7 EXTAL | IRQ3 P212 | AGTEE1 GTETRGB | GTIOC0B |     | RXD1/ |     |     |     |     |
| ------------ | ----------- | --------- | -------------- | ------- | --- | ----- | --- | --- | --- | --- |
MISO1/
SCL1
| 15 D10 11 11 | 9 9 8 VCC |      |     |     |     |       |       |     |     |     |
| ------------ | --------- | ---- | --- | --- | --- | ----- | ----- | --- | --- | --- |
| 16 F8        |           | P708 |     |     |     | RXD1/ | SSLA3 |     |     |     |
MISO1/
SCL1
| 17 E8 |     | IRQ8 P415 |     | GTIOC0A |     |       | SSLA2 |     |     |     |
| ----- | --- | --------- | --- | ------- | --- | ----- | ----- | --- | --- | --- |
| 18 E7 |     | IRQ9 P414 |     | GTIOC0B |     |       | SSLA1 |     |     |     |
| 19 C9 |     | P413      |     |         |     | CTS0_ | SSLA0 |     |     |     |
RTS0/
SS0
| 20 C10 |     | P412 |     |     |     | SCK0 | RSPCKA |     |     |     |
| ------ | --- | ---- | --- | --- | --- | ---- | ------ | --- | --- | --- |
21 D8 12 12 IRQ4 P411 AGTOA1 GTOVUP GTIOC6A TXD0/ MOSIA SEG07 TS07
MOSI0/
SDA0
22 E6 13 13 IRQ5 P410 AGTOB1 GTOVLO GTIOC6B RXD0/ MISOA SEG08 TS06
MISO0/
SCL0
23 B10 14 14 10 10 IRQ6 P409 GTOWUP GTIOC5A USB_EXITXD9/ SEG09 TS05
CEN MOSI9/
SDA9
24 D7 15 15 11 11 9 IRQ7 P408 GTOWLO GTIOC5B USB_ID CTS1_ SCL0 SEG10 TS04
RTS1/
SS1
RXD9/
MISO9/
SCL9
25 A10 16 16 12 12 10 P407 AGTIO0 RTCOUT USB_VB CTS0_ SDA0 SSLB3 ADTRG0 SEG11 TS03
US RTS0/
SS0
| 26 B8 17 17     | 13 13 11 VSS_USB |      |     |     |        |     |     |     |                |     |
| --------------- | ---------------- | ---- | --- | --- | ------ | --- | --- | --- | -------------- | --- |
| 27 A9 18 18     | 14 14 12         | P915 |     |     | USB_DM |     |     |     |                |     |
| 28 B9 19 19     | 15 15 13         | P914 |     |     | USB_DP |     |     |     |                |     |
| R01DS0355EJ0110 | Rev.1.10         |      |     |     |        |     |     |     | Page 23 of 131 |     |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     | 1. Overview |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | ----------- | --- |
Pin number ,kcolC ,metsyS ,rewoP Timers Communication interfaces Analogs HMI
TTABV ,CAC ,gubeD
GEOP ,SPO_TPG
PMAPO ,21CAD
NAC,SFBSU
| 001PFQL 001AGL | 46PFQL 84PFQL |             | tpurretnI strop O/I |         |     |         |       | PLPMCA |      |
| -------------- | ------------- | ----------- | ------------------- | ------- | --- | ------- | ----- | ------ | ---- |
|                | 46NFQ 84NFQ   | 04NFQ       |                     |         |     |         | 41CDA | CDCLS  |      |
|                |               |             | TGA                 |         |     |         | EISS  |        | USTC |
|                |               |             |                     | TPG CTR | ICS | CII IPS |       |        |      |
| 29 A8          | 20 20 16 16   | 14 VCC_USB  |                     |         |     |         |       |        |      |
| 30 C8          | 21 21 17 17   | 15 VCC_USB_ |                     |         |     |         |       |        |      |
LDO
31 C7 22 22 18 18 IRQ0 P206 GTIU USB_VB RXD0/ SDA1 SSLB1 SEG12 TS01
USEN MISO0/
SCL0
32 A7 23 23 CLKOUT IRQ1 P205 AGTO1 GTIV GTIOC4A USB_OV TXD0/ SCL1 SSLB0 SEG13 TSCAP
RCURA MOSI0/
SDA0
CTS9_
RTS9/
SS9
33 B7 24 24 CACREF P204 AGTIO1 GTIW GTIOC4B USB_OV SCK0 SCL0 RSPCKB SEG14 TS00
RCURB SCK9
| 34 D6 |     |     | P203 | GTIOC5A | CTS2_ | MOSIB |     | SEG15 | TSCAP |
| ----- | --- | --- | ---- | ------- | ----- | ----- | --- | ----- | ----- |
RTS2/
SS2
TXD9/
MOSI9/
SDA9
| 35 C6 |     |     | P202 | GTIOC5B | SCK2 | MISOB |     | SEG16 |     |
| ----- | --- | --- | ---- | ------- | ---- | ----- | --- | ----- | --- |
RXD9/
MISO9/
SCL9
| 36 A6 |             | VSS    |      |         |     |     |     |            |      |
| ----- | ----------- | ------ | ---- | ------- | --- | --- | --- | ---------- | ---- |
| 37 B6 |             | VCC    |      |         |     |     |     |            |      |
| 38 D5 | 25 25 19 19 | 16 RES |      |         |     |     |     |            |      |
| 39 B5 | 26 26 20 20 | 17 MD  | P201 |         |     |     |     |            |      |
| 40 A5 | 27 27 21 21 | 18 NMI | P200 |         |     |     |     |            |      |
| 41 C5 |             |        | P307 |         |     |     |     | SEG17      |      |
| 42 D4 |             |        | P306 |         |     |     |     | SEG18      |      |
| 43 A4 |             | IRQ8   | P305 |         |     |     |     | SEG19      |      |
| 44 B4 | 28 28       | IRQ9   | P304 | GTIOC7A |     |     |     | SEG20      | TS11 |
| 45 C4 |             |        | P808 |         |     |     |     | SEG21      |      |
| 46 A3 |             |        | P809 |         |     |     |     | SEG22      |      |
| 47 B3 | 29 29       |        | P303 | GTIOC7B |     |     |     | SEG03/TS02 |      |
COM7
48 B2 30 30 22 22 IRQ5 P302 GTOUUP GTIOC4A TXD2/ SSLB3 SEG02/TS08
|     |     |     |     |     | MOSI2/ |     |     | COM6 |     |
| --- | --- | --- | --- | --- | ------ | --- | --- | ---- | --- |
SDA2
49 C2 31 31 23 23 19 IRQ6 P301 AGTIO0 GTOULO GTIOC4B RXD2/ SSLB2 SEG01/TS09
|     |     |     |     |     | MISO2/ |     |     | COM5 |     |
| --- | --- | --- | --- | --- | ------ | --- | --- | ---- | --- |
SCL2
CTS9_
RTS9/
SS9
| 50 A2 | 32 32 24 24 | 20 TCK/ | P300 GTOUUP | GTIOC0A |     | SSLB1 |     |     |     |
| ----- | ----------- | ------- | ----------- | ------- | --- | ----- | --- | --- | --- |
SWCLK
| 51 A1 | 33 33 25 25 | 21 TMS/ | P108 GTOULO | GTIOC0B | CTS9_ | SSLB0 |     |     |     |
| ----- | ----------- | ------- | ----------- | ------- | ----- | ----- | --- | --- | --- |
|       |             | SWDIO   |             |         | RTS9/ |       |     |     |     |
SS9
52 B1 34 34 26 26 22 TDO/SWO/ P109 GTOVUP GTIOC1A CTX0 SCK1 MOSIB SEG23 TS10
|     |     | CLKOUT |     |     | TXD9/ |     |     |     |     |
| --- | --- | ------ | --- | --- | ----- | --- | --- | --- | --- |
MOSI9/
SDA9
53 C3 35 35 27 27 23 TDI IRQ3 P110 GTOVLO GTIOC1B CRX0 CTS2_ MISOB VCOUT SEG24
RTS2/
SS2
RXD9/
MISO9/
SCL9
| R01DS0355EJ0110 |     | Rev.1.10 |     |     |     |     |     | Page 24 of 131 |     |
| --------------- | --- | -------- | --- | --- | --- | --- | --- | -------------- | --- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------- |
Pin number ,kcolC ,metsyS ,rewoP Timers Communication interfaces Analogs HMI
TTABV ,CAC ,gubeD
GEOP ,SPO_TPG
PMAPO ,21CAD
NAC,SFBSU
| 001PFQL 001AGL | 46PFQL 84PFQL |       | tpurretnI strop O/I |     |         |         |         |       | PLPMCA |           |
| -------------- | ------------- | ----- | ------------------- | --- | ------- | ------- | ------- | ----- | ------ | --------- |
|                | 46NFQ 84NFQ   | 04NFQ |                     |     |         |         |         | 41CDA |        | CDCLS     |
|                |               |       |                     | TGA |         |         |         | EISS  |        | USTC      |
|                |               |       |                     |     | TPG     | CTR ICS | CII IPS |       |        |           |
| 54 D3          | 36 36 28 28   | 24    | IRQ4 P111           |     | GTIOC3A | SCK2    | RSPCKB  |       |        | CAPH TS12 |
SCK9
55 C1 37 37 29 29 25 P112 GTIOC3B TXD2/ SSLB0 SSIBCK0 CAPL TSCAP
MOSI2/
SDA2
SCK1
| 56 E5 | 38 38       |        | P113 |     | GTIOC2A |       |     | SSILRCK0/ |     | SEG00/TS27 |
| ----- | ----------- | ------ | ---- | --- | ------- | ----- | --- | --------- | --- | ---------- |
|       |             |        |      |     |         |       |     | SSIFS0    |     | COM4       |
| 57 D2 |             |        | P114 |     | GTIOC2B |       |     | SSIRXD0   |     | SEG25 TS29 |
| 58 E4 |             |        | P115 |     | GTIOC4A |       |     | SSITXD0   |     | SEG26 TS35 |
| 59 D1 |             |        | P608 |     | GTIOC4B |       |     |           |     | SEG27      |
| 60 E3 |             |        | P609 |     | GTIOC5A |       |     |           |     | SEG28      |
| 61 E2 |             |        | P610 |     | GTIOC5B |       |     |           |     | SEG29      |
| 62 E1 | 39 39 30 30 | 26 VCC |      |     |         |       |     |           |     |            |
| 63 F1 | 40 40 31 31 | 27 VSS |      |     |         |       |     |           |     |            |
| 64 F2 |             |        | P603 |     | GTIOC7A | CTS9_ |     |           |     | SEG30      |
RTS9/
SS9
| 65 F3 |     |     | P602 |     | GTIOC7B | TXD9/ |     |     |     | SEG31 |
| ----- | --- | --- | ---- | --- | ------- | ----- | --- | --- | --- | ----- |
MOSI9/
SDA9
| 66 F4 |     |     | P601 |     | GTIOC6A | RXD9/ |     |     |     | SEG32 |
| ----- | --- | --- | ---- | --- | ------- | ----- | --- | --- | --- | ----- |
MISO9/
SCL9
| 67 F5 |       |     | P600       |         | GTIOC6B | SCK9 |       |     |     | SEG33     |
| ----- | ----- | --- | ---------- | ------- | ------- | ---- | ----- | --- | --- | --------- |
| 68 G3 | 41 41 |     | KR07 P107  |         | GTIOC0A |      |       |     |     | COM3      |
| 69 G2 | 42 42 |     | KR06 P106  |         | GTIOC0B |      | SSLA3 |     |     | COM2      |
| 70 G1 | 43 43 |     | KR05/ P105 | GTETRGA | GTIOC1A |      | SSLA2 |     |     | COM1 TS34 |
IRQ0
71 H1 44 44 32 32 KR04/ P104 GTETRGB GTIOC1B RXD0/ SSLA1 COM0 TS13
|     |     |     | IRQ1 |     |     | MISO0/ |     |     |     |     |
| --- | --- | --- | ---- | --- | --- | ------ | --- | --- | --- | --- |
SCL0
72 H3 45 45 33 33 KR03 P103 GTOWUP GTIOC2A CTX0 CTS0_ SSLA0 AN019 CMPREF1 VL4
RTS0/
SS0
73 J1 46 46 34 34 28 KR02 P102 AGTO0 GTOWLO GTIOC2B CRX0 SCK0 RSPCKA AN020/ CMPIN1 VL3
|     |     |     |     |     |     | TXD2/ |     | ADTRG0 |     |     |
| --- | --- | --- | --- | --- | --- | ----- | --- | ------ | --- | --- |
MOSI2/
SDA2
74 H2 47 47 35 35 29 KR01/ P101 AGTEE0 GTETRGB GTIOC5A TXD0/ SDA1 MOSIA AN021 CMPREF0 VL2
|     |     |     | IRQ1 |     |     | MOSI0/ |     |     |     |     |
| --- | --- | --- | ---- | --- | --- | ------ | --- | --- | --- | --- |
SDA0
CTS1_
RTS1/
SS1
75 H4 48 48 36 36 30 KR00/ P100 AGTIO0 GTETRGA GTIOC5B RXD0/ SCL1 MISOA AN022 CMPIN0 VL1
|     |     |     | IRQ2 |     |     | MISO0/ |     |     |     |     |
| --- | --- | --- | ---- | --- | --- | ------ | --- | --- | --- | --- |
SCL0
SCK1
76 K1 49 49 37 37 P500 AGTOA0 GTIU GTIOC2A USB_VB AN016 CMPREF1 SEG34
USEN
77 J2 50 50 IRQ11 P501 AGTOB0 GTIV GTIOC2B USB_OV TXD1/ AN017 CMPIN1 SEG35
RCURA MOSI1/
SDA1
78 K2 51 51 IRQ12 P502 GTIW GTIOC3B USB_OV RXD1/ AN018 CMPREF0 SEG36
RCURB MISO1/
SCL1
| 79 G4 |     |     | P503 |     |     | USB_EXISCK1 |     | AN023 | CMPIN0 | SEG37 |
| ----- | --- | --- | ---- | --- | --- | ----------- | --- | ----- | ------ | ----- |
CEN
| 80 G5 |     |     | P504 |     |     | USB_ID CTS1_ |     | AN024 |     |     |
| ----- | --- | --- | ---- | --- | --- | ------------ | --- | ----- | --- | --- |
RTS1/
SS1
| 81 G6           |     |          | IRQ14 P505 |     |     |     |     | AN025 |                |     |
| --------------- | --- | -------- | ---------- | --- | --- | --- | --- | ----- | -------------- | --- |
| 82 K3           |     | VCC      |            |     |     |     |     |       |                |     |
| R01DS0355EJ0110 |     | Rev.1.10 |            |     |     |     |     |       | Page 25 of 131 |     |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     | 1. Overview |
| ----------- | --- | --- | --- | --- | --- | --- | ----------- |
Pin number ,kcolC ,metsyS ,rewoP Timers Communication interfaces Analogs HMI
TTABV ,CAC ,gubeD
GEOP ,SPO_TPG
PMAPO ,21CAD
NAC,SFBSU
| 001PFQL 001AGL | 46PFQL 84PFQL |       | tpurretnI strop O/I |     |       | PLPMCA |       |
| -------------- | ------------- | ----- | ------------------- | --- | ----- | ------ | ----- |
|                | 46NFQ 84NFQ   | 04NFQ |                     |     | 41CDA |        | CDCLS |
|                |               |       |                     | TGA | EISS  |        | USTC  |
TPG CTR ICS CII IPS
| 83 J3           |             | VSS       |            |     |             |                |      |
| --------------- | ----------- | --------- | ---------- | --- | ----------- | -------------- | ---- |
| 84 J4           | 52 52 38 38 | 31        | IRQ7 P015  |     | AN010       |                | TS28 |
| 85 K4           | 53 53 39 39 | 32        | P014       |     | AN009 DA0   |                |      |
| 86 J5           | 54 54 40 40 | 33 VREFL  | P013       |     | AN008 AMP1+ |                |      |
| 87 K5           | 55 55 41 41 | 34 VREFH  | P012       |     | AN007 AMP1- |                |      |
| 88 H5           | 56 56 42 42 | 35 AVCC0  |            |     |             |                |      |
| 89 H6           | 57 57 43 43 | 36 AVSS0  |            |     |             |                |      |
| 90 J6           | 58 58 44 44 | 37 VREFL0 | IRQ15 P011 |     | AN006 AMP2+ |                | TS31 |
| 91 K6           | 59 59 45 45 | 38 VREFH0 | P010       |     | AN005 AMP2- |                | TS30 |
| 92 J7           |             |           | P008       |     | AN014       |                |      |
| 93 H7           |             |           | P007       |     | AN013 AMP3O |                |      |
| 94 G7           |             |           | P006       |     | AN012 AMP3- |                |      |
| 95 K7           |             |           | IRQ10 P005 |     | AN011 AMP3+ |                |      |
| 96 J8           | 60 60       |           | IRQ3 P004  |     | AN004 AMP2O |                |      |
| 97 H8           | 61 61       |           | P003       |     | AN003 AMP1O |                |      |
| 98 K8           | 62 62 46 46 |           | IRQ2 P002  |     | AN002 AMP0O |                |      |
| 99 K9           | 63 63 47 47 | 39        | IRQ7 P001  |     | AN001 AMP0- |                | TS22 |
| 100 K10         | 64 64 48 48 | 40        | IRQ6 P000  |     | AN000 AMP0+ |                | TS21 |
| R01DS0355EJ0110 |             | Rev.1.10  |            |     |             | Page 26 of 131 |      |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
2. Electrical Characteristics
Unless otherwise specified, the electrical characteristics of the MCU are defined under the following conditions:
VCC*1 = AVCC0 = VCC_USB*2 = VCC_USB_LDO*2 = 1.6 to 5.5V, VREFH = VREFH0 = 1.6 to AVCC0, VBATT =
1.6 to 3.6V, VSS = AVSS0 = VREFL = VREFL0 = VSS_USB = 0V, T = T .
a opr
Note 1. The typical condition is set to VCC = 3.3V.
Note 2. When USBFS is not used.
Figure 2.1 shows the timing conditions.
For example P100
C
V = VCC × 0.7, V = VCC × 0.3
OH OL
V = VCC × 0.7, V = VCC × 0.3
IH IL
Load capacitance C = 30 pF
Figure 2.1 Input or output timing measurement conditions
The recommended measurement conditions for the timing specification of each peripheral provided are for the best
peripheral operation. Make sure to adjust the driving abilities of each pin to meet your conditions.
Each function pin used for the same function must select the same drive ability. If the I/O drive ability of each function
pin is mixed, the AC specification of each function is not guaranteed.
R01DS0355EJ0110 Rev.1.10 Page 27 of 131
Sep 29, 2023

| RA4M1 Group |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | ----------------------------- | --- |
2.1 Absolute Maximum Ratings
| Table 2.1 Absolute maximum ratings |                      |        |              |      |
| ---------------------------------- | -------------------- | ------ | ------------ | ---- |
| Parameter                          |                      | Symbol | Value        | Unit |
| Power supply voltage               |                      | VCC    | -0.5 to +6.5 | V    |
| Input voltage                      | 5 V-tolerant ports*1 | V      | -0.3 to +6.5 | V    |
in
|     | P000 to P008, P010 to P015 | V   | -0.3 to AVCC0 + 0.3 | V   |
| --- | -------------------------- | --- | ------------------- | --- |
in
|     | Others | V   | -0.3 to VCC + 0.3 | V   |
| --- | ------ | --- | ----------------- | --- |
in
| Reference power supply voltage |     | VREFH0      | -0.3 to +6.5 | V   |
| ------------------------------ | --- | ----------- | ------------ | --- |
|                                |     | VREFH       |              | V   |
| VBATT power supply voltage     |     | VBATT       | -0.5 to +6.5 | V   |
| Analog power supply voltage    |     | AVCC0       | -0.5 to +6.5 | V   |
| USB power supply voltage       |     | VCC_USB     | -0.5 to +6.5 | V   |
|                                |     | VCC_USB_LDO | -0.5 to +6.5 | V   |
Analog input voltage When AN000 to AN014 are  V AN -0.3 to AVCC0 + 0.3 V
used
|     | When AN016 to AN025 are  |     | -0.3 to VCC + 0.3 | V   |
| --- | ------------------------ | --- | ----------------- | --- |
used
| LCD voltage | VL1 voltage | V   | -0.3 to +2.8 | V   |
| ----------- | ----------- | --- | ------------ | --- |
L1
|     | VL2 voltage | V   | -0.3 to +6.5 | V   |
| --- | ----------- | --- | ------------ | --- |
L2
|     | VL3 voltage | V   | -0.3 to +6.5 | V   |
| --- | ----------- | --- | ------------ | --- |
L3
|     | VL4 voltage | V   | -0.3 to +6.5 | V   |
| --- | ----------- | --- | ------------ | --- |
L4
| Operating temperature*2,*3,*4 |     | T   | -40 to +105 | °C  |
| ----------------------------- | --- | --- | ----------- | --- |
opr
-40 to +85
| Storage temperature |     | T   | -55 to +125 | °C  |
| ------------------- | --- | --- | ----------- | --- |
stg
Caution: Permanent damage to the MCU may result if absolute maximum ratings are exceeded.
To preclude any malfunctions due to noise interference, insert capacitors of high frequency characteristics
between the VCC and VSS pins, between the AVCC0 and AVSS0 pins, between the VCC_USB and VSS_USB pins,
between the VREFH0 and VREFL0 pins, and between the VREFH and VREFL pins. Place capacitors of about 0.1 μF
as close as possible to every power supply pin and use the shortest and heaviest possible traces. Also, connect
capacitors as stabilization capacitance.
Connect the VCL pin to a VSS pin by a 4.7 µF capacitor. The capacitor must be placed close to the pin.
Do not input signals or an I/O pull-up power supply while the device is not powered. The current injection that
results from input of such a signal or I/O pull-up might cause malfunction and the abnormal current that passes in
the device at this time might cause degradation of internal elements.
Note 1. Ports P205, P206, P400 to P404, P407, P408 are 5 V tolerant.
Note 2. See section 2.2.1, Tj/Ta Definition.
Note 3. Contact a Renesas Electronics sales office for information on derating operation under T  = +85°C to +105°C. Derating is the
a
systematic reduction of load for improved reliability.
Note 4. The upper limit of operating temperature is +85°C or +105°C, depending on the product. For details, see section 1.3, Part
Numbering.
| R01DS0355EJ0110 | Rev.1.10 |     |     | Page 28 of 131 |
| --------------- | -------- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group                                |        |       |         | 2. Electrical Characteristics |
| ------------------------------------------ | ------ | ----- | ------- | ----------------------------- |
| Table 2.2 Recommended operating conditions |        |       |         |                               |
| Parameter                                  | Symbol | Value | Min Typ | Max Unit                      |
Power supply voltages VCC*1, *2 When USBFS is not  1.6 - 5.5 V
used
|     |     | When USBFS is used | VCC_USB - | 3.6 V |
| --- | --- | ------------------ | --------- | ----- |
USB Regulator
Disable
|     |     | When USBFS is used | VCC_USB - | 5.5 V |
| --- | --- | ------------------ | --------- | ----- |
|     |     | USB Regulator      | _LDO      |       |
Enable
|     | VSS |     | - 0 | - V |
| --- | --- | --- | --- | --- |
USB power supply voltages VCC_USB When USBFS is not  - VCC - V
used
|     |     | When USBFS is used | 3.0 3.3 | 3.6 V |
| --- | --- | ------------------ | ------- | ----- |
USB Regulator
Disable
(Input)
|     | VCC_USB_LDO | When USBFS is not  | - VCC | - V |
| --- | ----------- | ------------------ | ----- | --- |
used
|     |     | When USBFS is used  | - VCC | - V |
| --- | --- | ------------------- | ----- | --- |
USB Regulator
Disable
|     |     | When USBFS is used | 3.8 - | 5.5 V |
| --- | --- | ------------------ | ----- | ----- |
USB Regulator
Enable
|                            | VSS_USB |                   | - 0   | - V |
| -------------------------- | ------- | ----------------- | ----- | --- |
| VBATT power supply voltage | VBATT   | When the battery  | - VCC | - V |
backup function is not
used
|     |     | When the battery  | 1.6 - | 3.6 V |
| --- | --- | ----------------- | ----- | ----- |
backup function is
used
| Analog power supply voltages | AVCC0*1, *2 |               | 1.6 - | 5.5 V   |
| ---------------------------- | ----------- | ------------- | ----- | ------- |
|                              | AVSS0       |               | - 0   | - V     |
|                              | VREFH0      | When used as  | 1.6 - | AVCC0 V |
ADC14 Reference
|     | VREFL0 |               | - 0   | - V     |
| --- | ------ | ------------- | ----- | ------- |
|     | VREFH  | When used as  | 1.6 - | AVCC0 V |
DAC12 Reference
|     | VREFL |     | - 0 | - V |
| --- | ----- | --- | --- | --- |
Note 1. Use AVCC0 and VCC under the following conditions:
AVCC0 and VCC can be set individually within the operating range when VCC ≥ 2.2 V and AVCC0 ≥ 2.2 V.
AVCC0 = VCC when VCC < 2.2 V or AVCC0 < 2.2 V.
Note 2. When powering on the VCC and AVCC0 pins, power them on at the same time, or power the VCC pin first and then the AVCC0
pin.
| R01DS0355EJ0110 | Rev.1.10 |     |     | Page 29 of 131 |
| --------------- | -------- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |                    |                  |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ------------------ | ---------------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
| 2.2         | DC Characteristics |                  |     |     |     |     |     |                               |     |
| 2.2.1       |                    | Tj/T  Definition |     |     |     |     |     |                               |     |
a
| Table 2.3                                          |     | DC Characteristics |        |                   |     |      |                 |     |     |
| -------------------------------------------------- | --- | ------------------ | ------ | ----------------- | --- | ---- | --------------- | --- | --- |
| Conditions: Products with operating temperature (T |     |                    |        | a ) -40 to +105°C |     |      |                 |     |     |
| Parameter                                          |     |                    | Symbol | Typ               | Max | Unit | Test conditions |     |     |
| Permissible junction temperature                   |     |                    | Tj     | -                 | 125 | °C   | High-speed mode |     |     |
Middle-speed mode
105*1
Low-voltage mode
Low-speed mode
Subosc-speed mode
| Note: | Make sure that Tj = T |  + θja × total power consumption (W),  |     |     |     |     |     |     |     |
| ----- | --------------------- | -------------------------------------- | --- | --- | --- | --- | --- | --- | --- |
a
|     | where total power consumption = (VCC - V |     |     | OH ) × ΣI OH |  + V OL  × ΣI OL |  + I CC max × VCC. |     |     |     |
| --- | ---------------------------------------- | --- | --- | ------------ | ---------------- | ------------------ | --- | --- | --- |
Note 1. The upper limit of operating temperature is +85°C or +105°C, depending on the product. For details, see section 1.3, Part
Numbering. If the part number shows the operation temperature at 85°C, then the maximum value of T is +105°C, otherwise, it
j
is +125°C.
| 2.2.2     |     | I/O V , V      |     |     |     |     |     |     |     |
| --------- | --- | -------------- | --- | --- | --- | --- | --- | --- | --- |
|           |     | IH IL          |     |     |     |     |     |     |     |
| Table 2.4 |     | I/O V , V  (1) |     |     |     |     |     |     |     |
IH IL
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 2.7 to 5.5V, VBATT = 1.6 to 3.6 V, VSS = AVSS0 = 0 V
| Parameter |     |     |     | Symbol | Min | Typ Max |     | Unit | Test conditions |
| --------- | --- | --- | --- | ------ | --- | ------- | --- | ---- | --------------- |
Schmitt trigger  IIC*1 (except for SMBus) V VCC × 0.7 - 5.8 V -
IH
input voltage
|     |     |     |     | V   | -   | - VCC × 0.3 |     |     |     |
| --- | --- | --- | --- | --- | --- | ----------- | --- | --- | --- |
IL
|     |     |                              |     | ΔV T | VCC × 0.05 | - -         |     |     |     |
| --- | --- | ---------------------------- | --- | ---- | ---------- | ----------- | --- | --- | --- |
|     |     | RES, NMI                     |     | V IH | VCC × 0.8  | - -         |     |     |     |
|     |     | Other peripheral input pins  |     | V    | -          | - VCC × 0.2 |     |     |     |
IL
excluding IIC
|     |     |     |     | ΔV  | VCC × 0.1 | - - |     |     |     |
| --- | --- | --- | --- | --- | --------- | --- | --- | --- | --- |
T
| Input voltage  |     | IIC (SMBus)*2 |     | V   | 2.2 | - - |     |     | VCC = 3.6 to 5.5 V |
| -------------- | --- | ------------- | --- | --- | --- | --- | --- | --- | ------------------ |
IH
(except for
|                  |     |     |     | V    | 2.0 | - -   |     |     | VCC = 2.7 to 3.6 V |
| ---------------- | --- | --- | --- | ---- | --- | ----- | --- | --- | ------------------ |
| Schmitt trigger  |     |     |     | IH   |     |       |     |     |                    |
| input pin)       |     |     |     | V IL | -   | - 0.8 |     |     | -                  |
5 V-tolerant ports*3
|     |     |     |     | V IH | VCC × 0.8 | - 5.8       |     |     |     |
| --- | --- | --- | --- | ---- | --------- | ----------- | --- | --- | --- |
|     |     |     |     | V    | -         | - VCC × 0.2 |     |     |     |
IL
|     |     | P914, P915 |     | V   | VCC_USB × 0.8 | - VCC_USB + 0.3 |     |     |     |
| --- | --- | ---------- | --- | --- | ------------- | --------------- | --- | --- | --- |
IH
|     |     |     |     | V   | -   | - VCC_USB × 0.2 |     |     |     |
| --- | --- | --- | --- | --- | --- | --------------- | --- | --- | --- |
IL
|     |     | P000 to P008, P010 to P015 |     | V IH | AVCC0 × 0.8 | - -           |     |     |     |
| --- | --- | -------------------------- | --- | ---- | ----------- | ------------- | --- | --- | --- |
|     |     |                            |     | V IL | -           | - AVCC0 × 0.2 |     |     |     |
|     |     | EXTAL                      |     | V    | VCC × 0.8   | - -           |     |     |     |
IH
Input ports pins except for
|     |     |                        |     | V   | -   | - VCC × 0.2 |     |     |     |
| --- | --- | ---------------------- | --- | --- | --- | ----------- | --- | --- | --- |
|     |     | P000 to P008, P010 to  |     | IL  |     |             |     |     |     |
P015, P914, P915
| When V |      |   P402, P403, P404 |     | V   | V  × 0.8 | - V  |  + 0.3 |     |     |
| ------ | ---- | ------------------ | --- | --- | -------- | ---- | ------ | --- | --- |
|        | BATT |                    |     | IH  | BATT     | BATT |        |     |     |
power supply is
|                 |                                                                      |          |     | V   | -         | - V  |  × 0.2 |     |                |
| --------------- | -------------------------------------------------------------------- | -------- | --- | --- | --------- | ---- | ------ | --- | -------------- |
| selected        |                                                                      |          |     | IL  |           | BATT |        |     |                |
|                 |                                                                      |          |     | ΔV  | V  × 0.05 | - -  |        |     |                |
|                 |                                                                      |          |     | T   | BATT      |      |        |     |                |
| Note 1.         | P205, P206, P400, P401, P407, P408 (total 6 pins).                   |          |     |     |           |      |        |     |                |
| Note 2.         | P100, P101, P204, P205, P206, P400, P401, P407, P408 (total 9 pins). |          |     |     |           |      |        |     |                |
| Note 3.         | P205, P206, P400 to P404, P407, P408 (total 9 pins).                 |          |     |     |           |      |        |     |                |
| R01DS0355EJ0110 |                                                                      | Rev.1.10 |     |     |           |      |        |     | Page 30 of 131 |
Sep 29, 2023

| RA4M1 Group |                |     |     |     | 2. Electrical Characteristics |     |
| ----------- | -------------- | --- | --- | --- | ----------------------------- | --- |
| Table 2.5   | I/O V , V  (2) |     |     |     |                               |     |
IH IL
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 1.6 to 2.7 V, VBATT = 1.6 to 3.6 V, VSS = AVSS0 = 0 V
Test
| Parameter        |          | Symbol Min  |     | Typ Max |     | Unit conditions |
| ---------------- | -------- | ----------- | --- | ------- | --- | --------------- |
| Schmitt trigger  | RES, NMI | V VCC × 0.8 |     | - -     |     | V -             |
IH
| input voltage | Peripheral input pins |     |     |             |     |     |
| ------------- | --------------------- | --- | --- | ----------- | --- | --- |
|               |                       | V - |     | - VCC × 0.2 |     |     |
IL
|     |     | ΔV T VCC × 0.01 |     | - - |     |     |
| --- | --- | --------------- | --- | --- | --- | --- |
5 V-tolerant ports*1
| Input voltage  |     | V IH VCC × 0.8 |     | - 5.8       |     |     |
| -------------- | --- | -------------- | --- | ----------- | --- | --- |
| (except for    |     | V -            |     | - VCC × 0.2 |     |     |
IL
Schmitt trigger
|            | P914, P915 | V VCC_USB × 0.8 |     | - VCC_USB + 0.3 |     |     |
| ---------- | ---------- | --------------- | --- | --------------- | --- | --- |
| input pin) |            | IH              |     |                 |     |     |
|            |            | V -             |     | - VCC_USB × 0.2 |     |     |
IL
|     | P000 to P008, P010 to P015 | V AVCC0 × 0.8 |     | - - |     |     |
| --- | -------------------------- | ------------- | --- | --- | --- | --- |
IH
|     |                              | V IL -         |     | - AVCC0 × 0.2 |     |     |
| --- | ---------------------------- | -------------- | --- | ------------- | --- | --- |
|     | EXTAL                        | V IH VCC × 0.8 |     | - -           |     |     |
|     | Input ports pins except for  | V -            |     | - VCC × 0.2   |     |     |
IL
P000 to P008, P010 to P015,
P914, P915
| When V | P402, P403, P404 | V V |  × 0.8 | - V |  + 0.3 |     |
| ------ | ---------------- | --- | ------ | --- | ------ | --- |
| BATT   |                  | IH  | BATT   |     | BATT   |     |
power supply is
|          |     | V -  |         | - V |  × 0.2 |     |
| -------- | --- | ---- | ------- | --- | ------ | --- |
| selected |     | IL   |         |     | BATT   |     |
|          |     | ΔV V |  × 0.01 | - - |        |     |
|          |     | T    | BATT    |     |        |     |
Note 1. P205, P206, P400 to P404, P407, P408 (total 9 pins)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 31 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | ----------------------------- | --- |
| 2.2.3 I/O I | , I |     |     |                               |     |
OH OL
| Table 2.6 I/O I | , I  (1 of 2) |     |     |     |     |
| --------------- | ------------- | --- | --- | --- | --- |
OH OL
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 1.6 to 5.5 V
| Parameter |     |     | Symbol Min | Typ | Max Unit |
| --------- | --- | --- | ---------- | --- | -------- |
Permissible output current  Ports P212, P213 - I OH - - -4.0 mA
(average value per pin)
|     |     |     | I - | -   | 4.0 mA |
| --- | --- | --- | --- | --- | ------ |
OL
|     | Port P408 | Low drive*1 | I - | -   | -4.0 mA |
| --- | --------- | ----------- | --- | --- | ------- |
OH
|     |     |     | I - | -   | 4.0 mA |
| --- | --- | --- | --- | --- | ------ |
OL
|     |     | Middle drive for IIC  | I -    | -   | -8.0 mA  |
| --- | --- | --------------------- | ------ | --- | -------- |
|     |     | Fast-mode*4           | OH     |     |          |
|     |     | VCC = 2.7 to 5.5 V    | I OL - | -   | 8.0 mA   |
|     |     | Middle drive*2        | I OH - | -   | -20.0 mA |
VCC = 3.0 to 5.5 V
|     |     |     | I - | -   | 20.0 mA |
| --- | --- | --- | --- | --- | ------- |
OL
|     | Port P409 | Low drive*1 | I - | -   | -4.0 mA |
| --- | --------- | ----------- | --- | --- | ------- |
OH
|     |     |     | I - | -   | 4.0 mA |
| --- | --- | --- | --- | --- | ------ |
OL
|     |     | Middle drive*2  | I - | -   | -8.0 mA |
| --- | --- | --------------- | --- | --- | ------- |
OH
VCC = 2.7 to 3.0 V
|     |     |     | I OL - | -   | 8.0 mA |
| --- | --- | --- | ------ | --- | ------ |
Middle drive*2
|     |     |     | I OH - | -   | -20.0 mA |
| --- | --- | --- | ------ | --- | -------- |
VCC = 3.0 to 5.5 V
|     |     |     | I - | -   | 20.0 mA |
| --- | --- | --- | --- | --- | ------- |
OL
|     | Ports P100 to P115,  | Low drive*1 | I - | -   | -4.0 mA |
| --- | -------------------- | ----------- | --- | --- | ------- |
OH
P201 to P204, P300 to P307,
|     | P500 to P503, P600 to P603,  |     | I - | -   | 4.0 mA |
| --- | ---------------------------- | --- | --- | --- | ------ |
OL
P608 to P610, P808, P809
|     |                  | Middle drive*2 | I -    | -   | -4.0 mA |
| --- | ---------------- | -------------- | ------ | --- | ------- |
|     | (total 41 pins)  |                | OH     |     |         |
|     |                  |                | I OL - | -   | 8.0 mA  |
|     | Ports P914, P915 | -              | I OH - | -   | -4.0 mA |
|     |                  |                | I -    | -   | 4.0 mA  |
OL
|     | Other output pin*3  | Low drive*1 | I - | -   | -4.0 mA |
| --- | ------------------- | ----------- | --- | --- | ------- |
OH
|     |     |     | I - | -   | 4.0 mA |
| --- | --- | --- | --- | --- | ------ |
OL
|                 |          | Middle drive*2 | I OH - | -   | -8.0 mA        |
| --------------- | -------- | -------------- | ------ | --- | -------------- |
|                 |          |                | I OL - | -   | 8.0 mA         |
| R01DS0355EJ0110 | Rev.1.10 |                |        |     | Page 32 of 131 |
Sep 29, 2023

| RA4M1 Group     |               |     |     | 2. Electrical Characteristics |     |
| --------------- | ------------- | --- | --- | ----------------------------- | --- |
| Table 2.6 I/O I | , I  (2 of 2) |     |     |                               |     |
OH OL
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 1.6 to 5.5 V
| Parameter                   |                  |     | Symbol Min | Typ | Max Unit |
| --------------------------- | ---------------- | --- | ---------- | --- | -------- |
| Permissible output current  | Ports P212, P213 | -   | I -        | -   | -4.0 mA  |
OH
(Max value per pin)
|     |     |     | I OL - | -   | 4.0 mA |
| --- | --- | --- | ------ | --- | ------ |
Low drive*1
|     | Port P408 |     | I OH - | -   | -4.0 mA |
| --- | --------- | --- | ------ | --- | ------- |
|     |           |     | I -    | -   | 4.0 mA  |
OL
|     |     | Middle drive for IIC  | I - | -   | -8.0 mA |
| --- | --- | --------------------- | --- | --- | ------- |
OH
Fast-mode*4
|     |     | VCC = 2.7 to 5.5 V | I - | -   | 8.0 mA |
| --- | --- | ------------------ | --- | --- | ------ |
OL
|     |     | Middle drive*2 | I - | -   | -20.0 mA |
| --- | --- | -------------- | --- | --- | -------- |
OH
VCC = 3.0 to 5.5 V
|     |     |     | I - | -   | 20.0 mA |
| --- | --- | --- | --- | --- | ------- |
OL
Low drive*1
|     | Port P409 |                | I OH - | -   | -4.0 mA |
| --- | --------- | -------------- | ------ | --- | ------- |
|     |           |                | I OL - | -   | 4.0 mA  |
|     |           | Middle drive*2 | I -    | -   | -8.0 mA |
OH
VCC = 2.7 to 3.0 V
|     |     |     | I - | -   | 8.0 mA |
| --- | --- | --- | --- | --- | ------ |
OL
|     |     | Middle drive*2  | I - | -   | -20.0 mA |
| --- | --- | --------------- | --- | --- | -------- |
OH
VCC = 3.0 to 5.5 V
|     |     |     | I OL - | -   | 20.0 mA |
| --- | --- | --- | ------ | --- | ------- |
Low drive*1
|     | Ports P100 to P115,  |     | I OH - | -   | -4.0 mA |
| --- | -------------------- | --- | ------ | --- | ------- |
P201 to P204, P300 to P307,
|     |                              |     | I - | -   | 4.0 mA |
| --- | ---------------------------- | --- | --- | --- | ------ |
|     | P500 to P503, P600 to P603,  |     | OL  |     |        |
P608 to P610, P808, P809
|     |                  | Middle drive*2 | I - | -   | -4.0 mA |
| --- | ---------------- | -------------- | --- | --- | ------- |
|     | (total 41 pins)  |                | OH  |     |         |
|     |                  |                | I - | -   | 8.0 mA  |
OL
|     | Ports P914, P915 | -   | I - | -   | -4.0 mA |
| --- | ---------------- | --- | --- | --- | ------- |
OH
|     |                    |             | I OL - | -   | 4.0 mA  |
| --- | ------------------ | ----------- | ------ | --- | ------- |
|     | Other output pin*3 | Low drive*1 |        |     |         |
|     |                    |             | I OH - | -   | -4.0 mA |
|     |                    |             | I -    | -   | 4.0 mA  |
OL
|     |     | Middle drive*2 | I - | -   | -8.0 mA |
| --- | --- | -------------- | --- | --- | ------- |
OH
|     |     |     | I - | -   | 8.0 mA |
| --- | --- | --- | --- | --- | ------ |
OL
Permissible output current  Total of ports P000 to P008, P010 to P015 ΣI OH (max) - - -30 mA
(max value total pins)
|     |                  |     | ΣI OL (max) - | -   | 30 mA   |
| --- | ---------------- | --- | ------------- | --- | ------- |
|     | Ports P914, P915 |     | ΣI -          | -   | -2.0 mA |
OH (max)
|     |     |     | ΣI - | -   | 2.0 mA |
| --- | --- | --- | ---- | --- | ------ |
OL (min)
|     | Total of all output pin*5 |     | ΣI - | -   | -60 mA |
| --- | ------------------------- | --- | ---- | --- | ------ |
OH (max)
|     |     |     | ΣI - | -   | 60 mA |
| --- | --- | --- | ---- | --- | ----- |
OL (max)
Caution: To protect the reliability of the MCU, the output current values should not exceed the values in this table. The
average output current indicates the average value of current measured during 100 μs.
Note 1. This is the value when low driving ability is selected with the Port Drive Capability bit in PmnPFS register.
Note 2. This is the value when middle driving ability is selected with the Port Drive Capability bit in PmnPFS register.
Note 3. Except for ports P200, P214, P215, which are input ports.
Note 4. This is the value when middle driving ability for IIC Fast-mode is selected with the Port Drive Capability bit in PmnPFS register.
Note 5. For details on the permissible output current used with CTSU, see section 2.11, CTSU Characteristics.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 33 of 131 |
| --------------- | -------- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |           |                                 |     |     | 2. Electrical Characteristics |
| ----------- | --------- | ------------------------------- | --- | --- | ----------------------------- |
| 2.2.4       | I/O V     | , V , and Other Characteristics |     |     |                               |
|             | OH        | OL                              |     |     |                               |
| Table 2.7   | I/O V , V |  (1)                            |     |     |                               |
|             | OH        | OL                              |     |     |                               |
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 4.0 to 5.5 V
| Parameter |     |     | Symbol Min | Typ Max | Unit Test conditions |
| --------- | --- | --- | ---------- | ------- | -------------------- |
IIC*1
| Output voltage |                        |     | V OL -      | - 0.4 | V I OL  = 3.0 mA |
| -------------- | ---------------------- | --- | ----------- | ----- | ---------------- |
|                |                        |     | V *2,*5 -   | - 0.6 | I  = 6.0 mA      |
|                |                        |     | OL          |       | OL               |
|                | Ports P408, P409*2, *3 |     | V VCC - 1.0 | - -   | I  = -20 mA      |
|                |                        |     | OH          |       | OH               |
|                |                        |     | V -         | - 1.0 | I  = 20 mA       |
|                |                        |     | OL          |       | OL               |
Ports P000 to P008,  Low drive V AVCC0 - 0.8 - - I  = -2.0 mA
|     |     |     | OH  |     | OH  |
| --- | --- | --- | --- | --- | --- |
P010 to P015
|     |                  |              | V OL -          | - 0.8 | I OL  = 2.0 mA |
| --- | ---------------- | ------------ | --------------- | ----- | -------------- |
|     |                  | Middle drive | V AVCC0 - 0.8   | - -   | I  = -4.0 mA   |
|     |                  |              | OH              |       | OH             |
|     |                  |              | V -             | - 0.8 | I  = 4.0 mA    |
|     |                  |              | OL              |       | OL             |
|     | Ports P914, P915 |              | V VCC_USB - 0.8 | - -   | I  = -2.0 mA   |
|     |                  |              | OH              |       | OH             |
|     |                  |              | V -             | - 0.8 | I  = 2.0 mA    |
|     |                  |              | OL              |       | OL             |
Other output pins*4 Low drive V OH VCC - 0.8 - - I OH  = -2.0 mA
|     |     |         | V OL -      | - 0.8 | I OL  = 2.0 mA |
| --- | --- | ------- | ----------- | ----- | -------------- |
|     |     | Middle  | V VCC - 0.8 | - -   | I  = -4.0 mA   |
|     |     |         | OH          |       | OH             |
drive*6
|                                                                              |     |     | V - | - 0.8 | I  = 4.0 mA |
| ---------------------------------------------------------------------------- | --- | --- | --- | ----- | ----------- |
|                                                                              |     |     | OL  |       | OL          |
| Note 1. P100, P101, P204, P205, P206, P400, P401, P407, P408 (total 9 pins). |     |     |     |       |             |
Note 2. This is the value when middle driving ability is selected with the Port Drive Capability bit in PmnPFS register.
| Note 3. Based on characterization data, not tested in production. |     |     |     |     |     |
| ----------------------------------------------------------------- | --- | --- | --- | --- | --- |
| Note 4. Except for ports P200, P214, P215, which are input ports. |     |     |     |     |     |
Note 5. This is the value when middle driving ability for IIC is selected in the Port Drive Capability bit in PmnPFS register for P408.
| Note 6. Except for P212, P213. |           |      |     |     |     |
| ------------------------------ | --------- | ---- | --- | --- | --- |
| Table 2.8                      | I/O V , V |  (2) |     |     |     |
|                                | OH        | OL   |     |     |     |
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 2.7 to 4.0 V
| Parameter      |                        |     | Symbol Min  | Typ Max | Unit Test conditions |
| -------------- | ---------------------- | --- | ----------- | ------- | -------------------- |
| Output voltage | IIC*1                  |     | V -         | - 0.4   | V I  = 3.0 mA        |
|                |                        |     | OL          |         | OL                   |
|                |                        |     | V *2,*5 -   | - 0.6   | I  = 6.0 mA          |
|                |                        |     | OL          |         | OL                   |
|                | Ports P408, P409*2, *3 |     | V VCC - 1.0 | - -     | I  = -20 mA          |
|                |                        |     | OH          |         | OH                   |
VCC = 3.3 V
|     |     |     | V OL - | - 1.0 | I OL  = 20 mA |
| --- | --- | --- | ------ | ----- | ------------- |
VCC = 3.3 V
Ports P000 to P008,  Low drive V OH AVCC0 - 0.5 - - I OH  = -1.0 mA
|     | P010 to P015        |              | V -                | - 0.5 | I  = 1.0 mA     |
| --- | ------------------- | ------------ | ------------------ | ----- | --------------- |
|     |                     |              | OL                 |       | OL              |
|     |                     | Middle drive | V AVCC0 - 0.5      | - -   | I  = -2.0 mA    |
|     |                     |              | OH                 |       | OH              |
|     |                     |              | V -                | - 0.5 | I  = 2.0 mA     |
|     |                     |              | OL                 |       | OL              |
|     | Ports P914, P915    |              | V OH VCC_USB - 0.5 | - -   | I OH  = -1.0 mA |
|     |                     |              | V OL -             | - 0.5 | I OL  = 1.0 mA  |
|     | Other output pins*4 | Low drive    | V VCC - 0.5        | - -   | I  = -1.0 mA    |
|     |                     |              | OH                 |       | OH              |
|     |                     |              | V -                | - 0.5 | I  = 1.0 mA     |
|     |                     |              | OL                 |       | OL              |
|     |                     | Middle       | V VCC - 0.5        | - -   | I  = -2.0 mA    |
|     |                     |              | OH                 |       | OH              |
drive*6
|                                                                              |     |     | V - | - 0.5 | I  = 2.0 mA |
| ---------------------------------------------------------------------------- | --- | --- | --- | ----- | ----------- |
|                                                                              |     |     | OL  |       | OL          |
| Note 1. P100, P101, P204, P205, P206, P400, P401, P407, P408 (total 9 pins). |     |     |     |       |             |
Note 2. This is the value when middle driving ability is selected with the Port Drive Capability bit in PmnPFS register.
| Note 3. Based on characterization data, not tested in production. |     |     |     |     |     |
| ----------------------------------------------------------------- | --- | --- | --- | --- | --- |
| Note 4. Except for ports P200, P214, P215, which are input ports. |     |     |     |     |     |
Note 5. This is the value when middle driving ability for IIC is selected in the Port Drive Capability bit in PmnPFS register for P408.
| Note 6. Except for P212, P213. |          |     |     |     |                |
| ------------------------------ | -------- | --- | --- | --- | -------------- |
| R01DS0355EJ0110                | Rev.1.10 |     |     |     | Page 34 of 131 |
Sep 29, 2023

| RA4M1 Group |           |      |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --------- | ---- | --- | --- | --- | ----------------------------- | --- |
| Table 2.9   | I/O V , V |  (3) |     |     |     |                               |     |
|             | OH        | OL   |     |     |     |                               |     |
Conditions: VCC = AVCC0 = VCC_USB = VCC_USB_LDO = 1.6 to 2.7 V
| Parameter |     |     | Symbol | Min | Typ Max | Unit | Test conditions |
| --------- | --- | --- | ------ | --- | ------- | ---- | --------------- |
Output voltage Ports P000 to P015 Low drive V AVCC0 - 0.3 - - V I  = -0.5 mA
|     |                     |              | OH   |               |       |     | OH              |
| --- | ------------------- | ------------ | ---- | ------------- | ----- | --- | --------------- |
|     |                     |              | V    | -             | - 0.3 |     | I  = 0.5 mA     |
|     |                     |              | OL   |               |       |     | OL              |
|     |                     | Middle drive | V OH | AVCC0 - 0.3   | - -   |     | I OH  = -1.0 mA |
|     |                     |              | V OL | -             | - 0.3 |     | I OL  = 1.0 mA  |
|     | Ports P914, P915    |              | V    | VCC_USB - 0.3 | - -   |     | I  = -0.5 mA    |
|     |                     |              | OH   |               |       |     | OH              |
|     |                     |              | V    | -             | - 0.3 |     | I  = 0.5 mA     |
|     |                     |              | OL   |               |       |     | OL              |
|     | Other output pins*1 | Low drive    | V    | VCC - 0.3     | - -   |     | I  = -0.5 mA    |
|     |                     |              | OH   |               |       |     | OH              |
|     |                     |              | V    | -             | - 0.3 |     | I  = 0.5 mA     |
|     |                     |              | OL   |               |       |     | OL              |
|     |                     | Middle       | V OH | VCC - 0.3     | - -   |     | I OH  = -1.0 mA |
drive*2
|                                                                   |                           |     | V OL | -   | - 0.3 |     | I OL  = 1.0 mA |
| ----------------------------------------------------------------- | ------------------------- | --- | ---- | --- | ----- | --- | -------------- |
| Note 1. Except for ports P200, P214, P215, which are input ports. |                           |     |      |     |       |     |                |
| Note 2. Except for P212, P213.                                    |                           |     |      |     |       |     |                |
| Table 2.10                                                        | I/O other characteristics |     |      |     |       |     |                |
Conditions: VCC = AVCC0 = 1.6 to 5.5 V
| Parameter |     |     | Symbol | Min | Typ Max | Unit | Test conditions |
| --------- | --- | --- | ------ | --- | ------- | ---- | --------------- |
Input leakage current RES, P200, P214, P215 | I in  | - - 1.0 μA V in  = 0 V
|     |     |     |     |     |     |     | V in  = VCC |
| --- | --- | --- | --- | --- | --- | --- | ----------- |
Three-state leakage  5 V-tolerant ports | I  | - - 1.0 μA V  = 0 V
|                     |     |     | TSI |     |     |     | in         |
| ------------------- | --- | --- | --- | --- | --- | --- | ---------- |
| current (off state) |     |     |     |     |     |     | V  = 5.8 V |
in
|     |     | Other ports  |     | -   | - 1.0 |     | V  = 0 V |
| --- | --- | ------------ | --- | --- | ----- | --- | -------- |
in
|     |     | (except for ports P200, P214,  |     |     |     |     | V  = VCC |
| --- | --- | ------------------------------ | --- | --- | --- | --- | -------- |
in
P215 and 5 V tolerant)
| Input pull-up resistor |     | All ports  | R   | 10  | 20 50 | kΩ  | V  = 0 V |
| ---------------------- | --- | ---------- | --- | --- | ----- | --- | -------- |
|                        |     |            | U   |     |       |     | in       |
(except for ports P200, P214,
P215, P914, P915)
| Input capacitance |     | P914, P915,                | C in | -   | - 30 | pF  | V in  = 0 V |
| ----------------- | --- | -------------------------- | ---- | --- | ---- | --- | ----------- |
|                   |     | P100 to P103, P111, P112,  |      |     |      |     | f = 1 MHz   |
|                   |     | P200                       |      |     |      |     | T  = 25°C   |
a
|                 |          | Other input pins |     | -   | - 15 |     |                |
| --------------- | -------- | ---------------- | --- | --- | ---- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10 |                  |     |     |      |     | Page 35 of 131 |
Sep 29, 2023

| RA4M1 Group |                                                      |     |      |                      |     | 2. Electrical Characteristics |     |
| ----------- | ---------------------------------------------------- | --- | ---- | -------------------- | --- | ----------------------------- | --- |
| 2.2.5       | I/O Pin Output Characteristics of Low Drive Capacity |     |      |                      |     |                               |     |
|             |                                                      |     | I OH | /I OL  vs V OH /V OL |     |                               |     |
60
50
VCC = 5.5 V
40
30
|     | 20        |     |     | VCC = 3.3 V |     |     |     |
| --- | --------- | --- | --- | ----------- | --- | --- | --- |
|     | ]Am[  10  |     |     | VCC = 2.7 V |     |     |     |
VCC = 1.6 V
LO 0
I/ VCC = 1.6 V
HO -10
I VCC = 2.7 V
-20
VCC = 3.3 V
-30
-40
-50
VCC = 5.5 V
-60
|     | 0   | 1   | 2   | 3               | 4   | 5   | 6   |
| --- | --- | --- | --- | --------------- | --- | --- | --- |
|     |     |     |     | V OH /V OL  [V] |     |     |     |
Figure 2.2 V /V  and I /I  Voltage Characteristics at T  = 25°C when low drive output is selected
|     | OH OL | OH OL |     | a   |     |     |     |
| --- | ----- | ----- | --- | --- | --- | --- | --- |
(reference data)
|     |     |     | I OH | /I OL  vs V OH /V OL |     |     |     |
| --- | --- | --- | ---- | -------------------- | --- | --- | --- |
3
Ta = -40°C
|     | 2   |     |     |     |     | Ta = 25°C |     |
| --- | --- | --- | --- | --- | --- | --------- | --- |
Ta = 105°C
1
 ]Am[
LO 0
I/
HO
I
-1
Ta = 105°C
-2  Ta = 25°C
Ta = -40°C
-3
|     | 0   | 0.2 0.4 | 0.6 0.8 | 1 1.2           | 1.4 | 1.6 1.8 | 2   |
| --- | --- | ------- | ------- | --------------- | --- | ------- | --- |
|     |     |         |         | V OH /V OL  [V] |     |         |     |
Figure 2.3 V /V  and I /I  temperature characteristics at VCC = 1.6 V when low drive output is selected
|     | OH OL | OH OL |     |     |     |     |     |
| --- | ----- | ----- | --- | --- | --- | --- | --- |
(reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     | Page 36 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
I /I vs V /V
OH OL OH OL
20
15
10
5
0
-5
-10
-15
-20
0 0.5 1 1.5 2 2.5 3
V /V [V]
OH OL
Figure 2.4 V /V and I /I temperature characteristics at VCC = 2.7 V when low drive output is selected
OH OL OH OL
(reference data)
Figure 2.5 V /V and I /I temperature characteristics at VCC = 3.3 V when low drive output is selected
OH OL OH OL
(reference data)
R01DS0355EJ0110 Rev.1.10 Page 37 of 131
Sep 29, 2023
]Am[
I/
I
LO
HO
Ta = -40°C
Ta = 25°C
Ta = 105°C
Ta = 105°C
Ta = 25°C
Ta = -40°C
I /I vs V /V
OH OL OH OL
30
20
10
0
-10
-20
-30
0 0.5 1 1.5 2 2.5 3 3.5
V /V [V]
OH OL
]Am[
I/
I
LO
HO
Ta = -40°C
Ta = 25°C
Ta = 105°C
Ta = 105°C
Ta = 25°C
Ta = -40°C

| RA4M1 Group |     |     |      |                      |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | ---- | -------------------- | --- | ----------------------------- | --- |
|             |     |     | I OH | /I OL  vs V OH /V OL |     |                               |     |
60
Ta = -40°C
|     | 40  |     |     |     |     | Ta = 25°C |     |
| --- | --- | --- | --- | --- | --- | --------- | --- |
Ta = 105°C
20
]Am[
LO 0
I/
HO
I
-20
Ta = 105°C
-40
Ta = 25°C
Ta = -40°C
-60
|     | 0   | 1   | 2   | 3               | 4   | 5   | 6   |
| --- | --- | --- | --- | --------------- | --- | --- | --- |
|     |     |     |     | V OH /V OL  [V] |     |     |     |
Figure 2.6 V /V  and I /I  temperature characteristics at VCC = 5.5 V when low drive output is selected
|     | OH OL | OH OL |     |     |     |     |     |
| --- | ----- | ----- | --- | --- | --- | --- | --- |
(reference data)
2.2.6 I/O Pin Output Characteristics of Middle Drive Capacity
|     |     |     |     | I OH /I OL  vs V OH /V | OL  |     |     |
| --- | --- | --- | --- | ---------------------- | --- | --- | --- |
140
120
VCC = 5.5 V
100
80
60
VCC = 3.3 V
40
|     | ]Am[  |             |     | VCC = 2.7 V |     |     |     |
| --- | ----- | ----------- | --- | ----------- | --- | --- | --- |
|     | 20    | VCC = 1.6 V |     |             |     |     |     |
LO 0
I/ -20  VCC = 1.6 V
HO
I VCC = 2.7 V
-40
-60  VCC = 3.3 V
-80
-100
-120
VCC = 5.5 V
-140
|     | 0   | 1   | 2   | 3   | 4   | 5   | 6   |
| --- | --- | --- | --- | --- | --- | --- | --- |
V OH /V OL  [V]
Figure 2.7 V /V  and I /I  voltage characteristics at T  = 25°C when middle drive output is selected
|     | OH OL | OH OL |     | a   |     |     |     |
| --- | ----- | ----- | --- | --- | --- | --- | --- |
(reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     | Page 38 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |                     |       | 2. Electrical Characteristics |     |
| ----------- | --- | --- | ------------------- | ----- | ----------------------------- | --- |
|             |     |     | I OH /I OL  vs V OH | /V OL |                               |     |
6
Ta = -40°C
Ta = 25°C
Ta = 105°C
4
2
]Am[
LO 0
I/
HO
I
-2
Ta = 105°C
-4
Ta = 25°C
Ta = -40°C
-6
|     | 0 0.2 | 0.4 | 0.6 0.8 1       | 1.2 1.4 | 1.6 1.8 | 2   |
| --- | ----- | --- | --------------- | ------- | ------- | --- |
|     |       |     | V OH /V OL  [V] |         |         |     |
Figure 2.8 V /V  and I /I  temperature characteristics at VCC = 1.6 V when middle drive output is
|     | OH OL OH | OL  |     |     |     |     |
| --- | -------- | --- | --- | --- | --- | --- |
selected (reference data)
|     |     |     | I OH /I OL  vs V OH | /V OL |     |     |
| --- | --- | --- | ------------------- | ----- | --- | --- |
40
Ta = -40°C
Ta = 25°C
30
Ta = 105°C
20
10
]Am[
LO 0
I/
HO
I -10
-20
Ta = 105°C
Ta = 25°C
-30
Ta = -40°C
-40
|     | 0   | 0.5 | 1 1.5           | 2   | 2.5 | 3   |
| --- | --- | --- | --------------- | --- | --- | --- |
|     |     |     | V OH /V OL  [V] |     |     |     |
Figure 2.9 V /V  and I /I  temperature characteristics at VCC = 2.7 V when middle drive output is
|     | OH OL OH | OL  |     |     |     |     |
| --- | -------- | --- | --- | --- | --- | --- |
selected (reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 39 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
I /I vs V /V
OH OL OH OL
60
40
20
0
-20
-40
-60
0 0.5 1 1.5 2 2.5 3 3.5
V /V [V]
OH OL
Figure 2.10 V /V and I /I temperature characteristics at VCC = 3.3 V when middle drive output is
OH OL OH OL
selected (reference data)
Figure 2.11 V /V and I /I temperature characteristics at VCC = 5.5 V when middle drive output is
OH OL OH OL
selected (reference data)
R01DS0355EJ0110 Rev.1.10 Page 40 of 131
Sep 29, 2023
]Am[
I/
I
LO
HO
Ta = -40°C
Ta = 25°C
Ta = 105°C
Ta = 105°C
Ta = 25°C
Ta = -40°C
I /I vs V /V
OH OL OH OL
140
120
100
80
60
40
20
0
-20
-40
-60
-80
-100
-120
-140
0 1 2 3 4 5 6
V /V [V]
OH OL
]Am[
I/
I
LO
HO
Ta = -40°C
Ta = 25°C
Ta = 105°C
Ta = 105°C
Ta = 25°C
Ta = -40°C

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
2.2.7 P408, P409 I/O Pin Output Characteristics of Middle Drive Capacity
|     |     |     | I OH | /I OL  vs V OH /V OL |     |     |     |
| --- | --- | --- | ---- | -------------------- | --- | --- | --- |
200
|     | 180 |     |     |     |     | VCC = 5.5 V |     |
| --- | --- | --- | --- | --- | --- | ----------- | --- |
160
140
120
100
|     | 80  |     |     | VCC = 3.3 V |     |     |     |
| --- | --- | --- | --- | ----------- | --- | --- | --- |
60
VCC = 2.7 V
]Am[  40
20
LO 0
I/ -20
HO -40  VCC = 2.7 V
I
-60  VCC = 3.3 V
-80
-100
-120
-140
-160
VCC = 5.5 V
-180
-200
|     | 0   | 1   | 2   | 3               | 4   | 5   | 6   |
| --- | --- | --- | --- | --------------- | --- | --- | --- |
|     |     |     |     | V OH /V OL  [V] |     |     |     |
Figure 2.12 V /V  and I /I  voltage characteristics at Ta = 25°C when middle drive output is selected
|     | OH OL | OH OL |     |     |     |     |     |
| --- | ----- | ----- | --- | --- | --- | --- | --- |
(reference data)
|     |     |     | I OH | /I OL  vs V OH /V OL |     |     |     |
| --- | --- | --- | ---- | -------------------- | --- | --- | --- |
60
Ta = -40°C
Ta = 25°C
Ta = 105°C
40
20
]Am[
LO 0
I/
HO
I
-20
Ta = 105°C
-40
Ta = 25°C
Ta = -40°C
-60
|     | 0   | 0.5 | 1   | 1.5             | 2   | 2.5 | 3   |
| --- | --- | --- | --- | --------------- | --- | --- | --- |
|     |     |     |     | V OH /V OL  [V] |     |     |     |
Figure 2.13 V /V  and I /I  temperature characteristics at VCC = 2.7 V when middle drive output is
|     | OH OL | OH OL |     |     |     |     |     |
| --- | ----- | ----- | --- | --- | --- | --- | --- |
selected (reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     | Page 41 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |         |                   |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | ------- | ----------------- | --- | ----------------------------- | --- |
|             |     |     | I OH /I | OL  vs V OH /V OL |     |                               |     |
100
Ta = -40°C
80
Ta = 25°C
Ta = 105°C
60
40
]Am[  20
LO 0
I/
HO -20
I
-40
Ta = 105°C
-60
Ta = 25°C
-80  Ta = -40°C
-100
|     | 0   | 0.5 | 1 1.5 | 2               | 2.5 | 3   | 3.5 |
| --- | --- | --- | ----- | --------------- | --- | --- | --- |
|     |     |     |       | V OH /V OL  [V] |     |     |     |
Figure 2.14 V /V  and I /I  temperature characteristics at VCC = 3.3 V when middle drive output is
|     | OH OL OH | OL  |     |     |     |     |     |
| --- | -------- | --- | --- | --- | --- | --- | --- |
selected (reference data)
|     |     |     | I OH | /I OL  vs V OH /V OL |     |     |     |
| --- | --- | --- | ---- | -------------------- | --- | --- | --- |
220
Ta = -40°C
180
Ta = 25°C
|     | 140  |     |     |     |     | Ta = 105°C |     |
| --- | ---- | --- | --- | --- | --- | ---------- | --- |
100
60
]Am[
20
LO
I/ -20
HO
I
-60
-100
-140  Ta = 105°C
Ta = 25°C
-180
Ta = -40°C
-220
|     | 0   | 1   | 2   | 3               | 4   | 5   | 6   |
| --- | --- | --- | --- | --------------- | --- | --- | --- |
|     |     |     |     | V OH /V OL  [V] |     |     |     |
Figure 2.15 V /V  and I /I  temperature characteristics at VCC = 5.5 V when middle drive output is
|     | OH OL OH | OL  |     |     |     |     |     |
| --- | -------- | --- | --- | --- | --- | --- | --- |
selected (reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     | Page 42 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |                                    |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ---------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| 2.2.8       | IIC I/O Pin Output Characteristics |     |     |     |     |                               |     |
I OL  vs V OL
120
110
VCC = 5.5 V (Middle drive)
100
90
80
]Am[
70
LO 60
I
|     | 50  |     | VCC = 3.3V (Middle drive) |     |     |     |     |
| --- | --- | --- | ------------------------- | --- | --- | --- | --- |
VCC = 5.5V (Low drive)
40
VCC = 2.7V (Middle drive)
30
|     | 20  |     | VCC = 3.3V (Low drive) |     |     |     |     |
| --- | --- | --- | ---------------------- | --- | --- | --- | --- |
10
VCC = 2.7V (Low drive)
0
|     | 0   | 1   | 2   | 3   | 4   | 5   | 6   |
| --- | --- | --- | --- | --- | --- | --- | --- |
V OL  [V]
| Figure 2.16     | V /V  and I | /I  voltage characteristics at Ta = 25°C |     |     |     |     |                |
| --------------- | ----------- | ---------------------------------------- | --- | --- | --- | --- | -------------- |
|                 | OH OL       | OH OL                                    |     |     |     |     |                |
| R01DS0355EJ0110 | Rev.1.10    |                                          |     |     |     |     | Page 43 of 131 |
Sep 29, 2023

| RA4M1 Group                                           |     |     |     |     | 2. Electrical Characteristics |     |
| ----------------------------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| 2.2.9 Operating and Standby Current                   |     |     |     |     |                               |     |
| Table 2.11 Operating and standby current (1) (1 of 2) |     |     |     |     |                               |     |
Conditions: VCC = AVCC0 = 1.6 to 5.5 V
Test
| Parameter |     |     |     | Symbol Typ*10 | Max Unit | conditions |
| --------- | --- | --- | --- | ------------- | -------- | ---------- |
*7
Supply  High-speed  Normal mode All peripheral clock  ICLK = 48 MHz ICC 8.3 - mA
| current*1 mode*2  |     | disabled, while (1) code  |               |      |     |     |
| ----------------- | --- | ------------------------- | ------------- | ---- | --- | --- |
|                   |     | executing from flash*5    | ICLK = 32 MHz | 5.8  | -   |     |
|                   |     |                           | ICLK = 16 MHz | 3.5  | -   |     |
|                   |     |                           | ICLK = 8 MHz  | 2.2  | -   |     |
|                   |     | All peripheral clock      | ICLK = 48 MHz | 16.4 | -   |     |
disabled, CoreMark code
|     |     | executing from flash*5 | ICLK = 32 MHz | 11.3 | -   |     |
| --- | --- | ---------------------- | ------------- | ---- | --- | --- |
|     |     |                        | ICLK = 16 MHz | 6.4  | -   |     |
|     |     |                        | ICLK = 8 MHz  | 4.0  | -   |     |
*9
|     |     | All peripheral clock  | ICLK = 48 MHz | 18.5 | -   |     |
| --- | --- | --------------------- | ------------- | ---- | --- | --- |
enabled, while (1) code
|     |     | executing from flash*5 | ICLK = 32 MHz | 13.8 | -    | *8  |
| --- | --- | ---------------------- | ------------- | ---- | ---- | --- |
|     |     |                        | ICLK = 16 MHz | 7.7  | -    |     |
|     |     |                        | ICLK = 8 MHz  | 4.5  | -    |     |
|     |     | All peripheral clock   | ICLK = 48 MHz | -    | 50.0 | *9  |
enabled, code executing
from SRAM*5
|     | Sleep mode | All peripheral clock  | ICLK = 48 MHz | 3.3 | -   | *7  |
| --- | ---------- | --------------------- | ------------- | --- | --- | --- |
disabled*5
|     |     |     | ICLK = 32 MHz | 2.4 | -   |     |
| --- | --- | --- | ------------- | --- | --- | --- |
|     |     |     | ICLK = 16 MHz | 1.8 | -   |     |
|     |     |     | ICLK = 8 MHz  | 1.4 | -   |     |
*9
|     |     | All peripheral clock  | ICLK = 48 MHz | 13.4 | -   |     |
| --- | --- | --------------------- | ------------- | ---- | --- | --- |
enabled*5
|     |                                 |     | ICLK = 32 MHz | 10.4 | -   | *8  |
| --- | ------------------------------- | --- | ------------- | ---- | --- | --- |
|     |                                 |     | ICLK = 16 MHz | 6.0  | -   |     |
|     |                                 |     | ICLK = 8 MHz  | 3.6  | -   |     |
|     | Increase during BGO operation*6 |     |               | 2.5  | -   | -   |
Middle-speed  Normal mode All peripheral clock  ICLK = 12 MHz ICC 2.5 - mA *7
| mode*2 |     | disabled, while (1) code  |              |     |     |     |
| ------ | --- | ------------------------- | ------------ | --- | --- | --- |
|        |     |                           | ICLK = 8 MHz | 2.0 | -   |     |
executing from flash*5
|     |     |                       | ICLK = 1 MHz  | 0.9 | -   |     |
| --- | --- | --------------------- | ------------- | --- | --- | --- |
|     |     | All peripheral clock  | ICLK = 12 MHz | 4.7 | -   |     |
disabled, CoreMark code
|     |     |     | ICLK = 8 MHz | 3.7 | -   |     |
| --- | --- | --- | ------------ | --- | --- | --- |
executing from flash*5
|     |     |     | ICLK = 1 MHz | 1.2 | -   |     |
| --- | --- | --- | ------------ | --- | --- | --- |
*8
|     |     | All peripheral clock  | ICLK = 12 MHz | 5.7 | -   |     |
| --- | --- | --------------------- | ------------- | --- | --- | --- |
enabled, while (1) code
|     |     | executing from flash*5 | ICLK = 8 MHz  | 4.3 | -    |     |
| --- | --- | ---------------------- | ------------- | --- | ---- | --- |
|     |     |                        | ICLK = 1 MHz  | 1.5 | -    |     |
|     |     | All peripheral clock   | ICLK = 12 MHz | -   | 20.0 |     |
enabled, code executing
from SRAM*5
|     | Sleep mode | All peripheral clock  | ICLK = 12 MHz | 1.2 | -   | *7  |
| --- | ---------- | --------------------- | ------------- | --- | --- | --- |
disabled*5
|     |     |                       | ICLK = 8 MHz  | 1.2 | -   |     |
| --- | --- | --------------------- | ------------- | --- | --- | --- |
|     |     |                       | ICLK = 1 MHz  | 0.8 | -   |     |
|     |     | All peripheral clock  | ICLK = 12 MHz | 4.4 | -   | *8  |
enabled*5
|                 |                                 |     | ICLK = 8 MHz | 3.4 | -   |                |
| --------------- | ------------------------------- | --- | ------------ | --- | --- | -------------- |
|                 |                                 |     | ICLK = 1 MHz | 1.4 | -   |                |
|                 | Increase during BGO operation*6 |     |              | 2.5 | -   | -              |
| R01DS0355EJ0110 | Rev.1.10                        |     |              |     |     | Page 44 of 131 |
Sep 29, 2023

| RA4M1 Group                                           |     |     |     |     | 2. Electrical Characteristics |     |
| ----------------------------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| Table 2.11 Operating and standby current (1) (2 of 2) |     |     |     |     |                               |     |
Conditions: VCC = AVCC0 = 1.6 to 5.5 V
Test
| Parameter |     |     |     | Symbol Typ*10 | Max Unit | conditions |
| --------- | --- | --- | --- | ------------- | -------- | ---------- |
Supply  Low-speed  Normal mode All peripheral clock  ICLK = 1 MHz ICC 0.4 - mA *7
| current*1 mode*3 |     |     |     |     |     |     |
| ---------------- | --- | --- | --- | --- | --- | --- |
disabled, while (1) code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 1 MHz | 0.6 | -   |     |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
disabled, CoreMark code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 1 MHz | 1.0 | -   | *8  |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
enabled, while (1) code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 1 MHz | -   | 2.2 |     |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
enabled, code executing
from SRAM*5
*7
|     | Sleep mode | All peripheral clock  | ICLK = 1 MHz | 0.3 | -   |     |
| --- | ---------- | --------------------- | ------------ | --- | --- | --- |
disabled*5
|     |     | All peripheral clock  | ICLK = 1 MHz | 0.9 | -   | *8  |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
enabled*5
Low-voltage  Normal mode All peripheral clock  ICLK = 4 MHz ICC 1.7 - mA *7
mode*3
disabled, while (1) code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 4 MHz | 2.8 | -   |     |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
disabled, CoreMark code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 4 MHz | 3.0 | -   | *8  |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
enabled, while (1) code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 4 MHz | -   | 8.0 |     |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
enabled, code executing
from SRAM*5
|     | Sleep mode | All peripheral clock  | ICLK = 4 MHz | 1.3 | -   | *7  |
| --- | ---------- | --------------------- | ------------ | --- | --- | --- |
disabled*5
*8
|     |     | All peripheral clock  | ICLK = 4 MHz | 2.5 | -   |     |
| --- | --- | --------------------- | ------------ | --- | --- | --- |
enabled*5
Subosc- Normal mode All peripheral clock  ICLK = 32.768 kHz ICC 8.5 - μA *8
| speed  |     | disabled, while (1) code  |                   |      |     |     |
| ------ | --- | ------------------------- | ----------------- | ---- | --- | --- |
| mode*4 |     | executing from flash*5    |                   |      |     |     |
|        |     | All peripheral clock      | ICLK = 32.768 kHz | 14.9 | -   |     |
enabled, while (1) code
executing from flash*5
|     |     | All peripheral clock  | ICLK = 32.768 kHz | -   | 83.0 |     |
| --- | --- | --------------------- | ----------------- | --- | ---- | --- |
enabled, code executing
from SRAM*5
|     | Sleep mode | All peripheral clock  | ICLK = 32.768 kHz | 5.0 | -   |     |
| --- | ---------- | --------------------- | ----------------- | --- | --- | --- |
disabled*5
|     |     | All peripheral clock  | ICLK = 32.768 kHz | 11.4 | -   |     |
| --- | --- | --------------------- | ----------------- | ---- | --- | --- |
enabled*5
Note 1. Supply current values do not include output charge/discharge current from all pins. The values apply when internal pull-up
MOSs are in the off state.
Note 2. The clock source is HOCO.
Note 3. The clock source is MOCO.
Note 4. The clock source is the sub-clock oscillator.
Note 5. This does not include BGO operation.
Note 6. This is the increase for programming or erasure of the flash memory for data storage during program execution.
Note 7. FCLK, PCLKA, PCLKB, PCLKC, and PCLKD are set to divided by 64.
Note 8. FCLK, PCLKA, PCLKB, PCLKC, and PCLKD are the same frequency as that of ICLK.
Note 9. FCLK and PCLKB are set to divided by 2 and PCLKA, PCLKC, and PCLKD are the same frequency as that of ICLK.
Note 10. VCC = 3.3 V.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 45 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
40
|     |     |     |     | Ta = 105 C, ICLK = 48 MHz | *2  |     |
| --- | --- | --- | --- | -------------------------- | --- | --- |
35
30
Ta = 105 C, ICLK = 32 MHz*2
25
| )Am( |     |     |     | Ta = 25 C, ICLK = 48 MHz*1  |                              |     |
| ---- | --- | --- | --- | ---------------------------- | ---------------------------- | --- |
| 20   |     |     |     | Ta = 105 C, ICLK = 16 MHz*2 |                              |     |
| CCI  |     |     |     | Ta = 25 C, ICLK = 32 MHz    | *1                           |     |
| 15   |     |     |     | T a   =   1 0 5   C         | ,   IC L K  =  8   M H z * 2 |     |
|      |     |     |     | T a   =   2 5   C ,         | I C L K  =  1 6   M H z* 1   |     |
| 10   |     |     |     | Ta = 105 C, ICLK = 4 MHz*2  |                              |     |
|      |     |     |     | Ta = 25 C                   | , ICLK = 8 MHz*1             |     |
|      |     |     |     | Ta = 25 C                   | , ICLK = 4 MHz*1             |     |
5
0
| 1.5 2.0 | 2.5 3.0 | 3.5 4.0 | 4.5 5.0 | 5.5 6.0 |     |     |
| ------- | ------- | ------- | ------- | ------- | --- | --- |
VCC (V)
|     | Ta = 25 C, ICLK = 48 MHz *1 |     | Ta = 105 C, ICLK = 48 MHz *2 |     |     |     |
| --- | ---------------------------- | --- | ----------------------------- | --- | --- | --- |
|     | Ta = 25 C, ICLK = 32 MHz *1 |     | Ta = 105 C, ICLK = 32 MHz *2 |     |     |     |
|     | Ta = 25 C, ICLK = 16 MHz *1 |     | Ta = 105 C, ICLK = 16 MHz *2 |     |     |     |
|     | Ta = 25 C, ICLK = 8 MHz *1  |     | Ta = 105 C, ICLK = 8 MHz *2  |     |     |     |
|     | Ta = 25 C, ICLK = 4 MHz *1  |     | Ta = 105 C, ICLK = 4 MHz *2  |     |     |     |
Note 1. All peripheral operations except any BGO operation are operating normally. This is the average of the actual
measurements of the sample cores during product evaluation.
Note 2. All peripheral operations except any BGO operation are operating at maximum. This is the average of the
actual measurements for the upper limit samples during product evaluation.
Figure 2.17 Voltage dependency in high-speed operating mode (reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 46 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
12
  C, ICLK = 12 MHz*2
Ta = 105
10
8   C, ICLK = 8 MHz*2
Ta = 105
)Am(
6
  C, ICLK = 12 MHz*1
Ta = 25
| CCI |     |     |     |     | Ta = 105 C, ICLK = 4 MHz*2 |     |
| --- | --- | --- | --- | --- | --------------------------- | --- |
|     |     |     |     |     | Ta = 25 C, ICLK = 8 MHz    | *1  |
4
Ta = 25 C, ICLK = 4 MHz*1
  C, ICLK = 1 MHz*2
Ta = 105
2
Ta = 25 C, ICLK = 1 MHz*1
0
|     | 1.5 2.0 | 2.5 3.0 | 3.5 4.0 | 4.5 5.0 | 5.5 6.0 |     |
| --- | ------- | ------- | ------- | ------- | ------- | --- |
VCC (V)
|     | Ta = 25 C, ICLK = 12 MHz *1 |     |     | Ta = 105 C, ICLK = 12 MHz *2 |     |     |
| --- | ---------------------------- | --- | --- | ----------------------------- | --- | --- |
|     | Ta = 25 C, ICLK = 8 MHz *1  |     |     | Ta = 105 C, ICLK = 8 MHz *2  |     |     |
Ta = 105 C, ICLK = 4 MHz *2
Ta = 25 C, ICLK = 4 MHz *1
|     | Ta = 25 C, ICLK = 1 MHz *1 |     |     | Ta = 105 C, ICLK = 1 MHz *2 |     |     |
| --- | --------------------------- | --- | --- | ---------------------------- | --- | --- |
Note 1. All peripheral operations except any BGO operation are operating normally. This is the average of the actual
measurements of the sample cores during product evaluation.
Note 2. All peripheral operations except any BGO operation are operating at maximum. This is the average of the
actual measurements for the upper limit samples during product evaluation.
Figure 2.18 Voltage dependency in middle-speed operating mode (reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 47 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
1.6
1.4
1.2
1.0
0.8
0.6
0.4
0.2
0.0
1.5 2.0 2.5 3.0 3.5 4.0 4.5 5.0 5.5 6.0
Figure 2.19 Voltage dependency in Low-speed mode (reference data)
R01DS0355EJ0110 Rev.1.10 Page 48 of 131
Sep 29, 2023
)Am(CCI
Ta = 105  C, ICLK = 1 MHz*2
Ta = 25  C, ICLK = 1 MHz *1
VCC (V)

Ta = 25 C, ICLK = 1 MHz *1
Note 1. All peripheral operations except any BGO operation are operating normally. This is the average of the
actual measurements of the sample cores during product evaluation.
Note 2. All peripheral operations except any BGO operation are operating at maximum. This is the average of the
actual measurements for the upper limit samples during product evaluation.

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | ----------------------------- | --- |
6
5
Ta = 105 C, ICLK = 4 MHz*2
4
)Am(
| 3   |     |     |     | Ta = 25 C, ICLK = 4 MHz*1  |     |
| --- | --- | --- | --- | --------------------------- | --- |
| CCI |     |     |     |                            |     |
|     |     |     |     | Ta = 105  C, ICLK = 1 MHz*2 |     |
2
Ta = 25 C, ICLK = 1 MHz*1
1
0
| 1.5 | 2.0 2.5 3.0 | 3.5 4.0 | 4.5 5.0 | 5.5 6.0 |     |
| --- | ----------- | ------- | ------- | ------- | --- |
VCC (V)
|     | Ta = 25 C, ICLK = 4 MHz *1 |     | Ta = 105 C, ICLK = 4 MHz *2 |     |     |
| --- | --------------------------- | --- | ---------------------------- | --- | --- |
|     | Ta = 25 C, ICLK = 1 MHz *1 |     | Ta = 105 C, ICLK = 1 MHz*2  |     |     |
Note 1. All peripheral operations except any BGO operation are operating normally. This is the average of the
actual measurements of the sample cores during product evaluation.
Note 2. All peripheral operations except any BGO operation are operating at maximum. This is the average of the
actual measurements for the upper limit samples during product evaluation.
| Figure 2.20     |  Voltage dependency in low-voltage mode (reference data) |     |     |     |                |
| --------------- | -------------------------------------------------------- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10                                                 |     |     |     | Page 49 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
55.0
Ta = 105 C, ICLK = 32 MHz*2
50.0
45.0
40.0
35.0
30.0
)A( CCI
25.0
20.0
Ta = 25 C, ICLK = 32 MHz*1
15.0
10.0
5.0
0.0
|     | 1.5 2.0 | 2.5 3.0 | 3.5 4.0 | 4.5 5.0 | 5.5 6.0 |     |
| --- | ------- | ------- | ------- | ------- | ------- | --- |
VCC (V)
|     |     | Ta = 25 C, ICLK = 32 MHz *1 |     | Ta = 105 C, ICLK = 32 MHz *2 |     |     |
| --- | --- | ---------------------------- | --- | ----------------------------- | --- | --- |
Note 1. All peripheral operations except any BGO operation are operating normally. This is the average of the
actual measurements of the sample cores during product evaluation.
Note 2. All peripheral operations except any BGO operation are operating at maximum. This is the average of the
actual measurements for the upper limit samples during product evaluation.
| Figure 2.21 | Voltage dependency in Subosc-speed mode (reference data) |     |     |     |     |     |
| ----------- | -------------------------------------------------------- | --- | --- | --- | --- | --- |
| Table 2.12  | Operating and standby current (2)                        |     |     |     |     |     |
Conditions: VCC = AVCC0 = 1.6 to 5.5 V
| Parameter |                   |          | Symbol Typ*4 | Max | Unit Test conditions |     |
| --------- | ----------------- | -------- | ------------ | --- | -------------------- | --- |
| Supply    | Software Standby  | T = 25°C | I 0.8        | 4.5 | μA -                 |     |
|           |                   | a        | CC           |     |                      |     |
| current*1 | mode*2            |          |              |     |                      |     |
|           |                   | T = 55°C | 1.3          | 7.1 |                      |     |
a
|     |     | T = 85°C | 3.5 | 20.2 |     |     |
| --- | --- | -------- | --- | ---- | --- | --- |
a
|     |     | T = 105°C | 8.7 | 53.7 |     |     |
| --- | --- | --------- | --- | ---- | --- | --- |
a
|     | Increment for RTC operation with  |     | 0.5 | -   | -   |     |
| --- | --------------------------------- | --- | --- | --- | --- | --- |
low-speed on-chip oscillator*3
Increment for RTC operation with  0.4 - SOMCR.SODRV[1:0] are 11b
|     | sub-clock oscillator*3 |     |     |     | (Low power mode 3)        |     |
| --- | ---------------------- | --- | --- | --- | ------------------------- | --- |
|     |                        |     | 1.2 | -   | SOMCR.SODRV[1:0] are 00b  |     |
(Normal mode)
Note 1. Supply current values do not include output charge/discharge current from all pins. The values apply when internal pull-up
MOSs are in the off state.
| Note 2.         | The IWDT and LVD are not operating.                                              |     |     |     |     |                |
| --------------- | -------------------------------------------------------------------------------- | --- | --- | --- | --- | -------------- |
| Note 3.         | Includes the current of sub-oscillation circuit or low-speed on-chip oscillator. |     |     |     |     |                |
| Note 4.         | VCC = 3.3 V.                                                                     |     |     |     |     |                |
| R01DS0355EJ0110 | Rev.1.10                                                                         |     |     |     |     | Page 50 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- |
100
10
)Am( )A( CCI
CCI
1
0.1
|     | -40 | -20 | 0   |     | 20  | 40  | 60  | 80  | 100 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Ta (C)
Average value of the tested middle samples during product evaluation.
Average value of the tested upper-limit samples during product evaluation.
Figure 2.22 Temperature dependency in Software Standby mode all SRAM (reference data)
| Table 2.13 | Operating and standby current (3) |     |     |     |     |     |     |     |     |
| ---------- | --------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 0V, VBATT = 1.6 to 3.6 V, VSS = AVSS0 = 0V
| Parameter |                 |     |          | Symbol | Typ | Max Unit | Test conditions         |     |     |
| --------- | --------------- | --- | -------- | ------ | --- | -------- | ----------------------- | --- | --- |
| Supply    | RTC operation   |     | T = 25°C | I      | 0.8 | - μA     | VBATT = 2.0 V           |     |     |
|           |                 |     | a        | CC     |     |          |                         |     |     |
| current*1 | when VCC is off |     |          |        |     |          | SOMCR.SORDRV[1:0] = 11b |     |     |
|           |                 |     | T = 55°C |        | 0.9 | -        |                         |     |     |
|           |                 |     | a        |        |     |          | (Low power mode 3)      |     |     |
|           |                 |     | T = 85°C |        | 1.0 | -        |                         |     |     |
a
|     |     |     | T = 105°C |     | 1.1 | -   |     |     |     |
| --- | --- | --- | --------- | --- | --- | --- | --- | --- | --- |
a
|     |     |     | T = 25°C |     | 0.9 | -   | VBATT = 3.3 V |     |     |
| --- | --- | --- | -------- | --- | --- | --- | ------------- | --- | --- |
a
SOMCR.SORDRV[1:0] = 11b
|     |     |     | T = 55°C |     | 1.0 | -   |                    |     |     |
| --- | --- | --- | -------- | --- | --- | --- | ------------------ | --- | --- |
|     |     |     | a        |     |     |     | (Low power mode 3) |     |     |
|     |     |     | T = 85°C |     | 1.1 | -   |                    |     |     |
a
|     |     |     | T = 105°C a |     | 1.2 | -   |               |     |     |
| --- | --- | --- | ----------- | --- | --- | --- | ------------- | --- | --- |
|     |     |     | T = 25°C a  |     | 1.5 | -   | VBATT = 2.0 V |     |     |
SOMCR.SORDRV[1:0] = 00b
|     |     |     | T = 55°C a |     | 1.7 | -   |     |     |     |
| --- | --- | --- | ---------- | --- | --- | --- | --- | --- | --- |
(Normal mode)
|     |     |     | T = 85°C a |     | 2.0 | -   |     |     |     |
| --- | --- | --- | ---------- | --- | --- | --- | --- | --- | --- |
|     |     |     | T = 105°C  |     | 2.2 | -   |     |     |     |
a
|     |     |     | T = 25°C |     | 1.6 | -   | VBATT = 3.3 V |     |     |
| --- | --- | --- | -------- | --- | --- | --- | ------------- | --- | --- |
a
SOMCR.SORDRV[1:0] = 00b
|     |     |     | T = 55°C |     | 1.8 | -   |               |     |     |
| --- | --- | --- | -------- | --- | --- | --- | ------------- | --- | --- |
|     |     |     | a        |     |     |     | (Normal mode) |     |     |
|     |     |     | T = 85°C |     | 2.1 | -   |               |     |     |
a
|     |     |     | T = 105°C |     | 2.3 | -   |     |     |     |
| --- | --- | --- | --------- | --- | --- | --- | --- | --- | --- |
a
Note 1. Supply current values do not include output charge/discharge current from all pins. The values apply when internal pull-up
MOSs are in the off state.
| R01DS0355EJ0110 |     | Rev.1.10 |     |     |     |     |     |     | Page 51 of 131 |
| --------------- | --- | -------- | --- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- | --- |
10
Normal drive capacity*1
)A( CCI
1
Low drive capacity*1
0
| -40 | -20 | 0   | 20  | 40  | 60  | 80  | 100 | 120 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
Ta(  C)
|     |     | Low drive capacity*1 |     |     | Normal drive capacity*1 |     |     |     |
| --- | --- | -------------------- | --- | --- | ----------------------- | --- | --- | --- |
Note 1. Average value of the tested middle sample during product evaluatio.n
Figure 2.23 Temperature dependency of RTC operation with VCC off (reference data)
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     | Page 52 of 131 |     |
| --------------- | -------- | --- | --- | --- | --- | --- | -------------- | --- |
Sep 29, 2023

| RA4M1 Group |                                   |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --------------------------------- | --- | --- | --- | ----------------------------- | --- |
| Table 2.14  | Operating and standby current (4) |     |     |     |                               |     |
Conditions: VCC = AVCC0 = 1.6 to 5.5 V, VREFH0 = 2.7 V to AVCC0
Test
| Parameter |     |     | Symbol Min | Typ | Max Unit | conditions |
| --------- | --- | --- | ---------- | --- | -------- | ---------- |
Analog power  During A/D conversion (at high-speed conversion) IAVCC - - 3.0 mA -
supply current
|     | During A/D conversion (at low-power conversion) |     | -   | -   | 1.0 mA | -   |
| --- | ----------------------------------------------- | --- | --- | --- | ------ | --- |
|     | During D/A conversion (per channel)*1           |     | -   | 0.4 | 0.8 mA | -   |
Waiting for A/D and D/A conversion (all units)*6
|            |                       |     | -        | -   | 1.0 μA | -   |
| ---------- | --------------------- | --- | -------- | --- | ------ | --- |
| Reference  | During A/D conversion |     | IREFH0 - | -   | 150 μA | -   |
power supply
| current            | Waiting for A/D conversion (all units) |     | -        | -   | 60 nA  | -   |
| ------------------ | -------------------------------------- | --- | -------- | --- | ------ | --- |
|                    | During D/A conversion                  |     | IREFH -  | 50  | 100 μA | -   |
|                    | Waiting for D/A conversion (all units) |     | -        | -   | 100 μA | -   |
| Temperature sensor |                                        |     | ITNS  -  | 75  | - μA   | -   |
| Low-Power          | Window mode                            |     | ICMPLP - | 15  | - μA   | -   |
Analog
| Comparator  | Comparator High-speed mode |     | -   | 10  | - μA | -   |
| ----------- | -------------------------- | --- | --- | --- | ---- | --- |
| operating   | Comparator Low-speed mode  |     | -   | 2   | - μA | -   |
current
|     | Comparator Low-speed mode using DAC8 |     | -   | 820 | - μA | -   |
| --- | ------------------------------------ | --- | --- | --- | ---- | --- |
Operational  Low power mode 1 unit operating IAMP - 2.5 4.0 μA -
Amplifier
|     |     | 2 units operating | -   | 4.5 | 8.0 μA | -   |
| --- | --- | ----------------- | --- | --- | ------ | --- |
operating
| current |                 | 3 units operating | -   | 6.5 | 11.0 μA | -   |
| ------- | --------------- | ----------------- | --- | --- | ------- | --- |
|         |                 | 4 units operating | -   | 8.5 | 14.0 μA | -   |
|         | High-speed mode | 1 unit operating  | -   | 140 | 220 μA  | -   |
|         |                 | 2 units operating | -   | 280 | 410 μA  | -   |
|         |                 | 3 units operating | -   | 420 | 600 μA  | -   |
|         |                 | 4 units operating | -   | 560 | 780 μA  | -   |
LCD operating  External resistance division method ILCD1*5 - 0.34 - μA -
| current | fLCD = fSUB = 128 Hz, 1/3 bias, and 4-time slice |     |     |     |     |     |
| ------- | ------------------------------------------------ | --- | --- | --- | --- | --- |
Internal voltage boosting method (VLCD.VLCD = 04) ILCD2*5 - 0.92 - μA -
fLCD = fSUB = 128 Hz, 1/3 bias, and 4-time slice
|     | Capacitor split method |     | ILCD3*5 - | 0.19 | - μA | -   |
| --- | ---------------------- | --- | --------- | ---- | ---- | --- |
fLCD = fSUB = 128 Hz, 1/3 bias, and 4-time slice
USB operating  During USB communication operation under the  IUSBH*2 - 4.3 (VCC) - mA -
| current | following settings and conditions: |     |     | 0.9 (VCC_USB)*4 |     |     |
| ------- | ---------------------------------- | --- | --- | --------------- | --- | --- |
 Host controller operation is set to full-speed mode
Bulk OUT transfer (64 bytes) × 1,
bulk IN transfer (64 bytes) × 1
 Connect peripheral devices via a 1-meter USB
cable from the USB port.
IUSBF*2
During USB communication operation under the  - 3.6 (VCC) - mA -
|     | following settings and conditions:  |     |     | 1.1 (VCC_USB)*4 |     |     |
| --- | ----------------------------------- | --- | --- | --------------- | --- | --- |
 Device controller operation is set to full-speed mode
Bulk OUT transfer (64 bytes) × 1,
bulk IN transfer (64 bytes) × 1
 Connect the host device via a 1-meter USB cable
from the USB port.
During suspended state under the following setting  ISUSP*3 - 0.35 (VCC) - μA -
|     | and conditions: |     |     | 170 (VCC_USB)*4 |     |     |
| --- | --------------- | --- | --- | --------------- | --- | --- |
 Device controller operation is set to full-speed mode
(pull up the USB_DP pin)
Software standby mode

 Connect the host device via a 1-meter USB cable
from the USB port.
Note 1. The reference power supply current is included in the power supply current value for D/A conversion.
| Note 2. Current consumed only by the USBFS. |     |     |     |     |     |     |
| ------------------------------------------- | --- | --- | --- | --- | --- | --- |
Note 3. Includes the current supplied from the pull-up resistor of the USB_DP pin to the pull-down resistor of the host device, in addition
to the current consumed by the MCU during the suspended state.
| Note 4. When VCC = VCC_USB = 3.3 V. |     |     |     |     |     |     |
| ----------------------------------- | --- | --- | --- | --- | --- | --- |
Note 5. Current flowing only to the LCD controller. Not including the current that flows through the LCD panel.
Note 6. When the MCU is in Software Standby mode or the MSTPCRD.MSTPD16 (ADC140 module stop bit) is in the module-stop
state.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 53 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |                                                 |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ----------------------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| 2.2.10      | VCC Rise and Fall Gradient and Ripple Frequency |     |     |     |     |                               |     |
| Table 2.15  | Rise and fall gradient characteristics          |     |     |     |     |                               |     |
Conditions: VCC = AVCC0 = 0 to 5.5 V
| Parameter |     |     |     | Symbol | Min Typ | Max Unit | Test conditions |
| --------- | --- | --- | --- | ------ | ------- | -------- | --------------- |
Power-on VCC  Voltage monitor 0 reset disabled at startup (normal  SrVCC 0.02 - 2 ms/V -
| rising gradient | startup)                                      |     |     |     |        |     |     |
| --------------- | --------------------------------------------- | --- | --- | --- | ------ | --- | --- |
|                 | Voltage monitor 0 reset enabled at startup*1  |     |     |     | 0.02 - | -   |     |
|                 | SCI/USB boot mode*2                           |     |     |     | 0.02 - | 2   |     |
Note 1. When OFS1.LVDAS = 0.
Note 2. At boot mode, the reset from voltage monitor 0 is disabled regardless of the value of OFS1.LVDAS bit.
Table 2.16 Rising and falling gradient and ripple frequency characteristics
Conditions: VCC = AVCC0 = VCC_USB = 1.6 to 5.5 V
The ripple voltage must meet the allowable ripple frequency f  within the range between the VCC upper limit (5.5 V) and lower limit
r(VCC)
(1.6 V).
When VCC change exceeds VCC ±10%, the allowable voltage change rising/falling gradient dt/dVCC must be met.
| Parameter                  |     | Symbol | Min Typ | Max | Unit Test conditions |     |     |
| -------------------------- | --- | ------ | ------- | --- | -------------------- | --- | --- |
| Allowable ripple frequency |     | f      | - -     | 10  | kHz Figure 2.24      |     |     |
r (VCC)
|     |     |     |     |     | V   |  ≤ VCC × 0.2 |     |
| --- | --- | --- | --- | --- | --- | ------------ | --- |
r (VCC)
|     |     |     | - - | 1   | MHz Figure 2.24 |               |     |
| --- | --- | --- | --- | --- | --------------- | ------------- | --- |
|     |     |     |     |     | V               |  ≤ VCC × 0.08 |     |
r (VCC)
|     |     |     | - - | 10  | MHz Figure 2.24 |               |     |
| --- | --- | --- | --- | --- | --------------- | ------------- | --- |
|     |     |     |     |     | V               |  ≤ VCC × 0.06 |     |
r (VCC)
Allowable voltage change rising and  dt/dVCC 1.0 - - ms/V When VCC change exceeds VCC ±10%
falling gradient
1/f r(VCC)
VCC
V r(VCC)
| Figure 2.24     | Ripple waveform |     |     |     |     |     |                |
| --------------- | --------------- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10        |     |     |     |     |     | Page 54 of 131 |
Sep 29, 2023

| RA4M1 Group            |                                                        |     |     | 2. Electrical Characteristics |     |
| ---------------------- | ------------------------------------------------------ | --- | --- | ----------------------------- | --- |
| 2.3 AC Characteristics |                                                        |     |     |                               |     |
| 2.3.1                  | Frequency                                              |     |     |                               |     |
| Table 2.17             | Operation frequency value in high-speed operating mode |     |     |                               |     |
Conditions: VCC = AVCC0 = 2.4 to 5.5 V
| Parameter |     |     | Symbol Min | Typ | Max*5 Unit |
| --------- | --- | --- | ---------- | --- | ---------- |
Operation  System clock (ICLK)*4 2.7 to 5.5 V f 0.032768 - 48 MHz
frequency
|     |     | 2.4 to 2.7 V | 0.032768 | -   | 16  |
| --- | --- | ------------ | -------- | --- | --- |
Flash interface clock (FCLK)*1, *2, *4 2.7 to 5.5 V 0.032768 - 32
|     |                                       | 2.4 to 2.7 V | 0.032768 | -   | 16  |
| --- | ------------------------------------- | ------------ | -------- | --- | --- |
|     | Peripheral module clock (PCLKA)*4     | 2.7 to 5.5 V | -        | -   | 48  |
|     |                                       | 2.4 to 2.7 V | -        | -   | 16  |
|     | Peripheral module clock (PCLKB)*4     | 2.7 to 5.5 V | -        | -   | 32  |
|     |                                       | 2.4 to 2.7 V | -        | -   | 16  |
|     | Peripheral module clock (PCLKC)*3, *4 | 2.7 to 5.5 V | -        | -   | 64  |
|     |                                       | 2.4 to 2.7 V | -        | -   | 16  |
|     | Peripheral module clock (PCLKD)*4     | 2.7 to 5.5 V | -        | -   | 64  |
|     |                                       | 2.4 to 2.7 V | -        | -   | 16  |
Note 1. The lower-limit frequency of FCLK is 1 MHz while programming or erasing the flash memory. When using FCLK for
programming or erasing the flash memory at below 4MHz, the frequency can be set to 1MHz, 2MHz, or 3MHz. A non-integer
frequency such as 1.5MHz cannot be set.
Note 2. The frequency accuracy of FCLK must be ±3.5% while programming or erasing the flash memory. Confirm the frequency
accuracy of the clock source.
Note 3. The lower-limit frequency of PCLKC is 4 MHz at 2.4V or above and 1 MHz at below 2.4V when the 14-bit A/D converter is in
use.
Note 4. See section 8, Clock Generation Circuit in Userʼs Manual for the relationship of frequencies between ICLK, PCLKA, PCLKB,
PCLKC, PCLKD, and FCLK.
Note 5. The maximum value of operation frequency does not include internal oscillator errors. For details on the range of guaranteed
operation, see Table 2.22, Clock timing.
| Table 2.18 | Operation frequency value in middle-speed operating mode |     |     |     |     |
| ---------- | -------------------------------------------------------- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
| Parameter |     |     | Symbol Min | Typ Max*5 | Unit |
| --------- | --- | --- | ---------- | --------- | ---- |
Operation  System clock (ICLK)*4 2.7 to 5.5 V f 0.032768 - 12 MHz
frequency
|     |     | 2.4 to 2.7 V | 0.032768 | - 12 |     |
| --- | --- | ------------ | -------- | ---- | --- |
|     |     | 1.8 to 2.4 V | 0.032768 | - 8  |     |
Flash interface clock (FCLK)*1, *2, *4 2.7 to 5.5 V 0.032768 - 12
|     |                                   | 2.4 to 2.7 V | 0.032768 | - 12 |     |
| --- | --------------------------------- | ------------ | -------- | ---- | --- |
|     |                                   | 1.8 to 2.4 V | 0.032768 | - 8  |     |
|     | Peripheral module clock (PCLKA)*4 | 2.7 to 5.5 V | -        | - 12 |     |
|     |                                   | 2.4 to 2.7 V | -        | - 12 |     |
|     |                                   | 1.8 to 2.4 V | -        | - 8  |     |
Peripheral module clock (PCLKB)*4
|     |                                       | 2.7 to 5.5 V | -   | - 12 |     |
| --- | ------------------------------------- | ------------ | --- | ---- | --- |
|     |                                       | 2.4 to 2.7 V | -   | - 12 |     |
|     |                                       | 1.8 to 2.4 V | -   | - 8  |     |
|     | Peripheral module clock (PCLKC)*3, *4 | 2.7 to 5.5 V | -   | - 12 |     |
|     |                                       | 2.4 to 2.7 V | -   | - 12 |     |
|     |                                       | 1.8 to 2.4 V | -   | - 8  |     |
Peripheral module clock (PCLKD)*4
|                 |          | 2.7 to 5.5 V | -   | - 12 |                |
| --------------- | -------- | ------------ | --- | ---- | -------------- |
|                 |          | 2.4 to 2.7 V | -   | - 12 |                |
|                 |          | 1.8 to 2.4 V | -   | - 8  |                |
| R01DS0355EJ0110 | Rev.1.10 |              |     |      | Page 55 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | ----------------------------- | --- |
Note 1. The lower-limit frequency of FCLK is 1 MHz while programming or erasing the flash memory. When using FCLK for
programming or erasing the flash memory at below 4 MHz, the frequency can be set to 1 MHz, 2 MHz, or 3 MHz. A non-integer
frequency such as 1.5 MHz cannot be set.
Note 2. The frequency accuracy of FCLK must be ±3.5% while programming or erasing the flash memory. Confirm the frequency
accuracy of the clock source.
Note 3. The lower-limit frequency of PCLKC is 4 MHz at 2.4 V or above and 1 MHz at below 2.4 V when the 14-bit A/D converter is in use.
Note 4. See section 8, Clock Generation Circuit in Userʼs Manual for the relationship of frequencies between ICLK, PCLKA, PCLKB,
PCLKC, PCLKD, FCLK.
Note 5. The maximum value of operation frequency does not include internal oscillator errors. For details on the range of guaranteed
operation, see Table 2.22, Clock timing.
| Table 2.19 | Operation frequency value in low-speed operating mode |     |     |     |     |
| ---------- | ----------------------------------------------------- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
| Parameter |     |     | Symbol Min | Typ Max*4 | Unit |
| --------- | --- | --- | ---------- | --------- | ---- |
Operation  System clock (ICLK)*3 1.8 to 5.5 V f 0.032768 - 1 MHz
frequency
|     | Flash interface clock (FCLK)*1, *3 | 1.8 to 5.5 V | 0.032768 | - 1 |     |
| --- | ---------------------------------- | ------------ | -------- | --- | --- |
|     | Peripheral module clock (PCLKA)*3  | 1.8 to 5.5 V | -        | - 1 |     |
Peripheral module clock (PCLKB)*3
|     |                                       | 1.8 to 5.5 V | -   | - 1 |     |
| --- | ------------------------------------- | ------------ | --- | --- | --- |
|     | Peripheral module clock (PCLKC)*2, *3 | 1.8 to 5.5 V | -   | - 1 |     |
|     | Peripheral module clock (PCLKD)*3     | 1.8 to 5.5 V | -   | - 1 |     |
Note 1. The lower-limit frequency of FCLK is 1MHz while programming or erasing the flash memory.
| Note 2. The lower-limit frequency of PCLKC is 1MHz when the A/D converter is in use. |     |     |     |     |     |
| ------------------------------------------------------------------------------------ | --- | --- | --- | --- | --- |
Note 3. See section 8, Clock Generation Circuit in Userʼs Manual for the relationship of frequencies between ICLK, PCLKA, PCLKB,
PCLKC, PCLKD, FCLK.
Note 4. The maximum value of operation frequency does not include internal oscillator errors. For details on the range of guaranteed
operation, see Table 2.22, Clock timing.
| Table 2.20 | Operation frequency value in low-voltage operating mode |     |     |     |     |
| ---------- | ------------------------------------------------------- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 1.6 to 5.5 V
| Parameter |     |     | Symbol Min | Typ Max*5 | Unit |
| --------- | --- | --- | ---------- | --------- | ---- |
Operation  System clock (ICLK)*4 1.6 to 5.5 V f 0.032768 - 4 MHz
frequency
Flash interface clock (FCLK)*1, *2, *4 1.6 to 5.5 V 0.032768 - 4
|     | Peripheral module clock (PCLKA)*4 | 1.6 to 5.5 V | -   | - 4 |     |
| --- | --------------------------------- | ------------ | --- | --- | --- |
|     | Peripheral module clock (PCLKB)*4 | 1.6 to 5.5 V | -   | - 4 |     |
Peripheral module clock (PCLKC)*3, *4
|     |                                   | 1.6 to 5.5 V | -   | - 4 |     |
| --- | --------------------------------- | ------------ | --- | --- | --- |
|     | Peripheral module clock (PCLKD)*4 | 1.6 to 5.5 V | -   | - 4 |     |
Note 1. The lower-limit frequency of FCLK is 1MHz while programming or erasing the flash memory. When using FCLK for
programming or erasing the flash memory at below 4MHz, the frequency can be set to 1MHz, 2MHz, or 3 MHz. A non-integer
frequency such as 1.5MHz cannot be set.
Note 2. The frequency accuracy of FCLK must be ±3.5% while programming or erasing the flash memory. Confirm the frequency
accuracy of the clock source.
Note 3. The lower-limit frequency of PCLKC is 4MHz at 2.4V or above and 1MHz at below 2.4V when the 14-Bit A/D converter is in
use.
Note 4. See section 8, Clock Generation Circuit in Userʼs Manual for the relationship of frequencies between ICLK, PCLKA, PCLKB,
PCLKC, PCLKD, FCLK.
Note 5. The maximum value of operation frequency does not include internal oscillator errors. For details on the range of guaranteed
operation, see Table 2.22, Clock timing.
| R01DS0355EJ0110 | Rev.1.10 |     |     | Page 56 of 131 |     |
| --------------- | -------- | --- | --- | -------------- | --- |
Sep 29, 2023

| RA4M1 Group |                                                          |     |     |     |     | 2. Electrical Characteristics |     |     |
| ----------- | -------------------------------------------------------- | --- | --- | --- | --- | ----------------------------- | --- | --- |
| Table 2.21  | Operation frequency value in Subosc-speed operating mode |     |     |     |     |                               |     |     |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
| Parameter |     |     |     | Symbol | Min | Typ | Max | Unit |
| --------- | --- | --- | --- | ------ | --- | --- | --- | ---- |
Operation  System clock (ICLK)*3 1.8 to 5.5 V f 27.8528 32.768 37.6832 kHz
frequency
Flash interface clock (FCLK)*1, *3 1.8 to 5.5 V 27.8528 32.768 37.6832
|     | Peripheral module clock (PCLKA)*3 | 1.8 to 5.5 V |     |     | -   | -   | 37.6832 |     |
| --- | --------------------------------- | ------------ | --- | --- | --- | --- | ------- | --- |
Peripheral module clock (PCLKB)*3
|     |     | 1.8 to 5.5 V |     |     | -   | -   | 37.6832 |     |
| --- | --- | ------------ | --- | --- | --- | --- | ------- | --- |
Peripheral module clock (PCLKC)*2, *3 1.8 to 5.5 V - - 37.6832
|                                                                   | Peripheral module clock (PCLKD)*3 | 1.8 to 5.5 V |     |     | -   | -   | 37.6832 |     |
| ----------------------------------------------------------------- | --------------------------------- | ------------ | --- | --- | --- | --- | ------- | --- |
| Note 1. Programming and erasing the flash memory is not possible. |                                   |              |     |     |     |     |         |     |
| Note 2. The 14-bit A/D converter cannot be used.                  |                                   |              |     |     |     |     |         |     |
Note 3. See section 8, Clock Generation Circuit in Userʼs Manual for the relationship of frequencies between ICLK, PCLKA, PCLKB,
PCLKC, PCLKD, FCLK.
| 2.3.2      | Clock Timing          |        |     |     |     |      |                 |     |
| ---------- | --------------------- | ------ | --- | --- | --- | ---- | --------------- | --- |
| Table 2.22 | Clock timing (1 of 2) |        |     |     |     |      |                 |     |
| Parameter  |                       | Symbol | Min | Typ | Max | Unit | Test conditions |     |
EXTAL external clock input cycle time  tXcyc 50 - - ns Figure 2.25
| EXTAL external clock input high pulse width |     | tXH   | 20  | -   | -   | ns  |     |     |
| ------------------------------------------- | --- | ----- | --- | --- | --- | --- | --- | --- |
| EXTAL external clock input low pulse width  |     | tXL   | 20  | -   | -   | ns  |     |     |
| EXTAL external clock rising time            |     | tXr   | -   | -   | 5   | ns  |     |     |
| EXTAL external clock falling time           |     | tXf   | -   | -   | 5   | ns  |     |     |
| EXTAL external clock input wait time*1      |     | tEXWT | 0.3 | -   | -   | μs  | -   |     |
EXTAL external clock input frequency fEXTAL - - 20 MHz 2.4 ≤ VCC ≤ 5.5
|     |     |     | -   | -   | 8   |     | 1.8 ≤ VCC < 2.4 |     |
| --- | --- | --- | --- | --- | --- | --- | --------------- | --- |
|     |     |     | -   | -   | 1   |     | 1.6 ≤ VCC < 1.8 |     |
Main clock oscillator oscillation frequency fMAIN 1 - 20 MHz 2.4 ≤ VCC ≤ 5.5
|     |     |     | 1   | -   | 8   |     | 1.8 ≤ VCC < 2.4 |     |
| --- | --- | --- | --- | --- | --- | --- | --------------- | --- |
|     |     |     | 1   | -   | 4   |     | 1.6 ≤ VCC < 1.8 |     |
Main clock oscillation stabilization wait time (crystal)*9 tMAINOSCWT - - -*9 ms -
LOCO clock oscillation frequency fLOCO 27.8528 32.768 37.6832 kHz -
LOCO clock oscillation stabilization time tLOCO - - 100 μs Figure 2.26
IWDT-dedicated clock oscillation frequency fILOCO 12.75 15 17.25 kHz -
| MOCO clock oscillation frequency          |          | fMOCO | 6.8 | 8   | 9.2 | MHz | -              |     |
| ----------------------------------------- | -------- | ----- | --- | --- | --- | --- | -------------- | --- |
| MOCO clock oscillation stabilization time |          | tMOCO | -   | -   | 1   | μs  | -              |     |
| R01DS0355EJ0110                           | Rev.1.10 |       |     |     |     |     | Page 57 of 131 |     |
Sep 29, 2023

| RA4M1 Group                      |     |        |         | 2. Electrical Characteristics |                 |
| -------------------------------- | --- | ------ | ------- | ----------------------------- | --------------- |
| Table 2.22 Clock timing (2 of 2) |     |        |         |                               |                 |
| Parameter                        |     | Symbol | Min Typ | Max Unit                      | Test conditions |
HOCO clock oscillation frequency fHOCO24 23.64 24 24.36 MHz Ta = -40 to -20°C
1.8 ≤ VCC ≤ 5.5
|     |     |     | 22.68 24 | 25.32 | Ta = -40 to 85°C |
| --- | --- | --- | -------- | ----- | ---------------- |
1.6 ≤ VCC < 1.8
|     |     |     | 23.76 24 | 24.24 | Ta = -20 to 85°C |
| --- | --- | --- | -------- | ----- | ---------------- |
1.8 ≤ VCC ≤ 5.5
|     |     |     | 23.52 24 | 24.48 | Ta = 85 to 105°C |
| --- | --- | --- | -------- | ----- | ---------------- |
2.4 ≤ VCC ≤ 5.5
|     |     | fHOCO32 | 31.52 32 | 32.48 | Ta = -40 to -20°C |
| --- | --- | ------- | -------- | ----- | ----------------- |
1.8 ≤ VCC ≤ 5.5
|     |     |     | 30.24 32 | 33.76 | Ta = -40 to 85°C |
| --- | --- | --- | -------- | ----- | ---------------- |
1.6 ≤ VCC < 1.8
|     |     |     | 31.68 32 | 32.32 | Ta = -20 to 85°C |
| --- | --- | --- | -------- | ----- | ---------------- |
1.8 ≤ VCC ≤ 5.5
|     |     |     | 31.36 32 | 32.64 | Ta = 85 to 105°C |
| --- | --- | --- | -------- | ----- | ---------------- |
2.4 ≤ VCC ≤ 5.5
fHOCO48*4
|     |     |     | 47.28 48 | 48.72 | Ta = -40 to -20°C |
| --- | --- | --- | -------- | ----- | ----------------- |
1.8 ≤ VCC ≤ 5.5
|     |     |     | 47.52 48 | 48.48 | Ta = -20 to 85°C |
| --- | --- | --- | -------- | ----- | ---------------- |
1.8 ≤ VCC ≤ 5.5
|     |     |     | 47.04 48 | 48.96 | Ta = 85 to 105°C |
| --- | --- | --- | -------- | ----- | ---------------- |
2.4 ≤ VCC ≤ 5.5
|     |     | fHOCO64*5 | 63.04 64 | 64.96 | Ta = -40 to -20°C |
| --- | --- | --------- | -------- | ----- | ----------------- |
2.4 ≤ VCC ≤ 5.5
|     |     |     | 63.36 64 | 64.64 | Ta = -20 to 85°C |
| --- | --- | --- | -------- | ----- | ---------------- |
2.4 ≤ VCC ≤ 5.5
|     |     |     | 62.72 64 | 65.28 | Ta = 85 to 105°C |
| --- | --- | --- | -------- | ----- | ---------------- |
2.4 ≤ VCC ≤ 5.5
HOCO clock oscillation  Except Low-Voltage  t HOCO24 - - 37.1 μs Figure 2.27
stabilization time*6, *7
|     | mode | t HOCO32 |     |      |     |
| --- | ---- | -------- | --- | ---- | --- |
|     |      | t HOCO48 | - - | 43.3 |     |
|     |      | t        | - - | 80.6 |     |
HOCO64
|     | Low-Voltage mode | t   | - - | 100.9 |     |
| --- | ---------------- | --- | --- | ----- | --- |
HOCO24
t
HOCO32
t
HOCO48
t
HOCO64
| PLL input frequency*2               |     | fPLLIN | 4 -  | 12.5 MHz | -   |
| ----------------------------------- | --- | ------ | ---- | -------- | --- |
| PLL circuit oscillation frequency*2 |     | fPLL   | 24 - | 64 MHz   | -   |
PLL clock oscillation stabilization time*8 tPLL - - 55.5 μs Figure 2.29
| PLL free-running oscillation frequency |     | fPLLFR | - 8 | - MHz | -   |
| -------------------------------------- | --- | ------ | --- | ----- | --- |
Sub-clock oscillator oscillation frequency fSUB - 32.768 - kHz -
Sub-clock oscillation stabilization time*3  tSUBOSC - - -*3 s Figure 2.30
Note 1. Time until the clock can be used after the Main Clock Oscillator Stop bit (MOSCCR.MOSTP) is set to 0 (operating) when the
external clock is stable.
Note 2. The VCC range that the PLL can be used is 2.4 to 5.5 V.
Note 3. After changing the setting of the SOSCCR.SOSTP bit so that the sub-clock oscillator operates, only start using the sub-clock
after the sub-clock oscillation stabilization wait time elapses, that is greater than or equal to the value recommended by the
oscillator manufacturer.
Note 4. The 48-MHz HOCO can be used within a VCC range of 1.8 V to 5.5 V.
Note 5. The 64-MHz HOCO can be used within a VCC range of 2.4 V to 5.5 V.
Note 6. This is a characteristic when HOCOCR.HCSTP bit is set to 0 (oscillation) in MOCO stop state.
When HOCOCR.HCSTP bit is set to 0 (oscillation) during MOCO oscillation, this specification is shortened by 1 μs.
Note 7. Whether stabilization time has elapsed can be confirmed by OSCSF.HOCOSF.
Note 8. This is a characteristic when PLLCR.PLLSTP bit is set to 0 (operation) in MOCO stop state.
When PLLCR.PLLSTP bit is set to 0 (operation) during MOCO oscillation, this specification is shortened by 1 μs.
Note 9. When setting up the main clock, ask the oscillator manufacturer for an oscillation evaluation and use the results as the
recommended oscillation stabilization time. Set the MOSCWTCR register to a value equal to or greater than the recommended
stabilization time. After changing the setting of the MOSCCR.MOSTP bit so that the main clock oscillator operates, read the
OSCSF.MOSCSF flag to confirm that it is 1, then start using the main clock.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 58 of 131 |
| --------------- | -------- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
t
Xcyc
t t
XH XL
EXTAL external clock input VCC × 0.5
t t
Xr Xf
Figure 2.25 EXTAL external clock input timing
LOCOCR.LCSTP
t
LOCO
LOCO clock oscillator output
Figure 2.26 LOCO clock oscillation start timing
HOCOCR.HCSTP
t *1
HOCOx
HOCO clock
Note 1. x = 24, 32, 48, 64
Figure 2.27 HOCO clock oscillation start timing (started by setting HOCOCR.HCSTP bit)
MOSCCR.MOSTP
Main clock oscillator output
t
MAINOSCWT
Main clock
Figure 2.28 Main clock oscillation start timing
PLLCR.PLLSTP
t
PLL
PLL clock
Figure 2.29 PLL clock oscillation start timing (PLL is operated after main clock oscillation has settled)
R01DS0355EJ0110 Rev.1.10 Page 59 of 131
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
SOSCCR.SOSTP
t SUBOSC
Sub-clock oscillator output
| Figure 2.30 | Sub-clock oscillation start timing |     |
| ----------- | ---------------------------------- | --- |
MOCOCR.MCSTP
t
MOCO
MOCO clock oscillator output
| Figure 2.31     | MOCO clock oscillation start timing |                |
| --------------- | ----------------------------------- | -------------- |
| R01DS0355EJ0110 | Rev.1.10                            | Page 60 of 131 |
Sep 29, 2023

| RA4M1 Group             |     |     |     | 2. Electrical Characteristics |     |
| ----------------------- | --- | --- | --- | ----------------------------- | --- |
| 2.3.3 Reset Timing      |     |     |     |                               |     |
| Table 2.23 Reset timing |     |     |     |                               |     |
Test
| Parameter       |     |             | Symbol Min | Typ Max | Unit conditions |
| --------------- | --- | ----------- | ---------- | ------- | --------------- |
| RES pulse width |     | At power-on | t 3        | - -     | ms Figure 2.32  |
RESWP
|     |     | Other than above | t 30 | - - | μs Figure 2.33 |
| --- | --- | ---------------- | ---- | --- | -------------- |
RESW
Wait time after RES cancellation LVD0: enable*1 t - 0.7 - ms Figure 2.32
RESWT
(at power-on)
|     |     | LVD0: disable*2 | -   | 0.3 - |     |
| --- | --- | --------------- | --- | ----- | --- |
Wait time after RES cancellation LVD0: enable*1 t - 0.5 - ms Figure 2.33
RESWT2
(during powered-on state)
|     |     | LVD0: disable*2 | -   | 0.05 - |     |
| --- | --- | --------------- | --- | ------ | --- |
Internal reset cancellation time (Watchdog  LVD0: enable*1 t - 0.6 - ms -
RESWT3
timer reset, SRAM parity error reset,
|     |     | LVD0: disable*2 | -   | 0.15 - |     |
| --- | --- | --------------- | --- | ------ | --- |
SRAM ECC error reset, Bus master MPU
error reset, Bus slave MPU error reset,
Stack pointer error reset, Software reset)
Note 1. When OFS1.LVDAS = 0.
Note 2. When OFS1.LVDAS = 1.
VCC
RES
t
RESWP
Internal reset
t RESWT
| Figure 2.32 | Reset input timing at power-on |     |     |     |     |
| ----------- | ------------------------------ | --- | --- | --- | --- |
t RESW
RES
Internal reset
t
RESWT2
| Figure 2.33     | Reset input timing |     |     |     |                |
| --------------- | ------------------ | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10           |     |     |     | Page 61 of 131 |
Sep 29, 2023

| RA4M1 Group |                                             |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ------------------------------------------- | --- | --- | --- | ----------------------------- | --- |
| 2.3.4       | Wakeup Time                                 |     |     |     |                               |     |
| Table 2.24  | Timing of recovery from low power modes (1) |     |     |     |                               |     |
Test
| Parameter |     |     |     | Symbol Min | Typ Max | Unit conditions |
| --------- | --- | --- | --- | ---------- | ------- | --------------- |
Recovery time  High-speed  Crystal  System clock source is  t - 2 3 ms Figure 2.34
SBYMC
| from Software  | mode | resonator     | main clock oscillator  |     |     |     |
| -------------- | ---- | ------------- | ---------------------- | --- | --- | --- |
| Standby mode*1 |      | connected to  | (20 MHz)*2             |     |     |     |
main clock
|     |     |            | System clock source is  | t -   | 2 3 | ms  |
| --- | --- | ---------- | ----------------------- | ----- | --- | --- |
|     |     | oscillator |                         | SBYPC |     |     |
PLL (48 MHz) with main
clock oscillator*2
|     |     | External clock  | System clock source is  | t - | 14 25 | μs  |
| --- | --- | --------------- | ----------------------- | --- | ----- | --- |
SBYEX
|     |     | input to main    | main clock oscillator   |     |       |     |
| --- | --- | ---------------- | ----------------------- | --- | ----- | --- |
|     |     | clock oscillator | (20 MHz)*3              |     |       |     |
|     |     |                  | System clock source is  | t - | 53 76 | μs  |
SBYPE
PLL (48 MHz) with main
clock oscillator*3
|     |     | System clock source is HOCO*4 |     | t - | 43 52 | μs  |
| --- | --- | ----------------------------- | --- | --- | ----- | --- |
SBYHO
(HOCO clock is 32 MHz)
|     |     | System clock source is HOCO*4 |     | t - | 44 52 | μs  |
| --- | --- | ----------------------------- | --- | --- | ----- | --- |
SBYHO
(HOCO clock is 48 MHz)
|     |     | System clock source is HOCO*5 |     | t - | 82 110 | μs  |
| --- | --- | ----------------------------- | --- | --- | ------ | --- |
SBYHO
(HOCO clock is 64 MHz)
|     |     | System clock source is MOCO |     | t SBYMO - | 16 25 | μs  |
| --- | --- | --------------------------- | --- | --------- | ----- | --- |
Note 1. The division ratio of ICK, FCK, and PCKx is the minimum division ratio within the allowable frequency range. The recovery time
is determined by the system clock source.
| Note 2. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 05h. |                                             |     |     |     |     |     |
| --------------------------------------------------------------------------------- | ------------------------------------------- | --- | --- | --- | --- | --- |
| Note 3. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 00h. |                                             |     |     |     |     |     |
| Note 4. The HOCO Clock Wait Control Register (HOCOWTCR) is set to 05h.            |                                             |     |     |     |     |     |
| Note 5. The HOCO Clock Wait Control Register (HOCOWTCR) is set to 06h.            |                                             |     |     |     |     |     |
| Table 2.25                                                                        | Timing of recovery from low power modes (2) |     |     |     |     |     |
Test
| Parameter |     |     |     | Symbol Min | Typ Max | Unit conditions |
| --------- | --- | --- | --- | ---------- | ------- | --------------- |
Recovery time  Middle-speed  Crystal  System clock source is  t - 2 3 ms Figure 2.34
SBYMC
| from Software  | mode | resonator     | main clock oscillator  |     |     |     |
| -------------- | ---- | ------------- | ---------------------- | --- | --- | --- |
| Standby mode*1 |      | connected to  | (12 MHz)*2             |     |     |     |
main clock
|     |     |            | System clock source is  | t -   | 2 3 | ms  |
| --- | --- | ---------- | ----------------------- | ----- | --- | --- |
|     |     | oscillator |                         | SBYPC |     |     |
PLL (24 MHz) with main
clock oscillator*2
|     |     | External clock  | System clock source is  | t - | 2.9 10 | μs  |
| --- | --- | --------------- | ----------------------- | --- | ------ | --- |
SBYEX
|     |     | input to main    | main clock oscillator   |           |       |     |
| --- | --- | ---------------- | ----------------------- | --------- | ----- | --- |
|     |     | clock oscillator | (12 MHz)*3              |           |       |     |
|     |     |                  | System clock source is  | t SBYPE - | 49 76 | μs  |
PLL (24 MHz) with main
clock oscillator*3
|     |     | System clock source is HOCO (24 MHz) |     | t SBYHO - | 38 50   | μs  |
| --- | --- | ------------------------------------ | --- | --------- | ------- | --- |
|     |     | System clock source is MOCO          |     | t SBYMO - | 3.5 5.5 | μs  |
Note 1. The division ratio of ICK, FCK, and PCKx is the minimum division ratio within the allowable frequency range. The recovery time
is determined by the system clock source.
| Note 2. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 05h. |          |     |     |     |     |                |
| --------------------------------------------------------------------------------- | -------- | --- | --- | --- | --- | -------------- |
| Note 3. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 00h. |          |     |     |     |     |                |
| R01DS0355EJ0110                                                                   | Rev.1.10 |     |     |     |     | Page 62 of 131 |
Sep 29, 2023

| RA4M1 Group |                                             |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ------------------------------------------- | --- | --- | --- | ----------------------------- | --- |
| Table 2.26  | Timing of recovery from low power modes (3) |     |     |     |                               |     |
Test
| Parameter |     |     |     | Symbol Min | Typ Max | Unit conditions |
| --------- | --- | --- | --- | ---------- | ------- | --------------- |
Recovery time  Low-speed  Crystal  System clock source is  t - 2 3 ms Figure 2.34
SBYMC
| from Software  | mode | resonator     | main clock oscillator |     |     |     |
| -------------- | ---- | ------------- | --------------------- | --- | --- | --- |
| Standby mode*1 |      | connected to  | (1 MHz)*2             |     |     |     |
main clock
oscillator
|     |     | External clock  | System clock source is  | t - | 28 50 | μs  |
| --- | --- | --------------- | ----------------------- | --- | ----- | --- |
SBYEX
|     |     | input to main               | main clock oscillator |     |       |     |
| --- | --- | --------------------------- | --------------------- | --- | ----- | --- |
|     |     | clock oscillator            | (1 MHz)*3             |     |       |     |
|     |     | System clock source is MOCO |                       | t - | 25 35 | μs  |
SBYMO
Note 1. The division ratio of ICK, FCK, and PCKx is the minimum division ratio within the allowable frequency range. The recovery time
is determined by the system clock source.
| Note 2. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 05h. |                                             |     |     |     |     |     |
| --------------------------------------------------------------------------------- | ------------------------------------------- | --- | --- | --- | --- | --- |
| Note 3. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 00h. |                                             |     |     |     |     |     |
| Table 2.27                                                                        | Timing of recovery from low power modes (4) |     |     |     |     |     |
Test
| Parameter |     |     |     | Symbol Min | Typ Max | Unit conditions |
| --------- | --- | --- | --- | ---------- | ------- | --------------- |
Recovery time  Low-voltage  Crystal  System clock source is  t - 2 3 ms Figure 2.34
SBYMC
| from Software  | mode | resonator     | main clock oscillator |     |     |     |
| -------------- | ---- | ------------- | --------------------- | --- | --- | --- |
| Standby mode*1 |      | connected to  | (4 MHz)*2             |     |     |     |
main clock
oscillator
|     |     | External clock  | System clock source is  | t - | 108 130 | μs  |
| --- | --- | --------------- | ----------------------- | --- | ------- | --- |
SBYEX
|     |     | input to main               | main clock oscillator |     |         |     |
| --- | --- | --------------------------- | --------------------- | --- | ------- | --- |
|     |     | clock oscillator            | (4 MHz)*3             |     |         |     |
|     |     | System clock source is HOCO |                       | t - | 108 130 | μs  |
SBYHO
Note 1. The division ratio of ICK, FCK, and PCKx is the minimum division ratio within the allowable frequency range. The recovery time
is determined by the system clock source. When multiple oscillators are active, the recovery time can be determined by the
following expression.
| Note 2. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 05h. |                                             |     |     |     |     |     |
| --------------------------------------------------------------------------------- | ------------------------------------------- | --- | --- | --- | --- | --- |
| Note 3. The Main Clock Oscillator Wait Control Register (MOSCWTCR) is set to 00h. |                                             |     |     |     |     |     |
| Table 2.28                                                                        | Timing of recovery from low power modes (5) |     |     |     |     |     |
Test
| Parameter |     |     |     | Symbol Min | Typ Max | Unit conditions |
| --------- | --- | --- | --- | ---------- | ------- | --------------- |
Recovery time  Subosc-speed mode System clock source is sub-clock  t - 0.85 1 ms Figure 2.34
SBYSC
| from Software  |     | oscillator (32.768 kHz) |     |     |     |     |
| -------------- | --- | ----------------------- | --- | --- | --- | --- |
Standby mode*1
|     |     | System clock source is LOCO  |     | t - | 0.85 1.2 | ms  |
| --- | --- | ---------------------------- | --- | --- | -------- | --- |
SBYLO
(32.768 kHz)
Note 1. The sub-clock oscillator or LOCO itself continues to oscillate in Software Standby mode during Subosc-speed mode.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 63 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | ----------------------------- | --- |
Oscillator
ICLK
IRQ
Software Standby mode
t t t
SBYMC, SBYPC, SBYEX,
t , t , t
SBYPE SBYMO SBYHO
Oscillator
ICLK
IRQ
Software Standby mode
t SBYSC , t SBYLO
| Figure 2.34                                            | Software Standby mode cancellation timing |            |         |      |                 |
| ------------------------------------------------------ | ----------------------------------------- | ---------- | ------- | ---- | --------------- |
| Table 2.29 Timing of recovery from low power modes (6) |                                           |            |         |      |                 |
| Parameter                                              |                                           | Symbol Min | Typ Max | Unit | Test conditions |
| Recovery time from                                     | High-speed mode                           | t -        | 36 45   | μs   | Figure 2.35     |
SNZ
| Software Standby  | System clock source is HOCO |     |     |     |     |
| ----------------- | --------------------------- | --- | --- | --- | --- |
mode to Snooze
|      | Middle-speed mode | t - | 1.3 3.6 | μs  |     |
| ---- | ----------------- | --- | ------- | --- | --- |
| mode |                   | SNZ |         |     |     |
System clock source is MOCO
|     | Low-speed mode | t - | 10 13 | μs  |     |
| --- | -------------- | --- | ----- | --- | --- |
SNZ
System clock source is MOCO
|     | Low-voltage mode | t - | 87 110 | μs  |     |
| --- | ---------------- | --- | ------ | --- | --- |
SNZ
System clock source is HOCO
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 64 of 131 |
| --------------- | -------- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
Oscillator
ICLK (except DTC, SRAM)
ICLK (to DTC, SRAM)*1
PCLK
IRQ
Software Standby mode Snooze mode
t
SNZ
Note 1. When SNZCR.SNZDTCEN is set to 1, ICLK is supplied to DTC and SRAM.
Figure 2.35 Software Standby mode to Snooze mode recovery timing
R01DS0355EJ0110 Rev.1.10 Page 65 of 131
Sep 29, 2023

| RA4M1 Group |                          |         | 2. Electrical Characteristics |     |
| ----------- | ------------------------ | ------- | ----------------------------- | --- |
| 2.3.5       | NMI and IRQ Noise Filter |         |                               |     |
| Table 2.30  | NMI and IRQ noise filter |         |                               |     |
| Parameter   | Symbol Min               | Typ Max | Unit Test conditions          |     |
NMI pulse width t 200 - - ns NMI digital filter disabled t  × 2 ≤ 200 ns
|     | NMIW     |     |     | Pcyc            |
| --- | -------- | --- | --- | --------------- |
|     | t  × 2*1 | - - |     | t  × 2 > 200 ns |
Pcyc Pcyc
|     | 200 | - - | NMI digital filter enabled | t  × 3 ≤ 200 ns |
| --- | --- | --- | -------------------------- | --------------- |
NMICK
|     | t  × 3.5*2 | - - |     | t  × 3 > 200 ns |
| --- | ---------- | --- | --- | --------------- |
NMICK NMICK
IRQ pulse width t 200 - - ns IRQ digital filter disabled t  × 2 ≤ 200 ns
|     | IRQW     |     |     | Pcyc            |
| --- | -------- | --- | --- | --------------- |
|     | t  × 2*1 | - - |     | t  × 2 > 200 ns |
Pcyc Pcyc
|     | 200 | - - | IRQ digital filter enabled | t  × 3 ≤ 200 ns |
| --- | --- | --- | -------------------------- | --------------- |
IRQCK
|     | t IRQCK  × 3.5*3 | - - |     | t IRQCK  × 3 > 200 ns |
| --- | ---------------- | --- | --- | --------------------- |
Note: 200 ns minimum in Software Standby mode.
Note: If the clock source is switched, add 4 clock cycles of the switched source.
| Note 1. t  indicates the cycle of PCLKB. |     |     |     |     |
| ---------------------------------------- | --- | --- | --- | --- |
Pcyc
| Note 2. t NMICK |  indicates the cycle of the NMI digital filter sampling clock. |     |     |     |
| --------------- | -------------------------------------------------------------- | --- | --- | --- |
Note 3. t  indicates the cycle of the IRQi digital filter sampling clock (i = 0 to 12, 14, 15).
IRQCK
NMI
t
NMIW
| Figure 2.36 | NMI interrupt input timing |     |     |     |
| ----------- | -------------------------- | --- | --- | --- |
IRQ
t
IRQW
| Figure 2.37     | IRQ interrupt input timing |     |     |                |
| --------------- | -------------------------- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10                   |     |     | Page 66 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     | 2. Electrical Characteristics |     |     |
| ----------- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- | --- |
2.3.6 I/O Ports, POEG, GPT, AGT, KINT, and ADC14 Trigger Timing
| Table 2.31 | I/O Ports, POEG, GPT, AGT, KINT, and ADC14 trigger timing |     |     |     |     |     |     |     |     |
| ---------- | --------------------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
Test
| Parameter |     |                                                  |     |     | Symbol | Min | Max | Unit | conditions  |
| --------- | --- | ------------------------------------------------ | --- | --- | ------ | --- | --- | ---- | ----------- |
| I/O ports |     | Input data pulse width                           |     |     | t      | 1.5 | -   | t    | Figure 2.38 |
|           |     |                                                  |     |     | PRW    |     |     | Pcyc |             |
|           |     | Input/output data cycle (P002, P003, P004, P007) |     |     | t      | 10  | -   | us   |             |
POcyc
| POEG |     | POEG input trigger pulse width |     |     | t    | 3   | -   | t    | Figure 2.39 |
| ---- | --- | ------------------------------ | --- | --- | ---- | --- | --- | ---- | ----------- |
|      |     |                                |     |     | POEW |     |     | Pcyc |             |
GPT Input capture pulse width Single edge t 1.5 - t Figure 2.40
|     |     |     |     |           | GTICW |     |     | PDcyc |     |
| --- | --- | --- | --- | --------- | ----- | --- | --- | ----- | --- |
|     |     |     |     | Dual edge |       | 2.5 | -   |       |     |
AGT AGTIO, AGTEE input cycle 2.7 V ≤ VCC ≤ 5.5 V t *1 250 - ns Figure 2.41
ACYC
|     |     |     |     | 2.4 V ≤ VCC < 2.7 V |     | 500  | -   | ns  |     |
| --- | --- | --- | --- | ------------------- | --- | ---- | --- | --- | --- |
|     |     |     |     | 1.8 V ≤ VCC < 2.4 V |     | 1000 | -   | ns  |     |
|     |     |     |     | 1.6 V ≤ VCC < 1.8 V |     | 2000 | -   | ns  |     |
AGTIO, AGTEE input high level  2.7 V ≤ VCC ≤ 5.5 V t ACKWH ,  100 - ns
|     |     | width, low-level width |     |                     | t ACKWL |     |     |     |     |
| --- | --- | ---------------------- | --- | ------------------- | ------- | --- | --- | --- | --- |
|     |     |                        |     | 2.4 V ≤ VCC < 2.7 V |         | 200 | -   | ns  |     |
|     |     |                        |     | 1.8 V ≤ VCC < 2.4 V |         | 400 | -   | ns  |     |
|     |     |                        |     | 1.6 V ≤ VCC < 1.8 V |         | 800 | -   | ns  |     |
AGTIO, AGTO, AGTOA, AGTOB  2.7 V ≤ VCC ≤ 5.5 V t 62.5 - ns Figure 2.41
ACYC2
output cycle
|     |     |     |     | 2.4 V ≤ VCC < 2.7 V |     | 125 | -   | ns  |     |
| --- | --- | --- | --- | ------------------- | --- | --- | --- | --- | --- |
|     |     |     |     | 1.8 V ≤ VCC < 2.4 V |     | 250 | -   | ns  |     |
|     |     |     |     | 1.6 V ≤ VCC < 1.8 V |     | 500 | -   | ns  |     |
ADC14 14-bit A/D converter trigger input pulse width t TRGW 1.5 - t Pcyc Figure 2.42
KINT KRn (n = 00 to 07) pulse width t KR 250 - ns Figure 2.43
| Note 1. | Constraints on input cycle:            |     |     |                                |     |     |     |     |     |
| ------- | -------------------------------------- | --- | --- | ------------------------------ | --- | --- | --- | --- | --- |
|         | When not switching the source clock: t |     |     |  × 2 < t  should be satisfied. |     |     |     |     |     |
Pcyc ACYC
|       | When switching the source clock: t |     | Pcyc  × 6 < t | ACYC  should be satisfied. |     |     |     |     |     |
| ----- | ---------------------------------- | --- | ------------- | -------------------------- | --- | --- | --- | --- | --- |
| Note: | t : PCLKB cycle, t                 |     | : PCLKD cycle |                            |     |     |     |     |     |
|       | Pcyc                               |     | PDcyc         |                            |     |     |     |     |     |
Port
t
PRW
| Figure 2.38 |     | I/O ports input timing |     |     |     |     |     |     |     |
| ----------- | --- | ---------------------- | --- | --- | --- | --- | --- | --- | --- |
POEG input trigger
t POEW
| Figure 2.39     |     | POEG input trigger timing |     |     |     |     |     |     |                |
| --------------- | --- | ------------------------- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 |     | Rev.1.10                  |     |     |     |     |     |     | Page 67 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | ----------------------------- | --- |
Input capture
t GTICW
| Figure 2.40 | GPT input capture timing |     |     |     |     |     |
| ----------- | ------------------------ | --- | --- | --- | --- | --- |
t
ACYC
|     |     |     | t ACKWL | t ACKWH |     |     |
| --- | --- | --- | ------- | ------- | --- | --- |
AGTIO, AGTEE
(input)
t
ACYC2
AGTIO, AGTO,
AGTOA, AGTOB
(output)
| Figure 2.41 | AGT I/O timing |     |     |     |     |     |
| ----------- | -------------- | --- | --- | --- | --- | --- |
ADTRG0
t TRGW
| Figure 2.42 | ADC14 trigger input timing |     |     |     |     |     |
| ----------- | -------------------------- | --- | --- | --- | --- | --- |
KR00 to KR07
t
KR
| Figure 2.43           | Key interrupt input timing |     |     |     |     |     |
| --------------------- | -------------------------- | --- | --- | --- | --- | --- |
| 2.3.7 CAC Timing      |                            |     |     |     |     |     |
| Table 2.32 CAC timing |                            |     |     |     |     |     |
Test
| Parameter |     |     | Symbol Min |     | Typ Max | Unit conditions |
| --------- | --- | --- | ---------- | --- | ------- | --------------- |
CAC CACREF input pulse width t *1 ≤ t *2 t 4.5 × t  + 3 × t *1 - - ns -
|                 |          | PBcyc cac | CACREF   | cac        | PBcyc  |                |
| --------------- | -------- | --------- | -------- | ---------- | ------ | -------------- |
|                 |          | t *1 > t  | *2 5 × t |  + 6.5 × t | *1 - - | ns             |
|                 |          | PBcyc cac |          | cac        | PBcyc  |                |
| R01DS0355EJ0110 | Rev.1.10 |           |          |            |        | Page 68 of 131 |
Sep 29, 2023

| RA4M1 Group |                                   |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| Note 1.     | t PBcyc : PCLKB cycle.            |     |     |     |     |                               |     |
| Note 2.     | t : CAC count clock source cycle. |     |     |     |     |                               |     |
cac
| 2.3.8      | SCI Timing     |     |     |     |     |     |     |
| ---------- | -------------- | --- | --- | --- | --- | --- | --- |
| Table 2.33 | SCI timing (1) |     |     |     |     |     |     |
Test
| Parameter |     |     |     | Symbol | Min | Max Unit*1 | conditions |
| --------- | --- | --- | --- | ------ | --- | ---------- | ---------- |
SCI Input clock cycle Asynchronous t Scyc 4 - t Pcyc Figure 2.44
|     |                         | Clock synchronous |     |      | 6   | -     |     |
| --- | ----------------------- | ----------------- | --- | ---- | --- | ----- | --- |
|     | Input clock pulse width |                   |     | t    | 0.4 | 0.6 t |     |
|     |                         |                   |     | SCKW |     | Scyc  |     |
|     | Input clock rise time   |                   |     | t    | -   | 20 ns |     |
SCKr
|     | Input clock fall time |     |     | t   | -   | 20 ns |     |
| --- | --------------------- | --- | --- | --- | --- | ----- | --- |
SCKf
|     | Output clock cycle       | Asynchronous      |                | t Scyc | 6   | - t Pcyc |     |
| --- | ------------------------ | ----------------- | -------------- | ------ | --- | -------- | --- |
|     |                          | Clock synchronous |                |        | 4   | -        |     |
|     | Output clock pulse width |                   |                | t      | 0.4 | 0.6 t    |     |
|     |                          |                   |                | SCKW   |     | Scyc     |     |
|     | Output clock rise time   |                   | 1.8 V or above | t      | -   | 20 ns    |     |
SCKr
|     |                        |     | 1.6 V or above |     | -   | 30    |     |
| --- | ---------------------- | --- | -------------- | --- | --- | ----- | --- |
|     | Output clock fall time |     | 1.8 V or above | t   | -   | 20 ns |     |
SCKf
|     |     |     | 1.6 V or above |     | -   | 30  |     |
| --- | --- | --- | -------------- | --- | --- | --- | --- |
Transmit data delay  Clock  1.8 V or above t TXD - 40 ns Figure 2.45
|     | (master)             | synchronous | 1.6 V or above |     | -   | 45    |     |
| --- | -------------------- | ----------- | -------------- | --- | --- | ----- | --- |
|     | Transmit data delay  | Clock       | 2.7 V or above |     | -   | 55 ns |     |
|     | (slave)              | synchronous |                |     |     |       |     |
|     |                      |             | 2.4 V or above |     | -   | 60    |     |
|     |                      |             | 1.8 V or above |     | -   | 100   |     |
|     |                      |             | 1.6 V or above |     | -   | 125   |     |
|     | Receive data setup   | Clock       | 2.7 V or above | t   | 45  | - ns  |     |
RXS
|     | time (master)       | synchronous       |                |     |     |      |     |
| --- | ------------------- | ----------------- | -------------- | --- | --- | ---- | --- |
|     |                     |                   | 2.4 V or above |     | 55  | -    |     |
|     |                     |                   | 1.8 V or above |     | 90  | -    |     |
|     |                     |                   | 1.6 V or above |     | 110 | -    |     |
|     | Receive data setup  | Clock             | 2.7 V or above |     | 40  | - ns |     |
|     | time (slave)        | synchronous       |                |     |     |      |     |
|     |                     |                   | 1.6 V or above |     | 45  | -    |     |
|     | Receive data hold   | Clock synchronous |                | t   | 5   | - ns |     |
RXH
time (master)
|     | Receive data hold  | Clock synchronous |     | t   | 40  | - ns |     |
| --- | ------------------ | ----------------- | --- | --- | --- | ---- | --- |
RXH
time (slave)
| Note 1. | t : PCLKA cycle. |     |     |     |     |     |     |
| ------- | ---------------- | --- | --- | --- | --- | --- | --- |
Pcyc
|     |     |     | t    | t    | t    |     |     |
| --- | --- | --- | ---- | ---- | ---- | --- | --- |
|     |     |     | SCKW | SCKr | SCKf |     |     |
SCKn
(n = 0 to 2, 9)
t
Scyc
| Figure 2.44     | SCK clock input timing |     |     |     |     |     |                |
| --------------- | ---------------------- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10               |     |     |     |     |     | Page 69 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- |
SCKn
t TXD
TXDn
|     |     |     | t   | t   |     |     |
| --- | --- | --- | --- | --- | --- | --- |
|     |     |     | RXS | RXH |     |     |
RXDn
(n = 0 to 2, 9)
| Figure 2.45                        | SCI input/output timing in clock synchronous mode |     |        |     |     |                      |
| ---------------------------------- | ------------------------------------------------- | --- | ------ | --- | --- | -------------------- |
| Table 2.34 SCI timing (2) (1 of 2) |                                                   |     |        |     |     |                      |
| Parameter                          |                                                   |     | Symbol | Min | Max | Unit Test conditions |
Simple  SCK clock cycle output (master) t SPcyc 4 65,536 t Pcyc Figure 2.46
| SPI SCK clock cycle input (slave) |     |                |        | 6   | 65,536 |       |
| --------------------------------- | --- | -------------- | ------ | --- | ------ | ----- |
| SCK clock high pulse width        |     |                | t      | 0.4 | 0.6    | t     |
|                                   |     |                | SPCKWH |     |        | SPcyc |
| SCK clock low pulse width         |     |                | t      | 0.4 | 0.6    | t     |
|                                   |     |                | SPCKWL |     |        | SPcyc |
| SCK clock rise and fall time      |     | 1.8 V or above | t      | -   | 20     | ns    |
SPCKr,
t
|     |     | 1.6 V or above | SPCKf | -   | 30  |     |
| --- | --- | -------------- | ----- | --- | --- | --- |
Data input setup  Master 2.7 V or above t SU 45 - ns Figure 2.47 to
| time                 |        | 2.4 V or above |     | 55   | -   | Figure 2.50 |
| -------------------- | ------ | -------------- | --- | ---- | --- | ----------- |
|                      |        | 1.8 V or above |     | 80   | -   |             |
|                      |        | 1.6 V or above |     | 110  | -   |             |
|                      | Slave  | 2.7 V or above |     | 40   | -   |             |
|                      |        | 1.6 V or above |     | 45   | -   |             |
| Data input hold time | Master |                | t   | 33.3 | -   | ns          |
H
|                     | Slave  |                |      | 40  | -   |       |
| ------------------- | ------ | -------------- | ---- | --- | --- | ----- |
| SS input setup time |        |                | t    | 1   | -   | t     |
|                     |        |                | LEAD |     |     | SPcyc |
| SS input hold time  |        |                | t    | 1   | -   | t     |
|                     |        |                | LAG  |     |     | SPcyc |
| Data output delay   | Master | 1.8 V or above | t OD | -   | 40  | ns    |
|                     |        | 1.6 V or above |      | -   | 50  |       |
|                     | Slave  | 2.4 V or above |      | -   | 65  |       |
|                     |        | 1.8 V or above |      | -   | 100 |       |
|                     |        | 1.6 V or above |      | -   | 125 |       |
| Data output hold    | Master | 2.7 V or above | t OH | -10 | -   | ns    |
time
|                     |        | 2.4 V or above |     | -20 | -   |     |
| ------------------- | ------ | -------------- | --- | --- | --- | --- |
|                     |        | 1.8 V or above |     | -30 | -   |     |
|                     |        | 1.6 V or above |     | -40 | -   |     |
|                     | Slave  |                |     | -10 | -   |     |
| Data rise and fall  | Master | 1.8 V or above | t t | -   | 20  | ns  |
Dr, Df
time
|                 |          | 1.6 V or above |     | -   | 30  |                |
| --------------- | -------- | -------------- | --- | --- | --- | -------------- |
|                 | Slave    | 1.8 V or above |     | -   | 20  |                |
|                 |          | 1.6 V or above |     | -   | 30  |                |
| R01DS0355EJ0110 | Rev.1.10 |                |     |     |     | Page 70 of 131 |
Sep 29, 2023

| RA4M1 Group |                   |                         |     |     |     |        |     |     |     |             |     | 2. Electrical Characteristics |                  |
| ----------- | ----------------- | ----------------------- | --- | --- | --- | ------ | --- | --- | --- | ----------- | --- | ----------------------------- | ---------------- |
| Table 2.34  |                   | SCI timing (2) (2 of 2) |     |     |     |        |     |     |     |             |     |                               |                  |
| Parameter   |                   |                         |     |     |     | Symbol |     | Min |     | Max         |     | Unit                          | Test conditions  |
| Simple      | Slave access time |                         |     |     |     | t      |     | -   |     | 10 (PCLKA > |     | t                             | Figure 2.49 and  |
|             |                   |                         |     |     |     | SA     |     |     |     |             |     | Pcyc                          |                  |
| SPI         |                   |                         |     |     |     |        |     |     |     | 32 MHz),    |     |                               | Figure 2.50      |
6 (PCLKA ≤
32 MHz)
|     | Slave output release time |     |     |     |     | t REL |     | -   |     | 10 (PCLKA > |     | t Pcyc |     |
| --- | ------------------------- | --- | --- | --- | --- | ----- | --- | --- | --- | ----------- | --- | ------ | --- |
32 MHz),
6 (PCLKA ≤
32 MHz)
|     |     |                |     |     | t      |     |     |     | t     |     |     | t     |     |
| --- | --- | -------------- | --- | --- | ------ | --- | --- | --- | ----- | --- | --- | ----- | --- |
|     |     |                |     |     | SPCKWH |     |     |     | SPCKr |     |     | SPCKf |     |
|     |     |                |     | V   |        |     | V   |     |       | V   | V   |       |     |
|     |     | SCKn           |     | OH  |        |     | OH  |     |       | OH  | OH  |       |     |
|     |     | master select  |     |     |        |     |     | V   | V     |     |     | V     |     |
|     |     | output         |     |     |        |     |     | OL  | OL    |     |     |       | OL  |
t
SPCKWL
t
SPcyc
|     |     |     |     |     | t   |     |     |     | t SPCKr |     |     | t SPCKf |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ------- | --- | --- | ------- | --- |
SPCKWH
|     |     |     |     | V   |     |     | V   |     |     | V   | V   |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     |     | IH  |     |     |     | IH  |     | IH  | IH  |     |     |
SCKn
|     |     | slave select input |     |     |     |     |     | V IL | V IL |     |     | V   | IL  |
| --- | --- | ------------------ | --- | --- | --- | --- | --- | ---- | ---- | --- | --- | --- | --- |
t
|     |     | (n = 0 to 2, 9) |     |     |     |     |     | SPCKWL |     |     |     |     |     |
| --- | --- | --------------- | --- | --- | --- | --- | --- | ------ | --- | --- | --- | --- | --- |
t SPcyc
|             |     |                                  |     | V = 0.7 × VCC, V |     |     | = 0.3 × VCC, V |     | = 0.7 × VCC, V |     | = 0.3 × VCC |     |     |
| ----------- | --- | -------------------------------- | --- | ---------------- | --- | --- | -------------- | --- | -------------- | --- | ----------- | --- | --- |
|             |     |                                  |     | OH               |     | OL  |                |     | IH             |     | IL          |     |     |
| Figure 2.46 |     | SCI simple SPI mode clock timing |     |                  |     |     |                |     |                |     |             |     |     |
SCKn
CKPOL = 0
output
SCKn
CKPOL = 1
output
|     |     |     | t   | t   |     |     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | SU  | H   |     |     |     |     |     |     |     |     |     |
MISOn
|     |     |     | MSB IN |     | DATA |     |     | LSB IN |     |     |     |     | MSB IN |
| --- | --- | --- | ------ | --- | ---- | --- | --- | ------ | --- | --- | --- | --- | ------ |
input
t t
|     |     |     | Dr,  Df | t   | OH  |     | t OD |     |     |     |     |     |     |
| --- | --- | --- | ------- | --- | --- | --- | ---- | --- | --- | --- | --- | --- | --- |
MOSIn
|     |     |     | MSB OUT |     | DATA |     |     | LSB OUT |     |     | IDLE |     | MSB OUT |
| --- | --- | --- | ------- | --- | ---- | --- | --- | ------- | --- | --- | ---- | --- | ------- |
output
(n = 0 to 2, 9)
| Figure 2.47     |     | SCI simple SPI mode timing for master when CKPH = 1 |     |     |     |     |     |     |     |     |     |     |                |
| --------------- | --- | --------------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 |     | Rev.1.10                                            |     |     |     |     |     |     |     |     |     |     | Page 71 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- |
SCKn
CKPOL = 1
output
SCKn
CKPOL = 0
output
|     |       |     |     | t t    |     |      |     |        |     |     |        |
| --- | ----- | --- | --- | ------ | --- | ---- | --- | ------ | --- | --- | ------ |
|     |       |     |     | SU H   |     |      |     |        |     |     |        |
|     | MISOn |     |     | MSB IN |     | DATA |     | LSB IN |     |     | MSB IN |
input
|     |     |     | t   |     | t   |     |     |     | t      | t   |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ------ | --- | --- |
|     |     |     | OH  |     | OD  |     |     |     | Dr, Df |     |     |
MOSIn
|     |     |     |     | MSB OUT |     | DATA |     | LSB OUT |     | IDLE | MSB OUT |
| --- | --- | --- | --- | ------- | --- | ---- | --- | ------- | --- | ---- | ------- |
output
(n = 0 to 2, 9)
| Figure 2.48 |     | SCI simple SPI mode timing for master when CKPH = 0 |     |     |     |     |     |     |     |     |     |
| ----------- | --- | --------------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
t
TD
SSn
input
|     |     | t LEAD |     |     |     |     |     |     | t LAG |     |     |
| --- | --- | ------ | --- | --- | --- | --- | --- | --- | ----- | --- | --- |
SCKn
CKPOL = 0
input
SCKn
CKPOL = 1
input
|     |     | t   |     | t   |     | t   |     |     |     | t   |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     | SA  |     | OH  |     | OD  |     |     |     | REL |     |
MISOn
|     |     |     |     | MSB OUT |     | DATA | LSB OUT |     |     | MSB IN | MSB OUT |
| --- | --- | --- | --- | ------- | --- | ---- | ------- | --- | --- | ------ | ------- |
output
|     |     |     | t SU | t H |     |     |     | t Dr, Df | t   |     |     |
| --- | --- | --- | ---- | --- | --- | --- | --- | -------- | --- | --- | --- |
MOSIn
|     | input |     | MSB IN |     | DATA |     | LSB IN |     |     |     | MSB IN |
| --- | ----- | --- | ------ | --- | ---- | --- | ------ | --- | --- | --- | ------ |
(n = 0 to 2, 9)
| Figure 2.49     |     | SCI simple SPI mode timing for slave when CKPH = 1 |     |     |     |     |     |     |     |     |                |
| --------------- | --- | -------------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 |     | Rev.1.10                                           |     |     |     |     |     |     |     |     | Page 72 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- |
t TD
SSn
input
|     |     | t LEAD |     |     |     |     |     | t LAG |     |     |     |
| --- | --- | ------ | --- | --- | --- | --- | --- | ----- | --- | --- | --- |
SCKn
CKPOL = 1
input
SCKn
CKPOL = 0
input
|       | t SA |             | t OH |         | t OD |      |     |         | t REL |     |         |
| ----- | ---- | ----------- | ---- | ------- | ---- | ---- | --- | ------- | ----- | --- | ------- |
| MISOn |      | LSB OUT     |      |         |      |      |     |         |       |     |         |
|       |      | (Last data) |      | MSB OUT |      | DATA |     | LSB OUT |       |     | MSB OUT |
output
|     |     |     |     | t SU t H |     |     | t Dr, Df t |     |     |     |     |
| --- | --- | --- | --- | -------- | --- | --- | ---------- | --- | --- | --- | --- |
MOSIn
|     |     |     |     | MSB IN |     | DATA |     | LSB IN |     |     | MSB IN |
| --- | --- | --- | --- | ------ | --- | ---- | --- | ------ | --- | --- | ------ |
input

(n = 0 to 2, 9)
| Figure 2.50 | SCI simple SPI mode timing for slave when CKPH = 0 |     |     |     |     |     |     |     |     |     |     |
| ----------- | -------------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Table 2.35  | SCI timing (3)                                     |     |     |     |     |     |     |     |     |     |     |
Conditions: VCC = 2.7 to 5.5 V
| Parameter  |                     |     |     |     |     | Symbol | Min | Max  |     | Unit | Test conditions |
| ---------- | ------------------- | --- | --- | --- | --- | ------ | --- | ---- | --- | ---- | --------------- |
| Simple I2C | SDA input rise time |     |     |     |     | t      | -   | 1000 |     | ns   | Figure 2.51     |
Sr
(Standard mode)
|     | SDA input fall time |     |     |     |     | t   | -   | 300 |     | ns  |     |
| --- | ------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Sf
|     | SDA input spike pulse removal time |     |     |     |     | t   | 0   | 4 × t | *1     | ns  |     |
| --- | ---------------------------------- | --- | --- | --- | --- | --- | --- | ----- | ------ | --- | --- |
|     |                                    |     |     |     |     | SP  |     |       | IICcyc |     |     |
|     | Data input setup time              |     |     |     |     | t   | 250 | -     |        | ns  |     |
SDAS
|     | Data input hold time |     |     |     |     | t   | 0   | -   |     | ns  |     |
| --- | -------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
SDAH
|     | SCL, SDA capacitive load |     |     |     |     | C *2 | -   | 400 |     | pF  |     |
| --- | ------------------------ | --- | --- | --- | --- | ---- | --- | --- | --- | --- | --- |
b
| Simple I2C | SDA input rise time |     |     |     |     | t   | -   | 300 |     | ns  | Figure 2.51 |
| ---------- | ------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------- |
Sr
| (Fast mode) |                     |     |     |     |     |      |     |     |     |     | For all ports  |
| ----------- | ------------------- | --- | --- | --- | --- | ---- | --- | --- | --- | --- | -------------- |
|             | SDA input fall time |     |     |     |     | t Sf | -   | 300 |     | ns  |                |
except P408, use
SDA input spike pulse removal time t SP 0 4 × t IICcyc *1 ns PmnPFS.DSCR
of middle drive.
|     | Data input setup time |     |     |     |     | t SDAS | 100 | -   |     | ns  |     |
| --- | --------------------- | --- | --- | --- | --- | ------ | --- | --- | --- | --- | --- |
For port P408,
|     | Data input hold time     |     |     |     |     | t SDAH | 0   | -   |     | ns  | use          |
| --- | ------------------------ | --- | --- | --- | --- | ------ | --- | --- | --- | --- | ------------ |
|     | SCL, SDA capacitive load |     |     |     |     | C *1   | -   | 400 |     | pF  | PmnPFS.DSCR1 |
b
/DSCR of middle
drive for IIC
fast-mode.
Note 1. tIICcyc: Clock cycle selected by the SMR.CKS[1:0] bits.
Note 2. Cb indicates the total capacity of the bus line.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     |     |     |     |     | Page 73 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | ----------------------------- | --- |
V
IH
SDAn
V
IL
t t
Sr Sf
t SP
SCLn
|     | P*1 S*1 | Sr*1 |     | P*1 |
| --- | ------- | ---- | --- | --- |
(n = 0 to 2, 9)
t t
SDAH SDAS
|     | Note 1. S, P, and Sr indicate the following: | Test conditions:  |              |     |
| --- | -------------------------------------------- | ----------------- | ------------ | --- |
|     | S: Start condition                           | V  = VCC × 0.7, V |  = VCC × 0.3 |     |
|     |                                              | IH                | IL           |     |
|     | P: Stop condition                            | V  = 0.6 V, I     |  = 6 mA      |     |
|     |                                              | OL                | OL           |     |
Sr: Restart condition
| Figure 2.51     | SCI simple IIC mode timing |     |     |                |
| --------------- | -------------------------- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10                   |     |     | Page 74 of 131 |
Sep 29, 2023

| RA4M1 Group                    |     |     |     |     | 2. Electrical Characteristics |
| ------------------------------ | --- | --- | --- | --- | ----------------------------- |
| 2.3.9 SPI Timing               |     |     |     |     |                               |
| Table 2.36 SPI timing (1 of 2) |     |     |     |     |                               |
Conditions: Middle drive output is selected in the Port Drive Capability bit in PmnPFS register
| Parameter             |        |     | Symbol Min  | Max         | Unit*1 Test conditions |
| --------------------- | ------ | --- | ----------- | ----------- | ---------------------- |
| SPI RSPCK clock cycle | Master |     | t 2*4       | 4096        | t Figure 2.52          |
|                       |        |     | SPcyc       |             | Pcyc                   |
|                       | Slave  |     | 6           | 4096        |                        |
| RSPCK clock high      | Master |     | t SPCKWH (t | SPcyc  -  - | ns                     |
| pulse width           |        |     | t SPCKr     | -           |                        |
t SPCKf ) / 2 -
3
|                  | Slave  |     | 3 × t  | Pcyc - |     |
| ---------------- | ------ | --- | ------ | ------ | --- |
| RSPCK clock low  | Master |     | t (t   |  -  -  | ns  |
|                  |        |     | SPCKWL | SPcyc  |     |
| pulse width      |        |     | t      | -      |     |
SPCKr
t ) / 2 -
SPCKf
3
|     | Slave |     | 3 × t | -   |     |
| --- | ----- | --- | ----- | --- | --- |
Pcyc
| RSPCK clock rise  | Output | 2.7 V or above | t - | 10  | ns  |
| ----------------- | ------ | -------------- | --- | --- | --- |
SPCKr,
| and fall time         |         |                | t       |     |                    |
| --------------------- | ------- | -------------- | ------- | --- | ------------------ |
|                       |         | 2.4 V or above | SPCKf - | 15  |                    |
|                       |         | 1.8 V or above | -       | 20  |                    |
|                       |         | 1.6 V or above | -       | 30  |                    |
|                       | Input   |                | -       | 1   | µs                 |
| Data input setup time | Master  |                | t SU 10 | -   | ns Figure 2.53 to  |
|                       | Slave   | 2.4 V or above | 10      | -   | Figure 2.58        |
|                       |         | 1.8 V or above | 15      | -   |                    |
|                       |         | 1.6 V or above | 20      | -   |                    |
| Data input hold time  | Master  |                | t HF 0  | -   | ns                 |
(RSPCK is PCLKA/2)
|     | Master |     | t H t Pcyc | -   |     |
| --- | ------ | --- | ---------- | --- | --- |
(RSPCK is other than
above.)
|     | Slave |     | t 20 | -   |     |
| --- | ----- | --- | ---- | --- | --- |
H
| SSL setup time | Master | 1.8 V or above | t -30 + N ×  | -   | ns  |
| -------------- | ------ | -------------- | ------------ | --- | --- |
LEAD
t *2
Spcyc
|     |     | 1.6 V or above | -50 + N ×  | -   |     |
| --- | --- | -------------- | ---------- | --- | --- |
t *2
Spcyc
|     | Slave |     | 6 × t | -   |     |
| --- | ----- | --- | ----- | --- | --- |
Pcyc
| SSL hold time | Master |     | t LAG -30 + N ×  | -   |     |
| ------------- | ------ | --- | ---------------- | --- | --- |
t Spcyc *3
|                 | Slave    |     | 6 × t | Pcyc - |                |
| --------------- | -------- | --- | ----- | ------ | -------------- |
| R01DS0355EJ0110 | Rev.1.10 |     |       |        | Page 75 of 131 |
Sep 29, 2023

| RA4M1 Group                    |     |     |     |     | 2. Electrical Characteristics |
| ------------------------------ | --- | --- | --- | --- | ----------------------------- |
| Table 2.36 SPI timing (2 of 2) |     |     |     |     |                               |
Conditions: Middle drive output is selected in the Port Drive Capability bit in PmnPFS register
| Parameter |     |     | Symbol Min | Max | Unit*1 Test conditions |
| --------- | --- | --- | ---------- | --- | ---------------------- |
SPI Data output delay Master 2.7 V or above t - 14 ns Figure 2.53 to
OD
Figure 2.58
|                       |        | 2.4 V or above | -      | 20            |     |
| --------------------- | ------ | -------------- | ------ | ------------- | --- |
|                       |        | 1.8 V or above | -      | 25            |     |
|                       |        | 1.6 V or above | -      | 30            |     |
|                       | Slave  | 2.7 V or above | -      | 50            |     |
|                       |        | 2.4 V or above | -      | 60            |     |
|                       |        | 1.8 V or above | -      | 85            |     |
|                       |        | 1.6 V or above | -      | 110           |     |
| Data output hold time | Master |                | t OH 0 | -             | ns  |
|                       | Slave  |                | 0      | -             |     |
| Successive            | Master |                | t t    |  + 2 ×  8 × t | ns  |
|                       |        |                | TD     | SPcyc SPcyc   |     |
| transmission delay    |        |                | t      | + 2 × t       |     |
Pcyc Pcyc
|     | Slave |     | 6 × t | -   |     |
| --- | ----- | --- | ----- | --- | --- |
Pcyc
MOSI and MISO rise  Output  2.7 V or above t Dr, Df t - 10 ns
and fall time
|                        |         | 2.4 V or above | -   | 15  |     |
| ---------------------- | ------- | -------------- | --- | --- | --- |
|                        |         | 1.8 V or above | -   | 20  |     |
|                        |         | 1.6 V or above | -   | 30  |     |
|                        | Input   |                | -   | 1   | µs  |
| SSL rise and fall time | Output  | 2.7 V or above | t - | 10  | ns  |
SSLr,
t
|     |       | 2.4 V or above | SSLf - | 15  |     |
| --- | ----- | -------------- | ------ | --- | --- |
|     |       | 1.8 V or above | -      | 20  |     |
|     |       | 1.6 V or above | -      | 30  |     |
|     | Input |                | -      | 1   | µs  |
Slave access time 2.4 V or above t - 2 × t  + 100 ns Figure 2.57 and
SA Pcyc
Figure 2.58
|                           |     | 1.8 V or above | -   | 2 × t Pcyc  + 140 |     |
| ------------------------- | --- | -------------- | --- | ----------------- | --- |
|                           |     | 1.6 V or above | -   | 2 × t Pcyc  + 180 |     |
| Slave output release time |     | 2.4 V or above | t - | 2 × t  + 100      | ns  |
REL Pcyc
|     |     | 1.8 V or above | -   | 2 × t  + 140 |     |
| --- | --- | -------------- | --- | ------------ | --- |
Pcyc
|     |     | 1.6 V or above | -   | 2 × t  + 180 |     |
| --- | --- | -------------- | --- | ------------ | --- |
Pcyc
| Note 1. t Pcyc : PCLKA cycle. |     |     |     |     |     |
| ----------------------------- | --- | --- | --- | --- | --- |
Note 2. N is set as an integer from 1 to 8 by the SPCKD register.
Note 3. N is set as an integer from 1 to 8 by the SSLND register.
Note 4. The upper limit of RSPCK is 16 MHz.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 76 of 131 |
| --------------- | -------- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |        | 2. Electrical Characteristics |     |     |
| ----------- | --- | --- | --- | --- | --- | ------ | ----------------------------- | --- | --- |
|             |     |     |     |     |     | tSPCKr | tSPCKf                        |     |     |
tSPCKWH
|     |     |     | VOH |     | VOH | VOH | VOH |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
RSPCKn
m a s t e r select
|     |     |     |     |     | VOL | VOL |     | VOL |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
ou t p u t
tSPCKWL
tSPcyc
|     |     |     |     | tSPCKWH |     | tSPCKr | tSPCKf |     |     |
| --- | --- | --- | --- | ------- | --- | ------ | ------ | --- | --- |
|     |     |     | VIH |         | VIH | VIH    | VIH    |     |     |
RSPCKn
|     | slave select input |     |     |     | VIL | VIL |     | VIL |     |
| --- | ------------------ | --- | --- | --- | --- | --- | --- | --- | --- |
tSPCKWL
tSPcyc
VOH = 0.7 × VCC, VOL = 0.3 × VCC, VIH = 0.7 × VCC, VIL = 0.3 × VCC
n = A or B
| Figure 2.52 | SPI clock timing |     |     |     |     |     |     |     |     |
| ----------- | ---------------- | --- | --- | --- | --- | --- | --- | --- | --- |
tTD
SSLn0 to
SSLn3
output
|     |     | tLEAD |     |     |     | tLAG |     |     |     |
| --- | --- | ----- | --- | --- | --- | ---- | --- | --- | --- |
tSSLr, tSSLf
RSPCKn
CPOL = 0
output
RSPCKn
CPOL = 1
output
|     |     | tSU | tH  |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
MISOn
|     |     | MSB IN |     | DATA | LSB IN |     |     |     | MSB IN |
| --- | --- | ------ | --- | ---- | ------ | --- | --- | --- | ------ |
input
|     |     | tDr, tDf |     | tOH | tOD |     |     |     |     |
| --- | --- | -------- | --- | --- | --- | --- | --- | --- | --- |
MOSIn
|     |     | MSB OUT |     | DATA | LSB OUT |     | IDLE |     | MSB OUT |
| --- | --- | ------- | --- | ---- | ------- | --- | ---- | --- | ------- |
output
n = A or B
Figure 2.53 SPI timing for master when CPHA = 0 and the bit rate is set to any value other than PCLKA/2
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     |     |     | Page 77 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- |
t
TD
SSLn0 to
SSLn3
|     |        |     | t    |     |     |     |     |     | t   |     |     |
| --- | ------ | --- | ---- | --- | --- | --- | --- | --- | --- | --- | --- |
|     | output |     | LEAD |     |     |     |     |     | LAG |     |     |
t t
SSLr, SSLf
RSPCKn
CPOL = 0
output
RSPCKn
CPOL = 1
output
|     |       |     | t      | t   |      |     |     | tHF    |     |     |        |
| --- | ----- | --- | ------ | --- | ---- | --- | --- | ------ | --- | --- | ------ |
|     |       |     | SU     | HF  |      |     |     |        |     |     |        |
|     | MISOn |     |        |     |      |     |     | LSB IN |     |     |        |
|     |       |     | MSB IN |     | DATA |     |     |        |     |     | MSB IN |
input
tDr, tDf
|     |     |     |     | t OH |     |     | t OD |     |     |     |     |
| --- | --- | --- | --- | ---- | --- | --- | ---- | --- | --- | --- | --- |
MOSIn
|     |     |     | MSB OUT |     | DATA |     | LSB OUT |     |     | IDLE | MSB OUT |
| --- | --- | --- | ------- | --- | ---- | --- | ------- | --- | --- | ---- | ------- |
output
n = A or B
Figure 2.54 SPI timing for master when CPHA = 0 and the bit rate is set to PCLKA/2
tTD
SSLn0 to
SSLn3
output
|     |     | tLEAD |     |     |     |     |     |     | tLAG |     |     |
| --- | --- | ----- | --- | --- | --- | --- | --- | --- | ---- | --- | --- |
|     |     |       |     |     |     |     |     |     |      | t t |     |
SSLr, SSLf
RSPCKn
CPOL = 0
output
RSPCKn
CPOL = 1
output
|     |     |     |     | t t  |     |     |     |     |     |     |     |
| --- | --- | --- | --- | ---- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     |     | SU H |     |     |     |     |     |     |     |
MISOn
|     | input |     |     | MSB IN |     | DATA |     | LSB IN |        |     | MSB IN |
| --- | ----- | --- | --- | ------ | --- | ---- | --- | ------ | ------ | --- | ------ |
|     |       |     | t   |        | t   |      |     |        | t t    |     |        |
|     |       |     | OH  |        | OD  |      |     |        | Dr, Df |     |        |
MOSIn
|     |     |     |     | MSB OUT |     | DATA |     | LSB OUT |     | IDLE | MSB OUT |
| --- | --- | --- | --- | ------- | --- | ---- | --- | ------- | --- | ---- | ------- |
output
n = A or B
Figure 2.55 SPI timing for master when CPHA = 1 and the bit rate is set to any value other than PCLKA/2
| R01DS0355EJ0110 |     | Rev.1.10 |     |     |     |     |     |     |     |     | Page 78 of 131 |
| --------------- | --- | -------- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- |
t TD
SSLn0 to
SSLn3
output
|     |     |     | t    |     |     |     |     |     | t   |     |     |
| --- | --- | --- | ---- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | LEAD |     |     |     |     |     | LAG |     |     |
|     |     |     |      |     |     |     |     |     |     | t t |     |
SSLr, SSLf
RSPCKn
CPOL = 0
output
RSPCKn
CPOL = 1
|     | output |     |     |     | n = A or B |     |     |     |     |     |     |
| --- | ------ | --- | --- | --- | ---------- | --- | --- | --- | --- | --- | --- |
|     |        |     |     | tSU | tHF        |     |     |     | tH  |     |     |
MISOn
|     | input |     |     | MSB IN |     | DATA |     | LSB IN |        |     | MSB IN |
| --- | ----- | --- | --- | ------ | --- | ---- | --- | ------ | ------ | --- | ------ |
|     |       |     | t   |        | t   |      |     |        | t t    |     |        |
|     |       |     | OH  |        | OD  |      |     |        | Dr, Df |     |        |
MOSIn
|     | output |     |     | MSB OUT |     | DATA |     | LSB OUT |     | IDLE | MSB OUT |
| --- | ------ | --- | --- | ------- | --- | ---- | --- | ------- | --- | ---- | ------- |

n = A or B
Figure 2.56 SPI timing for master when CPHA = 1 and the bit rate is set to PCLKA/2
tTD
SSLn0
input
|     |     | t LEAD |     |     |     |     |     |     | t LAG |     |     |
| --- | --- | ------ | --- | --- | --- | --- | --- | --- | ----- | --- | --- |
RSPCKn
CPOL = 0
input
RSPCKn
CPOL = 1
input
|     |     | tSA |     | tOH |     | tOD |     |     |     | tREL |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | --- |
MISOn
|     | output |     |     | MSB OUT | DATA |     | LSB OUT |          | MSB IN |     | MSB OUT |
| --- | ------ | --- | --- | ------- | ---- | --- | ------- | -------- | ------ | --- | ------- |
|     |        |     | tSU | tH      |      |     |         | tDr, tDf |        |     |         |
MOSIn
|     | input |     | MSB IN |     | DATA |     | LSB IN |     |     |     | MSB IN |
| --- | ----- | --- | ------ | --- | ---- | --- | ------ | --- | --- | --- | ------ |
n = A or B
| Figure 2.57     |     | SPI timing for slave when CPHA = 0 |     |     |     |     |     |     |     |     |                |
| --------------- | --- | ---------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 |     | Rev.1.10                           |     |     |     |     |     |     |     |     | Page 79 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     |     |     | 2. Electrical Characteristics |     |     |
| ----------- | --- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- | --- |
t
TD
SSLn0
input
|     |     | t    |     |     |     |     |     | t   |     |     |
| --- | --- | ---- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     | LEAD |     |     |     |     |     | LAG |     |     |
RSPCKn
CPOL = 0
input
RSPCKn
CPOL = 1
input
|        | t SA |             | t OH |         | t OD |      |          | t REL   |     |         |
| ------ | ---- | ----------- | ---- | ------- | ---- | ---- | -------- | ------- | --- | ------- |
| MISOn  |      | LSB OUT     |      |         |      |      |          |         |     |         |
| output |      | (Last data) |      | MSB OUT |      | DATA |          | LSB OUT |     | MSB OUT |
|        |      |             |      | tSU tH  |      |      | tDr, tDf |         |     |         |
MOSIn
|     |     |     |     | MSB IN |     | DATA |     | LSB IN |     | MSB IN |
| --- | --- | --- | --- | ------ | --- | ---- | --- | ------ | --- | ------ |
input
n = A or B
| Figure 2.58                    | SPI timing for slave when CPHA = 1 |     |     |     |     |     |     |     |     |     |
| ------------------------------ | ---------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2.3.10 IIC Timing              |                                    |     |     |     |     |     |     |     |     |     |
| Table 2.37 IIC timing (1 of 2) |                                    |     |     |     |     |     |     |     |     |     |
Conditions: VCC = 2.7 to 5.5 V
Test
| Parameter |     |     |     |     |     | Symbol | Min*1 | Max | Unit | conditions |
| --------- | --- | --- | --- | --- | --- | ------ | ----- | --- | ---- | ---------- |
IIC SCL input cycle time t SCL 6 (12) × t IICcyc  +  - ns Figure 2.59
| (standard mode,  |     |     |     |     |     |     | 1300 |     |     |     |
| ---------------- | --- | --- | --- | --- | --- | --- | ---- | --- | --- | --- |
SMBus)
|     | SCL input high pulse width |     |     |     |     | t SCLH | 3 (6) × t IICcyc |  +  - | ns  |     |
| --- | -------------------------- | --- | --- | --- | --- | ------ | ---------------- | ----- | --- | --- |
300
|     | SCL input low pulse width |     |     |     |     | t SCLL | 3 (6) × t IICcyc |  +  - | ns  |     |
| --- | ------------------------- | --- | --- | --- | --- | ------ | ---------------- | ----- | --- | --- |
300
|     | SCL, SDA input rise time |     |     |     |     | t Sr | -   | 1,000 | ns  |     |
| --- | ------------------------ | --- | --- | --- | --- | ---- | --- | ----- | --- | --- |
|     | SCL, SDA input fall time |     |     |     |     | t    | -   | 300   | ns  |     |
Sf
|     | SCL, SDA input spike pulse removal  |     |     |     |     | t   | 0   | 1 (4) × t | ns  |     |
| --- | ----------------------------------- | --- | --- | --- | --- | --- | --- | --------- | --- | --- |
|     |                                     |     |     |     |     | SP  |     | IICcyc    |     |     |
time
|     | SDA input bus free time            |     |     |     |     | t   | 3 (6) × t   |  +  - | ns  |     |
| --- | ---------------------------------- | --- | --- | --- | --- | --- | ----------- | ----- | --- | --- |
|     |                                    |     |     |     |     | BUF | IICcyc      |       |     |     |
|     | (When wakeup function is disabled) |     |     |     |     |     | 300         |       |     |     |
|     | SDA input bus free time            |     |     |     |     | t   | 3 (6) × t   |  +  - | ns  |     |
|     |                                    |     |     |     |     | BUF | IICcyc      |       |     |     |
|     | (When wakeup function is enabled)  |     |     |     |     |     | 4 × t + 300 |       |     |     |
Pcyc
|     | START condition input hold time |     |     |     |     | t    | t  + 300 | -   | ns  |     |
| --- | ------------------------------- | --- | --- | --- | --- | ---- | -------- | --- | --- | --- |
|     |                                 |     |     |     |     | STAH | IICcyc   |     |     |     |
(When wakeup function is disabled)
|     | START condition input hold time   |     |     |     |     | t    | 1 (5) × t |  +  - | ns  |     |
| --- | --------------------------------- | --- | --- | --- | --- | ---- | --------- | ----- | --- | --- |
|     |                                   |     |     |     |     | STAH | IICcyc    |       |     |     |
|     | (When wakeup function is enabled) |     |     |     |     |      | t  + 300  |       |     |     |
Pcyc
|     | Repeated START condition input setup  |     |     |     |     | t   | 1,000 | -   | ns  |     |
| --- | ------------------------------------- | --- | --- | --- | --- | --- | ----- | --- | --- | --- |
STAS
time
|     | STOP condition input setup time |     |     |     |     | t   | 1,000 | -   | ns  |     |
| --- | ------------------------------- | --- | --- | --- | --- | --- | ----- | --- | --- | --- |
STOS
|                 | Data input setup time    |     |     |     |     | t SDAS | t IICcyc  + 50 | -   | ns  |                |
| --------------- | ------------------------ | --- | --- | --- | --- | ------ | -------------- | --- | --- | -------------- |
|                 | Data input hold time     |     |     |     |     | t SDAH | 0              | -   | ns  |                |
|                 | SCL, SDA capacitive load |     |     |     |     | C b    | -              | 400 | pF  |                |
| R01DS0355EJ0110 | Rev.1.10                 |     |     |     |     |        |                |     |     | Page 80 of 131 |
Sep 29, 2023

| RA4M1 Group |                     |     |     |     |     |     |     | 2. Electrical Characteristics |     |     |
| ----------- | ------------------- | --- | --- | --- | --- | --- | --- | ----------------------------- | --- | --- |
| Table 2.37  | IIC timing (2 of 2) |     |     |     |     |     |     |                               |     |     |
Conditions: VCC = 2.7 to 5.5 V
Test
| Parameter |     |     |     |     | Symbol | Min*1 | Max |     | Unit | conditions |
| --------- | --- | --- | --- | --- | ------ | ----- | --- | --- | ---- | ---------- |
IIC SCL input cycle time t SCL 6 (12) × t IICcyc  +  - ns Figure 2.59
| (Fast mode) |     |     |     |     |     | 600 |     |     |     | For all ports  |
| ----------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
except P408,
|     |     | SCL input high pulse width |     |     | t SCLH | 3 (6) × t | IICcyc  +  - |     | ns  |     |
| --- | --- | -------------------------- | --- | --- | ------ | --------- | ------------ | --- | --- | --- |
use
300
PmnPFS.DSC
SCL input low pulse width t SCLL 3 (6) × t IICcyc  +  - ns R of middle
|     |     |                          |     |     |     | 300 |     |     |     | drive.          |
| --- | --- | ------------------------ | --- | --- | --- | --- | --- | --- | --- | --------------- |
|     |     | SCL, SDA input rise time |     |     | t   | -   | 300 |     | ns  | For port P408,  |
Sr
use
|     |     | SCL, SDA input fall time |     |     | t   | -   | 300 |     | ns  |            |
| --- | --- | ------------------------ | --- | --- | --- | --- | --- | --- | --- | ---------- |
|     |     |                          |     |     | Sf  |     |     |     |     | PmnPFS.DSC |
SCL, SDA input spike pulse removal  t 0 1 (4) × t ns R1/DSCR of
|     |     |      |     |     | SP  |     |     | IICcyc |     |               |
| --- | --- | ---- | --- | --- | --- | --- | --- | ------ | --- | ------------- |
|     |     | time |     |     |     |     |     |        |     | middle drive  |
for IIC fast-
|     |     | SDA input bus free time            |     |     | t   | 3 (6) × t |  +  -  |     | ns  |       |
| --- | --- | ---------------------------------- | --- | --- | --- | --------- | ------ | --- | --- | ----- |
|     |     |                                    |     |     | BUF |           | IICcyc |     |     | mode. |
|     |     | (When wakeup function is disabled) |     |     |     | 300       |        |     |     |       |
|     |     | SDA input bus free time            |     |     | t   | 3 (6) × t |  +  -  |     | ns  |       |
|     |     |                                    |     |     | BUF |           | IICcyc |     |     |       |
|     |     | (When wakeup function is enabled)  |     |     |     | 4 × t     | + 300  |     |     |       |
Pcyc
|     |     | START condition input hold time |     |     | t    | t  + 300 | -   |     | ns  |     |
| --- | --- | ------------------------------- | --- | --- | ---- | -------- | --- | --- | --- | --- |
|     |     |                                 |     |     | STAH | IICcyc   |     |     |     |     |
(When wakeup function is disabled)
|     |     | START condition input hold time   |     |     | t    | 1(5) × t |  +  -  |     | ns  |     |
| --- | --- | --------------------------------- | --- | --- | ---- | -------- | ------ | --- | --- | --- |
|     |     |                                   |     |     | STAH |          | IICcyc |     |     |     |
|     |     | (When wakeup function is enabled) |     |     |      | t  + 300 |        |     |     |     |
Pcyc
|     |     | Repeated START condition input setup  |     |     | t   | 300 | -   |     | ns  |     |
| --- | --- | ------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
STAS
time
|     |     | STOP condition input setup time |     |     | t   | 300 | -   |     | ns  |     |
| --- | --- | ------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- |
STOS
|       |                                                  | Data input setup time    |     |      | t SDAS        | t IICcyc  + 50 | -   |     | ns  |     |
| ----- | ------------------------------------------------ | ------------------------ | --- | ---- | ------------- | -------------- | --- | --- | --- | --- |
|       |                                                  | Data input hold time     |     |      | t SDAH        | 0              | -   |     | ns  |     |
|       |                                                  | SCL, SDA capacitive load |     |      | C b           | -              | 400 |     | pF  |     |
| Note: | t : IIC internal reference clock (IICφ) cycle, t |                          |     |      | : PCLKB cycle |                |     |     |     |     |
|       | IICcyc                                           |                          |     | Pcyc |               |                |     |     |     |     |
Note 1. The value in parentheses apply when ICMR3.NF[1:0] is set to 11b while the digital filter is enabled with ICFER.NFE set to 1.
VIH
SDA0 to SDA1
VIL
tBUF
tSCLH
|     |     |     |       |     |     |     | tSTAS |     |     | tSTOS |
| --- | --- | --- | ----- | --- | --- | --- | ----- | --- | --- | ----- |
|     |     |     | tSTAH |     |     |     |       |     | tSP |       |
SCL0 to SCL1
|     |     | P*1 | S*1 |     |     | Sr*1 |     |     |     | P*1 |
| --- | --- | --- | --- | --- | --- | ---- | --- | --- | --- | --- |
tSCLL
|     |     |     | tSf | tSr |     |     |     | tSDAS |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | ----- | --- | --- |
tSCL
t
SDAH
| Note 1. | S, P, and Sr indicate the following conditions. |     |     |     |     |     |     |     |     |     |
| ------- | ----------------------------------------------- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
S: Start condition
P: Stop condition
Sr: Restart condition.
| Figure 2.59     |     | IIC bus interface input/output timing |     |     |     |     |     |     |     |                |
| --------------- | --- | ------------------------------------- | --- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 |     | Rev.1.10                              |     |     |     |     |     |     |     | Page 81 of 131 |
Sep 29, 2023

| RA4M1 Group |             |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ----------- | --- | --- | --- | ----------------------------- | --- |
| 2.3.11      | SSIE Timing |     |     |     |                               |     |
| Table 2.38  | SSIE timing |     |     |     |                               |     |
Conditions: VCC = 1.6 to 5.5 V
| Parameter |                  |                | Symbol Min | Max | Unit Test conditions |     |
| --------- | ---------------- | -------------- | ---------- | --- | -------------------- | --- |
| SSIE      | AUDIO_CLK input  | 2.7 V or above | t -        | 25  | MHz -                |     |
AUDIO
frequency
|     |                     | 1.6 V or above | -     | 4   |                |     |
| --- | ------------------- | -------------- | ----- | --- | -------------- | --- |
|     | Output clock period |                | t 250 | -   | ns Figure 2.60 |     |
O
|     | Input clock period |                | t I 250  | -   | ns  |     |
| --- | ------------------ | -------------- | -------- | --- | --- | --- |
|     | Clock high pulse   | 1.8 V or above | t HC 100 | -   | ns  |     |
|     | width              | 1.6 V or above | 200      | -   |     |     |
|     | Clock low pulse    | 1.8 V or above | t 100    | -   | ns  |     |
LC
width
|     |                 | 1.6 V or above | 200     | -   |                 |     |
| --- | --------------- | -------------- | ------- | --- | --------------- | --- |
|     | Clock rise time |                | t RC -  | 25  | ns              |     |
|     | Data delay      | 2.7 V or above | t DTR - | 65  | ns Figure 2.61, |     |
|     |                 | 1.8 V or above | -       | 105 | Figure 2.62     |     |
|     |                 | 1.6 V or above | -       | 140 |                 |     |
|     | Set-up time     | 2.7 V or above | t 65    | -   | ns              |     |
SR
|     |                 | 1.8 V or above | 90       | -   |                |     |
| --- | --------------- | -------------- | -------- | --- | -------------- | --- |
|     |                 | 1.6 V or above | 140      | -   |                |     |
|     | Hold time       |                | t HTR 40 | -   | ns             |     |
|     | SSITXD0 output  | 1.8 V or above | T -      | 105 | ns Figure 2.63 |     |
DTRW
delay from
|     |     | 1.6 V or above | -   | 140 |     |     |
| --- | --- | -------------- | --- | --- | --- | --- |
SSILRCK0/SSIFS0
change time
t HC t RC
t
LC
SSIBCK0
t, t
I O
| Figure 2.60     | SSIE clock input/output timing |     |     |     |     |                |
| --------------- | ------------------------------ | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10                       |     |     |     |     | Page 82 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
SSIBCK0
(Input or Output)
SSILRCK0/SSIFS0,
SSIRXD0
(Input)
t t
SR HTR
SSILRCK0/SSIFS0,
SSITXD0
(Output)
t
DTR
Figure 2.61 SSIE data transmit/receive timing (SSICR.BCKP = 0)
SSIBCK0
(Input or Output)
SSILRCK0/SSIFS0,
SSIRXD0
(Input)
t t
SR HTR
SSILRCK0/SSIFS0,
SSITXD0
(Output)
t
DTR
Figure 2.62 SSIE data transmit/receive timing (SSICR.BCKP = 1)
R01DS0355EJ0110 Rev.1.10 Page 83 of 131
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
SSILRCK0/SSIFS0
(Input)
SSITXD0 (Output)
t DTRW
MSB bit output delay from SSILRCK0/SSIFS0 change time for
slave transmitter when DEL = 1, SDTA = 0 or DEL = 1, SDTA = 1, SWL[2:0] = DWL[2:0]
| Figure 2.63 | SSIE data output delay from SSILRCK0/SSIFS0 change time |     |     |        |                |                 |
| ----------- | ------------------------------------------------------- | --- | --- | ------ | -------------- | --------------- |
| 2.3.12      | CLKOUT Timing                                           |     |     |        |                |                 |
| Table 2.39  | CLKOUT timing                                           |     |     |        |                |                 |
| Parameter   |                                                         |     |     | Symbol | Min Max Unit*1 | Test conditions |
CLKOUT pin output cycle*1
| CLKOUT  |     |  VCC = 2.7 V or above |     | t Ccyc | 62.5 - ns | Figure 2.64 |
| ------- | --- | --------------------- | --- | ------ | --------- | ----------- |
|         |     |  VCC = 1.8 V or above |     |        | 125 -     |             |
|         |     |  VCC = 1.6 V or above |     |        | 250 -     |             |
CLKOUT pin high pulse width*2  VCC = 2.7 V or above t 15 - ns
CH
|     |                              |  VCC = 1.8 V or above |     |     | 30 -    |     |
| --- | ---------------------------- | --------------------- | --- | --- | ------- | --- |
|     |                              |  VCC = 1.6 V or above |     |     | 150 -   |     |
|     | CLKOUT pin low pulse width*2 |  VCC = 2.7 V or above |     | t   | 15 - ns |     |
CL
|     |                             |  VCC = 1.8 V or above |     |     | 30 -    |     |
| --- | --------------------------- | --------------------- | --- | --- | ------- | --- |
|     |                             |  VCC = 1.6 V or above |     |     | 150 -   |     |
|     | CLKOUT pin output rise time |  VCC = 2.7 V or above |     | t   | - 12 ns |     |
Cr
|     |                             |  VCC = 1.8 V or above |     |     | - 25    |     |
| --- | --------------------------- | --------------------- | --- | --- | ------- | --- |
|     |                             |  VCC = 1.6 V or above |     |     | - 50    |     |
|     | CLKOUT pin output fall time |  VCC = 2.7 V or above |     | t   | - 12 ns |     |
Cf
|     |     |  VCC = 1.8 V or above |     |     | - 25 |     |
| --- | --- | --------------------- | --- | --- | ---- | --- |
|     |     |  VCC = 1.6 V or above |     |     | - 50 |     |
Note 1. When the EXTAL external clock input or an oscillator is used with division by 1 (the CKOCR.CKOSEL[2:0] bits are 011b and
the CKOCR.CKODIV[2:0] bits are 000b) to output from CLKOUT, the above should be satisfied with an input duty cycle of 45 to
55%.
Note 2. When the MOCO is selected as the clock output source (the CKOCR.CKOSEL[2:0] bits are 001b), set the clock output division
ratio selection to be divided by 2 (the CKOCR.CKODIV[2:0] bits are 001b).
t
Ccyc
t CH
t
Cf
CLKOUT pin output
t
|     |     |     |     | t   | Cr  |     |
| --- | --- | --- | --- | --- | --- | --- |
CL
|                 | Test conditions: V   | OH  = VCC × 0.7, V | OL  = VCC × 0.3, I | OH  = -1.0 mA,  I | OL  = 1.0 mA, C = 30 pF |                |
| --------------- | -------------------- | ------------------ | ------------------ | ----------------- | ----------------------- | -------------- |
| Figure 2.64     | CLKOUT output timing |                    |                    |                   |                         |                |
| R01DS0355EJ0110 | Rev.1.10             |                    |                    |                   |                         | Page 84 of 131 |
Sep 29, 2023

| RA4M1 Group             |                     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------------------- | ------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| 2.4 USB Characteristics |                     |     |     |     |     |                               |     |
| 2.4.1                   | USBFS Timing        |     |     |     |     |                               |     |
| Table 2.40              | USB characteristics |     |     |     |     |                               |     |
Conditions: VCC = VCC_USB = 3.0 to 3.6 V, T = -20 to +85°C (USBCLKSEL = 1), T = -40 to +105°C (USBCLKSEL = 0)
|           |                          |     | a      |     | a   |                      |     |
| --------- | ------------------------ | --- | ------ | --- | --- | -------------------- | --- |
| Parameter |                          |     | Symbol | Min | Max | Unit Test conditions |     |
| Input     | Input high level voltage |     | V      | 2.0 | -   | V -                  |     |
IH
characteristics
|     | Input low level voltage |     | V   | -   | 0.8 | V - |     |
| --- | ----------------------- | --- | --- | --- | --- | --- | --- |
IL
|     | Differential input sensitivity |     | V   | 0.2 | -   | V |USB_DP - USB_DM | |     |
| --- | ------------------------------ | --- | --- | --- | --- | -------------------- | --- |
DI
|     | Differential common mode  |     | V   | 0.8 | 2.5 | V - |     |
| --- | ------------------------- | --- | --- | --- | --- | --- | --- |
CM
range
Output  Output high level voltage V 2.8 VCC_USB V I  = -200 μA
|     |     |     | OH  |     |     | OH  |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
characteristics
|     | Output low level voltage |     | V   | 0.0 | 0.3 | V I = 2 mA     |     |
| --- | ------------------------ | --- | --- | --- | --- | -------------- | --- |
|     |                          |     | OL  |     |     | OL             |     |
|     | Cross-over voltage       |     | V   | 1.3 | 2.0 | V Figure 2.65, |     |
CRS
Figure 2.66,
|     | Rise time | FS  | t r | 4   | 20  | ns  |     |
| --- | --------- | --- | --- | --- | --- | --- | --- |
Figure 2.67
|     |                      | LS  |     | 75  | 300    |     |     |
| --- | -------------------- | --- | --- | --- | ------ | --- | --- |
|     | Fall time            | FS  | t f | 4   | 20     | ns  |     |
|     |                      | LS  |     | 75  | 300    |     |     |
|     | Rise/fall time ratio | FS  | t/t | 90  | 111.11 | %   |     |
r f
|     |                   | LS  |     | 80  | 125 |                              |     |
| --- | ----------------- | --- | --- | --- | --- | ---------------------------- | --- |
|     | Output resistance |     | Z   | 28  | 44  | Ω (Adjusting the resistance  |     |
DRV
of external elements is not
necessary.)
| VBUS  | VBUS input voltage |     | V   | VCC × 0.8 | -   | V - |     |
| ----- | ------------------ | --- | --- | --------- | --- | --- | --- |
IH
characteristics
|     |     |     | V   | -   | VCC × 0.2 | V - |     |
| --- | --- | --- | --- | --- | --------- | --- | --- |
IL
| Pull-up,  | Pull-down resistor |     | R   | 14.25 | 24.80 | kΩ - |     |
| --------- | ------------------ | --- | --- | ----- | ----- | ---- | --- |
PD
pull-down
|     | Pull-up resistor |     | R   | 0.9 | 1.575 | kΩ During idle state |     |
| --- | ---------------- | --- | --- | --- | ----- | -------------------- | --- |
PUI
|     |     |     | R   | 1.425 | 3.09 | kΩ During reception |     |
| --- | --- | --- | --- | ----- | ---- | ------------------- | --- |
PUA
| Battery Charging  | D + sink current |     | I DP_SINK | 25  | 175 | μA - |     |
| ----------------- | ---------------- | --- | --------- | --- | --- | ---- | --- |
Specification
|     | D - sink current |     | I DM_SINK | 25  | 175 | μA - |     |
| --- | ---------------- | --- | --------- | --- | --- | ---- | --- |
Ver 1.2
|     | DCD source current     |     | I DP_SRC  | 7    | 13  | μA - |     |
| --- | ---------------------- | --- | --------- | ---- | --- | ---- | --- |
|     | Data detection voltage |     | V DAT_REF | 0.25 | 0.4 | V -  |     |
D + source voltage V DP_SRC 0.5 0.7 V Output current = 250 μA
|     | D - source voltage |     | V   | 0.5 | 0.7 | V Output current = 250 μA |     |
| --- | ------------------ | --- | --- | --- | --- | ------------------------- | --- |
DM_SRC
|     |     | USB_DP, | V   | 90% | 90% |     |     |
| --- | --- | ------- | --- | --- | --- | --- | --- |
CRS
|                 |                                 | USB_DM | 10% |     | 10% |     |                |
| --------------- | ------------------------------- | ------ | --- | --- | --- | --- | -------------- |
|                 |                                 |        |     | t   | t   |     |                |
|                 |                                 |        |     | r   | f   |     |                |
| Figure 2.65     | USB_DP and USB_DM output timing |        |     |     |     |     |                |
| R01DS0355EJ0110 | Rev.1.10                        |        |     |     |     |     | Page 85 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | ----------------------------- | --- |
Observation
|     |     | DP  | point |     |
| --- | --- | --- | ----- | --- |
50 pF
DM
50 pF
| Figure 2.66 | Test circuit for Full-Speed (FS) connection |     |     |     |
| ----------- | ------------------------------------------- | --- | --- | --- |
Observation
point
DP
200 pF to
|     |     |     | 600 pF 3.6 V |     |
| --- | --- | --- | ------------ | --- |
1.5 K
DM
200 pF to
600 pF
Observation
point
| Figure 2.67               | Test circuit for Low-Speed (LS) connection |         |          |                 |
| ------------------------- | ------------------------------------------ | ------- | -------- | --------------- |
| 2.4.2 USB External Supply |                                            |         |          |                 |
| Table 2.41 USB regulator  |                                            |         |          |                 |
| Parameter                 |                                            | Min Typ | Max Unit | Test conditions |
| VCC_USB supply current    | VCC_USB_LDO ≥ 3.8V                         | - -     | 50 mA    | -               |
|                           | VCC_USB_LDO ≥ 4.5V                         | - -     | 100 mA   | -               |
| VCC_USB supply voltage    |                                            |  3.0 -  |  3.6 V   | -               |
| R01DS0355EJ0110           | Rev.1.10                                   |         |          | Page 86 of 131  |
Sep 29, 2023

| RA4M1 Group |     |                       |     |     |     |     |     |        |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --------------------- | --- | --- | --- | --- | --- | ------ | --- | --- | ----------------------------- | --- |
| 2.5         |     | ADC14 Characteristics |     |     |     |     |     |        |     |     |                               |     |
| VREFH0      |     |                       |     |     |     |     |     | VREFH0 |     |     |                               |     |
|             | 5.5 |                       |     |     |     |     |     | 5.5    |     |     |                               |     |
A/D Conversion
|     | 5.0 |     |     |     |     |     |     | 5.0 |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Characteristics (1)
|     |     |     |     |     |     | A/D Conversion      |     |     |     |     |     | A/D Conversion      |
| --- | --- | --- | --- | --- | --- | ------------------- | --- | --- | --- | --- | --- | ------------------- |
|     |     |     |     |     |     | Characteristics (2) |     |     |     |     |     | Characteristics (4) |
|     | 4.0 |     |     |     |     |                     |     | 4.0 |     |     |     |                     |
|     | 3.0 |     |     |     |     |                     |     | 3.0 |     |     |     |                     |
|     |     |     |     |     |     | A/D Conversion      |     |     |     |     |     | A/D Conversion      |
|     | 2.7 |     |     |     |     |                     |     | 2.7 |     |     |     | Characteristics (5) |
|     | 2.4 |     |     |     |     | Characteristics (3) |     | 2.4 |     |     |     |                     |
A/D Conversion
|     | 2.0 |     |     |     |     |     |     | 2.0 |     |     |     | Characteristics (6) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ------------------- |
1.8
|     |     |     |     |     |     |     |     | 1.6 |     |     |     | A/D Conversion  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --------------- |
Characteristics (7)
|             | 1.0 |     |                               |     |     |       |     | 1.0 |            |                 |         |       |
| ----------- | --- | --- | ----------------------------- | --- | --- | ----- | --- | --- | ---------- | --------------- | ------- | ----- |
|             |     |     | 2.42.7                        |     | 5.5 | AVCC0 |     |     | 1.8        | 2.42.7          | 5.5     | AVCC0 |
|             |     | 1.0 | 2.0 3.0                       | 4.0 | 5.0 |       |     |     | 1.0 1.62.0 | 3.0             | 4.0 5.0 |       |
|             |     |     | ADCSR.ADHSC = 0               |     |     |       |     |     |            | ADCSR.ADHSC = 1 |         |       |
| Figure 2.68 |     |     | AVCC0 to VREFH0 voltage range |     |     |       |     |     |            |                 |         |       |
Table 2.42 A/D conversion characteristics (1) in high-speed A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 4.5 to 5.5 V, VREFH0 = 4.5 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
|     | Parameter |     |     |     |     | Min | Typ | Max |     | Unit | Test conditions |     |
| --- | --------- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | --------------- | --- |
|     | Frequency |     |     |     |     | 1   | -   | 64  |     | MHz  | -               |     |
Analog input capacitance*2 Cs - - 8 (reference data) pF High-precision channel
|     |     |     |     |     |     | -   | -   | 9 (reference data) |     | pF  | Normal-precision channel |     |
| --- | --- | --- | --- | --- | --- | --- | --- | ------------------ | --- | --- | ------------------------ | --- |
Analog input resistance Rs - - 2.5 (reference  kΩ High-precision channel
data)
|     |     |     |     |     |     | -   | -   | 6.7 (reference  |     | kΩ  | Normal-precision channel |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --------------- | --- | --- | ------------------------ | --- |
data)
|     | Analog input voltage range |     |     |     | Ain | 0   | -   | VREFH0 |     | V   | -   |     |
| --- | -------------------------- | --- | --- | --- | --- | --- | --- | ------ | --- | --- | --- | --- |
12-bit mode
|     | Resolution |     |     |     |     | -   | -   | 12  |     | Bit | -   |     |
| --- | ---------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1
|     |                 |     | Permissible signal  |     |     | 0.70 | -   | -   |     | μs  | High-precision channel   |     |
| --- | --------------- | --- | ------------------- | --- | --- | ---- | --- | --- | --- | --- | ------------------------ | --- |
|     | (Operation at   |     | source impedance    |     |     |      |     |     |     |     | ADCSR.ADHSC = 0          |     |
|     | PCLKC = 64 MHz) |     | Max. = 0.3 kΩ       |     |     |      |     |     |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|     |                 |     |                     |     |     | 1.13 | -   | -   |     | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 0
ADSSTRn.SST[7:0] = 28h
|     | Offset error                        |     |     |     |     | -   | ±0.5  | ±4.5 |     | LSB | High-precision channel |     |
| --- | ----------------------------------- | --- | --- | --- | --- | --- | ----- | ---- | --- | --- | ---------------------- | --- |
|     |                                     |     |     |     |     |     |       | ±6.0 |     | LSB | Other than above       |     |
|     | Full-scale error                    |     |     |     |     | -   | ±0.75 | ±4.5 |     | LSB | High-precision channel |     |
|     |                                     |     |     |     |     |     |       | ±6.0 |     | LSB | Other than above       |     |
|     | Quantization error                  |     |     |     |     | -   | ±0.5  | -    |     | LSB | -                      |     |
|     | Absolute accuracy                   |     |     |     |     | -   | ±1.25 | ±5.0 |     | LSB | High-precision channel |     |
|     |                                     |     |     |     |     |     |       | ±8.0 |     | LSB | Other than above       |     |
|     | DNL differential nonlinearity error |     |     |     |     | -   | ±1.0  | -    |     | LSB | -                      |     |
|     | INL integral nonlinearity error     |     |     |     |     | -   | ±1.0  | ±3.0 |     | LSB | -                      |     |
14-bit mode
|                 | Resolution |     |          |     |     | -   | -   | 14  |     | Bit | -   |                |
| --------------- | ---------- | --- | -------- | --- | --- | --- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 |            |     | Rev.1.10 |     |     |     |     |     |     |     |     | Page 87 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
Table 2.42 A/D conversion characteristics (1) in high-speed A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 4.5 to 5.5 V, VREFH0 = 4.5 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     |     | Min Typ | Max | Unit | Test conditions |     |
| --------- | --- | --- | ------- | --- | ---- | --------------- | --- |
Conversion time*1 Permissible signal  0.80 - - μs High-precision channel
| (Operation at   | source impedance  |     |        |     |     | ADCSR.ADHSC = 0          |     |
| --------------- | ----------------- | --- | ------ | --- | --- | ------------------------ | --- |
| PCLKC = 64 MHz) | Max. = 0.3 kΩ     |     |        |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|                 |                   |     | 1.22 - | -   | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 0
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     |     | - ±2.0 | ±18   | LSB | High-precision channel |     |
| ----------------------------------- | --- | --- | ------ | ----- | --- | ---------------------- | --- |
|                                     |     |     |        | ±24.0 | LSB | Other than above       |     |
| Full-scale error                    |     |     | - ±3.0 | ±18   | LSB | High-precision channel |     |
|                                     |     |     |        | ±24.0 | LSB | Other than above       |     |
| Quantization error                  |     |     | - ±0.5 | -     | LSB | -                      |     |
| Absolute accuracy                   |     |     | - ±5.0 | ±20   | LSB | High-precision channel |     |
|                                     |     |     |        | ±32.0 | LSB | Other than above       |     |
| DNL differential nonlinearity error |     |     | - ±4.0 | -     | LSB | -                      |     |
| INL integral nonlinearity error     |     |     | - ±4.0 | ±12.0 | LSB | -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C ), see section 2.2.4, I/O V , V , and Other Characteristics.
|     |     | in  |     | OH OL |     |     |     |
| --- | --- | --- | --- | ----- | --- | --- | --- |
Table 2.43 A/D conversion characteristics (2) in high-speed A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 2.7 to 5.5 V, VREFH0 = 2.7 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     |     | Min Typ | Max | Unit | Test conditions |     |
| --------- | --- | --- | ------- | --- | ---- | --------------- | --- |
| Frequency |     |     | 1 -     | 48  | MHz  | -               |     |
Analog input capacitance*2 Cs - - 8 (reference data) pF High-precision channel
|     |     |     | - - | 9 (reference data) | pF  | Normal-precision channel |     |
| --- | --- | --- | --- | ------------------ | --- | ------------------------ | --- |
Analog input resistance Rs - - 2.5 (reference data) kΩ High-precision channel
|                            |     |     | - - | 6.7 (reference data) | kΩ  | Normal-precision channel |     |
| -------------------------- | --- | --- | --- | -------------------- | --- | ------------------------ | --- |
| Analog input voltage range |     | Ain | 0 - | VREFH0               | V   | -                        |     |
12-bit mode
| Resolution |     |     | - - | 12  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1 Permissible signal  0.94 - - μs High-precision channel
| (Operation at   | source impedance  |     |        |     |     | ADCSR.ADHSC = 0          |     |
| --------------- | ----------------- | --- | ------ | --- | --- | ------------------------ | --- |
| PCLKC = 48 MHz) | Max. = 0.3 kΩ     |     |        |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|                 |                   |     | 1.50 - | -   | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 0
ADSSTRn.SST[7:0] = 28h
| Offset error                        |          |     | - ±0.5  | ±4.5 | LSB | High-precision channel |                |
| ----------------------------------- | -------- | --- | ------- | ---- | --- | ---------------------- | -------------- |
|                                     |          |     |         | ±6.0 | LSB | Other than above       |                |
| Full-scale error                    |          |     | - ±0.75 | ±4.5 | LSB | High-precision channel |                |
|                                     |          |     |         | ±6.0 | LSB | Other than above       |                |
| Quantization error                  |          |     | - ±0.5  | -    | LSB | -                      |                |
| Absolute accuracy                   |          | -   | ±1.25   | ±5.0 | LSB | High-precision channel |                |
|                                     |          |     |         | ±8.0 | LSB | Other than above       |                |
| DNL differential nonlinearity error |          |     | - ±1.0  | -    | LSB | -                      |                |
| INL integral nonlinearity error     |          |     | - ±1.0  | ±3.0 | LSB | -                      |                |
| R01DS0355EJ0110                     | Rev.1.10 |     |         |      |     |                        | Page 88 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
Table 2.43 A/D conversion characteristics (2) in high-speed A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 2.7 to 5.5 V, VREFH0 = 2.7 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min | Typ | Max | Unit | Test conditions |     |
| --------- | --- | --- | --- | --- | ---- | --------------- | --- |
14-bit mode
| Resolution |     | -   | -   | 14  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1
|                 | Permissible signal  | 1.06 | -   | -   | μs  | High-precision channel   |     |
| --------------- | ------------------- | ---- | --- | --- | --- | ------------------------ | --- |
| (Operation at   | source impedance    |      |     |     |     | ADCSR.ADHSC = 0          |     |
| PCLKC = 48 MHz) | Max. = 0.3 kΩ       |      |     |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|                 |                     | 1.63 | -   | -   | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 0
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | -   | ±2.0 | ±18   | LSB | High-precision channel |     |
| ----------------------------------- | --- | --- | ---- | ----- | --- | ---------------------- | --- |
|                                     |     |     |      | ±24.0 | LSB | Other than above       |     |
| Full-scale error                    |     | -   | ±3.0 | ±18   | LSB | High-precision channel |     |
|                                     |     |     |      | ±24.0 | LSB | Other than above       |     |
| Quantization error                  |     | -   | ±0.5 | -     | LSB | -                      |     |
| Absolute accuracy                   |     | -   | ±5.0 | ±20   | LSB | High-precision channel |     |
|                                     |     |     |      | ±32.0 | LSB | Other than above       |     |
| DNL differential nonlinearity error |     | -   | ±4.0 | -     | LSB | -                      |     |
| INL integral nonlinearity error     |     | -   | ±4.0 | ±12.0 | LSB | -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C ), see section 2.2.4, I/O V , V , and Other Characteristics.
|     |     | in  |     | OH OL |     |     |     |
| --- | --- | --- | --- | ----- | --- | --- | --- |
Table 2.44 A/D conversion characteristics (3) in high-speed A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 2.4 to 5.5 V, VREFH0 = 2.4 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min | Typ | Max | Unit | Test conditions |     |
| --------- | --- | --- | --- | --- | ---- | --------------- | --- |
| Frequency |     | 1   | -   | 32  | MHz  | -               |     |
Analog input capacitance*2 Cs - - 8 (reference data) pF High-precision channel
|     |     | -   | -   | 9 (reference data) | pF  | Normal-precision channel |     |
| --- | --- | --- | --- | ------------------ | --- | ------------------------ | --- |
Analog input resistance Rs - - 2.5 (reference data) kΩ High-precision channel
|                            |     | -   | -   | 6.7 (reference data) | kΩ  | Normal-precision channel |     |
| -------------------------- | --- | --- | --- | -------------------- | --- | ------------------------ | --- |
| Analog input voltage range | Ain | 0   | -   | VREFH0               | V   | -                        |     |
12-bit mode
| Resolution |     | -   | -   | 12  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1 Permissible signal  1.41 - - μs High-precision channel
| (Operation at   | source impedance  |      |     |     |     | ADCSR.ADHSC = 0          |     |
| --------------- | ----------------- | ---- | --- | --- | --- | ------------------------ | --- |
| PCLKC = 32 MHz) | Max. = 1.3 kΩ     |      |     |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|                 |                   | 2.25 | -   | -   | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 0
ADSSTRn.SST[7:0] = 28h
| Offset error       |          | -   | ±0.5  | ±4.5 | LSB | High-precision channel |                |
| ------------------ | -------- | --- | ----- | ---- | --- | ---------------------- | -------------- |
|                    |          |     |       | ±6.0 | LSB | Other than above       |                |
| Full-scale error   |          | -   | ±0.75 | ±4.5 | LSB | High-precision channel |                |
|                    |          |     |       | ±6.0 | LSB | Other than above       |                |
| Quantization error |          | -   | ±0.5  | -    | LSB | -                      |                |
| Absolute accuracy  |          | -   | ±1.25 | ±5.0 | LSB | High-precision channel |                |
|                    |          |     |       | ±8.0 | LSB | Other than above       |                |
| R01DS0355EJ0110    | Rev.1.10 |     |       |      |     |                        | Page 89 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
Table 2.44 A/D conversion characteristics (3) in high-speed A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 2.4 to 5.5 V, VREFH0 = 2.4 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter                           |     | Min | Typ  | Max  | Unit | Test conditions |     |
| ----------------------------------- | --- | --- | ---- | ---- | ---- | --------------- | --- |
| DNL differential nonlinearity error |     | -   | ±1.0 | -    | LSB  | -               |     |
| INL integral nonlinearity error     |     | -   | ±1.0 | ±3.0 | LSB  | -               |     |
14-bit mode
| Resolution |     | -   | -   | 14  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1 Permissible signal  1.59 - - μs High-precision channel
| (Operation at   | source impedance  |      |     |     |     | ADCSR.ADHSC = 0          |     |
| --------------- | ----------------- | ---- | --- | --- | --- | ------------------------ | --- |
| PCLKC = 32 MHz) | Max. = 1.3 kΩ     |      |     |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|                 |                   | 2.44 | -   | -   | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 0
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | -   | ±2.0 | ±18   | LSB | High-precision channel |     |
| ----------------------------------- | --- | --- | ---- | ----- | --- | ---------------------- | --- |
|                                     |     |     |      | ±24.0 | LSB | Other than above       |     |
| Full-scale error                    |     | -   | ±3.0 | ±18   | LSB | High-precision channel |     |
|                                     |     |     |      | ±24.0 | LSB | Other than above       |     |
| Quantization error                  |     | -   | ±0.5 | -     | LSB | -                      |     |
| Absolute accuracy                   |     | -   | ±5.0 | ±20   | LSB | High-precision channel |     |
|                                     |     |     |      | ±32.0 | LSB | Other than above       |     |
| DNL differential nonlinearity error |     | -   | ±4.0 | -     | LSB | -                      |     |
| INL integral nonlinearity error     |     | -   | ±4.0 | ±12.0 | LSB | -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C in ), see section 2.2.4, I/O V OH , V OL , and Other Characteristics.
Table 2.45 A/D conversion characteristics (4) in low power A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 2.7 to 5.5 V, VREFH0 = 2.7 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min | Typ | Max | Unit | Test conditions |     |
| --------- | --- | --- | --- | --- | ---- | --------------- | --- |
| Frequency |     | 1   | -   | 24  | MHz  | -               |     |
Analog input capacitance*2 Cs - - 8 (reference data) pF High-precision channel
|     |     | -   | -   | 9 (reference data) | pF  | Normal-precision channel |     |
| --- | --- | --- | --- | ------------------ | --- | ------------------------ | --- |
Analog input resistance Rs - - 2.5 (reference data) kΩ High-precision channel
|                            |     | -   | -   | 6.7 (reference data) | kΩ  | Normal-precision channel |     |
| -------------------------- | --- | --- | --- | -------------------- | --- | ------------------------ | --- |
| Analog input voltage range | Ain | 0   | -   | VREFH0               | V   | -                        |     |
12-bit mode
| Resolution |     | -   | -   | 12  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1 Permissible  2.25 - - μs High-precision channel
| (Operation at   | signal source   |     |     |     |     | ADCSR.ADHSC = 1        |     |
| --------------- | --------------- | --- | --- | --- | --- | ---------------------- | --- |
| PCLKC = 24 MHz) | impedance Max.  |     |     |     |     | ADSSTRn.SST[7:0] = 0Dh |     |
= 1.1 kΩ
|     |     | 3.38 | -   | -   | μs  | Normal-precision channel |     |
| --- | --- | ---- | --- | --- | --- | ------------------------ | --- |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error       |          | -   | ±0.5  | ±4.5 | LSB | High-precision channel |                |
| ------------------ | -------- | --- | ----- | ---- | --- | ---------------------- | -------------- |
|                    |          |     |       | ±6.0 | LSB | Other than above       |                |
| Full-scale error   |          | -   | ±0.75 | ±4.5 | LSB | High-precision channel |                |
|                    |          |     |       | ±6.0 | LSB | Other than above       |                |
| Quantization error |          | -   | ±0.5  | -    | LSB | -                      |                |
| R01DS0355EJ0110    | Rev.1.10 |     |       |      |     |                        | Page 90 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
Table 2.45 A/D conversion characteristics (4) in low power A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 2.7 to 5.5 V, VREFH0 = 2.7 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter                           |     | Min Typ | Max  | Unit Test conditions       |     |
| ----------------------------------- | --- | ------- | ---- | -------------------------- | --- |
| Absolute accuracy                   |     | - ±1.25 | ±5.0 | LSB High-precision channel |     |
|                                     |     |         | ±8.0 | LSB Other than above       |     |
| DNL differential nonlinearity error |     | - ±1.0  | -    | LSB -                      |     |
| INL integral nonlinearity error     |     | - ±1.0  | ±3.0 | LSB -                      |     |
14-bit mode
| Resolution |     | - - | 14  | Bit - |     |
| ---------- | --- | --- | --- | ----- | --- |
Conversion time*1 Permissible  2.50 - - μs High-precision channel
| (Operation at   | signal source   |     |     | ADCSR.ADHSC = 1        |     |
| --------------- | --------------- | --- | --- | ---------------------- | --- |
| PCLKC = 24 MHz) | impedance Max.  |     |     | ADSSTRn.SST[7:0] = 0Dh |     |
= 1.1 kΩ
|     |     | 3.63 - | -   | μs Normal-precision channel |     |
| --- | --- | ------ | --- | --------------------------- | --- |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | - ±2.0 | ±18   | LSB High-precision channel |     |
| ----------------------------------- | --- | ------ | ----- | -------------------------- | --- |
|                                     |     |        | ±24.0 | LSB Other than above       |     |
| Full-scale error                    |     | - ±3.0 | ±18   | LSB High-precision channel |     |
|                                     |     |        | ±24.0 | LSB Other than above       |     |
| Quantization error                  |     | - ±0.5 | -     | LSB -                      |     |
| Absolute accuracy                   |     | - ±5.0 | ±20   | LSB High-precision channel |     |
|                                     |     |        | ±32.0 | LSB Other than above       |     |
| DNL differential nonlinearity error |     | - ±4.0 | -     | LSB -                      |     |
| INL integral nonlinearity error     |     | - ±4.0 | ±12.0 | LSB -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C ), see section 2.2.4, I/O V , V , and Other Characteristics.
|     |     | in  | OH OL |     |     |
| --- | --- | --- | ----- | --- | --- |
Table 2.46 A/D conversion characteristics (5) in low power A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 2.4 to 5.5 V, VREFH0 = 2.4 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min Typ | Max | Unit Test conditions |     |
| --------- | --- | ------- | --- | -------------------- | --- |
| Frequency |     | 1 -     | 16  | MHz -                |     |
Analog input capacitance*2
|     | Cs  | - - | 8 (reference data) | pF High-precision channel   |     |
| --- | --- | --- | ------------------ | --------------------------- | --- |
|     |     | - - | 9 (reference data) | pF Normal-precision channel |     |
Analog input resistance Rs - - 2.5 (reference data) kΩ High-precision channel
|                            |     | - - | 6.7 (reference data) | kΩ Normal-precision channel |     |
| -------------------------- | --- | --- | -------------------- | --------------------------- | --- |
| Analog input voltage range | Ain | 0 - | VREFH0               | V -                         |     |
12-bit mode
| Resolution |     | - - | 12  | Bit - |     |
| ---------- | --- | --- | --- | ----- | --- |
Conversion time*1 Permissible signal  3.38 - - μs High-precision channel
| (Operation at   | source impedance  |        |     | ADCSR.ADHSC = 1             |     |
| --------------- | ----------------- | ------ | --- | --------------------------- | --- |
| PCLKC = 16 MHz) | Max. = 2.2 kΩ     |        |     | ADSSTRn.SST[7:0] = 0Dh      |     |
|                 |                   | 5.06 - | -   | μs Normal-precision channel |     |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error    |          | - ±0.5 | ±4.5 | LSB High-precision channel |                |
| --------------- | -------- | ------ | ---- | -------------------------- | -------------- |
|                 |          |        | ±6.0 | LSB Other than above       |                |
| R01DS0355EJ0110 | Rev.1.10 |        |      |                            | Page 91 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
Table 2.46 A/D conversion characteristics (5) in low power A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 2.4 to 5.5 V, VREFH0 = 2.4 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter                           |     | Min | Typ   | Max  | Unit | Test conditions        |     |
| ----------------------------------- | --- | --- | ----- | ---- | ---- | ---------------------- | --- |
| Full-scale error                    |     | -   | ±0.75 | ±4.5 | LSB  | High-precision channel |     |
|                                     |     |     |       | ±6.0 | LSB  | Other than above       |     |
| Quantization error                  |     | -   | ±0.5  | -    | LSB  | -                      |     |
| Absolute accuracy                   |     | -   | ±1.25 | ±5.0 | LSB  | High-precision channel |     |
|                                     |     |     |       | ±8.0 | LSB  | Other than above       |     |
| DNL differential nonlinearity error |     | -   | ±1.0  | -    | LSB  | -                      |     |
| INL integral nonlinearity error     |     | -   | ±1.0  | ±3.0 | LSB  | -                      |     |
14-bit mode
| Resolution |     | -   | -   | 14  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1 Permissible signal  3.75 - - μs High-precision channel
| (Operation at   | source impedance  |      |     |     |     | ADCSR.ADHSC = 1          |     |
| --------------- | ----------------- | ---- | --- | --- | --- | ------------------------ | --- |
| PCLKC = 16 MHz) | Max. = 2.2 kΩ     |      |     |     |     | ADSSTRn.SST[7:0] = 0Dh   |     |
|                 |                   | 5.44 | -   | -   | μs  | Normal-precision channel |     |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | -   | ±2.0 | ±18   | LSB | High-precision channel |     |
| ----------------------------------- | --- | --- | ---- | ----- | --- | ---------------------- | --- |
|                                     |     |     |      | ±24.0 | LSB | Other than above       |     |
| Full-scale error                    |     | -   | ±3.0 | ±18   | LSB | High-precision channel |     |
|                                     |     |     |      | ±24.0 | LSB | Other than above       |     |
| Quantization error                  |     | -   | ±0.5 | -     | LSB | -                      |     |
| Absolute accuracy                   |     | -   | ±5.0 | ±20   | LSB | High-precision channel |     |
|                                     |     |     |      | ±32.0 | LSB | Other than above       |     |
| DNL differential nonlinearity error |     | -   | ±4.0 | -     | LSB | -                      |     |
| INL integral nonlinearity error     |     | -   | ±4.0 | ±12.0 | LSB | -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C in ), see section 2.2.4, I/O V OH , V OL , and Other Characteristics.
Table 2.47 A/D conversion characteristics (6) in low power A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 1.8 to 5.5 V (AVCC0 = VCC when VCC < 2.0 V), VREFH0 = 1.8 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min | Typ | Max | Unit | Test conditions |     |
| --------- | --- | --- | --- | --- | ---- | --------------- | --- |
| Frequency |     | 1   | -   | 8   | MHz  | -               |     |
Analog input capacitance*2 Cs - - 8 (reference data) pF High-precision channel
|     |     | -   | -   | 9 (reference data) | pF  | Normal-precision channel |     |
| --- | --- | --- | --- | ------------------ | --- | ------------------------ | --- |
Analog input resistance Rs - - 3.8 (reference data) kΩ High-precision channel
|                            |     | -   | -   | 8.2 (reference data) | kΩ  | Normal-precision channel |     |
| -------------------------- | --- | --- | --- | -------------------- | --- | ------------------------ | --- |
| Analog input voltage range | Ain | 0   | -   | VREFH0               | V   | -                        |     |
12-bit mode
| Resolution |     | -   | -   | 12  | Bit | -   |     |
| ---------- | --- | --- | --- | --- | --- | --- | --- |
Conversion time*1 Permissible signal  6.75 - - μs High-precision channel
| (Operation at  | source          |     |     |     |     | ADCSR.ADHSC = 1        |     |
| -------------- | --------------- | --- | --- | --- | --- | ---------------------- | --- |
| PCLKC = 8 MHz) | impedance Max.  |     |     |     |     | ADSSTRn.SST[7:0] = 0Dh |     |
= 5 kΩ
|     |     | 10.13 | -   | -   | μs  | Normal-precision channel |     |
| --- | --- | ----- | --- | --- | --- | ------------------------ | --- |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     | Page 92 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
Table 2.47 A/D conversion characteristics (6) in low power A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 1.8 to 5.5 V (AVCC0 = VCC when VCC < 2.0 V), VREFH0 = 1.8 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter                           |     | Min Typ | Max   | Unit Test conditions       |     |
| ----------------------------------- | --- | ------- | ----- | -------------------------- | --- |
| Offset error                        |     | - ±1.0  | ±7.5  | LSB High-precision channel |     |
|                                     |     |         | ±10.0 | LSB Other than above       |     |
| Full-scale error                    |     | - ±1.5  | ±7.5  | LSB High-precision channel |     |
|                                     |     |         | ±10.0 | LSB Other than above       |     |
| Quantization error                  |     | - ±0.5  | -     | LSB -                      |     |
| Absolute accuracy                   |     | - ±3.0  | ±8.0  | LSB High-precision channel |     |
|                                     |     |         | ±12.0 | LSB Other than above       |     |
| DNL differential nonlinearity error |     | - ±1.0  | -     | LSB -                      |     |
| INL integral nonlinearity error     |     | - ±1.0  | ±3.0  | LSB -                      |     |
14-bit mode
| Resolution |     | - - | 14  | Bit - |     |
| ---------- | --- | --- | --- | ----- | --- |
Conversion time*1 Permissible signal  7.50 - - μs High-precision channel
| (Operation at  | source          |     |     | ADCSR.ADHSC = 1        |     |
| -------------- | --------------- | --- | --- | ---------------------- | --- |
| PCLKC = 8 MHz) | impedance Max.  |     |     | ADSSTRn.SST[7:0] = 0Dh |     |
= 5 kΩ
|     |     | 10.88 - | -   | μs Normal-precision channel |     |
| --- | --- | ------- | --- | --------------------------- | --- |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | - ±4.0  | ±30.0 | LSB High-precision channel |     |
| ----------------------------------- | --- | ------- | ----- | -------------------------- | --- |
|                                     |     |         | ±40.0 | LSB Other than above       |     |
| Full-scale error                    |     | - ±6.0  | ±30.0 | LSB High-precision channel |     |
|                                     |     |         | ±40.0 | LSB Other than above       |     |
| Quantization error                  |     | - ±0.5  | -     | LSB -                      |     |
| Absolute accuracy                   |     | - ±12.0 | ±32.0 | LSB High-precision channel |     |
|                                     |     |         | ±48.0 | LSB Other than above       |     |
| DNL differential nonlinearity error |     | - ±4.0  | -     | LSB -                      |     |
| INL integral nonlinearity error     |     | - ±4.0  | ±12.0 | LSB -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C ), see section 2.2.4, I/O V , V , and Other Characteristics.
|     |     | in  | OH OL |     |     |
| --- | --- | --- | ----- | --- | --- |
Table 2.48 A/D conversion characteristics (7) in low power A/D conversion mode (1 of 2)
Conditions: VCC = AVCC0 = 1.6 to 5.5 V (AVCC0 = VCC when VCC < 2.0 V), VREFH0 = 1.6 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min Typ | Max | Unit Test conditions |     |
| --------- | --- | ------- | --- | -------------------- | --- |
| Frequency |     | 1 -     | 4   | MHz -                |     |
Analog input capacitance*2 Cs - - 8 (reference data) pF High-precision channel
|     |     | - - | 9 (reference data) | pF Normal-precision channel |     |
| --- | --- | --- | ------------------ | --------------------------- | --- |
Analog input resistance Rs - - 13.1 (reference data) kΩ High-precision channel
|                            |     | - - | 14.3 (reference data) | kΩ Normal-precision channel |     |
| -------------------------- | --- | --- | --------------------- | --------------------------- | --- |
| Analog input voltage range | Ain | 0 - | VREFH0                | V -                         |     |
12-bit mode
| Resolution      |          | - - | 12  | Bit - |                |
| --------------- | -------- | --- | --- | ----- | -------------- |
| R01DS0355EJ0110 | Rev.1.10 |     |     |       | Page 93 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
Table 2.48 A/D conversion characteristics (7) in low power A/D conversion mode (2 of 2)
Conditions: VCC = AVCC0 = 1.6 to 5.5 V (AVCC0 = VCC when VCC < 2.0 V), VREFH0 = 1.6 to 5.5 V
Reference voltage range applied to the VREFH0 and VREFL0.
| Parameter |     | Min Typ | Max | Unit Test conditions |     |
| --------- | --- | ------- | --- | -------------------- | --- |
Conversion time*1 Permissible  13.5 - - μs High-precision channel
| (Operation at  | signal source   |     |     | ADCSR.ADHSC = 1        |     |
| -------------- | --------------- | --- | --- | ---------------------- | --- |
| PCLKC = 4 MHz) | impedance Max.  |     |     | ADSSTRn.SST[7:0] = 0Dh |     |
= 9.9 kΩ
|     |     | 20.25 - | -   | μs Normal-precision channel |     |
| --- | --- | ------- | --- | --------------------------- | --- |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | - ±1.0 | ±7.5  | LSB High-precision channel |     |
| ----------------------------------- | --- | ------ | ----- | -------------------------- | --- |
|                                     |     |        | ±10.0 | LSB Other than above       |     |
| Full-scale error                    |     | - ±1.5 | ±7.5  | LSB High-precision channel |     |
|                                     |     |        | ±10.0 | LSB Other than above       |     |
| Quantization error                  |     | - ±0.5 | -     | LSB -                      |     |
| Absolute accuracy                   |     | - ±3.0 | ±8.0  | LSB High-precision channel |     |
|                                     |     |        | ±12.0 | LSB Other than above       |     |
| DNL differential nonlinearity error |     | - ±1.0 | -     | LSB -                      |     |
| INL integral nonlinearity error     |     | - ±1.0 | ±3.0  | LSB -                      |     |
14-bit mode
| Resolution |     | - - | 14  | Bit - |     |
| ---------- | --- | --- | --- | ----- | --- |
Conversion time*1
|                | Permissible     | 15.0 -  | -   | μs High-precision channel   |     |
| -------------- | --------------- | ------- | --- | --------------------------- | --- |
| (Operation at  | signal source   |         |     | ADCSR.ADHSC = 1             |     |
| PCLKC = 4 MHz) | impedance Max.  |         |     | ADSSTRn.SST[7:0] = 0Dh      |     |
|                | = 9.9 kΩ        | 21.75 - | -   | μs Normal-precision channel |     |
ADCSR.ADHSC = 1
ADSSTRn.SST[7:0] = 28h
| Offset error                        |     | - ±4.0  | ±30.0 | LSB High-precision channel |     |
| ----------------------------------- | --- | ------- | ----- | -------------------------- | --- |
|                                     |     |         | ±40.0 | LSB Other than above       |     |
| Full-scale error                    |     | - ±6.0  | ±30.0 | LSB High-precision channel |     |
|                                     |     |         | ±40.0 | LSB Other than above       |     |
| Quantization error                  |     | - ±0.5  | -     | LSB -                      |     |
| Absolute accuracy                   |     | - ±12.0 | ±32.0 | LSB High-precision channel |     |
|                                     |     |         | ±48.0 | LSB Other than above       |     |
| DNL differential nonlinearity error |     | - ±4.0  | -     | LSB -                      |     |
| INL integral nonlinearity error     |     | - ±4.0  | ±12.0 | LSB -                      |     |
Note: The characteristics apply when no pin functions other than 14-bit A/D converter input are used. Absolute accuracy does not
include quantization errors. Offset error, full-scale error, DNL differential nonlinearity error, and INL integral nonlinearity error do
not include quantization errors.
Note 1. The conversion time is the sum of the sampling time and the comparison time. The number of sampling states is indicated for
the test conditions.
Note 2. Except for I/O input capacitance (C ), see section 2.2.4, I/O V , V , and Other Characteristics.
|                 |          | in  | OH OL |     |                |
| --------------- | -------- | --- | ----- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10 |     |       |     | Page 94 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | ----------------------------- | --- |
MCU
Analog input
Sensor Rs
ANn
Cin
ADC
Cs
Analog input
Rs
ANn
Cin
| Figure 2.69                                            | Equivalent circuit for analog input |     |            |     |         |     |
| ------------------------------------------------------ | ----------------------------------- | --- | ---------- | --- | ------- | --- |
| Table 2.49 14-Bit A/D converter channel classification |                                     |     |            |     |         |     |
| Classification                                         | Channel                             |     | Conditions |     | Remarks |     |
High-precision channel AN000 to AN014 AVCC0 = 1.6 to 5.5 V Pins AN000 to AN014 cannot be used
Normal-precision channel AN016 to AN025 as general I/O, IRQ2, IRQ3 inputs,
and TS transmission, when the A/D
converter is in use
Internal reference voltage  Internal reference voltage AVCC0 = 2.0 to 5.5 V -
input channel
Temperature sensor input  Temperature sensor output AVCC0 = 2.0 to 5.5 V -
channel
| Table 2.50 A/D internal reference voltage characteristics |     |     |     |     |     |     |
| --------------------------------------------------------- | --- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = VREFH0 = 2.0 to 5.5 V*1
| Parameter                         | Min  | Typ  | Max  | Unit | Test conditions |     |
| --------------------------------- | ---- | ---- | ---- | ---- | --------------- | --- |
| Internal reference voltage input  | 1.36 | 1.43 | 1.50 | V    | -               |     |
channel*2
Frequency*3
|                 | 1   | -   | 2   | MHz | -   |     |
| --------------- | --- | --- | --- | --- | --- | --- |
| Sampling time*4 | 5.0 | -   | -   | μs  | -   |     |
Note 1. The internal reference voltage cannot be selected for input channels when AVCC0 < 2.0 V.
Note 2. The 14-bit A/D internal reference voltage indicates the voltage when the internal reference voltage is input to the 14-bit A/D
converter.
Note 3. This is a parameter for ADC14 when the internal reference voltage is used as the high-potential reference voltage.
Note 4. This is a parameter for ADC14 when the internal reference voltage is selected for an analog input channel in ADC14.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     | Page 95 of 131 |
| --------------- | -------- | --- | --- | --- | --- | -------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
3FFFh
Full-scale error
Integral nonlinearity
error (INL)
A/D converter
output code Ideal line of actual A/D
conversion characteristic
Actual A/D conversion
characteristic
Ideal A/D conversion
characteristic Differential nonlinearity error (DNL)
1-LSB width for ideal A/D
conversion characteristic
Differential nonlinearity error (DNL)
1-LSB width for ideal A/D
conversion characteristic
Absolute accuracy
0000h Offset error
0 Analog input voltage VREFH0
(full-scale)
Figure 2.70 Illustration of 14-bit A/D converter characteristic terms
Absolute accuracy
Absolute accuracy is the difference between output code based on the theoretical A/D conversion characteristics, and the
actual A/D conversion result. When measuring absolute accuracy, the voltage at the midpoint of the width of analog
input voltage (1-LSB width), which can meet the expectation of outputting an equal code based on the theoretical A/D
conversion characteristics, is used as the analog input voltage. For example, if 12-bit resolution is used and the reference
voltage VREFH0 = 3.072 V, then 1-LSB width becomes 0.75 mV, and 0 mV, 0.75 mV, and 1.5 mV are used as the analog
input voltages. If analog input voltage is 6 mV, an absolute accuracy of ±5 LSB means that the actual A/D conversion
result is in the range of 003h to 00Dh, though an output code of 008h can be expected from the theoretical A/D
conversion characteristics.
Integral nonlinearity error (INL)
Integral nonlinearity error is the maximum deviation between the ideal line when the measured offset and full-scale
errors are zeroed, and the actual output code.
Differential nonlinearity error (DNL)
Differential nonlinearity error is the difference between 1-LSB width based on the ideal A/D conversion characteristics
and the width of the actually output code.
Offset error
Offset error is the difference between the transition point of the ideal first output code and the actual first output code.
Full-scale error
Full-scale error is the difference between the transition point of the ideal last output code and the actual last output code.
R01DS0355EJ0110 Rev.1.10 Page 96 of 131
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | ----------------------------- | --- |
2.6 DAC12 Characteristics
| Table 2.51 D/A conversion characteristics (1) |     |     |     |     |     |     |
| --------------------------------------------- | --- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
Reference voltage = VREFH or VREFL selected
| Parameter                                     |     | Min  | Typ  | Max          | Unit Test conditions |     |
| --------------------------------------------- | --- | ---- | ---- | ------------ | -------------------- | --- |
| Resolution                                    |     | -    | -    | 12           | bit -                |     |
| Resistive load                                |     | 30   | -    | -            | kΩ -                 |     |
| Load capacitance                              |     | -    | -    | 50           | pF -                 |     |
| Output voltage range                          |     | 0.35 | -    | AVCC0 - 0.47 | V -                  |     |
| DNL differential nonlinearity error           |     | -    | ±0.5 | ±1.0         | LSB -                |     |
| INL integral nonlinearity error               |     | -    | ±2.0 | ±8.0         | LSB -                |     |
| Offset error                                  |     | -    | -    | ±20          | mV -                 |     |
| Full-scale error                              |     | -    | -    | ±20          | mV -                 |     |
| Output impedance                              |     | -    | 5    | -            | Ω -                  |     |
| Conversion time                               |     | -    | -    | 30           | μs -                 |     |
| Table 2.52 D/A conversion characteristics (2) |     |      |      |              |                      |     |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
Reference voltage = AVCC0 or AVSS0 selected
| Parameter                                     |     | Min  | Typ  | Max          | Unit Test conditions |     |
| --------------------------------------------- | --- | ---- | ---- | ------------ | -------------------- | --- |
| Resolution                                    |     | -    | -    | 12           | bit -                |     |
| Resistive load                                |     | 30   | -    | -            | kΩ -                 |     |
| Load capacitance                              |     | -    | -    | 50           | pF -                 |     |
| Output voltage range                          |     | 0.35 | -    | AVCC0 - 0.47 | V -                  |     |
| DNL differential nonlinearity error           |     | -    | ±0.5 | ±2.0         | LSB -                |     |
| INL integral nonlinearity error               |     | -    | ±2.0 | ±8.0         | LSB -                |     |
| Offset error                                  |     | -    | -    | ±30          | mV -                 |     |
| Full-scale error                              |     | -    | -    | ±30          | mV -                 |     |
| Output impedance                              |     | -    | 5    | -            | Ω -                  |     |
| Conversion time                               |     | -    | -    | 30           | μs -                 |     |
| Table 2.53 D/A conversion characteristics (3) |     |      |      |              |                      |     |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
Reference voltage = internal reference voltage selected
| Parameter                           |          | Min  | Typ  | Max   | Unit Test conditions |                |
| ----------------------------------- | -------- | ---- | ---- | ----- | -------------------- | -------------- |
| Resolution                          |          | -    | -    | 12    | bit -                |                |
| Internal reference voltage (Vbgr)   |          | 1.36 | 1.43 | 1.50  | V -                  |                |
| Resistive load                      |          | 30   | -    | -     | kΩ -                 |                |
| Load capacitance                    |          | -    | -    | 50    | pF -                 |                |
| Output voltage range                |          | 0.35 | -    | Vbgr  | V -                  |                |
| DNL differential nonlinearity error |          | -    | ±2.0 | ±16.0 | LSB -                |                |
| INL integral nonlinearity error     |          | -    | ±8.0 | ±16.0 | LSB -                |                |
| Offset error                        |          | -    | -    | ±30   | mV -                 |                |
| Output impedance                    |          | -    | 5    | -     | Ω -                  |                |
| Conversion time                     |          | -    | -    | 30    | μs -                 |                |
| R01DS0355EJ0110                     | Rev.1.10 |      |      |       |                      | Page 97 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
Full-scale error Gain error
Upper output limit
Integral nonlinearity error (INL)
Offset error
1-LSB width for ideal D/A conversion
Output analog voltage characteristic Ideal output voltage
Differential nonlinearity error
(DNL) *1
Lower output limit
Actual D/A conversion characteristic
Offset error
Ideal output voltage
000h FFFh
D/A converter input code
Note 1. Ideal D/A conversion output voltage that is adjusted so that offset and full scale errors are zeroed.
Figure 2.71 Illustration of D/A converter characteristic terms
Integral nonlinearity error (INL)
Integral nonlinearity error is the maximum deviation between the ideal output voltage based on the ideal conversion
characteristic when the measured offset and full-scale errors are zeroed, and the actual output voltage.
Differential nonlinearity error (DNL)
Differential nonlinearity error is the difference between 1-LSB voltage width based on the ideal D/A conversion
characteristics and the width of the actual output voltage.
Offset error
Offset error is the difference between the highest actual output voltage that falls below the lower output limit and the
ideal output voltage based on the input code.
Full-scale error
Full-scale error is the difference between the lowest actual output voltage that exceeds the upper output limit and the
ideal output voltage based on the input code.
R01DS0355EJ0110 Rev.1.10 Page 98 of 131
Sep 29, 2023

| RA4M1 Group |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | --- | ----------------------------- | --- |
2.7 TSN Characteristics
| Table 2.54 TSN characteristics |     |     |     |     |     |     |     |
| ------------------------------ | --- | --- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 2.0 to 5.5 V
| Parameter                     |     | Symbol | Min | Typ   | Max | Unit  | Test conditions |
| ----------------------------- | --- | ------ | --- | ----- | --- | ----- | --------------- |
| Relative accuracy             |     | -      | -   | ±1.5  | -   | °C    | 2.4 V or above  |
|                               |     | -      | -   | ±2.0  | -   | °C    | Below 2.4 V     |
| Temperature slope             |     | -      | -   | -3.65 | -   | mV/°C | -               |
| Output voltage (at 25°C)      |     | -      | -   | 1.05  | -   | V     | VCC = 3.3 V     |
| Temperature sensor start time |     | t      | -   | -     | 5   | μs    | -               |
START
| Sampling time |     | -   | 5   | -   | -   | μs  | -   |
| ------------- | --- | --- | --- | --- | --- | --- | --- |
2.8 OSC Stop Detect Characteristics
| Table 2.55 Oscillation stop detection circuit characteristics |     |        |     |     |     |      |                 |
| ------------------------------------------------------------- | --- | ------ | --- | --- | --- | ---- | --------------- |
| Parameter                                                     |     | Symbol | Min | Typ | Max | Unit | Test conditions |
| Detection time                                                |     | t      | -   | -   | 1   | ms   | Figure 2.72     |
dr
| Main clock |     |     |     | Main clock |     |     |     |
| ---------- | --- | --- | --- | ---------- | --- | --- | --- |
tdr
tdr
OSTDSR.OSTDF
OSTDSR.OSTDF
MOCO clock
PLL clock
ICLK
MOCO clock
ICLK
When the main clock is selected
When the PLL clock is selected
| Figure 2.72     | Oscillation stop detection timing |     |     |     |     |     |                |
| --------------- | --------------------------------- | --- | --- | --- | --- | --- | -------------- |
| R01DS0355EJ0110 | Rev.1.10                          |     |     |     |     |     | Page 99 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | ----------------------------- | --- |
2.9 POR and LVD Characteristics
Table 2.56 Power-on reset circuit and voltage detection circuit characteristics (1)
| Parameter |     | Symbol | Min Typ | Max Unit | Test conditions |
| --------- | --- | ------ | ------- | -------- | --------------- |
Voltage detection  Power-on reset (POR) V 1.27 1.42 1.57 V Figure 2.73,
POR
| level*1 |     |     |     |     | Figure 2.74 |
| ------- | --- | --- | --- | --- | ----------- |
Voltage detection circuit (LVD0)*2 V 3.68 3.85 4.00 V Figure 2.75
det0_0
At falling edge
|     |     | V      | 2.68 2.85 | 2.96 |     |
| --- | --- | ------ | --------- | ---- | --- |
|     |     | det0_1 |           |      | VCC |
|     |     | V      | 2.38 2.53 | 2.64 |     |
det0_2
|     |     | V   | 1.78 1.90 | 2.02 |     |
| --- | --- | --- | --------- | ---- | --- |
det0_3
|     |     | V   | 1.60 1.69 | 1.82 |     |
| --- | --- | --- | --------- | ---- | --- |
det0_4
Voltage detection circuit (LVD1)*3 V 4.13 4.29 4.45 V Figure 2.76
det1_0
At falling edge
|     |     | V det1_1 | 3.98 4.16 | 4.30 |     |
| --- | --- | -------- | --------- | ---- | --- |
VCC
|     |     | V det1_2 | 3.86 4.03 | 4.18 |     |
| --- | --- | -------- | --------- | ---- | --- |
|     |     | V det1_3 | 3.68 3.86 | 4.00 |     |
|     |     | V det1_4 | 2.98 3.10 | 3.22 |     |
|     |     | V        | 2.89 3.00 | 3.11 |     |
det1_5
|     |     | V   | 2.79 2.90 | 3.01 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_6
|     |     | V   | 2.68 2.79 | 2.90 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_7
|     |     | V   | 2.58 2.68 | 2.78 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_8
|     |     | V   | 2.48 2.58 | 2.68 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_9
|     |     | V   | 2.38 2.48 | 2.58 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_A
|     |     | V   | 2.10 2.20 | 2.30 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_B
|     |     | V   | 1.84 1.96 | 2.05 |     |
| --- | --- | --- | --------- | ---- | --- |
det1_C
|     |     | V det1_D | 1.74 1.86 | 1.95 |     |
| --- | --- | -------- | --------- | ---- | --- |
|     |     | V det1_E | 1.63 1.75 | 1.84 |     |
|     |     | V det1_F | 1.60 1.65 | 1.73 |     |
Voltage detection circuit (LVD2)*4
|     |     | V det2_0 | 4.11 4.31 | 4.48 V | Figure 2.77 |
| --- | --- | -------- | --------- | ------ | ----------- |
At falling edge
|     |     | V det2_1 | 3.97 4.17 | 4.34 |     |
| --- | --- | -------- | --------- | ---- | --- |
VCC
|     |     | V   | 3.83 4.03 | 4.20 |     |
| --- | --- | --- | --------- | ---- | --- |
det2_2
|     |     | V   | 3.64 3.84 | 4.01 |     |
| --- | --- | --- | --------- | ---- | --- |
det2_3
Note 1. These characteristics apply when noise is not superimposed on the power supply. When a setting causes this voltage detection
level to overlap with that of the voltage detection circuit, it cannot be specified whether LVD1 or LVD2 is used for voltage
detection.
| Note 2. # in the symbol V |  denotes the value of the OFS1.VDSEL1[2:0] bits. |     |     |     |     |
| ------------------------- | ------------------------------------------------ | --- | --- | --- | --- |
det0_#
| Note 3. # in the symbol V |  denotes the value of the LVDLVLR.LVD1LVL[4:0] bits. |     |     |     |     |
| ------------------------- | ---------------------------------------------------- | --- | --- | --- | --- |
det1_#
| Note 4. # in the symbol V | det2_#  denotes the value of the LVDLVLR.LVD2LVL[2:0] bits. |     |     |     |                 |
| ------------------------- | ----------------------------------------------------------- | --- | --- | --- | --------------- |
| R01DS0355EJ0110           | Rev.1.10                                                    |     |     |     | Page 100 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | ----------------------------- | --- |
Table 2.57 Power-on reset circuit and voltage detection circuit characteristics (2)
| Parameter                 |             | Symbol | Min Typ | Max Unit | Test conditions |     |
| ------------------------- | ----------- | ------ | ------- | -------- | --------------- | --- |
| Wait time after power-on  | LVD0:enable | t POR  | - 1.7   | - ms     | -               |     |
reset cancellation
|     | LVD0:disable | t   | - 1.3 | - ms | -   |     |
| --- | ------------ | --- | ----- | ---- | --- | --- |
POR
| Wait time after voltage  | LVD0:enable*1 | t   | - 0.6 | - ms | -   |     |
| ------------------------ | ------------- | --- | ----- | ---- | --- | --- |
LVD0,1,2
monitor 0,1,2 reset
LVD0:disable*2
| cancellation     |     | t LVD1,2 | - 0.2 | - ms   | -            |     |
| ---------------- | --- | -------- | ----- | ------ | ------------ | --- |
| Response delay*3 |     | t        | - -   | 350 μs | Figure 2.73, |     |
det
Figure 2.74
| Minimum VCC down time |     | t VOFF | 450 - | - μs | Figure 2.73, |     |
| --------------------- | --- | ------ | ----- | ---- | ------------ | --- |
VCC = 1.0 V or above
| Power-on reset enable time |     | t   | 1 - | - ms | Figure 2.74, |     |
| -------------------------- | --- | --- | --- | ---- | ------------ | --- |
W (POR)
VCC = below 1.0 V
LVD operation stabilization time (after LVD is  T - - 300 μs Figure 2.76,
d (E-A)
| enabled)               |     |        |       |      | Figure 2.77 |     |
| ---------------------- | --- | ------ | ----- | ---- | ----------- | --- |
| Hysteresis width (POR) |     | V PORH | - 110 | - mV | -           |     |
Hysteresis width (LVD0, LVD1 and LVD2) V - 60 - mV LVD0 selected
LVH
|     |     |     | - 100 | - mV | V det1_0  to V | det1_2  selected. |
| --- | --- | --- | ----- | ---- | -------------- | ----------------- |
|     |     |     | - 60  | -    | V  to V        |  selected.        |
|     |     |     |       |      | det1_3         | det1_9            |
|     |     |     | - 50  | -    | V  or V        |  selected.        |
|     |     |     |       |      | det1_A         | det1_B            |
|     |     |     | - 40  | -    | V det1_C  or V | det1_F  selected. |
|     |     |     | - 60  | -    | LVD2 selected  |                   |
Note 1. When OFS1.LVDAS = 0.
Note 2. When OFS1.LVDAS = 1.
Note 3. The minimum VCC down time indicates the time when VCC is below the minimum value of voltage detection levels V POR ,
V , V , and V  for the POR/LVD.
det0 det1 det2
t
VOFF
VCC
V
POR
1.0 V
Internal reset signal
(active-low)
|                 |                                |     | t t     | t   |     |                 |
| --------------- | ------------------------------ | --- | ------- | --- | --- | --------------- |
|                 |                                |     | det det | POR |     |                 |
| Figure 2.73     | Voltage detection reset timing |     |         |     |     |                 |
| R01DS0355EJ0110 | Rev.1.10                       |     |         |     |     | Page 101 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
V
POR
VCC
1.0 V
t W(POR)
|     | Internal reset signal | *1  |     |     |     |
| --- | --------------------- | --- | --- | --- | --- |
(active-low)
t t
det POR
Note: t W(POR)  is the time required for a power-on reset to be enabled while the external power VCC is being held
below the valid voltage (1.0 V).
|     | When VCC turns on, maintain t |     |  for 1.0 ms or more. |     |     |
| --- | ----------------------------- | --- | -------------------- | --- | --- |
W(POR)
| Figure 2.74 | Power-on reset timing |     |     |     |     |
| ----------- | --------------------- | --- | --- | --- | --- |
t
VOFF
|     |     | VCC | V    | V   |     |
| --- | --- | --- | ---- | --- | --- |
|     |     |     | det0 | LVH |     |
Internal reset signal
(active-low)
|             |                                     |     | t det t | det t LVD0 |     |
| ----------- | ----------------------------------- | --- | ------- | ---------- | --- |
| Figure 2.75 | Voltage detection circuit timing (V |     | )       |            |     |
det0
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 102 of 131 |
| --------------- | -------- | --- | --- | --- | --------------- |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
t
VOFF
VCC V det1 V LVH
LVCMPCR.LVD1E
T
d(E-A)
LVD1
Comparator output
LVD1CR0.CMPE
LVD1SR.MON
Internal reset signal
(active-low)
When LVD1CR0.RN = 0
t det t det t LVD1
When LVD1CR0.RN = 1
t
LVD1
Figure 2.76 Voltage detection circuit timing (V )
det1
t
VOFF
VCC V det2 V LVH
LVCMPCR.LVD2E
T
d(E-A)
LVD2
Comparator output
LVD2CR0.CMPE
LVD2SR.MON
Internal reset signal
(active-low)
When LVD2CR0.RN = 0
t t t
det det LVD2
When LVD2CR0.RN = 1
t
LVD2
Figure 2.77 Voltage detection circuit timing (V )
det2
R01DS0355EJ0110 Rev.1.10 Page 103 of 131
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | ----------------------------- | --- |
2.10 VBATT Characteristics
| Table 2.58 Battery backup function characteristics |     |     |     |     |     |     |
| -------------------------------------------------- | --- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 1.6V to 5.5V, VBATT = 1.6 to 3.6 V
| Parameter |     |     | Symbol | Min Typ | Max Unit | Test conditions |
| --------- | --- | --- | ------ | ------- | -------- | --------------- |
Voltage level for switching to battery backup (falling) V 1.99 2.09 2.19 V Figure 2.78,
DETBATT
Figure 2.79
Hysteresis width for switching to battery back up V -  100 -  mV
VBATTH
VCC-off period for starting power supply switching t 300 -  -  μs -
VOFFBATT
| Voltage detection level  |     |     | V   | 1.30 1.40 | 1.50 V | Figure 2.78,  |
| ------------------------ | --- | --- | --- | --------- | ------ | ------------- |
VBATPOR
| VBATT_Power-on reset (VBATT_POR) |     |     |     |     |     | Figure 2.79 |
| -------------------------------- | --- | --- | --- | --- | --- | ----------- |
Wait time after VBATT_POR reset time cancellation t   -  -  3 mS -
VBATPOR
Level for detection of voltage drop on  VBTLVDLVL[1:0] = 10b V 2.11 2.2 2.29 V Figure 2.80
DETBATLVD
the VBATT pin (falling)
|                                    |     | VBTLVDLVL[1:0] = 11b |             | 1.92 2 | 2.08 V |     |
| ---------------------------------- | --- | -------------------- | ----------- | ------ | ------ | --- |
| Hysteresis width for VBATT pin LVD |     |                      | V VBATLVDTH | -  50  | -  mV  |     |
VBATT pin LVD operation stabilization time t d_vbat -  -  300 μs Figure 2.80
| VBATT pin LVD response delay time |     |     | t det_vbat | -  -  | 350 μs |     |
| --------------------------------- | --- | --- | ---------- | ----- | ------ | --- |
Allowable voltage change rising/falling gradient dt/dVCC 1.0 -  -  ms/V -
VCC voltage level for access to the VBATT backup registers V 1.8 -  -  V -
_BKBATT
Note: The VCC-off period for starting power supply switching indicates the period in which VCC is below the minimum value of the
voltage level for switching to battery backup (V ).
DETBATT
|     | V   |      |     |     | V   |     |
| --- | --- | ---- | --- | --- | --- | --- |
|     |     | det0 |     |     |     | LVH |
VCC
V VBATH
V
DETBATT
V
POR
VBATT
V
VBATPOR
Internal reset signal
(active-low)
|                   |                                              | t   |                |     | t t          |                 |
| ----------------- | -------------------------------------------- | --- | -------------- | --- | ------------ | --------------- |
| Backup power area |                                              | det |                |     | det LVD0     |                 |
|                   | VCC supplied                                 |     | VBATT supplied |     | VCC supplied |                 |
| Figure 2.78       | Power supply switching and LVD0 reset timing |     |                |     |              |                 |
| R01DS0355EJ0110   | Rev.1.10                                     |     |                |     |              | Page 104 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |
| ----------- | --- | --- | --- | ----------------------------- |
VCC
|     | V   |     |     | V VBATH |
| --- | --- | --- | --- | ------- |
DETBATT
VBATT
V
VBATPOR
VBATT_POR
(active-low)
t
VBATPOR
Backup power area
|             | VCC supplied           | VBATT supplied | not supplied | VCC supplied |
| ----------- | ---------------------- | -------------- | ------------ | ------------ |
| Figure 2.79 | VBATT_POR reset timing |                |              |              |
|             | VBATT                  | V              | V VBATLVDTH  |              |
DETBATLVD
VBTCR2.VBTLVDEN
T
d_vbat
VBATT pin LVD
Comparator output
VBTCMPCR.VBTCMPE
VBTSR.VBTBLDF
t det_vbat t det_vbat
| Figure 2.80     | VBATT pin voltage detection circuit timing |     |     |                 |
| --------------- | ------------------------------------------ | --- | --- | --------------- |
| R01DS0355EJ0110 | Rev.1.10                                   |     |     | Page 105 of 131 |
Sep 29, 2023

| RA4M1 Group |                           |     |     |     |     |        |     |     |     | 2. Electrical Characteristics |                 |
| ----------- | ------------------------- | --- | --- | --- | --- | ------ | --- | --- | --- | ----------------------------- | --------------- |
| Table 2.59  | VBATT-I/O characteristics |     |     |     |     |        |     |     |     |                               |                 |
| Parameter   |                           |     |     |     |     | Symbol | Min | Typ | Max | Unit                          | Test conditions |
VBATWIOn I/O  VCC > V VCC = 4.0 to 5.5 V V VCC - 0.8 - - V I = -200 µA
|     |     | DETBATT |     |     |     | OH  |     |     |     |     | OH  |
| --- | --- | ------- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
output
|                           |                      |         |                      |         |           | V    | -             | -   | 0.8 |     | I = 200 µA     |
| ------------------------- | -------------------- | ------- | -------------------- | ------- | --------- | ---- | ------------- | --- | --- | --- | -------------- |
| characteristics           |                      |         |                      |         |           | OL   |               |     |     |     | OL             |
| (n = 0 to 2)              |                      |         | VCC = 2.7 to 4.0 V   |         |           | V    | VCC - 0.5     | -   | -   |     | I = -100 µA    |
|                           |                      |         |                      |         |           | OH   |               |     |     |     | OH             |
|                           |                      |         |                      |         |           | V    | -             | -   | 0.5 |     | I = 100 µA     |
|                           |                      |         |                      |         |           | OL   |               |     |     |     | OL             |
|                           |                      |         | VCC = V              |         |  to 2.7 V | V    | VCC - 0.3     | -   | -   |     | I = -50 µA     |
|                           |                      |         |                      | DETBATT |           | OH   |               |     |     |     | OH             |
|                           |                      |         |                      |         |           | V    | -             | -   | 0.3 |     | I = 50 µA      |
|                           |                      |         |                      |         |           | OL   |               |     |     |     | OL             |
|                           | VCC < V              |         | VBATT = 2.7 to 3.6 V |         |           | V    | V  - 0.5      | -   | -   |     | I = -100 µA    |
|                           |                      | DETBATT |                      |         |           | OH   | BATT          |     |     |     | OH             |
|                           |                      |         |                      |         |           | V OL | -             | -   | 0.5 |     | I OL  = 100 µA |
|                           |                      |         | VBATT = 1.6 to 2.7 V |         |           | V OH | V BATT  - 0.3 | -   | -   |     | I OH  = -50 µA |
|                           |                      |         |                      |         |           | V OL | -             | -   | 0.3 |     | I OL  = 50 µA  |
| 2.11 CTSU Characteristics |                      |         |                      |         |           |      |               |     |     |     |                |
| Table 2.60                | CTSU characteristics |         |                      |         |           |      |               |     |     |     |                |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V
| Parameter |     |     |     |     | Symbol | Min | Typ | Max |     | Unit | Test conditions |
| --------- | --- | --- | --- | --- | ------ | --- | --- | --- | --- | ---- | --------------- |
External capacitance connected to TSCAP pin C tscap 9 10 11 nF -
| TS pin capacitive load |     |     |     |     | C base | -   | -   | 50  |     | pF  | -   |
| ---------------------- | --- | --- | --- | --- | ------ | --- | --- | --- | --- | --- | --- |
Permissible output high current ΣIoH - - -24 mA When the mutual
capacitance method
is applied
| R01DS0355EJ0110 |     | Rev.1.10 |     |     |     |     |     |     |     |     | Page 106 of 131 |
| --------------- | --- | -------- | --- | --- | --- | --- | --- | --- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |
| ----------- | --- | --- | --- | --- | ----------------------------- |
2.12 Segment LCD Controller Characteristics
| 2.12.1 Resistance Division Method |     |     |     |     |     |
| --------------------------------- | --- | --- | --- | --- | --- |
[Static Display Mode]
| Table 2.61 Resistance division method LCD characteristics (1) |     |     |     |     |     |
| ------------------------------------------------------------- | --- | --- | --- | --- | --- |
Conditions: VL4 ≤ VCC ≤ 5.5 V
| Parameter         |     | Symbol | Min Typ | Max Unit | Test conditions |
| ----------------- | --- | ------ | ------- | -------- | --------------- |
| LCD drive voltage |     | V      | 2.0 -   | VCC V    | -               |
L4
[1/2 Bias Method, 1/4 Bias Method]
| Table 2.62 Resistance division method LCD characteristics (2) |     |     |     |     |     |
| ------------------------------------------------------------- | --- | --- | --- | --- | --- |
Conditions: VL4 ≤ VCC ≤ 5.5 V
| Parameter         |     | Symbol | Min Typ | Max Unit | Test conditions |
| ----------------- | --- | ------ | ------- | -------- | --------------- |
| LCD drive voltage |     | V L4   | 2.7 -   | VCC V    | -               |
[1/3 Bias Method]
| Table 2.63 Resistance division method LCD characteristics (3) |     |     |     |     |     |
| ------------------------------------------------------------- | --- | --- | --- | --- | --- |
Conditions: VL4 ≤ VCC ≤ 5.5 V
| Parameter                               |     | Symbol | Min Typ | Max Unit | Test conditions |
| --------------------------------------- | --- | ------ | ------- | -------- | --------------- |
| LCD drive voltage                       |     | V L4   | 2.5 -   | VCC V    | -               |
| 2.12.2 Internal Voltage Boosting Method |     |        |         |          |                 |
[1/3 Bias Method]
| Table 2.64 Internal voltage boosting method LCD characteristics |     |     |     |     |     |
| --------------------------------------------------------------- | --- | --- | --- | --- | --- |
Conditions: VCC = 1.8 V to 5.5 V
Test
| Parameter | Symbol Conditions |     | Min | Typ | Max Unit conditions |
| --------- | ----------------- | --- | --- | --- | ------------------- |
LCD output voltage  V C1 to C4*1 = 0.47 μF VLCD = 04h 0.90 1.0 1.08 V -
L1
variation range
|     |     | VLCD = 05h | 0.95 | 1.05 | 1.13 V - |
| --- | --- | ---------- | ---- | ---- | -------- |
|     |     | VLCD = 06h | 1.00 | 1.10 | 1.18 V - |
|     |     | VLCD = 07h | 1.05 | 1.15 | 1.23 V - |
|     |     | VLCD = 08h | 1.10 | 1.20 | 1.28 V - |
|     |     | VLCD = 09h | 1.15 | 1.25 | 1.33 V - |
|     |     | VLCD = 0Ah | 1.20 | 1.30 | 1.38 V - |
|     |     | VLCD = 0Bh | 1.25 | 1.35 | 1.43 V - |
|     |     | VLCD = 0Ch | 1.30 | 1.40 | 1.48 V - |
|     |     | VLCD = 0Dh | 1.35 | 1.45 | 1.53 V - |
|     |     | VLCD = 0Eh | 1.40 | 1.50 | 1.58 V - |
|     |     | VLCD = 0Fh | 1.45 | 1.55 | 1.63 V - |
|     |     | VLCD = 10h | 1.50 | 1.60 | 1.68 V - |
|     |     | VLCD = 11h | 1.55 | 1.65 | 1.73 V - |
|     |     | VLCD = 12h | 1.60 | 1.70 | 1.78 V - |
|     |     | VLCD = 13h | 1.65 | 1.75 | 1.83 V - |
C1 to C4*1 = 0.47 μF
Doubler output voltage V L2 2 × V L1  - 0.1 2 × V L1 2 × V L1 V -
| Tripler output voltage | V C1 to C4*1 = 0.47 μF |     |                  |          |                  |
| ---------------------- | ---------------------- | --- | ---------------- | -------- | ---------------- |
|                        | L4                     |     | 3 × V L1  - 0.15 | 3 × V L1 | 3 × V L1 V -     |
| Reference voltage      | t                      |     | 5                | -        | - ms Figure 2.81 |
VL1S
setup time*2
| LCD output voltage  | t C1 to C4*1 = 0.47 μF |     | 500 | -   | - ms |
| ------------------- | ---------------------- | --- | --- | --- | ---- |
VLWT
variation range*3
Note 1. This is a capacitor that is connected between voltage pins used to drive the LCD.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 107 of 131 |
| --------------- | -------- | --- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | ----------------------------- | --- |
C1: A capacitor connected between CAPH and CAPL
C2: A capacitor connected between VL1 and GND
C3: A capacitor connected between VL2 and GND
C4: A capacitor connected between VL4 and GND
C1 = C2 = C3 = C4 = 0.47 μF ±30%.
Note 2. This is the time required to wait from when the reference voltage is specified using the VLCD register (or when the internal
voltage boosting method is selected (by setting the MDSET[1:0] bits in the LCDM0 register to 01b) if the default value reference
voltage is used) until voltage boosting starts (VLCON = 1).
Note 3. This is the wait time from when voltage boosting is started (VLCON = 1) until display is enabled (LCDON = 1).
[1/4 Bias Method]
| Table 2.65 Internal voltage boosting method LCD characteristics |     |     |     |     |     |
| --------------------------------------------------------------- | --- | --- | --- | --- | --- |
Conditions: VCC = 1.8 V to 5.5 V
Test
| Parameter | Symbol Conditions |     | Min | Typ Max | Unit conditions |
| --------- | ----------------- | --- | --- | ------- | --------------- |
LCD output voltage  V C1 to C5*1 = 0.47 μF VLCD = 04h 0.90 1.0 1.08 V -
L1
variation range
|     |     | VLCD = 05h | 0.95 | 1.05 1.13 | V - |
| --- | --- | ---------- | ---- | --------- | --- |
|     |     | VLCD = 06h | 1.00 | 1.10 1.18 | V - |
|     |     | VLCD = 07h | 1.05 | 1.15 1.23 | V - |
|     |     | VLCD = 08h | 1.10 | 1.20 1.28 | V - |
|     |     | VLCD = 09h | 1.15 | 1.25 1.33 | V - |
|     |     | VLCD = 0Ah | 1.20 | 1.30 1.38 | V - |
|     |     | VLCD = 0Bh | 1.25 | 1.35 1.43 | V - |
|     |     | VLCD = 0Ch | 1.30 | 1.40 1.48 | V - |
C1 to C5*1 = 0.47 μF
| Doubler output voltage | V L2 |     | 2V L1  - 0.08 | 2V L1 2V L1 | V - |
| ---------------------- | ---- | --- | ------------- | ----------- | --- |
C1 to C5*1 = 0.47 μF
| Tripler output voltage | V L3 |     | 3V L1  - 0.12 | 3V L1 3V L1 | V - |
| ---------------------- | ---- | --- | ------------- | ----------- | --- |
Quadruply output  V *4 C1 to C5*1 = 0.47 μF 4V  - 0.16 4V 4V V -
|     | L4  |     | L1  | L1 L1 |     |
| --- | --- | --- | --- | ----- | --- |
voltage
| Reference voltage  | t   |     | 5   | - - | ms Figure 2.81 |
| ------------------ | --- | --- | --- | --- | -------------- |
VL1S
setup time*2
| LCD output voltage  | t C1 to C5*1 = 0.47 μF |     | 500 | - - | ms  |
| ------------------- | ---------------------- | --- | --- | --- | --- |
VLWT
variation range*3
Note 1. This is a capacitor that is connected between voltage pins used to drive the LCD.
C1: A capacitor connected between CAPH and CAPL
C2: A capacitor connected between V  and GND
L1
C3: A capacitor connected between V  and GND
L2
C4: A capacitor connected between V L3  and GND
C5: A capacitor connected between V and GND
L4
C1 = C2 = C3 = C4 = C5 = 0.47 μF ± 30%
Note 2. This is the time required to wait from when the reference voltage is specified by using the VLCD register (or when the internal
voltage boosting method is selected (by setting the MDSET1 and MDSET0 bits in the LCDM0 register to 01b) if the default
value reference voltage is used) until voltage boosting starts (VLCON = 1).
Note 3. This is the wait time from when voltage boosting is started (VLCON = 1) until display is enabled (LCDON = 1).
| Note 4. V  must be 5.5 V or lower. |     |     |     |     |     |
| ---------------------------------- | --- | --- | --- | --- | --- |
L4
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 108 of 131 |
| --------------- | -------- | --- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group                   |     |     | 2. Electrical Characteristics |     |
| ----------------------------- | --- | --- | ----------------------------- | --- |
| 2.12.3 Capacitor Split Method |     |     |                               |     |
[1/3 Bias Method]
| Table 2.66 Internal voltage boosting method LCD characteristics |     |     |     |     |
| --------------------------------------------------------------- | --- | --- | --- | --- |
Conditions: VCC = 2.2 V to 5.5 V
Test
| Parameter     | Symbol Conditions      | Min | Typ Max | Unit conditions |
| ------------- | ---------------------- | --- | ------- | --------------- |
| VL4 voltage*1 | V C1 to C4 = 0.47 μF*2 | -   | VCC -   | V -             |
L4
VL2 voltage*1 V C1 to C4 = 0.47 μF*2 2/3 × V  - 0.07 2/3 × V 2/3 × V  + 0.07 V -
|     | L2  | L4  | L4 L4 |     |
| --- | --- | --- | ----- | --- |
VL1 voltage*1 V C1 to C4 = 0.47 μF*2 1/3 × V  - 0.08 1/3 × V 1/3 × V  + 0.08 V -
|                             | L1  | L4  | L4 L4 |                |
| --------------------------- | --- | --- | ----- | -------------- |
| Capacitor split wait time*1 | t   | 100 | - -   | ms Figure 2.81 |
WAIT
Note 1. This is the wait time from when voltage bucking is started (VLCON = 1) until display is enabled (LCDON = 1).
Note 2. This is a capacitor that is connected between voltage pins used to drive the LCD.
C1: A capacitor connected between CAPH and CAPL
C2: A capacitor connected between V L1  and GND
C3: A capacitor connected between V  and GND
L2
C4: A capacitor connected between V  and GND
L4
C1 = C2 = C3 = C4 = 0.47 μF ± 30%.
MDSET0,
00b 01b or 10b
MDSET1
t
|     | VLCON VL1S |     |     |     |
| --- | ---------- | --- | --- | --- |
t t
VLWT, WAIT
LCDON
Figure 2.81 LCD reference voltage setup time, voltage boosting wait time, and capacitor split wait time
| R01DS0355EJ0110 | Rev.1.10 |     |     | Page 109 of 131 |
| --------------- | -------- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |
| ----------- | --- | --- | --- | --- | ----------------------------- |
2.13 Comparator Characteristics
| Table 2.67 ACMPLP characteristics |     |     |     |     |     |
| --------------------------------- | --- | --- | --- | --- | --- |
Conditions: VCC = 1.8 to 5.5 V
| Parameter |     |     | Symbol Min | Typ Max | Unit Test conditions |
| --------- | --- | --- | ---------- | ------- | -------------------- |
Reference voltage range Standard  IVREFn (n= 0,1) VREF 0 - VCC-1.4 V -
mode
|     | Window  | IVREF1 | VREFH 1.4 | - VCC | V - |
| --- | ------- | ------ | --------- | ----- | --- |
mode*2
|                            |                 | IVREF0 | VREFL 0 | - VCC-1.4 | V -          |
| -------------------------- | --------------- | ------ | ------- | --------- | ------------ |
| Input voltage range        |                 |        | VI 0    | - VCC     | V -          |
| Internal reference voltage |                 |        | - 1.36  | 1.44 1.50 | V -          |
| Output delay               | High-speed mode |        | Td -    | - 1.2     | μs VCC = 3.0 |
Slew rate of input
|     | Low-speed mode |     | -   | - 5 | μs  |
| --- | -------------- | --- | --- | --- | --- |
signal > 50 mV/μs
|                                   | Window mode     |     | -     | - 2  | μs   |
| --------------------------------- | --------------- | --- | ----- | ---- | ---- |
| Offset voltage*1                  | High-speed mode |     | - -   | - 50 | mV - |
|                                   | Low-speed mode  |     | - -   | - 40 | mV - |
|                                   | Window mode     |     | - -   | - 60 | mV - |
| Operation stabilization wait time |                 |     | T 100 | - -  | μs - |
cmp
Note 1. When 8-bit DAC output is used as the reference voltage, the offset voltage increases up to 2.5 x VCC/256.
Note 2. In window mode, be sure to satisfy the following condition: IVREF1 - IVREF0 ≥ 0.2 V.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 110 of 131 |
| --------------- | -------- | --- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group |     |     | 2. Electrical Characteristics |     |     |
| ----------- | --- | --- | ----------------------------- | --- | --- |
2.14 OPAMP Characteristics
| Table 2.68 OPAMP characteristics |     |     |     |     |     |
| -------------------------------- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 1.8 to 5.5 V (AVCC0 = VCC when VCC < 2.0 V)
| Parameter          | Symbol Conditions    |     | Min Typ | Max         | Unit |
| ------------------ | -------------------- | --- | ------- | ----------- | ---- |
| Common mode input  | Vicm1 Low power mode |     | 0.2 -   | AVCC0 - 0.5 | V    |
range
|                      | Vicm2 High-speed mode |     | 0.3 -  | AVCC0 - 0.6 | V   |
| -------------------- | --------------------- | --- | ------ | ----------- | --- |
| Output voltage range | Vo1 Low power mode    |     | 0.1 -  | AVCC0 - 0.1 | V   |
|                      | Vo2 High-speed mode   |     | 0.1 -  | AVCC0 - 0.1 | V   |
| Input offset voltage | Vioff 3σ              |     | -10 -  | 10          | mV  |
| Open gain            | Av                    |     | 60 120 | -           | dB  |
| Gain-bandwidth (GB)  | GBW1 Low power mode   |     | - 0.04 | -           | MHz |
product
|              | GBW2 High-speed mode |     | - 1.7 | -   | MHz |
| ------------ | -------------------- | --- | ----- | --- | --- |
| Phase margin | PM CL = 20 pF        |     | 50 -  | -   | deg |
| Gain margin  | GM CL = 20 pF        |     | 10 -  | -   | dB  |
Equivalent input noise Vnoise1 f = 1 kHz Low power mode - 230 - nV/√Hz
|               | Vnoise2 f = 10 kHz |                 | - 200 | -   | nV/√Hz |
| ------------- | ------------------ | --------------- | ----- | --- | ------ |
|               | Vnoise3 f = 1 kHz  | High-speed mode | - 90  | -   | nV/√Hz |
|               | Vnoise4 f = 2 kHz  |                 | - 70  | -   | nV/√Hz |
| Power supply  | PSRR               |                 | - 90  | -   | dB     |
reduction ratio
| Common mode signal  | CMRR |     | - 90 | -   | dB  |
| ------------------- | ---- | --- | ---- | --- | --- |
reduction ratio
Stabilization wait time Tstd1 CL = 20 pF Low power mode 650 - - μs
Only operational amplifier is
|     | Tstd2 activated *1 | High-speed mode | 13 -  | -   | μs  |
| --- | ------------------ | --------------- | ----- | --- | --- |
|     | Tstd3 CL = 20 pF   | Low power mode  | 650 - | -   | μs  |
Operational amplifier and
Tstd4 reference current circuit are  High-speed mode 13 - - μs
activated simultaneously
| Settling time    | Tset1 CL = 20 pF       | Low power mode  | - -    | 750 | μs   |
| ---------------- | ---------------------- | --------------- | ------ | --- | ---- |
|                  | Tset2                  | High-speed mode | - -    | 13  | μs   |
| Slew rate        | Tslew1 CL = 20 pF      | Low power mode  | - 0.02 | -   | V/μs |
|                  | Tslew2                 | High-speed mode | - 1.1  | -   | V/μs |
| Load current     | Iload1 Low-power mode  |                 | -100 - | 100 | μA   |
|                  | Iload2 High-speed mode |                 | -100 - | 100 | μA   |
| Load capacitance | CL                     |                 | - -    | 20  | pF   |
Note 1. When the operational amplifier reference current circuit is activated in advance.
| R01DS0355EJ0110 | Rev.1.10 |     |     | Page 111 of 131 |     |
| --------------- | -------- | --- | --- | --------------- | --- |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |
| ----------- | --- | --- | --- | --- | ----------------------------- |
2.15 Flash Memory Characteristics
| 2.15.1 Code Flash Memory Characteristics  |     |        |      |         |                      |
| ----------------------------------------- | --- | ------ | ---- | ------- | -------------------- |
| Table 2.69 Code flash characteristics (1) |     |        |      |         |                      |
| Parameter                                 |     | Symbol | Min  | Typ Max | Unit Test conditions |
| Reprogramming/erasure cycle*1             |     | N      | 1000 | - -     | Times -              |
PEC
Data hold time After 1000 times of N t 20*2, *3 - - Year T  = +85°C
PEC DRP a
Note 1. The reprogram/erase cycle is the number of erasures for each block. When the reprogram/erase cycle is n times (n = 1,000),
erasing can be done n times for each block. For instance, when 8-byte programming is performed 256 times for different
addresses in 2-KB blocks, and then the entire block is erased, the reprogram/erase cycle is counted as one. However,
programming the same address for several times as one erasure is not enabled (overwriting is prohibited).
Note 2. Characteristic when using the flash memory programmer and the self-programming library provided by Renesas Electronics.
Note 3. This result is obtained from reliability testing.
| Table 2.70 Code flash characteristics (2) |     |     |     |     |     |
| ----------------------------------------- | --- | --- | --- | --- | --- |
High-speed operating mode
Conditions: VCC = 2.7 to 5.5 V
|                  |        |            | FCLK = 1 MHz | FCLK = 32 MHz |              |
| ---------------- | ------ | ---------- | ------------ | ------------- | ------------ |
| Parameter        |        | Symbol Min | Typ          | Max Min       | Typ Max Unit |
| Programming time | 8-byte | t -        | 116          | 998 -         | 54 506 μs    |
P8
| Erasure time | 2-KB | t - | 9.03 | 287 - | 5.67 222 ms |
| ------------ | ---- | --- | ---- | ----- | ----------- |
E2K
| Blank check time | 8-byte | t - | -   | 56.8 - | - 16.6 μs |
| ---------------- | ------ | --- | --- | ------ | --------- |
BC8
|     | 2-KB | t - | -   | 1899 - | - 140 μs |
| --- | ---- | --- | --- | ------ | -------- |
BC2K
| Erase suspended time |     | t - | -   | 22.5 - | - 10.7 μs |
| -------------------- | --- | --- | --- | ------ | --------- |
SED
Startup area switching setting time t - 21.7 585 - 12.1 447 ms
SAS
| Access window time |     | t - | 21.7 | 585 - | 12.1 447 ms |
| ------------------ | --- | --- | ---- | ----- | ----------- |
AWS
OCD/serial programmer ID setting time t - 21.7 585 - 12.1 447 ms
OSIS
| Flash memory mode transition wait  |     | t DIS 2 | -   | - 2 | - - μs |
| ---------------------------------- | --- | ------- | --- | --- | ------ |
time 1
| Flash memory mode transition wait  |     | t MS 5 | -   | - 5 | - - μs |
| ---------------------------------- | --- | ------ | --- | --- | ------ |
time 2
Note: Does not include the time until each operation of the flash memory is started after instructions are executed by software.
Note: The lower-limit frequency of FCLK is 1 MHz during programming or erasing the flash memory. When using FCLK at below
4MHz, the frequency can be set to 1 MHz, 2 MHz, or 3 MHz. A non-integer frequency such as 1.5 MHz cannot be set.
Note: The frequency accuracy of FCLK must be ±3.5%. Confirm the frequency accuracy of the clock source.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     | Page 112 of 131 |
| --------------- | -------- | --- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group |                                |     |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | ------------------------------ | --- | --- | --- | --- | --- | ----------------------------- | --- |
| Table 2.71  | Code flash characteristics (3) |     |     |     |     |     |                               |     |
Middle-speed operating mode
Conditions: VCC = 1.8 to 5.5 V, T = -40 to +85°C
a
|                  |        |        |     | FCLK = 1 MHz |      |     | FCLK = 8 MHz |          |
| ---------------- | ------ | ------ | --- | ------------ | ---- | --- | ------------ | -------- |
| Parameter        |        | Symbol | Min | Typ          | Max  | Min | Typ          | Max Unit |
| Programming time | 8-byte | t      | -   | 157          | 1411 | -   | 101          | 966 μs   |
P8
| Erasure time | 2-KB | t   | -   | 9.10 | 289 | -   | 6.10 | 228 ms |
| ------------ | ---- | --- | --- | ---- | --- | --- | ---- | ------ |
E2K
| Blank check time | 8-byte | t   | -   | -   | 87.7 | -   | -   | 52.5 μs |
| ---------------- | ------ | --- | --- | --- | ---- | --- | --- | ------- |
BC8
|     | 2-KB | t   | -   | -   | 1930 | -   | -   | 414 μs |
| --- | ---- | --- | --- | --- | ---- | --- | --- | ------ |
BC2K
| Erase suspended time |     | t   | -   | -   | 32.7 | -   | -   | 21.6 μs |
| -------------------- | --- | --- | --- | --- | ---- | --- | --- | ------- |
SED
Startup area switching setting time t - 22.5 592 - 14.0 464 ms
SAS
| Access window time |     | t AWS | -   | 22.5 | 592 | -   | 14.0 | 464 ms |
| ------------------ | --- | ----- | --- | ---- | --- | --- | ---- | ------ |
OCD/serial programmer ID setting time t OSIS - 22.5 592 - 14.0 464 ms
Flash memory mode transition wait time 1 t DIS 2 - - 2 - -μ s
Flash memory mode transition wait time 2 t MS 720 - - 720 - - ns
Note: Does not include the time until each operation of the flash memory is started after instructions are executed by software.
Note: The lower-limit frequency of FCLK is 1 MHz during programming or erasing the flash memory. When using FCLK at below
4MHz, the frequency can be set to 1 MHz, 2 MHz, or 3 MHz. A non-integer frequency such as 1.5 MHz cannot be set.
Note: The frequency accuracy of FCLK must be ±3.5%. Confirm the frequency accuracy of the clock source.
| 2.15.2     | Data Flash Memory Characteristics |     |        |     |     |     |      |                 |
| ---------- | --------------------------------- | --- | ------ | --- | --- | --- | ---- | --------------- |
| Table 2.72 | Data flash characteristics (1)    |     |        |     |     |     |      |                 |
| Parameter  |                                   |     | Symbol | Min | Typ | Max | Unit | Test conditions |
Reprogramming/erasure cycle*1 N DPEC 100,000 1,000,000 - Times -
Data hold time After 10,000 times of N DPEC t DDRP 20*2, *3 - - Year T = +85°C a
5*2, *3
|     | After 100,000 times of N | DPEC |     |     | -   | -   | Year |     |
| --- | ------------------------ | ---- | --- | --- | --- | --- | ---- | --- |
1*2, *3
|     | After 1,000,000 times of  |     |     | -   |     | -   | Year | T = +25°C a |
| --- | ------------------------- | --- | --- | --- | --- | --- | ---- | ----------- |
N DPEC
Note 1. The reprogram/erase cycle is the number of erasure for each block. When the reprogram/erase cycle is n times (n = 100,000),
erasing can be performed n times for each block. For instance, when 1-byte programming is performed 1,000 times for different
addresses in 1-byte blocks, and then the entire block is erased, the reprogram/erase cycle is counted as one. However,
programming the same address for several times as one erasure is not enabled. Overwriting is prohibited.
Note 2. Characteristics when using the flash memory programmer and the self-programming library provided by Renesas Electronics.
| Note 3. These results are obtained from reliability testing. |                                |     |     |     |     |     |     |     |
| ------------------------------------------------------------ | ------------------------------ | --- | --- | --- | --- | --- | --- | --- |
| Table 2.73                                                   | Data flash characteristics (2) |     |     |     |     |     |     |     |
High-speed operating mode
Conditions: VCC = 2.7 to 5.5 V
|                  |        |        |     | FCLK = 4 MHz |      |     | FCLK = 32 MHz |          |
| ---------------- | ------ | ------ | --- | ------------ | ---- | --- | ------------- | -------- |
| Parameter        |        | Symbol | Min | Typ          | Max  | Min | Typ           | Max Unit |
| Programming time | 1-byte | t DP1  | -   | 52.4         | 463  | -   | 42.1          | 387 μs   |
| Erasure time     | 1-KB   | t DE1K | -   | 8.98         | 286  | -   | 6.42          | 237 ms   |
| Blank check time | 1-byte | t DBC1 | -   | -            | 24.3 | -   | -             | 16.6 μs  |
|                  | 1-KB   | t      | -   | -            | 1872 | -   | -             | 512 μs   |
DBC1K
| Suspended time during erasing |     | t   | -   | -   | 13.0 | -   | -   | 10.7 μs |
| ----------------------------- | --- | --- | --- | --- | ---- | --- | --- | ------- |
DSED
| Data flash STOP recovery time |     | t   | 5   | -   | -   | 5   | -   | - μs |
| ----------------------------- | --- | --- | --- | --- | --- | --- | --- | ---- |
DSTOP
Note: Does not include the time until each operation of the flash memory is started after instructions are executed by software.
Note: The lower-limit frequency of FCLK is 1 MHz during programming or erasing the flash memory. When using FCLK at below
4MHz, the frequency can be set to 1 MHz, 2 MHz, or 3 MHz. A non-integer frequency such as 1.5 MHz cannot be set.
Note: The frequency accuracy of FCLK must be ±3.5%. Confirm the frequency accuracy of the clock source.
| R01DS0355EJ0110 | Rev.1.10 |     |     |     |     |     |     | Page 113 of 131 |
| --------------- | -------- | --- | --- | --- | --- | --- | --- | --------------- |
Sep 29, 2023

| RA4M1 Group                               |     |     |     |     | 2. Electrical Characteristics |     |
| ----------------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| Table 2.74 Data flash characteristics (3) |     |     |     |     |                               |     |
Middle-speed operating mode
Conditions: VCC = 1.8 to 5.5 V, T = -40 to +85°C
a
|                  |        |            | FCLK = 4 MHz |     | FCLK = 8 MHz |          |
| ---------------- | ------ | ---------- | ------------ | --- | ------------ | -------- |
| Parameter        |        | Symbol Min | Typ Max      | Min | Typ          | Max Unit |
| Programming time | 1-byte | t -        | 94.7 886     | -   | 89.3         | 849 μs   |
DP1
| Erasure time | 1-KB | t - | 9.59 299 | -   | 8.29 | 273 ms |
| ------------ | ---- | --- | -------- | --- | ---- | ------ |
DE1K
| Blank check time | 1-byte | t - | - 56.2 | -   | -   | 52.5 μs |
| ---------------- | ------ | --- | ------ | --- | --- | ------- |
DBC1
|     | 1-KB | t - | - 2.17 | -   | -   | 1.51 ms |
| --- | ---- | --- | ------ | --- | --- | ------- |
DBC1K
| Suspended time during erasing |     | t - | - 23.0 | -   | -   | 21.7 μs |
| ----------------------------- | --- | --- | ------ | --- | --- | ------- |
DSED
| Data flash STOP recovery time |     | t 720 | - - | 720 | -   | - ns |
| ----------------------------- | --- | ----- | --- | --- | --- | ---- |
DSTOP
Note: Does not include the time until each operation of the flash memory is started after instructions are executed by software.
Note: The lower-limit frequency of FCLK is 1 MHz during programming or erasing the flash memory. When using FCLK at below
4MHz, the frequency can be set to 1 MHz, 2 MHz, or 3 MHz. A non-integer frequency such as 1.5 MHz cannot be set.
Note: The frequency accuracy of FCLK must be ±3.5%. Confirm the frequency accuracy of the clock source.
2.16 Boundary Scan
| Table 2.75 Boundary scan |     |     |     |     |     |     |
| ------------------------ | --- | --- | --- | --- | --- | --- |
Conditions: VCC = AVCC0 = 2.4 to 5.5 V
| Parameter            |     | Symbol Min | Typ | Max | Unit | Test conditions |
| -------------------- | --- | ---------- | --- | --- | ---- | --------------- |
| TCK clock cycle time |     | t 100      | -   | -   | ns   | Figure 2.82     |
TCKcyc
| TCK clock high pulse width |     | t 45 | -   | -   | ns  |     |
| -------------------------- | --- | ---- | --- | --- | --- | --- |
TCKH
| TCK clock low pulse width |     | t 45 | -   | -   | ns  |     |
| ------------------------- | --- | ---- | --- | --- | --- | --- |
TCKL
| TCK clock rise time |     | t - | -   | 5   | ns  |     |
| ------------------- | --- | --- | --- | --- | --- | --- |
TCKr
| TCK clock fall time |     | t TCKf -  | -   | 5   | ns  |             |
| ------------------- | --- | --------- | --- | --- | --- | ----------- |
| TMS setup time      |     | t TMSS 20 | -   | -   | ns  | Figure 2.83 |
| TMS hold time       |     | t TMSH 20 | -   | -   | ns  |             |
| TDI setup time      |     | t TDIS 20 | -   | -   | ns  |             |
| TDI hold time       |     | t 20      | -   | -   | ns  |             |
TDIH
| TDO data delay |     | t - | -   | 70  | ns  |     |
| -------------- | --- | --- | --- | --- | --- | --- |
TDOD
| Boundary Scan circuit start up time*1 |     | t t           | -   | -   | -Figure 2.84 |     |
| ------------------------------------- | --- | ------------- | --- | --- | ------------ | --- |
|                                       |     | BSSTUP RESWP  |     |     |              |     |
Note 1. Boundary scan does not function until power-on-reset becomes negative.
t TCKcyc
t
TCKH
|     | TCK |     | t TCKf |     |     |     |
| --- | --- | --- | ------ | --- | --- | --- |
t
TCKr
t TCKL
| Figure 2.82     | Boundary scan TCK timing |     |     |     |     |                 |
| --------------- | ------------------------ | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 | Rev.1.10                 |     |     |     |     | Page 114 of 131 |
Sep 29, 2023

| RA4M1 Group |     |     |     |     | 2. Electrical Characteristics |     |
| ----------- | --- | --- | --- | --- | ----------------------------- | --- |
TCK
tTMSS tTMSH
TMS
tTDIS tTDIH
TDI
tTDOD
TDO
| Figure 2.83 | Boundary scan input/output timing |     |     |     |     |     |
| ----------- | --------------------------------- | --- | --- | --- | --- | --- |
VCC
RES
t
BSSTUP Boundary scan
(= t )
RESWP execute
| Figure 2.84 | Boundary scan circuit start up timing |     |     |     |     |     |
| ----------- | ------------------------------------- | --- | --- | --- | --- | --- |
2.17 Joint Test Action Group (JTAG)
| Table 2.76 JTAG (debug) characteristics (1) |     |     |     |     |     |     |
| ------------------------------------------- | --- | --- | --- | --- | --- | --- |
Conditions: VCC = 2.4 to 5.5 V
| Parameter            |     | Symbol Min | Typ | Max | Unit | Test conditions |
| -------------------- | --- | ---------- | --- | --- | ---- | --------------- |
| TCK clock cycle time |     | t 80       | -   | -   | ns   | Figure 2.85     |
TCKcyc
| TCK clock high pulse width |     | t 35 | -   | -   | ns  |     |
| -------------------------- | --- | ---- | --- | --- | --- | --- |
TCKH
| TCK clock low pulse width |     | t 35 | -   | -   | ns  |     |
| ------------------------- | --- | ---- | --- | --- | --- | --- |
TCKL
| TCK clock rise time |     | t - | -   | 5   | ns  |     |
| ------------------- | --- | --- | --- | --- | --- | --- |
TCKr
| TCK clock fall time |     | t - | -   | 5   | ns  |     |
| ------------------- | --- | --- | --- | --- | --- | --- |
TCKf
| TMS setup time |     | t 16 | -   | -   | ns  | Figure 2.86 |
| -------------- | --- | ---- | --- | --- | --- | ----------- |
TMSS
| TMS hold time |     | t 16 | -   | -   | ns  |     |
| ------------- | --- | ---- | --- | --- | --- | --- |
TMSH
| TDI setup time |     | t 16 | -   | -   | ns  |     |
| -------------- | --- | ---- | --- | --- | --- | --- |
TDIS
| TDI hold time       |          | t TDIH 16 | -   | -   | ns  |                 |
| ------------------- | -------- | --------- | --- | --- | --- | --------------- |
| TDO data delay time |          | t TDOD -  | -   | 70  | ns  |                 |
| R01DS0355EJ0110     | Rev.1.10 |           |     |     |     | Page 115 of 131 |
Sep 29, 2023

| RA4M1 Group                                 |     |     |     |     | 2. Electrical Characteristics |     |
| ------------------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| Table 2.77 JTAG (debug) characteristics (2) |     |     |     |     |                               |     |
Conditions: VCC = 1.6 to 2.4 V
| Parameter            |     | Symbol Min | Typ | Max | Unit | Test conditions |
| -------------------- | --- | ---------- | --- | --- | ---- | --------------- |
| TCK clock cycle time |     | t 250      | -   | -   | ns   | Figure 2.85     |
TCKcyc
| TCK clock high pulse width |     | t 120 | -   | -   | ns  |     |
| -------------------------- | --- | ----- | --- | --- | --- | --- |
TCKH
| TCK clock low pulse width |     | t 120 | -   | -   | ns  |     |
| ------------------------- | --- | ----- | --- | --- | --- | --- |
TCKL
| TCK clock rise time |     | t - | -   | 5   | ns  |     |
| ------------------- | --- | --- | --- | --- | --- | --- |
TCKr
| TCK clock fall time |     | t - | -   | 5   | ns  |     |
| ------------------- | --- | --- | --- | --- | --- | --- |
TCKf
| TMS setup time |     | t 50 | -   | -   | ns  | Figure 2.86 |
| -------------- | --- | ---- | --- | --- | --- | ----------- |
TMSS
| TMS hold time |     | t 50 | -   | -   | ns  |     |
| ------------- | --- | ---- | --- | --- | --- | --- |
TMSH
| TDI setup time      |     | t TDIS 50 | -   | -   | ns  |     |
| ------------------- | --- | --------- | --- | --- | --- | --- |
| TDI hold time       |     | t TDIH 50 | -   | -   | ns  |     |
| TDO data delay time |     | t TDOD -  | -   | 150 | ns  |     |
t TCKcyc
t TCKH
TCK
t TCKf
t
|     |     |     | t   | TCKr |     |     |
| --- | --- | --- | --- | ---- | --- | --- |
TCKL
| Figure 2.85 | JTAG TCK timing |     |     |     |     |     |
| ----------- | --------------- | --- | --- | --- | --- | --- |
TCK
t t
TMSS TMSH
TMS
t t
TDIS TDIH
TDI
t
TDOD
TDO
| Figure 2.86     | JTAG input/output timing |     |     |     |     |                 |
| --------------- | ------------------------ | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 | Rev.1.10                 |     |     |     |     | Page 116 of 131 |
Sep 29, 2023

| RA4M1 Group                        |     |     |     |     | 2. Electrical Characteristics |     |
| ---------------------------------- | --- | --- | --- | --- | ----------------------------- | --- |
| 2.17.1 Serial Wire Debug (SWD)     |     |     |     |     |                               |     |
| Table 2.78 SWD characteristics (1) |     |     |     |     |                               |     |
Conditions: VCC = 2.4 to 5.5 V
| Parameter              |     | Symbol Min | Typ | Max | Unit | Test conditions |
| ---------------------- | --- | ---------- | --- | --- | ---- | --------------- |
| SWCLK clock cycle time |     | t 80       | -   | -   | ns   | Figure 2.87     |
SWCKcyc
| SWCLK clock high pulse width |     | t 35 | -   | -   | ns  |     |
| ---------------------------- | --- | ---- | --- | --- | --- | --- |
SWCKH
| SWCLK clock low pulse width |     | t 35 | -   | -   | ns  |     |
| --------------------------- | --- | ---- | --- | --- | --- | --- |
SWCKL
| SWCLK clock rise time |     | t - | -   | 5   | ns  |     |
| --------------------- | --- | --- | --- | --- | --- | --- |
SWCKr
| SWCLK clock fall time |     | t - | -   | 5   | ns  |     |
| --------------------- | --- | --- | --- | --- | --- | --- |
SWCKf
| SWDIO setup time |     | t 16 | -   | -   | ns  | Figure 2.88 |
| ---------------- | --- | ---- | --- | --- | --- | ----------- |
SWDS
| SWDIO hold time |     | t 16 | -   | -   | ns  |     |
| --------------- | --- | ---- | --- | --- | --- | --- |
SWDH
| SWDIO data delay time              |     | t SWDD 2 | -   | 70  | ns  |     |
| ---------------------------------- | --- | -------- | --- | --- | --- | --- |
| Table 2.79 SWD characteristics (2) |     |          |     |     |     |     |
Conditions: VCC = 1.6 to 2.4 V
| Parameter              |     | Symbol Min | Typ | Max | Unit | Test conditions |
| ---------------------- | --- | ---------- | --- | --- | ---- | --------------- |
| SWCLK clock cycle time |     | t 250      | -   | -   | ns   | Figure 2.87     |
SWCKcyc
| SWCLK clock high pulse width |     | t 120 | -   | -   | ns  |     |
| ---------------------------- | --- | ----- | --- | --- | --- | --- |
SWCKH
| SWCLK clock low pulse width |     | t 120 | -   | -   | ns  |     |
| --------------------------- | --- | ----- | --- | --- | --- | --- |
SWCKL
| SWCLK clock rise time |     | t SWCKr - | -   | 5   | ns  |             |
| --------------------- | --- | --------- | --- | --- | --- | ----------- |
| SWCLK clock fall time |     | t SWCKf - | -   | 5   | ns  |             |
| SWDIO setup time      |     | t SWDS 50 | -   | -   | ns  | Figure 2.88 |
| SWDIO hold time       |     | t SWDH 50 | -   | -   | ns  |             |
| SWDIO data delay time |     | t 2       | -   | 150 | ns  |             |
SWDD
t SWCKcyc
t SWCKH
|     | SWCLK |     | t   |     |     |     |
| --- | ----- | --- | --- | --- | --- | --- |
SWCKf
t
|     |     |     | t   | SWCKr |     |     |
| --- | --- | --- | --- | ----- | --- | --- |
SWCKL
| Figure 2.87     | SWD SWCLK timing |     |     |     |     |                 |
| --------------- | ---------------- | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 | Rev.1.10         |     |     |     |     | Page 117 of 131 |
Sep 29, 2023

RA4M1 Group 2. Electrical Characteristics
SWCLK
t t
SWDS SWDH
SWDIO
(Input)
t
SWDD
SWDIO
(Output)
t
SWDD
SWDIO
(Output)
t
SWDD
SWDIO
(Output)
Figure 2.88 SWD input/output timing
R01DS0355EJ0110 Rev.1.10 Page 118 of 131
Sep 29, 2023

| RA4M1 Group |     |     |     |     | Appendix 1. Package Dimensions  |     |
| ----------- | --- | --- | --- | --- | ------------------------------- | --- |
Appendix 1.Package Dimensions
Information on the latest version of the package dimensions or mountings is shown in “Packages” on the Renesas
Electronics Corporation website.
| JEITA Package Code  | RENESAS Code | Previous Code | MASS[Typ.] |     |     |     |
| ------------------- | ------------ | ------------- | ---------- | --- | --- | --- |
| P-TFLGA100-7x7-0.65 | PTLG0100JA-A | 100F0G        | 0.1g       |     |     |     |
φ b1
φ×M S AB
B
|     | D   |       |     | φ b | φ×M S AB |     |
| --- | --- | ----- | --- | --- | -------- | --- |
| S   |     | w S A |     |     |          |     |
| w   |     |       |     | ZD  | e        |     |
|     |     |       | A   |     | A        |     |
e
K
J
H
G
B
F
E
E
D
C
B
|     |     |     |     | A   | EZ  | ReferenceDimension in Millimeters |
| --- | --- | --- | --- | --- | --- | --------------------------------- |
Symbol Min Nom Max
|     | ×4  |     | y S | 1 2 3 4 5 6 | 7 8 9 10 | D 7.0 |
| --- | --- | --- | --- | ----------- | -------- | ----- |
v
|     |     | Index mark | S   | Index mark |     | E 7.0 |
| --- | --- | ---------- | --- | ---------- | --- | ----- |
(Laser mark)
v 0.15
w 0.20
A 1.05
e 0.65
b 0.31 0.35 0.39
b1 0.3850.4350.485
x 0.08
y 0.10
ZD 0.575
ZE 0.575
| Figure 1.1      | 100-pin LGA  |     |     |     |     |                 |
| --------------- | ------------ | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 | Rev.1.10     |     |     |     |     | Page 119 of 131 |
Sep 29, 2023

| RA4M1 Group |                       |     |              |     |               | Appendix 1. Package Dimensions  |     |
| ----------- | --------------------- | --- | ------------ | --- | ------------- | ------------------------------- | --- |
|             | JEITA Package Code    |     | RENESAS Code |     | Previous Code | MASS (Typ) [g]                  |     |
|             | P-LFQFP100-14x14-0.50 |     | PLQP0100KB-B |     | —             |                                 | 0.6 |
HD
Unit: mm
*1
D
|     | 75  |     | 51  |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- |
|     | 76  |     |     | 50  |     |     |     |
E EH
2*
100
26
|     | 1          |     | 25  |        |                                                         |     |     |
| --- | ---------- | --- | --- | ------ | ------------------------------------------------------- | --- | --- |
|     | Index area |     |     | NOTE 4 | NOTE)                                                   |     |     |
|     | NOTE 3     |     |     | F      | 1.  DIMENSIONS “*1” AND “*2” DO NOT INCLUDE MOLD FLASH. |     |     |
2.  DIMENSION “*3” DOES NOT INCLUDE TRIM OFFSET.
3.  PIN 1 VISUAL INDEX FEATURE MAY VARY, BUT MUST BE
S
     LOCATED WITHIN THE HATCHED AREA.
4.  CHAMFERS AT CORNERS ARE OPTIONAL, SIZE MAY VARY.
Reference Dimensions in millimeters
|     |     | y S | *3  |     |     | Symbol Min | Nom Max                 |
| --- | --- | --- | --- | --- | --- | ---------- | ----------------------- |
|     | e   |     | bp  |     |     |            |                         |
|     |     |     |     | M   |     | D 13.9     | 14.0 14.1               |
|     |     |     |     |     |     | E 13.9     | 14.0 14.1               |
|     |     |     |     |     |     | A2         | (cid:127) 1.4 (cid:127) |
|     |     |     |     |     |     | HD 15.8    | 16.0 16.2               |
|     |     |     |     |     |     | HE 15.8    | 16.0 16.2               |
52.0
|     | 2A  |     |          |     |     | A       | (cid:127) (cid:127) 1.7 |
| --- | --- | --- | -------- | --- | --- | ------- | ----------------------- |
|     | A   | c   |          |     |     |         |                         |
|     |     |     |          |     |     | A1 0.05 | (cid:127) 0.15          |
|     |     |     | (cid:84) |     |     | bp 0.15 | 0.20 0.27               |
|     | 1A  |     |          |     |     | c 0.09  | (cid:127) 0.20          |
Lp
|     |     |     |     |     |     | (cid:84) | 0(cid:113) 3.5(cid:113) 8(cid:113) |
| --- | --- | --- | --- | --- | --- | -------- | ---------------------------------- |
L1
|     |     |     |     |     |     | e   | (cid:127) 0.5 (cid:127) |
| --- | --- | --- | --- | --- | --- | --- | ----------------------- |
Detail F
|     |     |     |     |     |     | x       | (cid:127) (cid:127) 0.08 |
| --- | --- | --- | --- | --- | --- | ------- | ------------------------ |
|     |     |     |     |     |     | y       | (cid:127) (cid:127) 0.08 |
|     |     |     |     |     |     | Lp 0.45 | 0.6 0.75                 |
|     |     |     |     |     |     | L1      | (cid:127) 1.0 (cid:127)  |
© 2015 Renesas Electronics Corporation. All rights reserved.
| Figure 1.2      | 100-pin LQFP  |     |     |     |     |     |                 |
| --------------- | ------------- | --- | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 | Rev.1.10      |     |     |     |     |     | Page 120 of 131 |
Sep 29, 2023

| RA4M1 Group |                      |     |              |     |               | Appendix 1. Package Dimensions  |     |
| ----------- | -------------------- | --- | ------------ | --- | ------------- | ------------------------------- | --- |
|             | JEITA Package Code   |     | RENESAS Code |     | Previous Code | MASS (Typ) [g]                  |     |
|             | P-LFQFP64-10x10-0.50 |     | PLQP0064KB-C |     | —             |                                 | 0.3 |
Unit: mm
HD
*1
D
|     |     | 48         | 33  |        |     |     |     |
| --- | --- | ---------- | --- | ------ | --- | --- | --- |
|     |     | 49         |     | 32     |     |     |     |
|     |     |            |     | E2* EH |     |     |     |
|     |     | 64         |     | 17     |     |     |     |
|     |     | 1          | 16  |        |     |     |     |
|     |     | Index area |     | NOTE 4 |     |     |     |
NOTE 3
|     |     |     |     | F   | NOTE) |     |     |
| --- | --- | --- | --- | --- | ----- | --- | --- |
1.  DIMENSIONS “*1” AND “*2” DO NOT INCLUDE MOLD FLASH.
2.  DIMENSION “*3” DOES NOT INCLUDE TRIM OFFSET.
S
3.  PIN 1 VISUAL INDEX FEATURE MAY VARY, BUT MUST BE
     LOCATED WITHIN THE HATCHED AREA.
4.  CHAMFERS AT CORNERS ARE OPTIONAL, SIZE MAY VARY.
y S *3
Reference Dimensions in millimeters
|     |     | e   | bp  |     |     |         |                         |
| --- | --- | --- | --- | --- | --- | ------- | ----------------------- |
|     |     |     |     | M   |     | Symbol  | Min Nom Max             |
|     |     |     |     |     |     | D 9.9   | 10.0 10.1               |
|     |     |     |     |     |     | E 9.9   | 10.0 10.1               |
|     |     |     |     |     |     | A2      | (cid:127) 1.4 (cid:127) |
|     |     |     |     |     |     | HD 11.8 | 12.0 12.2               |
|     |     |     |     |     |     | HE 11.8 | 12.0 12.2               |
52.0
|     |     | 2A  |          |     |     | A       | (cid:127) (cid:127) 1.7            |
| --- | --- | --- | -------- | --- | --- | ------- | ---------------------------------- |
|     |     | A   | c        |     |     |         |                                    |
|     |     |     |          |     |     | A1 0.05 | (cid:127) 0.15                     |
|     |     |     | (cid:84) |     |     | bp 0.15 | 0.20 0.27                          |
|     |     | 1A  |          |     |     | c 0.09  | (cid:127) 0.20                     |
|     |     |     | Lp       |     |     |         | 0(cid:113) 3.5(cid:113) 8(cid:113) |
(cid:84)
L1
|     |     |     |     |     |     | e   | (cid:127) 0.5 (cid:127) |
| --- | --- | --- | --- | --- | --- | --- | ----------------------- |
Detail F
|     |     |     |     |     |     | x       | (cid:127) (cid:127) 0.08 |
| --- | --- | --- | --- | --- | --- | ------- | ------------------------ |
|     |     |     |     |     |     | y       | (cid:127) (cid:127) 0.08 |
|     |     |     |     |     |     | Lp 0.45 | 0.6 0.75                 |
|     |     |     |     |     |     | L1      | (cid:127) 1.0 (cid:127)  |
© 2015 Renesas Electronics Corporation. All rights reserved.
| Figure 1.3      | 64-pin LQFP |          |     |     |     |     |                 |
| --------------- | ----------- | -------- | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 |             | Rev.1.10 |     |     |     |     | Page 121 of 131 |
Sep 29, 2023

| RA4M1 Group |                    |               |      |     |              |                | Appendix 1. Package Dimensions  |               |      |
| ----------- | ------------------ | ------------- | ---- | --- | ------------ | -------------- | ------------------------------- | ------------- | ---- |
|             |                    | JEITA Package | code |     | RENESAS code | Previous       | code                            | MASS(TYP.)[g] |      |
|             | P-HWQFN64-8x8-0.40 |               |      |     | PWQN0064LA-A | P64K8-40-9B5-3 |                                 |               | 0.16 |
D
|     |     | 48  |     | 33  |     |     |                          |     |     |
| --- | --- | --- | --- | --- | --- | --- | ------------------------ | --- | --- |
|     |     | 49  |     |     | 32  |     | DETAIL OF   A   PART     |     |     |
E
A
|     |     | 64  |     |     | 17  |     | A1  |     | c2  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     | 1   |     | 16  |     |     |     |     |     |
INDEX AREA
A
S
y S
|     |     |     |     |     |     |     | Referance | Dimension in Millimeters |     |
| --- | --- | --- | --- | --- | --- | --- | --------- | ------------------------ | --- |
|     |     |     |     |     |     |     | Symbol    | Min Nom                  | Max |
8.05
|     |     |     | D2  |     |                 |     | D   | 7.95 8.00 |      |
| --- | --- | --- | --- | --- | --------------- | --- | --- | --------- | ---- |
|     |     |     |     |     |                 |     | E   | 7.95 8.00 | 8.05 |
|     | Lp  |     | A   |     | EXPOSED DIE PAD |     |     |           |      |
|     |     |     |     |     |                 |     | A   |           | 0.80 |
|     |     | 1   |     | 16  |                 |     | A1  | 0.00      |      |
|     | 64  |     |     |     | 17              |     | b   | 0.17 0.20 | 0.23 |
|     |     |     |     |     |                 |     | e   | 0.40      |      |
|     |     |     |     |     |                 |     | Lp  | 0.30 0.40 | 0.50 |
B
|     |     |     |     |     |     |     | x   |     | 0.05 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- |
E2
y
0.05
|     |     |     |     |     |     |     | ZD  | 1.00 |     |
| --- | --- | --- | --- | --- | --- | --- | --- | ---- | --- |
ZE
|     |     |     |     |     |     |     | ZE  | 1.00 |     |
| --- | --- | --- | --- | --- | --- | --- | --- | ---- | --- |
32
|     |     | 49  |     |     |     |     | c2  | 0.15 | 0.25 |
| --- | --- | --- | --- | --- | --- | --- | --- | ---- | ---- |
0.20
48
|     |     |     |     | 33  |       |     | D2  | 6.50 |     |
| --- | --- | --- | --- | --- | ----- | --- | --- | ---- | --- |
|     | Z   |     |     | e   |       |     |     |      |     |
|     | D   |     |     |     |       |     | E 2 | 6.50 |     |
|     |     |     | b   | x M | S A B |     |     |      |     |
2013 Renesas Electronics Corporation. All rights reserved.
| Figure 1.4      |     | 64-pin QFN (1) |     |     |     |     |     |     |                 |
| --------------- | --- | -------------- | --- | --- | --- | --- | --- | --- | --------------- |
| R01DS0355EJ0110 |     | Rev.1.10       |     |     |     |     |     |     | Page 122 of 131 |
Sep 29, 2023

| RA4M1 Group |     |                     |     |              |     |               |      | Appendix 1. Package Dimensions  |     |     |
| ----------- | --- | ------------------- | --- | ------------ | --- | ------------- | ---- | ------------------------------- | --- | --- |
|             |     | JEITA Package code  |     | RENESAS code |     | MASS(TYP.)[g] |      |                                 |     |     |
|             |     | P-HWQFN064-8x8-0.40 |     | PWQN0064LB-B |     |               | 0.14 |                                 |     |     |
2X
|     |     |     | aaa C |     |     |     |     |     |     |     |
| --- | --- | --- | ----- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | 48    |     | 33  |     |     |     |     |     |
|     |     |     | 49    |     |     | 32  |     |     |     |     |
D
INDEX AREA
(D/2 X E/2)
|     |     |     | 64  |     |     | 17  |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
2X
|     |     | aaa | C 1 |     | 16  |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | B   | E   |     | A   |     |     |     |     |
|     | ccc | C   |     |     |     |     |     |     |     |     |
C
SEATING PLANE
|     |     | A (A3) | A1  |            |     |     |     |     |     |     |
| --- | --- | ------ | --- | ---------- | --- | --- | --- | --- | --- | --- |
|     |     |        |     |   e b(64X) | bbb | C A | B   |     |     |     |
64X
|     |     |     |     |     |     | ddd C |     | Reference | Dimension in Millimeters |     |
| --- | --- | --- | --- | --- | --- | ----- | --- | --------- | ------------------------ | --- |
|     |     | eee | C   |     |     |       |     |           |                          |     |
Symbol
|     |     |     |     |     |     |       |     |     | Min.        | Nom. Max.        |
| --- | --- | --- | --- | --- | --- | ----- | --- | --- | ----------- | ---------------- |
|     |     |     |     | E2  |     | fff C | A B |     |             |                  |
|     |     |     |     |     |     |       |     | A   | (cid:18441) | (cid:18441) 0.80 |
|     |     |     | 1   |     | 16  |       |     |     |             |                  |
|     |     |     |     |     |     |       |     | A 1 | 0.00        | 0.02 0.05        |
EXPOSED
|                 | fff | C A B |                |     |     |  DIE PAD |     | A 3 |      | 0.203 REF.              |
| --------------- | --- | ----- | -------------- | --- | --- | -------- | --- | --- | ---- | ----------------------- |
|                 |     |       | 64             |     |     | 17       |     |     |      |                         |
|                 |     |       |                |     |     |          |     | b   | 0.15 | 0.20 0.25               |
|                 |     |       |                |     |     |          |     | D   |      | 8.00 BSC                |
|                 |     |       |                |     |     |          |     | E   |      | 8.00 BSC                |
|                 |     |       |                |     |     |          |     | e   |      | 0.40 BSC                |
|                 |     |       | D2             |     |     |          |     | L   | 0.35 | 0.40 0.45               |
|                 |     |       |                |     |     |          |     | K   | 0.20 | (cid:18441) (cid:18441) |
|                 |     |       |                |     |     |          |     | D 2 | 6.40 | 6.50 6.60               |
|                 |     |       |                |     |     |          |     | E 2 | 6.40 | 6.50 6.60               |
|                 |     |       |                |     |     |          |     | aaa |      | 0.10                    |
|                 |     |       | 49             |     |     | 32       |     |     |      |                         |
|                 |     |       |                |     |     |          |     | bbb |      | 0.07                    |
|                 |     |       | 48             |     | 33  |          |     | ccc |      | 0.10                    |
|                 |     |       | L(64X)         |     |     | K(64X)   |     |     |      |                         |
|                 |     |       |                |     |     |          |     | ddd |      | 0.05                    |
|                 |     |       |                |     |     |          |     | eee |      | 0.08                    |
|                 |     |       |                |     |     |          |     | fff |      | 0.10                    |
| Figure 1.5      |     |       | 64-pin QFN (2) |     |     |          |     |     |      |                         |
| R01DS0355EJ0110 |     |       | Rev.1.10       |     |     |          |     |     |      | Page 123 of 131         |
Sep 29, 2023

| RA4M1 Group |                    |     |              |               | Appendix 1. Package Dimensions  |          |
| ----------- | ------------------ | --- | ------------ | ------------- | ------------------------------- | -------- |
|             | JEITA Package Code |     | RENESAS Code | Previous Code | MASS (Typ) [g]                  |          |
|             | P-LFQFP48-7x7-0.50 |     | PLQP0048KB-B | —             | 0.2                             |          |
|             |                    | HD  |              |               |                                 | Unit: mm |
*1
D
|     |     | 36  | 2255 |     |     |     |
| --- | --- | --- | ---- | --- | --- | --- |
37 24
E EH
2*
48
13
|     |     | 1   | 12 NOTE 4 |     |     |     |
| --- | --- | --- | --------- | --- | --- | --- |
Index area
NOTE)
|     |     | NOTE 3 |     | 1.  DIMENSIONS “*1” AND “*2” DO NOT INCLUDE MOLD FLASH. |     |     |
| --- | --- | ------ | --- | ------------------------------------------------------- | --- | --- |
F 2.  DIMENSION “*3” DOES NOT INCLUDE TRIM OFFSET.
3.  PIN 1 VISUAL INDEX FEATURE MAY VARY, BUT MUST BE
     LOCATED WITHIN THE HATCHED AREA.
4.  CHAMFERS AT CORNERS ARE OPTIONAL, SIZE MAY VARY.
S
Reference Dimensions in millimeters
|     |     |     |     |     | Symbol Min Nom | Max |
| --- | --- | --- | --- | --- | -------------- | --- |
|     |     |     |     |     | D 6.9 7.0      | 7.1 |
|     |     | y S | *3  |     |                |     |
|     |     |     |     |     | E 6.9 7.0      | 7.1 |
|     |     | e   | bp  |     |                |     |
M
|     |     |     |     |     | A2 (cid:127) 1.4      | (cid:127) |
| --- | --- | --- | --- | --- | --------------------- | --------- |
|     |     |     |     |     | HD 8.8 9.0            | 9.2       |
|     |     |     |     |     | HE 8.8 9.0            | 9.2       |
|     |     |     |     |     | A (cid:127) (cid:127) | 1.7       |
52.0
|     |     | 2A  |     |     | A1 0.05   | 0.15 |
| --- | --- | --- | --- | --- | --------- | ---- |
|     |     | A   |     |     | (cid:127) |      |
c
|     |     |     |     |     | bp 0.17 0.20 | 0.27 |
| --- | --- | --- | --- | --- | ------------ | ---- |
(cid:84)
|     |     |     |     |     | c 0.09 (cid:127) | 0.20 |
| --- | --- | --- | --- | --- | ---------------- | ---- |
1A
|     |     |     |     |     | (cid:84) 0(cid:113) 3.5(cid:113) | 8(cid:113) |
| --- | --- | --- | --- | --- | -------------------------------- | ---------- |
Lp
|     |     |     |     |     | e (cid:127) 0.5 | (cid:127) |
| --- | --- | --- | --- | --- | --------------- | --------- |
L1
|     |     |     |     |     | x                   | 0.08 |
| --- | --- | --- | --- | --- | ------------------- | ---- |
|     |     |     |     |     | (cid:127) (cid:127) |      |
Detail F
|                 |             |     |     |     | y (cid:127) (cid:127) | 0.08            |
| --------------- | ----------- | --- | --- | --- | --------------------- | --------------- |
|                 |             |     |     |     | Lp 0.45 0.6           | 0.75            |
|                 |             |     |     |     | L1 (cid:127) 1.0      | (cid:127)       |
| Figure 1.6      | 48-pin LQFP |     |     |     |                       |                 |
| R01DS0355EJ0110 | Rev.1.10    |     |     |     |                       | Page 124 of 131 |
Sep 29, 2023

| RA4M1 Group |               |      |              |          | Appendix 1. Package Dimensions  |               |     |
| ----------- | ------------- | ---- | ------------ | -------- | ------------------------------- | ------------- | --- |
|             | JEITA Package | code | RENESAS code | Previous | code                            | MASS(TYP.)[g] |     |
48PJN-A
|     | P-HWQFN48-7x7-0.50 |     | PWQN0048KB-A | P48K8-50-5B4-6 |     | 0.13 |     |
| --- | ------------------ | --- | ------------ | -------------- | --- | ---- | --- |
D
|     | 36  |     | 25  |     |                          |     |     |
| --- | --- | --- | --- | --- | ------------------------ | --- | --- |
|     | 37  |     | 24  |     | DETAIL OF   A   PART     |     |     |
|     |     |     | E   | A   |                          |     |     |
|     |     |     |     |     | A1                       |     | c2  |
|     | 48  |     | 13  |     |                          |     |     |
|     | 1   |     | 12  |     |                          |     |     |
INDEX AREA
A
S
y S
Dimension in Millimeters
Referance
Symbol
|     |     |     |                 |     |         | Min Nom   | Max  |
| --- | --- | --- | --------------- | --- | ------- | --------- | ---- |
|     |     |     |                 |     |         | 6.95 7.00 | 7.05 |
|     |     | D2  |                 |     | D       |           |      |
|     |     |     |                 |     | E       | 6.95 7.00 | 7.05 |
|     | Lp  | A   | EXPOSED DIE PAD |     |         |           |      |
|     |     |     |                 |     | A       |           | 0.80 |
|     | 1   |     | 12              |     |         |           |      |
|     |     |     |                 |     | A1      | 0.00      |      |
|     |     |     | 13              |     | b       | 0.18      | 0.30 |
|     | 48  |     |                 |     |         | 0.25      |      |
|     |     |     |                 |     | e       | 0.50      |      |
|     | B   |     |                 |     | Lp 0.30 | 0.40      | 0.50 |
|     |     |     |                 |     | x       |           | 0.05 |
E2
|     |     |     |     |     | y   |      | 0.05 |
| --- | --- | --- | --- | --- | --- | ---- | ---- |
|     |     |     |     |     | ZD  | 0.75 |      |
ZE
|     |     |     |     |     | ZE  | 0.75 |     |
| --- | --- | --- | --- | --- | --- | ---- | --- |
37
|     |     |       | 24    |     | c2  | 0.15 0.20 | 0.25 |
| --- | --- | ----- | ----- | --- | --- | --------- | ---- |
|     | 36  |       | 25    |     |     |           |      |
|     |     |       |       |     | D2  | 5.50      |      |
|     | Z   | e     |       |     |     |           |      |
|     | D   |       |       |     | E 2 | 5.50      |      |
|     |     | b x M | S A B |     |     |           |      |
2013 Renesas Electronics Corporation. All rights reserved.
| Figure 1.7      |     | 48-pin QFN (1) |     |     |     |                 |     |
| --------------- | --- | -------------- | --- | --- | --- | --------------- | --- |
| R01DS0355EJ0110 |     | Rev.1.10       |     |     |     | Page 125 of 131 |     |
Sep 29, 2023

| RA4M1 Group |                     |     |     |              |     |               |        | Appendix 1. Package Dimensions  |     |     |     |
| ----------- | ------------------- | --- | --- | ------------ | --- | ------------- | ------ | ------------------------------- | --- | --- | --- |
|             | JEITA Package code  |     |     | RENESAS code |     | MASS(TYP.)[g] |        |                                 |     |     |     |
|             | P-HWQFN048-7x7-0.50 |     |     | PWQN0048KC-A |     |               | 0.13 g |                                 |     |     |     |
2X
|     |     |     | aaa C |     |     |     |     |     |     |     |     |
| --- | --- | --- | ----- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | 36    |     | 25  |     |     |     |     |     |     |
|     |     | 37  |       |     |     | 24  |     |     |     |     |     |
D
INDEX AREA
(D/2 X E/2)
|     |     | 48  |     |     |     | 13  |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
2X
aaa C
|     |     |     | 1   |     | 12  |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
A
|     |     | B   |     | E   |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ccc | C   |     |     |     |     |     |     |     |     |     |     |
C
|     | (A3) |     |     |     |     | SEATING PLANE |     |     |     |     |     |
| --- | ---- | --- | --- | --- | --- | ------------- | --- | --- | --- | --- | --- |
A A1
|     |     |     |     |   e b(48X) |     | bbb C A | B   |     |     |     |     |
| --- | --- | --- | --- | ---------- | --- | ------- | --- | --- | --- | --- | --- |
48X
ddd C
|     | eee | C   |     |     |     |     |     |     | Dimension in Millimeters |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ------------------------ | --- | --- |
Reference
Symbol
|     |     |     |     | E2  |     | fff C   | A B |     | Min.        | Nom.        | Max. |
| --- | --- | --- | --- | --- | --- | ------- | --- | --- | ----------- | ----------- | ---- |
|     |     |     | 1   |     | 12  |         |     | A   | (cid:18441) | (cid:18441) | 0.80 |
|     |     |     |     |     |     | EXPOSED |     | A   | 0.00        | 0.02        | 0.05 |
1
| fff | C A B | 48  |     |     |     | 13  DIE PAD |     |     |     |            |     |
| --- | ----- | --- | --- | --- | --- | ----------- | --- | --- | --- | ---------- | --- |
|     |       |     |     |     |     |             |     | A   |     | 0.203 REF. |     |
3
|     |     |     |     |     |     |     |     | b   | 0.20 | 0.25        | 0.30        |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | ----------- | ----------- |
|     |     |     |     |     |     |     |     | D   |      | 7.00 BSC    |             |
|     |     |     |     |     |     |     |     | E   |      | 7.00 BSC    |             |
|     |     | D2  |     |     |     |     |     | e   |      | 0.50 BSC    |             |
|     |     |     |     |     |     |     |     | L   | 0.30 | 0.40        | 0.50        |
|     |     |     |     |     |     |     |     | K   | 0.20 | (cid:18441) | (cid:18441) |
|     |     |     |     |     |     |     |     | D   | 5.25 | 5.30        | 5.35        |
2
|                 |     |                |        |     |     |        |     | E   | 5.25 | 5.30            | 5.35 |
| --------------- | --- | -------------- | ------ | --- | --- | ------ | --- | --- | ---- | --------------- | ---- |
|                 |     | 37             |        |     |     | 24     |     | 2   |      |                 |      |
|                 |     |                |        |     |     |        |     | aaa |      | 0.15            |      |
|                 |     |                | 36     |     | 25  |        |     |     |      |                 |      |
|                 |     |                |        |     |     |        |     | bbb |      | 0.10            |      |
|                 |     |                | L(48X) |     |     | K(48X) |     | ccc |      | 0.10            |      |
|                 |     |                |        |     |     |        |     | ddd |      | 0.05            |      |
|                 |     |                |        |     |     |        |     | eee |      | 0.08            |      |
|                 |     |                |        |     |     |        |     | fff |      | 0.10            |      |
| Figure 1.8      |     | 48-pin QFN (2) |        |     |     |        |     |     |      |                 |      |
| R01DS0355EJ0110 |     | Rev.1.10       |        |     |     |        |     |     |      | Page 126 of 131 |      |
Sep 29, 2023

| RA4M1 Group |                    |     |      |              |                | Appendix 1. Package Dimensions  |               |     |
| ----------- | ------------------ | --- | ---- | ------------ | -------------- | ------------------------------- | ------------- | --- |
|             | JEITA Package      |     | code | RENESAS code | Previous       | code                            | MASS(TYP.)[g] |     |
|             | P-HWQFN40-6x6-0.50 |     |      | PWQN0040KC-A | P40K8-50-4B4-5 |                                 | 0.09          |     |
D
|     | 30  |     | 21  |     |     |                          |     |     |
| --- | --- | --- | --- | --- | --- | ------------------------ | --- | --- |
|     | 31  |     |     | 20  |     | DETAIL OF   A   PART     |     |     |
|     |     |     |     | E   | A   |                          |     |     |
|     | 40  |     |     | 11  |     | A1                       |     | c   |
2
|     | 1   |     | 10  |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
INDEX AREA
A
S
|     |     |     | y S |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
Dimension in Millimeters
Referance
Symbol
|     |     |     |     |                 |     |         | Min Nom   | Max  |
| --- | --- | --- | --- | --------------- | --- | ------- | --------- | ---- |
|     |     |     |     |                 |     |         | 5.95 6.00 | 6.05 |
|     |     |     | D2  |                 |     | D       |           |      |
|     |     |     |     |                 |     | E       | 5.95 6.00 | 6.05 |
|     | Lp  |     | A   | EXPOSED DIE PAD |     |         |           |      |
|     |     |     |     |                 |     | A       |           | 0.80 |
|     | 1   |     |     | 10              |     |         |           |      |
|     |     |     |     |                 |     | A1      | 0.00      |      |
|     |     |     |     |                 |     | b       | 0.18      | 0.30 |
|     | 40  |     |     | 11              |     |         | 0.25      |      |
|     |     |     |     |                 |     | e       | 0.50      |      |
|     | B   |     |     |                 |     | Lp 0.30 | 0.40      | 0.50 |
|     |     |     |     |                 |     | x       |           | 0.05 |
E2
|     |     |     |     |     |     | y   |      | 0.05 |
| --- | --- | --- | --- | --- | --- | --- | ---- | ---- |
|     |     |     |     |     |     | ZD  | 0.75 |      |
ZE
|                 |     |                |     |         |     | ZE  | 0.75            |      |
| --------------- | --- | -------------- | --- | ------- | --- | --- | --------------- | ---- |
|                 | 31  |                |     | 20      |     |     |                 |      |
|                 |     |                |     |         |     | c2  | 0.15 0.20       | 0.25 |
|                 | 30  |                | 21  |         |     |     |                 |      |
|                 |     |                |     |         |     | D2  | 4.50            |      |
|                 | Z   |                | e   |         |     |     |                 |      |
|                 | D   |                |     |         |     | E 2 | 4.50            |      |
|                 |     |                | b x | M S A B |     |     |                 |      |
| Figure 1.9      |     | 40-pin QFN (1) |     |         |     |     |                 |      |
| R01DS0355EJ0110 |     | Rev.1.10       |     |         |     |     | Page 127 of 131 |      |
Sep 29, 2023

| RA4M1 Group |                     |     |     |              |     |     |               | Appendix 1. Package Dimensions  |     |     |     |
| ----------- | ------------------- | --- | --- | ------------ | --- | --- | ------------- | ------------------------------- | --- | --- | --- |
|             | JEITA Package code  |     |     | RENESAS code |     |     | MASS(TYP.)[g] |                                 |     |     |     |
|             | P-HWQFN040-6x6-0.50 |     |     | PWQN0040KD-A |     |     | 0.08          |                                 |     |     |     |
2X
aaa C
|     |     |     | 30  |     | 21  |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |     | 31  |     |     | 20  |     |     |     |     |     |
D
INDEX AREA
(D/2 X E/2)
|     |     | 40  |     |     |     | 11  |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
2X
|     |     | aaa C |     |     |     |     |     |     |     |     |     |
| --- | --- | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     |     |       | 1   |     | 10  |     |     |     |     |     |     |
A
|     |       | B   |     | E   |     |     |     |     |     |     |     |
| --- | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|     | ccc C |     |     |     |     |     |     |     |     |     |     |
C
|     | (A3) |     |     |     |     | SEATING PLANE |     |     |     |     |     |
| --- | ---- | --- | --- | --- | --- | ------------- | --- | --- | --- | --- | --- |
A A1
|     |     |     |     |   e | b(40X) | bbb | C A B |     |     |     |     |
| --- | --- | --- | --- | --- | ------ | --- | ----- | --- | --- | --- | --- |
40X
|     |     |       |     |     |     | ddd | C   |     |                          |     |     |
| --- | --- | ----- | --- | --- | --- | --- | --- | --- | ------------------------ | --- | --- |
|     |     | eee C |     |     |     |     |     |     | Dimension in Millimeters |     |     |
Reference
Symbol
|     |     |     |     | E2  |     | fff     | C A B |     | Min.        | Nom.        | Max. |
| --- | --- | --- | --- | --- | --- | ------- | ----- | --- | ----------- | ----------- | ---- |
|     |     |     | 1   |     | 10  |         |       | A   | (cid:18441) | (cid:18441) | 0.80 |
|     |     |     |     |     |     | EXPOSED |       | A   | 0.00        | 0.02        | 0.05 |
1
|     | fff C A | B   |     |     |     | 11  DIE PAD |     |     |     |            |     |
| --- | ------- | --- | --- | --- | --- | ----------- | --- | --- | --- | ---------- | --- |
|     |         | 40  |     |     |     |             |     | A   |     | 0.203 REF. |     |
3
|     |     |     |     |     |     |     |     | b   | 0.18 | 0.25     | 0.30 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | -------- | ---- |
|     |     |     |     |     |     |     |     | D   |      | 6.00 BSC |      |
|     |     |     |     |     |     |     |     | E   |      | 6.00 BSC |      |
D2
|     |     |     |     |     |     |     |     | e   |      | 0.50 BSC    |             |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | ----------- | ----------- |
|     |     |     |     |     |     |     |     | L   | 0.30 | 0.40        | 0.50        |
|     |     |     |     |     |     |     |     | K   | 0.20 | (cid:18441) | (cid:18441) |
31
|     |     |     |     |     |     | 20  |     | D   | 4.45 | 4.50 | 4.55 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | ---- | ---- |
2
|                 |     |                |        |     |     |        |     | E   | 4.45 | 4.50            | 4.55 |
| --------------- | --- | -------------- | ------ | --- | --- | ------ | --- | --- | ---- | --------------- | ---- |
|                 |     |                | 30     |     | 21  |        |     | 2   |      |                 |      |
|                 |     |                | L(40X) |     |     | K(40X) |     | aaa |      | 0.15            |      |
|                 |     |                |        |     |     |        |     | bbb |      | 0.10            |      |
|                 |     |                |        |     |     |        |     | ccc |      | 0.10            |      |
|                 |     |                |        |     |     |        |     | ddd |      | 0.05            |      |
|                 |     |                |        |     |     |        |     | eee |      | 0.08            |      |
|                 |     |                |        |     |     |        |     | fff |      | 0.10            |      |
| Figure 1.10     |     | 40-pin QFN (2) |        |     |     |        |     |     |      |                 |      |
| R01DS0355EJ0110 |     | Rev.1.10       |        |     |     |        |     |     |      | Page 128 of 131 |      |
Sep 29, 2023

REVISION HISTORY
| RA4M1 Group |     |     |     | Revision History  |
| ----------- | --- | --- | --- | ----------------- |
RA4M1 Group Datasheet
Revision History
| Rev. Date          | Chapter |                       | Summary |     |
| ------------------ | ------- | --------------------- | ------- | --- |
| 1.00 Oct 8, 2019   | —       | First Edition issued  |         |     |
| 1.10  Sep 29, 2023 | —       | Second Edition issued |         |     |
Proprietary Notice
All text, graphics, photographs, trademarks, logos, artwork and computer code, collectively known as content, contained in
this document is owned, controlled or licensed by or to Renesas, and is protected by trade dress, copyright, patent and
trademark laws, and other intellectual property rights and unfair competition laws. Except as expressly provided herein, no
part of this document or content may be copied, reproduced, republished, posted, publicly displayed, encoded, translated,
transmitted or distributed in any other medium for publication or distribution or for any commercial enterprise, without prior
written consent from Renesas.
Arm® and Cortex® are registered trademarks of Arm Limited. CoreSight™ is a trademark of Arm Limited.
CoreMark® is a registered trademark of the Embedded Microprocessor Benchmark Consortium.
Magic Packet™ is a trademark of Advanced Micro Devices, Inc.
SuperFlash® is a registered trademark of Silicon Storage Technology, Inc. in several countries including the United States
and Japan.
Other brands and names mentioned in this document may be the trademarks or registered trademarks of their respective
holders.
| R01DS0355EJ0110 | Rev.1.10 |     |     | Page 129 of 131 |
| --------------- | -------- | --- | --- | --------------- |
Sep 29, 2023

General Precautions in the Handling of Microprocessing Unit and Microcontroller
Unit Products
The following usage notes are applicable to all Microprocessing unit and Microcontroller unit products from Renesas. For detailed usage notes on the
products covered by this document, refer to the relevant sections of the document as well as any technical updates that have been issued for the products.
1. Precaution against Electrostatic Discharge (ESD)
A strong electrical field, when exposed to a CMOS device, can cause destruction of the gate oxide and ultimately degrade the device operation. Steps
must be taken to stop the generation of static electricity as much as possible, and quickly dissipate it when it occurs. Environmental control must be
adequate. When it is dry, a humidifier should be used. This is recommended to avoid using insulators that can easily build up static electricity.
Semiconductor devices must be stored and transported in an anti-static container, static shielding bag or conductive material. All test and
measurement tools including work benches and floors must be grounded. The operator must also be grounded using a wrist strap. Semiconductor
devices must not be touched with bare hands. Similar precautions must be taken for printed circuit boards with mounted semiconductor devices.
2. Processing atpower-on
The state of the product is undefined at the time when power is supplied. The states of internal circuits in the LSI are indeterminate and the states of
register settings and pins are undefined at the time when power is supplied. In a finished product where the reset signal is applied to the external reset
pin, the states of pins are not guaranteed from the time when power is supplied until the reset process is completed. In a similar way, the states of pins
in a product that is reset by an on-chip power-on reset function are not guaranteed from the time when power is supplied until the power reaches the
level at which resetting is specified.
3. Input of signal during power-off state
Do not input signals or an I/O pull-up power supply while the device is powered off. The current injection that results from input of such a signal or I/O
pull-up power supply may cause malfunction and the abnormal current that passes in the device at this time may cause degradation of internal
elements. Follow the guideline for input signal during power-off state as described in your product documentation.
4. Handling of unused pins
Handle unused pins in accordance with the directions given under handling of unused pins in the manual. The input pins of CMOS products are
generally in the high-impedance state. In operation with an unused pin in the open-circuit state, extra electromagnetic noise is induced in the vicinity of
the LSI, an associated shoot-through current flows internally, and malfunctions occur due to the false recognition of the pin state as an input signal
become possible.
5. Clock signals
After applying a reset, only release the reset line after the operating clock signal becomes stable. When switching the clocksignal during program
execution, wait until the target clock signal is stabilized. When the clock signal is generated with an external resonator or from an external oscillator
during a reset, ensure that the reset line is only released after full stabilization of the clock signal. Additionally, whenswitching to a clock signal
produced with an external resonator or by an external oscillator while program execution is in progress, wait until the target clock signal is stable.
6. Voltage application waveform at input pin
Waveform distortion due to input noise or a reflected wave may cause malfunction. If the input of the CMOS device stays in the area between VIL
(Max.) and VIH(Min.) due to noise, for example, the device may malfunction. Take care to prevent chattering noise from entering the device when the
input level is fixed, and also in the transition period when the input level passes through the area between VIL(Max.) and VIH(Min.).
7. Prohibition of access to reserved addresses
Access to reserved addresses is prohibited. The reserved addresses are provided for possible future expansion of functions. Do not access these
addresses as the correct operation of the LSI is not guaranteed.
8. Differences between products
Before changing from one product to another, for example to a product with a different part number, confirm that the change will not lead to problems.
The characteristics of a microprocessing unit or microcontroller unit products in the same group but having a different part number might differ in terms
of internal memory capacity, layout pattern, and other factors, which can affect the ranges of electrical characteristics, such as characteristic values,
operating margins, immunity to noise, and amount of radiated noise. When changing to a product with a different part number, implement a system-
evaluation test for the given product.

Notice
1. Descriptions of circuits, software and other related information in this document are provided only to illustrate the operation of semiconductor products
and application examples. You are fully responsible for the incorporation or any other use of the circuits, software, and information in the design of your
product or system. Renesas Electronics disclaims any and all liability for any losses and damages incurred by you or third parties arising from the use of
these circuits, software, or information.
2. Renesas Electronics hereby expressly disclaims any warranties against and liability for infringement or any other claims involving patents, copyrights, or
other intellectual property rights of third parties, by or arising from the use of Renesas Electronics products or technical information described in this
document, including but not limited to, the product data, drawings, charts, programs, algorithms, and application examples.
3. No license, express, implied or otherwise, is granted hereby under any patents, copyrights or other intellectual property rights of Renesas Electronics or
others.
4. You shall be responsible for determining what licenses are required from any third parties, and obtaining such licenses for the lawful import, export,
manufacture, sales, utilization, distribution or other disposal of any products incorporating Renesas Electronics products, if required.
5. You shall not alter, modify, copy, or reverse engineer any Renesas Electronics product, whether in whole or in part. Renesas Electronics disclaims any
and all liability for any losses or damages incurred by you or third parties arising from such alteration, modification, copying or reverse engineering.
6. Renesas Electronics products are classified according to the following two quality grades: “Standard” and “High Quality”. The intended applications for
each Renesas Electronics product depends on the product’s quality grade, as indicated below.
"Standard": Computers; office equipment; communications equipment; test and measurement equipment; audio and visual equipment; home
electronic appliances; machine tools; personal electronic equipment; industrial robots; etc.
"High Quality": Transportation equipment (automobiles, trains, ships, etc.); traffic control (traffic lights); large-scale communication equipment; key
financial terminal systems; safety control equipment; etc.
Unless expressly designated as a high reliability product or a product for harsh environments in a Renesas Electronics data sheet or other Renesas
Electronics document, Renesas Electronics products are not intended or authorized for use in products or systems that may pose a direct threat to
human life or bodily injury (artificial life support devices or systems; surgical implantations; etc.), or may cause serious property damage (space system;
undersea repeaters; nuclear power control systems; aircraft control systems; key plant systems; military equipment; etc.). Renesas Electronics disclaims
any and all liability for any damages or losses incurred by you or any third parties arising from the use of any Renesas Electronics product that is
inconsistent with any Renesas Electronics data sheet, user’s manual or other Renesas Electronics document.
7. No semiconductor product is absolutely secure. Notwithstanding any security measures or features that may be implemented in Renesas Electronics
hardware or software products, Renesas Electronics shall have absolutely no liability arising out of any vulnerability or security breach, including but not
limited to any unauthorized access to or use of a Renesas Electronics product or a system that uses a Renesas Electronics product. RENESAS
ELECTRONICS DOES NOT WARRANT OR GUARANTEE THAT RENESAS ELECTRONICS PRODUCTS, OR ANY SYSTEMS CREATED USING
RENESAS ELECTRONICS PRODUCTS WILL BE INVULNERABLE OR FREE FROM CORRUPTION, ATTACK, VIRUSES, INTERFERENCE,
HACKING, DATA LOSS OR THEFT, OR OTHER SECURITY INTRUSION (“Vulnerability Issues”). RENESAS ELECTRONICS DISCLAIMS ANY AND
ALL RESPONSIBILITY OR LIABILITY ARISING FROM OR RELATED TO ANY VULNERABILITY ISSUES. FURTHERMORE, TO THE EXTENT
PERMITTED BY APPLICABLE LAW, RENESAS ELECTRONICS DISCLAIMS ANY AND ALL WARRANTIES, EXPRESS OR IMPLIED, WITH
RESPECT TO THIS DOCUMENT AND ANY RELATED OR ACCOMPANYING SOFTWARE OR HARDWARE, INCLUDING BUT NOT LIMITED TO
THE IMPLIED WARRANTIES OF MERCHANTABILITY, OR FITNESS FOR A PARTICULAR PURPOSE.
8. When using Renesas Electronics products, refer to the latest product information (data sheets, user’s manuals, application notes, “General Notes for
Handling and Using Semiconductor Devices” in the reliability handbook, etc.), and ensure that usage conditions are within the ranges specified by
Renesas Electronics with respect to maximum ratings, operating power supply voltage range, heat dissipation characteristics, installation, etc. Renesas
Electronics disclaims any and all liability for any malfunctions, failure or accident arising out of the use of Renesas Electronics products outside of such
specified ranges.
9. Although Renesas Electronics endeavors to improve the quality and reliability of Renesas Electronics products, semiconductor products have specific
characteristics, such as the occurrence of failure at a certain rate and malfunctions under certain use conditions. Unless designated as a high reliability
product or a product for harsh environments in a Renesas Electronics data sheet or other Renesas Electronics document, Renesas Electronics products
are not subject to radiation resistance design. You are responsible for implementing safety measures to guard against the possibility of bodily injury,
injury or damage caused by fire, and/or danger to the public in the event of a failure or malfunction of Renesas Electronics products, such as safety
design for hardware and software, including but not limited to redundancy, fire control and malfunction prevention, appropriate treatment for aging
degradation or any other appropriate measures. Because the evaluation of microcomputer software alone is very difficult and impractical, you are
responsible for evaluating the safety of the final products or systems manufactured by you.
10. Please contact a Renesas Electronics sales office for details as to environmental matters such as the environmental compatibility of each Renesas
Electronics product. You are responsible for carefully and sufficiently investigating applicable laws and regulations that regulate the inclusion or use of
controlled substances, including without limitation, the EU RoHS Directive, and using Renesas Electronics products in compliance with all these
applicable laws and regulations. Renesas Electronics disclaims any and all liability for damages or losses occurring as a result of your noncompliance
with applicable laws and regulations.
11. Renesas Electronics products and technologies shall not be used for or incorporated into any products or systems whose manufacture, use, or sale is
prohibited under any applicable domestic or foreign laws or regulations. You shall comply with any applicable export control laws and regulations
promulgated and administered by the governments of any countries asserting jurisdiction over the parties or transactions.
12. It is the responsibility of the buyer or distributor of Renesas Electronics products, or any other party who distributes, disposes of, or otherwise sells or
transfers the product to a third party, to notify such third party in advance of the contents and conditions set forth in this document.
13. This document shall not be reprinted, reproduced or duplicated in any form, in whole or in part, without prior written consent of Renesas Electronics.
14. Please contact a Renesas Electronics sales office if you have any questions regarding the information contained in this document or Renesas
Electronics products.
(Note1) “Renesas Electronics” as used in this document means Renesas Electronics Corporation and also includes its directly or indirectly controlled
subsidiaries.
(Note2) “Renesas Electronics product(s)” means any product developed or manufactured by or for Renesas Electronics.
(Rev.5.0-1 October 2020)
Corporate Headquarters Contact Information
TOYOSU FORESIA, 3-2-24 Toyosu, For further information on a product, technology, the most up-to-date
Koto-ku, Tokyo 135-0061, Japan version of a document, or your nearest sales office, please visit:
www.renesas.com www.renesas.com/contact/
Trademarks
Renesas and the Renesas logo are trademarks of Renesas Electronics
Corporation. All trademarks and registered trademarks are the property
of their respective owners.
© 2023 Renesas Electronics Corporation. All rights reserved.