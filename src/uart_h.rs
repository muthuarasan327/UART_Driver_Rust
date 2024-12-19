#![allow(dead_code)] // Disable dead code warnings
#![allow(non_camel_case_types)] // Disable naming convention warnings

/* Define UART Base Address*/
const USART1_BASE_ADDRESS :*const u32 = 0x40011000 as *const u32;
const USART2_BASE_ADDRESS :*const u32 = 0x40004400 as *const u32;
const USART6_BASE_ADDRESS :*const u32 = 0x40011400 as *const u32;

/*Define Baud Rate */
const BAUD_9600 :u16 = 1667;
const BAUD_115200 :u16 = 139;

/* enum definition */
#[repr(u8)]
pub enum bool_status {
    True=1,
    False=0,
}

/*Register Offset Value Enum*/
#[repr(u8)]
pub enum Register_Offset {
    SR_OFFSET = 0x00,
    DR_OFFSET = 0x04,
    BRR_OFFSET = 0x08,
    CR1_OFFSET = 0x0C,
    CR2_OFFSET = 0x10,
    CR3_OFFSET = 0x14,
    GTPR_OFFSET = 0x18,
}

/* Uart Congiguration */
pub struct Uart_Config
{
    pub baudrate : u32,
    pub word_length : u8,
    pub stop_bit : u8,
    pub parity_enable : bool,
    pub parity : u8,
}

#[repr(C)]
pub struct USART_SR {
    bits: u16, // The 16-bit register
}

impl USART_SR {
    // Bit positions (define each bit position as a constant)
    const PE: u16 = 1 << 0;   // Parity Error (Bit 0)
    const FE: u16 = 1 << 1;   // Framing Error (Bit 1)
    const NF: u16 = 1 << 2;   // Noise Error (Bit 2)
    const ORE: u16 = 1 << 3;  // Overrun Error (Bit 3)
    const IDLE: u16 = 1 << 4; // IDLE line detected (Bit 4)
    const RXNE: u16 = 1 << 5; // Read Data Register Not Empty (Bit 5)
    const TC: u16 = 1 << 6;   // Transmission Complete (Bit 6)
    const TXE: u16 = 1 << 7;  // Transmit Data Register Empty (Bit 7)
    const LBD: u16 = 1<< 8; 
    const CTS: u16 = 1<< 9; 
    const RES1: u16 = 1<< 10; 
    const RES2: u16 = 1<< 11; 
    const RES3: u16 = 1<< 12; 
    const RES4: u16 = 1<< 13; 
    const RES5: u16 = 1<< 14; 
    const RES6: u16 = 1<< 15; 

    // Setter for a specific bit
    fn set_bit(&mut self, mask: u16) {
        self.bits |= mask;
    }

    // Clear a specific bit
    fn clear_bit(&mut self, mask: u16) {
        self.bits &= !mask;
    }

    // Check if a specific bit is set
    fn is_bit_set(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    }
}

#[repr(C)]
pub struct USART_DR
{
    bits:u16,
}

impl USART_DR {
    const DR:u16 = 0x1FF;
    const RES:u16 = 0xFE00;

    fn set_bit(&mut self, mask: u16) {
        self.bits |= mask;
    }

    // Clear a specific bit
    fn clear_bit(&mut self, mask: u16) {
        self.bits &= !mask;
    }

    // Check if a specific bit is set
    fn is_bit_set(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    }
}

#[repr(C)]
struct USART_BRR
{
    brr : u16,
}

#[repr(C)]
pub struct USART_CR1
{
    bits :u16,
}

impl USART_CR1 
{
    const SBK: u16 = 1 << 0;
    const RWU: u16 = 1 << 1;
    const RE: u16 = 1 << 2;
    const TE: u16 = 1 << 3;
    const IDLEIE: u16 = 1 << 4;
    const RXNEIE: u16 = 1 << 5;
    const TCIE: u16 = 1 << 6;
    const TXEIE: u16 = 1 << 7;
    const PEIE: u16 = 1 << 8;
    const PS: u16 = 1 << 9;
    const PCE: u16 = 1 << 10;
    const WAKE: u16 = 1 << 11;
    const M: u16 = 1 << 12;
    const UE: u16 = 1 << 13;
    const RES: u16 = 1 << 14;
    const OVER8: u16 = 1 << 15;   

    fn set_bit(&mut self, mask: u16) {
        self.bits |= mask;
    }

    // Clear a specific bit
    fn clear_bit(&mut self, mask: u16) {
        self.bits &= !mask;
    }

    // Check if a specific bit is set
    fn is_bit_set(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    } 
}

#[repr(C)]
pub struct USART_CR2
{
    bits :u16,
}

impl USART_CR2 
{
    const ADD: u16 = 1 << 0x0f;
    const RES: u16 = 1 << 4;
    const LBDL: u16 = 1 << 5;
    const LBDIE: u16 = 1 << 6;
    const RES1: u16 = 1 << 7;
    const LBCL: u16 = 1 << 8;
    const CPHA: u16 = 1 << 9;
    const CPOL: u16 = 1 << 10;
    const CLKEN: u16 = 1 << 11;
    const STOP: u16 = 1 << 12;
    const STOP1: u16 = 1 << 13;
    const LINEN: u16 = 1 << 14;
    const RES2: u16 = 1 << 15;   

