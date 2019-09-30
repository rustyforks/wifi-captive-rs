use clap::{App, Arg};

use wifi_captive::network_manager::{
    errors::{Result,NetworkManagerError}, AccessPoint, AccessPointCredentials, Device, DeviceType, NetworkManager,
};

fn main() -> Result<()> {
    let matches = App::new(file!())
        .arg(
            Arg::with_name("ssid")
                .takes_value(true)
                .required(true)
                .help("Network ssid"),
        )
        .arg(
            Arg::with_name("PASSWORD")
                .takes_value(true)
                .required(true)
                .help("Network password"),
        )
        .get_matches();

    let manager = NetworkManager::new();

    let device = find_device(&manager)?;

    let wifi_device = device.as_wifi_device().unwrap();

    let access_points = wifi_device.get_access_points()?;

    let ap_index = find_access_point(&access_points, matches.value_of("ssid").unwrap())?;

    let credentials = AccessPointCredentials::Wpa {
        passphrase: matches.value_of("PASSWORD").unwrap().to_string(),
    };

    wifi_device.connect(&access_points[ap_index], &credentials)?;

    Ok(())
}

fn find_device(manager: &NetworkManager) -> Result<Device> {
    let devices = manager.get_devices()?;

    let index = devices
        .iter()
        .position(|d| *d.device_type() == DeviceType::WiFi);

    if let Some(index) = index {
        Ok(devices[index].clone())
    } else {
        Err(NetworkManagerError::Generic("Cannot find a WiFi device"))
    }
}

fn find_access_point(access_points: &[AccessPoint], ssid: &str) -> Result<usize> {
    if let Some(index) = access_points.iter().position(|ap| same_ssid(ap, ssid)) {
        Ok(index)
    } else {
        Err(NetworkManagerError::Generic("Access point not found"))
    }
}

fn same_ssid(ap: &AccessPoint, ssid: &str) -> bool {
    if let Ok(ap_ssid) = ap.ssid().as_str() {
        ap_ssid == ssid
    } else {
        false
    }
}
