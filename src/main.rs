use zbus::{Connection, fdo};
use futures_lite::stream::StreamExt;

mod dbus_upower;

// The zbus book is a pretty good general reference on DBus

// Example zbus signal watcher: https://dbus2.github.io/zbus/client.html#signals

// See the UPower DBus API reference:
// https://upower.freedesktop.org/docs/ref-dbus.html
// Note that the whole DBus API isn't documented there

// List system bus names:
//   busctl --system list
// Using a bus from the previous command, list e.g.:
//   busctl --system tree org.freedesktop.UPower

// Here's how the UPower CLI operates: https://gitlab.freedesktop.org/upower/upower/-/blob/029651a96dfc8e8e1dc6eca79bd3bf23d3aeb5ce/tools/up-tool.c#L154-176

#[derive(Debug)]
enum Error {
    ZBus(zbus::Error),
    ZVariant(zbus::zvariant::Error),
    Serde(serde_json::Error),
}

impl From<zbus::zvariant::Error> for Error {
    fn from(value: zbus::zvariant::Error) -> Self {
        Self::ZVariant(value)
    }
}

impl From<zbus::Error> for Error {
    fn from(value: zbus::Error) -> Self {
        Self::ZBus(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

#[derive(Debug, serde::Serialize)]
struct CurrentBatteryStatus {
    percentage: f64,
    time_to_empty_seconds: i64,
    energy_rate_watts: f64,
    time_to_full_seconds: i64,
    state: dbus_upower::BatteryState,
    capacity_percent: f64,
    battery_level: dbus_upower::BatteryLevel,
}

#[async_std::main]
async fn main() -> std::result::Result<(), Error> {
    let connection = Connection::system().await?;

    // 1. get devices
    //    - use the UPower Enumerate method (or something like this)
    //    - If the value is set to "Battery", you will need to verify that the property power-supply has the value "true" before considering it as a laptop battery. Otherwise it will likely be the battery for a device of an unknown type.
    //    - also could potentially use the root "/"
    // 2. get device state
    // 3. watch signals on devices of interest (take these as arguments?)
    // 4. update stored device state (or just re-fetch?)
    // 5. print new device state
    //
    // 6. notify if capacity is below 75%?
    //
    // use cap-std?

    let properties_proxy = fdo::PropertiesProxy::builder(&connection)
        .destination("org.freedesktop.UPower")?
        .path("/org/freedesktop/UPower/devices/DisplayDevice")?
        .build()
        .await?;

    let device = dbus_upower::DeviceProxy::new(&connection).await?;

    let mut state = CurrentBatteryStatus {
        percentage: device.percentage().await?,
        time_to_empty_seconds: device.time_to_empty().await?,
        energy_rate_watts: device.energy_rate().await?,
        time_to_full_seconds: device.time_to_full().await?,
        state: device.state().await?,
        capacity_percent: device.capacity().await?,
        battery_level: device.battery_level().await?,
    };

    println!("{}", serde_json::to_string(&state)?);

    let mut changes_stream = properties_proxy.receive_properties_changed().await?;

    while let Some(change) = changes_stream.next().await {
        let args = change.args()?;

        for (name, value) in args.changed_properties().iter() {
            match *name {
                "Percentage" => state.percentage = value.try_into()?,
                "TimeToEmpty" => state.time_to_empty_seconds = value.try_into()?,
                "EnergyRate" => state.energy_rate_watts = value.try_into()?,
                "TimeToFull" => state.time_to_full_seconds = value.try_into()?,
                "State" => state.state = value.try_into()?,
                "Capacity" => state.capacity_percent = value.try_into()?,
                "BatteryLevel" => state.battery_level = value.try_into()?,
                _ => (),
            }
        }

        println!("{}", serde_json::to_string(&state)?);
    }

    Ok(())
}
