//! Direct Memory Access

use crate::{
    dma::gdma::private::*,
    system::{Peripheral, PeripheralClockControl},
};

macro_rules! impl_channel {
    ($num: literal) => {
        paste::paste! {
            pub struct [<Channel $num>] {}

            impl RegisterAccess for [<Channel $num>] {
                fn init_channel() {
                    // nothing special to be done here
                }

                fn set_out_burstmode(burst_mode: bool) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<out_conf0_ch $num>].modify(|_,w| {
                        w.[<out_data_burst_en_ch $num>]().bit(burst_mode)
                            .[<outdscr_burst_en_ch $num>]().bit(burst_mode)
                    });

                    #[cfg(esp32s3)]
                    dma.[<out_conf0_ch $num>].modify(|_,w| {
                        w.out_data_burst_en_ch().bit(burst_mode)
                            .outdscr_burst_en_ch().bit(burst_mode)
                    });
                }

                fn set_out_priority(priority: DmaPriority) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<out_pri_ch $num>].write(|w| {
                        w.[<tx_pri_ch $num>]().variant(priority as u8)
                    });

                    #[cfg(esp32s3)]
                    dma.[<out_pri_ch $num>].write(|w| {
                        w.tx_pri_ch().variant(priority as u8)
                    });
                }

                fn clear_out_interrupts() {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<int_clr_ch $num>].write(|w| {
                        w.[<out_eof_ch $num _int_clr>]()
                            .set_bit()
                            .[<out_dscr_err_ch $num _int_clr>]()
                            .set_bit()
                            .[<out_done_ch $num _int_clr>]()
                            .set_bit()
                            .[<out_total_eof_ch $num _int_clr>]()
                            .set_bit()
                            .[<outfifo_ovf_ch $num _int_clr>]()
                            .set_bit()
                            .[<outfifo_udf_ch $num _int_clr>]()
                            .set_bit()
                    });

                    #[cfg(esp32s3)]
                    dma.[<out_int_clr_ch $num>].write(|w| {
                        w.out_eof_ch_int_clr()
                            .set_bit()
                            .out_dscr_err_ch_int_clr()
                            .set_bit()
                            .out_done_ch_int_clr()
                            .set_bit()
                            .out_total_eof_ch_int_clr()
                            .set_bit()
                            .outfifo_ovf_l1_ch_int_clr()
                            .set_bit()
                            .outfifo_ovf_l3_ch_int_clr()
                            .set_bit()
                            .outfifo_udf_l1_ch_int_clr()
                            .set_bit()
                            .outfifo_udf_l3_ch_int_clr()
                            .set_bit()
                    });
                }

                fn reset_out() {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<out_conf0_ch $num>].modify(|_, w| w.[<out_rst_ch $num>]().set_bit());
                    #[cfg(not(esp32s3))]
                    dma.[<out_conf0_ch $num>].modify(|_, w| w.[<out_rst_ch $num>]().clear_bit());

                    #[cfg(esp32s3)]
                    dma.[<out_conf0_ch $num>].modify(|_, w| w.out_rst_ch().set_bit());
                    #[cfg(esp32s3)]
                    dma.[<out_conf0_ch $num>].modify(|_, w| w.out_rst_ch().clear_bit());
                }

                fn set_out_descriptors(address: u32) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<out_link_ch $num>]
                        .modify(|_, w| unsafe { w.[<outlink_addr_ch $num>]().bits(address) });

                    #[cfg(esp32s3)]
                    dma.[<out_link_ch $num>].modify(|_, w| unsafe { w.outlink_addr_ch().bits(address) });
                }

                fn has_out_descriptor_error() -> bool {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(esp32s3)]
                    let ret = dma.[<out_int_raw_ch $num>].read().out_dscr_err_ch_int_raw().bit();

                    #[cfg(not(esp32s3))]
                    let ret = dma.[<int_raw_ch $num>].read().[<out_dscr_err_ch $num _int_raw>]().bit();

                    ret
                }

                fn set_out_peripheral(peripheral: u8) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<out_peri_sel_ch $num>]
                        .modify(|_, w| w.[<peri_out_sel_ch $num>]().variant(peripheral));

                    #[cfg(esp32s3)]
                    dma.[<out_peri_sel_ch $num>].modify(|_, w| w.peri_out_sel_ch().variant(peripheral));
                }

                fn start_out() {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<out_link_ch $num>]
                        .modify(|_, w| w.[<outlink_start_ch $num>]().set_bit());

                    #[cfg(esp32s3)]
                    dma.[<out_link_ch $num>].modify(|_, w| w.outlink_start_ch().set_bit());
                }

                fn is_out_done() -> bool {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    let ret = dma.[<int_raw_ch $num>].read().[<out_total_eof_ch $num _int_raw>]().bit();

                    #[cfg(esp32s3)]
                    let ret = dma.[<out_int_raw_ch $num>].read().out_total_eof_ch_int_raw().bit();

                    ret
                }

                fn set_in_burstmode(burst_mode: bool) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<in_conf0_ch $num>].modify(|_,w| {
                        w.[<in_data_burst_en_ch $num>]().bit(burst_mode)
                            .[<indscr_burst_en_ch $num>]().bit(burst_mode)
                    });

                    #[cfg(esp32s3)]
                    dma.[<in_conf0_ch $num>].modify(|_,w| {
                        w.in_data_burst_en_ch().bit(burst_mode).indscr_burst_en_ch().bit(burst_mode)
                    });
                }

                fn set_in_priority(priority: DmaPriority) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<in_pri_ch $num>].write(|w| {
                        w.[<rx_pri_ch $num>]().variant(priority as u8)
                    });

                    #[cfg(esp32s3)]
                    dma.[<in_pri_ch $num>].write(|w| {
                        w.rx_pri_ch().variant(priority as u8)
                    });
                }

                fn clear_in_interrupts() {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<int_clr_ch $num>].write(|w| {
                        w.[<in_suc_eof_ch $num _int_clr>]()
                            .set_bit()
                            .[<in_err_eof_ch $num _int_clr>]()
                            .set_bit()
                            .[<in_dscr_err_ch $num _int_clr>]()
                            .set_bit()
                            .[<in_dscr_empty_ch $num _int_clr>]()
                            .set_bit()
                            .[<in_done_ch $num _int_clr>]()
                            .set_bit()
                            .[<infifo_ovf_ch $num _int_clr>]()
                            .set_bit()
                            .[<infifo_udf_ch $num _int_clr>]()
                            .set_bit()
                    });

                    #[cfg(esp32s3)]
                    dma.[<in_int_clr_ch $num>].write(|w| {
                        w.in_suc_eof_ch_int_clr()
                            .set_bit()
                            .in_err_eof_ch_int_clr()
                            .set_bit()
                            .in_dscr_err_ch_int_clr()
                            .set_bit()
                            .in_dscr_empty_ch_int_clr()
                            .set_bit()
                            .in_done_ch_int_clr()
                            .set_bit()
                            .infifo_ovf_l1_ch_int_clr()
                            .set_bit()
                            .infifo_ovf_l3_ch_int_clr()
                            .set_bit()
                            .infifo_udf_l1_ch_int_clr()
                            .set_bit()
                            .infifo_udf_l3_ch_int_clr()
                            .set_bit()
                    });
                }

                fn reset_in() {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<in_conf0_ch $num>].modify(|_, w| w.[<in_rst_ch $num>]().set_bit());
                    #[cfg(not(esp32s3))]
                    dma.[<in_conf0_ch $num>].modify(|_, w| w.[<in_rst_ch $num>]().clear_bit());

                    #[cfg(esp32s3)]
                    dma.[<in_conf0_ch $num>].modify(|_, w| w.in_rst_ch().set_bit());
                    #[cfg(esp32s3)]
                    dma.[<in_conf0_ch $num>].modify(|_, w| w.in_rst_ch().clear_bit());
                }

                fn set_in_descriptors(address: u32) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<in_link_ch $num>]
                        .modify(|_, w| unsafe { w.[<inlink_addr_ch $num>]().bits(address) });

                    #[cfg(esp32s3)]
                    dma.[<in_link_ch $num>].modify(|_, w| unsafe { w.inlink_addr_ch().bits(address) });
                }

                fn has_in_descriptor_error() -> bool {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    let ret = dma.[<int_raw_ch $num>].read().[<in_dscr_err_ch $num _int_raw>]().bit();

                    #[cfg(esp32s3)]
                    let ret = dma.[<in_int_raw_ch $num>].read().in_dscr_err_ch_int_raw().bit();

                    ret
                }

                fn set_in_peripheral(peripheral: u8) {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<in_peri_sel_ch $num>]
                        .modify(|_, w| w.[<peri_in_sel_ch $num>]().variant(peripheral));

                    #[cfg(esp32s3)]
                    dma.[<in_peri_sel_ch $num>].modify(|_, w| w.peri_in_sel_ch().variant(peripheral));
                }

                fn start_in() {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    dma.[<in_link_ch $num>]
                        .modify(|_, w| w.[<inlink_start_ch $num>]().set_bit());

                    #[cfg(esp32s3)]
                    dma.[<in_link_ch $num>].modify(|_, w| w.inlink_start_ch().set_bit());
                }

                fn is_in_done() -> bool {
                    let dma = unsafe { &*crate::pac::DMA::PTR };

                    #[cfg(not(esp32s3))]
                    let ret = dma.[<int_raw_ch $num>].read().[<in_suc_eof_ch $num _int_raw>]().bit();

                    #[cfg(esp32s3)]
                    let ret = dma.[<in_int_raw_ch $num>].read().in_suc_eof_ch_int_raw().bit();

                    ret
                }
            }

            pub struct [<Channel $num TxImpl>] {}

            impl<'a> TxChannel<[<Channel $num>]> for [<Channel $num TxImpl>] {}

            pub struct [<Channel $num RxImpl>] {}

            impl<'a> RxChannel<[<Channel $num>]> for [<Channel $num RxImpl>] {}

            pub struct [<ChannelCreator $num>] {}

            impl [<ChannelCreator $num>] {
                pub fn configure<'a>(
                    self,
                    burst_mode: bool,
                    tx_descriptors: &'a mut [u32],
                    rx_descriptors: &'a mut [u32],
                    priority: DmaPriority,
                ) -> Channel<ChannelTx<'a, [<Channel $num TxImpl>], [<Channel $num>]>, ChannelRx<'a, [<Channel $num RxImpl>], [<Channel $num>]>, [<SuitablePeripheral $num>]> {
                    let mut tx_impl = [<Channel $num TxImpl>] {};
                    tx_impl.init(burst_mode, priority);

                    let tx_channel = ChannelTx {
                        descriptors: tx_descriptors,
                        burst_mode,
                        tx_impl: tx_impl,
                        _phantom: PhantomData::default(),
                    };

                    let mut rx_impl = [<Channel $num RxImpl>] {};
                    rx_impl.init(burst_mode, priority);

                    let rx_channel = ChannelRx {
                        descriptors: rx_descriptors,
                        burst_mode,
                        rx_impl: rx_impl,
                        _phantom: PhantomData::default(),
                    };

                    Channel {
                        tx: tx_channel,
                        rx: rx_channel,
                        _phantom: PhantomData::default(),
                    }
                }
            }

            pub struct [<SuitablePeripheral $num>] {}
            impl PeripheralMarker for [<SuitablePeripheral $num>] {}

            // with GDMA every channel can be used for any peripheral
            impl SpiPeripheral for [<SuitablePeripheral $num>] {}
            impl Spi2Peripheral for [<SuitablePeripheral $num>] {}
        }
    };
}

