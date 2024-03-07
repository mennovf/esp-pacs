#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - WIFI_MAC"]
    WIFI_MAC = 0,
    #[doc = "1 - WIFI_MAC_NMI"]
    WIFI_MAC_NMI = 1,
    #[doc = "2 - WIFI_PWR"]
    WIFI_PWR = 2,
    #[doc = "3 - WIFI_BB"]
    WIFI_BB = 3,
    #[doc = "4 - BT_MAC"]
    BT_MAC = 4,
    #[doc = "5 - BT_BB"]
    BT_BB = 5,
    #[doc = "6 - BT_BB_NMI"]
    BT_BB_NMI = 6,
    #[doc = "7 - LP_TIMER"]
    LP_TIMER = 7,
    #[doc = "8 - COEX"]
    COEX = 8,
    #[doc = "9 - BLE_TIMER"]
    BLE_TIMER = 9,
    #[doc = "10 - BLE_SEC"]
    BLE_SEC = 10,
    #[doc = "11 - I2C_MST"]
    I2C_MST = 11,
    #[doc = "12 - APB_CTRL"]
    APB_CTRL = 12,
    #[doc = "13 - GPIO"]
    GPIO = 13,
    #[doc = "14 - GPIO_NMI"]
    GPIO_NMI = 14,
    #[doc = "15 - SPI_INTR_1"]
    SPI_INTR_1 = 15,
    #[doc = "16 - SPI_INTR_2"]
    SPI_INTR_2 = 16,
    #[doc = "17 - UART0"]
    UART0 = 17,
    #[doc = "18 - UART1"]
    UART1 = 18,
    #[doc = "19 - LEDC"]
    LEDC = 19,
    #[doc = "20 - EFUSE"]
    EFUSE = 20,
    #[doc = "21 - RTC_CORE"]
    RTC_CORE = 21,
    #[doc = "22 - I2C_EXT0"]
    I2C_EXT0 = 22,
    #[doc = "23 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 23,
    #[doc = "24 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 24,
    #[doc = "25 - CACHE_IA"]
    CACHE_IA = 25,
    #[doc = "26 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 26,
    #[doc = "27 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 27,
    #[doc = "28 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 28,
    #[doc = "29 - SPI_MEM_REJECT_CACHE"]
    SPI_MEM_REJECT_CACHE = 29,
    #[doc = "30 - ICACHE_PRELOAD0"]
    ICACHE_PRELOAD0 = 30,
    #[doc = "31 - ICACHE_SYNC0"]
    ICACHE_SYNC0 = 31,
    #[doc = "32 - APB_ADC"]
    APB_ADC = 32,
    #[doc = "33 - DMA_CH0"]
    DMA_CH0 = 33,
    #[doc = "34 - SHA"]
    SHA = 34,
    #[doc = "35 - ECC"]
    ECC = 35,
    #[doc = "36 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 36,
    #[doc = "37 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 37,
    #[doc = "38 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 38,
    #[doc = "39 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 39,
    #[doc = "40 - Assist debug interrupt"]
    ASSIST_DEBUG = 40,
    #[doc = "41 - ETS_CORE0_PIF_PMS_SIZE"]
    ETS_CORE0_PIF_PMS_SIZE = 41,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::WIFI_MAC),
            1 => Ok(Interrupt::WIFI_MAC_NMI),
            2 => Ok(Interrupt::WIFI_PWR),
            3 => Ok(Interrupt::WIFI_BB),
            4 => Ok(Interrupt::BT_MAC),
            5 => Ok(Interrupt::BT_BB),
            6 => Ok(Interrupt::BT_BB_NMI),
            7 => Ok(Interrupt::LP_TIMER),
            8 => Ok(Interrupt::COEX),
            9 => Ok(Interrupt::BLE_TIMER),
            10 => Ok(Interrupt::BLE_SEC),
            11 => Ok(Interrupt::I2C_MST),
            12 => Ok(Interrupt::APB_CTRL),
            13 => Ok(Interrupt::GPIO),
            14 => Ok(Interrupt::GPIO_NMI),
            15 => Ok(Interrupt::SPI_INTR_1),
            16 => Ok(Interrupt::SPI_INTR_2),
            17 => Ok(Interrupt::UART0),
            18 => Ok(Interrupt::UART1),
            19 => Ok(Interrupt::LEDC),
            20 => Ok(Interrupt::EFUSE),
            21 => Ok(Interrupt::RTC_CORE),
            22 => Ok(Interrupt::I2C_EXT0),
            23 => Ok(Interrupt::TG0_T0_LEVEL),
            24 => Ok(Interrupt::TG0_WDT_LEVEL),
            25 => Ok(Interrupt::CACHE_IA),
            26 => Ok(Interrupt::SYSTIMER_TARGET0),
            27 => Ok(Interrupt::SYSTIMER_TARGET1),
            28 => Ok(Interrupt::SYSTIMER_TARGET2),
            29 => Ok(Interrupt::SPI_MEM_REJECT_CACHE),
            30 => Ok(Interrupt::ICACHE_PRELOAD0),
            31 => Ok(Interrupt::ICACHE_SYNC0),
            32 => Ok(Interrupt::APB_ADC),
            33 => Ok(Interrupt::DMA_CH0),
            34 => Ok(Interrupt::SHA),
            35 => Ok(Interrupt::ECC),
            36 => Ok(Interrupt::FROM_CPU_INTR0),
            37 => Ok(Interrupt::FROM_CPU_INTR1),
            38 => Ok(Interrupt::FROM_CPU_INTR2),
            39 => Ok(Interrupt::FROM_CPU_INTR3),
            40 => Ok(Interrupt::ASSIST_DEBUG),
            41 => Ok(Interrupt::ETS_CORE0_PIF_PMS_SIZE),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
