//! System
//!
//! The SYSTEM/DPORT peripheral needs to be split into several logical parts.
//!
//! Example
//! ```no_run
//! let peripherals = Peripherals::take().unwrap();
//! let system = peripherals.SYSTEM.split();
//! let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
//! ```
#[cfg(not(esp32))]
type SystemPeripheral = crate::pac::SYSTEM;
#[cfg(esp32)]
type SystemPeripheral = crate::pac::DPORT;

/// Peripherals which can be enabled via [PeripheralClockControl]
pub enum Peripheral {
    Spi2,
    #[cfg(not(esp32c2))]
    Spi3,
    I2cExt0,
    #[cfg(not(any(esp32c2, esp32c3)))]
    I2cExt1,
    #[cfg(not(esp32c2))]
    Rmt,
    Ledc,
    #[cfg(any(esp32c2, esp32c3))]
    ApbSarAdc,
    #[cfg(any(esp32c2, esp32c3, esp32s3))]
    Gdma,
    #[cfg(any(esp32, esp32s2))]
    Dma,
    #[cfg(any(esp32s2, esp32s3))]
    Usb,
}

/// Controls the enablement of peripheral clocks.
pub struct PeripheralClockControl {
    _private: (),
}

impl PeripheralClockControl {
    /// Enables and resets the given peripheral
    pub fn enable(&mut self, peripheral: Peripheral) {
        let system = unsafe { &*SystemPeripheral::PTR };

        #[cfg(not(esp32))]
        let (perip_clk_en0, perip_rst_en0) = { (&system.perip_clk_en0, &system.perip_rst_en0) };
        #[cfg(esp32)]
        let (perip_clk_en0, perip_rst_en0) = { (&system.perip_clk_en, &system.perip_rst_en) };

        #[cfg(any(esp32c2, esp32c3, esp32s3))]
        let (perip_clk_en1, perip_rst_en1) = { (&system.perip_clk_en1, &system.perip_rst_en1) };

        match peripheral {
            Peripheral::Spi2 => {
                perip_clk_en0.modify(|_, w| w.spi2_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi2_rst().clear_bit());
            }
            #[cfg(not(esp32c2))]
            Peripheral::Spi3 => {
                perip_clk_en0.modify(|_, w| w.spi3_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi3_rst().clear_bit());
            }
            #[cfg(esp32)]
            Peripheral::I2cExt0 => {
                perip_clk_en0.modify(|_, w| w.i2c0_ext0_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.i2c0_ext0_rst().clear_bit());
            }
            #[cfg(not(esp32))]
            Peripheral::I2cExt0 => {
                perip_clk_en0.modify(|_, w| w.i2c_ext0_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.i2c_ext0_rst().clear_bit());
            }
            #[cfg(not(any(esp32c2, esp32c3)))]
            Peripheral::I2cExt1 => {
                perip_clk_en0.modify(|_, w| w.i2c_ext1_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.i2c_ext1_rst().clear_bit());
            }
            #[cfg(not(esp32c2))]
            Peripheral::Rmt => {
                perip_clk_en0.modify(|_, w| w.rmt_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.rmt_rst().clear_bit());
            }
            Peripheral::Ledc => {
                perip_clk_en0.modify(|_, w| w.ledc_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.ledc_rst().clear_bit());
            }
            #[cfg(any(esp32c2, esp32c3))]
            Peripheral::ApbSarAdc => {
                perip_clk_en0.modify(|_, w| w.apb_saradc_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.apb_saradc_rst().clear_bit());
            }
            #[cfg(any(any(esp32c2, esp32c3, esp32s3)))]
            Peripheral::Gdma => {
                perip_clk_en1.modify(|_, w| w.dma_clk_en().set_bit());
                perip_rst_en1.modify(|_, w| w.dma_rst().clear_bit());
            }
            #[cfg(esp32)]
            Peripheral::Dma => {
                perip_clk_en0.modify(|_, w| w.spi_dma_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi_dma_rst().clear_bit());
            }
            #[cfg(esp32s2)]
            Peripheral::Dma => {
                perip_clk_en0.modify(|_, w| w.spi2_dma_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi2_dma_rst().clear_bit());
                perip_clk_en0.modify(|_, w| w.spi3_dma_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi3_dma_rst().clear_bit());
            }
            #[cfg(any(esp32s2, esp32s3))]
            Peripheral::Usb => {
                perip_clk_en0.modify(|_, w| w.usb_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.usb_rst().clear_bit());
            }
        }
    }
}

/// Controls the configuration of the chip's clocks.
pub struct SystemClockControl {
    _private: (),
}

/// Controls the configuration of the chip's clocks.
pub struct CpuControl {
    _private: (),
}

/// Dummy DMA peripheral.
#[cfg(any(esp32, esp32s2))]
pub struct Dma {
    _private: (),
}

/// The SYSTEM/DPORT splitted into it's different logical parts.
pub struct SystemParts {
    _private: (),
    pub peripheral_clock_control: PeripheralClockControl,
    pub clock_control: SystemClockControl,
    pub cpu_control: CpuControl,
    #[cfg(any(esp32, esp32s2))]
    pub dma: Dma,
}

/// Extension trait to split a SYSTEM/DPORT peripheral in independent logical
/// parts
pub trait SystemExt {
    type Parts;

    /// Splits the SYSTEM/DPORT peripheral into it's parts.
    fn split(self) -> Self::Parts;
}

impl SystemExt for SystemPeripheral {
    type Parts = SystemParts;

    fn split(self) -> Self::Parts {
        Self::Parts {
            _private: (),
            peripheral_clock_control: PeripheralClockControl { _private: () },
            clock_control: SystemClockControl { _private: () },
            cpu_control: CpuControl { _private: () },
            #[cfg(any(esp32, esp32s2))]
            dma: Dma { _private: () },
        }
    }
}