    fn set_bit(&mut self, mask: u16) {
        self.bits |= mask;
    }

    // Clear a specific bit
    fn clear_bit(&mut self, mask: u16) {
        self.bits &= !mask;
    }

    // Check if a specific bit is set
    fn is_bit_set(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    } 
}

#[repr(C)]
pub struct USART_CR3
{
    bits :u16,
}

impl USART_CR3 
{
    const EIE: u16=1<<0;
    const IREN: u16 = 1 << 1;
    const IRLP: u16 = 1 << 2;
    const HDSEL: u16 = 1 << 3;
    const NACK: u16 = 1 << 4;
    const SCEN: u16 = 1 << 5;
    const DMAR: u16 = 1 << 6;
    const DMAT: u16 = 1 << 7;
    const RTSE: u16 = 1 << 8;
    const CTSE: u16 = 1 << 9;
    const CTSIE: u16 = 1 << 10;
    const ONEBIT: u16 = 1 << 11;
    const RES: u16 = 1 << 12;  
    const RES1: u16 = 1 << 13;
    const RES2: u16 = 1 << 14;
    const RES3: u16 = 1 << 15; 

    fn set_bit(&mut self, mask: u16) {
        self.bits |= mask;
    }

    // Clear a specific bit
    fn clear_bit(&mut self, mask: u16) {
        self.bits &= !mask;
    }

    // Check if a specific bit is set
    fn is_bit_set(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    } 
}

#[repr(C)]
pub struct USART_GTPR
{
    bits :u16,
}

impl USART_GTPR {
    const PSC :u16  = 0x0FF;
    const GT  :u16  = 0xFF00;

    fn set_bit(&mut self, mask: u16) {
        self.bits |= mask;
    }

    // Clear a specific bit
    fn clear_bit(&mut self, mask: u16) {
        self.bits &= !mask;
    }

    // Check if a specific bit is set
    fn is_bit_set(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    }
}

#[repr(C)]
pub struct USART {
    sr: USART_SR,
    dr: USART_DR,
    brr: USART_BRR,
    cr1: USART_CR1,
    cr2: USART_CR2,
    cr3: USART_CR3,
    gtpr: USART_GTPR,
}

/*Decleard the UART*/
pub const USART1: *mut USART = USART1_BASE_ADDRESS as *mut USART;
pub const USART2: *mut USART = USART2_BASE_ADDRESS as *mut USART;
pub const USART6: *mut USART = USART6_BASE_ADDRESS as *mut USART;

pub fn uart_transmit(usart_base: *const USART, data: u8) {
    // Safety: Accessing raw pointers and offsets requires unsafe block
    unsafe {
        // Calculate the address of the Status Register (SR) using its offset
        let sr_addr = usart_base.add(Register_Offset::SR_OFFSET as usize) as *const u16;
        let dr_addr = usart_base.add(Register_Offset::DR_OFFSET as usize) as *mut u16;

        // Wait until the TXE (Transmit Data Register Empty) bit is set in the Status Register
        while (*sr_addr & USART_SR::TXE) == 0 {}

        // Write the data to the Data Register
        *dr_addr = data as u16;
    }
}

pub fn uart_transmit_string(usart_base: *const USART, str: &str) {
    for byte in str.bytes() {
        uart_transmit(usart_base, byte);
    }
}

pub fn uart_receive(usart_base: *const USART) -> u8 {
    unsafe {
        let sr_addr = usart_base.add(Register_Offset::SR_OFFSET as usize) as *const u16;
        let dr_addr = usart_base.add(Register_Offset::DR_OFFSET as usize) as *const u16;

        // Wait until RXNE (Read Data Register Not Empty) is set
        while (*sr_addr & USART_SR::RXNE) == 0 {}

        // Read and return received data
        *dr_addr as u8
    }
}

pub fn uart_init(usart_base: *mut USART, config: Uart_Config) {
    unsafe {
        // Set Baud Rate (dynamically calculate if needed)
        let brr_addr = usart_base.add(Register_Offset::BRR_OFFSET as usize) as *mut u16;
        let baud = calculate_baud_rate(config.baudrate); // Implement a function to calculate the baud rate
        *brr_addr = baud;

        // Control Register 1 Configuration (USART_CR1)
        let cr1_addr = usart_base.add(Register_Offset::CR1_OFFSET as usize) as *mut u16;
        let mut cr1_val = 0;

        // Enable the transmitter and receiver
        cr1_val |= USART_CR1::TE;
        cr1_val |= USART_CR1::RE;

        // Configure word length, stop bits, and other settings
        if config.parity_enable {
            cr1_val |= USART_CR1::PCE;  // Enable parity checking
        }
        if config.word_length == 9 {
            cr1_val |= USART_CR1::M;  // 9 data bits mode
        }

        *cr1_addr = cr1_val;

        // Enable the USART peripheral
        let cr1_enable = usart_base.add(Register_Offset::CR1_OFFSET as usize) as *mut u16;
        *cr1_enable |= USART_CR1::UE;
    }
}

fn calculate_baud_rate(baud: u32) -> u16 {
    // Example of calculating the baud rate based on system clock.
    // You need to use your system clock (e.g., 16 MHz) and compute the BRR value.
    // This is just a simple placeholder.
    let system_clock = 16_000_000; // Example clock frequency of 16 MHz
    let br = system_clock / baud;
    br as u16
}
