//! BLE Example https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/wifi_ble.rs
//!
//! - starts Bluetooth advertising
//! - offers one service with three characteristics (one is read/write, one is write only, one is read/write/notify)
//! - pressing the boot-button on a dev-board will send a notification if it is subscribed

//% FEATURES: esp-wifi esp-wifi/ble esp-hal/unstable
//% CHIPS: esp32 esp32s3 esp32c2 esp32c3 esp32c6 esp32h2

#![no_std]
#![no_main]
use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE //, AD_FLAG_LE_LIMITED_DISCOVERABLE
    },
    att::Uuid,
    Ble, HciConnector,
};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    main,
    rng::Rng,
    time,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::{ble::controller::BleConnector, init, EspWifiController};

struct BleCtrl {
    ble: Ble<'static>,
    hci: HciConnector<BleConnector<'static>>,
    wifi_ctrl: EspWifiController<'static>, 
}

impl BleCtrl {
    fn new() -> Self {
    
        let now = || time::now().duration_since_epoch().to_millis();
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
        let timg0 = TimerGroup::new(peripherals.TIMG0);

        let wifi_ctrl = init(
            timg0.timer0,
            Rng::new(peripherals.RNG),
            peripherals.RADIO_CLK,
        )
        .unwrap();
        
        let connector = BleConnector::new(&wifi_ctrl, peripherals.BT);
        let hci = HciConnector::new(connector, now);
        let ble = Ble::new(&hci);
        BleCtrl{ble, hci, wifi_ctrl}  
    }

    fn send_advertising_data(&self, data: &[u8]) {
        todo!("implement sending of {data:?}")
    }

}

#[main]
fn main() -> ! {
    esp_alloc::heap_allocator!(72 * 1024);
    let adv_svc_data:[u8;4] = [0x40, 0x2, 0xC4, 0x9];
    let _init_logger_from_env = esp_println::logger::init_logger_from_env();
    
    let ble_ctl = BleCtrl::new();
    ble_ctl.send_advertising_data(&adv_svc_data);

    // println!("{:?}", ble.init());
    // println!("{:?}", ble.cmd_set_le_advertising_parameters());
    // println!(
    //     "{:?}",
    //     ble.cmd_set_le_advertising_data(
    //         create_advertising_data(&[
    //             AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
    //             AdStructure::ServiceUuids16(&[Uuid::Uuid16(0xD2FC)]),
    //             AdStructure::CompleteLocalName("Hello"),
    //             AdStructure::ServiceData16 { uuid: 0xD2FC, data: &adv_svc_data }
    //         ])
    //         .unwrap()
    //     )
    // );
    // println!("{:?}", ble.cmd_set_le_advertise_enable(true));

    println!("started advertising");

    let delay = Delay::new();

    loop {
        delay.delay_millis(50);
    }

}