/// Crate private implementatin details
pub(crate) mod private {
    use crate::dma::{private::*, *};

    impl_channel!(0);
    #[cfg(not(esp32c2))]
    impl_channel!(1);
    #[cfg(not(esp32c2))]
    impl_channel!(2);
    #[cfg(esp32s3)]
    impl_channel!(3);
    #[cfg(esp32s3)]
    impl_channel!(4);
}

/// GDMA Peripheral
///
/// This offers the available DMA channels.
pub struct Gdma {
    _inner: crate::pac::DMA,
    pub channel0: ChannelCreator0,
    #[cfg(not(esp32c2))]
    pub channel1: ChannelCreator1,
    #[cfg(not(esp32c2))]
    pub channel2: ChannelCreator2,
    #[cfg(esp32s3)]
    pub channel3: ChannelCreator3,
    #[cfg(esp32s3)]
    pub channel4: ChannelCreator4,
}

impl Gdma {
    /// Create a DMA instance.
    pub fn new(
        dma: crate::pac::DMA,
        peripheral_clock_control: &mut PeripheralClockControl,
    ) -> Gdma {
        peripheral_clock_control.enable(Peripheral::Gdma);
        dma.misc_conf.modify(|_, w| w.ahbm_rst_inter().set_bit());
        dma.misc_conf.modify(|_, w| w.ahbm_rst_inter().clear_bit());
        dma.misc_conf.modify(|_, w| w.clk_en().set_bit());

        Gdma {
            _inner: dma,
            channel0: ChannelCreator0 {},
            #[cfg(not(esp32c2))]
            channel1: ChannelCreator1 {},
            #[cfg(not(esp32c2))]
            channel2: ChannelCreator2 {},
            #[cfg(esp32s3)]
            channel3: ChannelCreator3 {},
            #[cfg(esp32s3)]
            channel4: ChannelCreator4 {},
        }
    }
}
