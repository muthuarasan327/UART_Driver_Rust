#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f4::stm32f410;
use stm32f4xx_hal::{prelude::*, rcc::RccExt, gpio::GpioExt};
use panic_halt as _;
use core::clone::Clone;
use core::marker::Sized;

mod uart_h;

use crate::uart_h::USART2;
use crate::uart_h::Uart_Config;





#[entry]
fn main() -> !{

    // Get access to the device-specific peripherals
    let dp = stm32f410::Peripherals::take().unwrap();
    
    // Get the core peripherals
    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
    
    // Access the RCC and configure system clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    
    // Enable the GPIOA and USART2 peripherals
    let gpioa = dp.GPIOA.split();
    
    // Enable the USART2 peripheral clock (USART2 is on APB1, so we enable it here)
    let _ = rcc.apb1.set_usart2();
    
    // Configure the TX and RX pins (PA2 for TX, PA3 for RX)
    let tx_pin = gpioa.pa2.into_alternate();  // PA2 for TX (USART2)
    let rx_pin = gpioa.pa3.into_alternate(); // PA3 for RX (USART2)

    // Initialize Uart_Config with specific values
    let uart1_config = Uart_Config {
        baudrate: 9600,
        word_length: 8,
        stop_bit: 1,
        parity_enable: false,
        parity: 0,
    };

    uart_h::uart_init(USART2, uart1_config);
    uart_h::uart_transmit_string(USART2,"RUST PROGRAM");
    

    // Main loop
    loop {
        // Your code here (e.g., UART receive, other tasks)
        let receive_byte = uart_h::uart_receive(USART2);
        uart_h::uart_transmit(USART2, receive_byte);
    }
}
