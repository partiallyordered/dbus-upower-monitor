//! # DBus interface proxy for: `org.freedesktop.UPower.Device`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/UPower/devices/battery_BAT0' from service 'org.freedesktop.UPower' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/devices/battery_BAT0"
)]
trait Device {
    /// GetHistory method
    fn get_history(
        &self,
        type_: &str,
        timespan: u32,
        resolution: u32,
    ) -> zbus::Result<Vec<(u32, f64, u32)>>;

    /// GetStatistics method
    fn get_statistics(&self, type_: &str) -> zbus::Result<Vec<(f64, f64)>>;

    /// Refresh method
    fn refresh(&self) -> zbus::Result<()>;

    /// BatteryLevel property
    #[dbus_proxy(property)]
    fn battery_level(&self) -> zbus::Result<u32>;

    /// Capacity property
    #[dbus_proxy(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    /// ChargeCycles property
    #[dbus_proxy(property)]
    fn charge_cycles(&self) -> zbus::Result<i32>;

    /// Energy property
    #[dbus_proxy(property)]
    fn energy(&self) -> zbus::Result<f64>;

    /// EnergyEmpty property
    #[dbus_proxy(property)]
    fn energy_empty(&self) -> zbus::Result<f64>;

    /// EnergyFull property
    #[dbus_proxy(property)]
    fn energy_full(&self) -> zbus::Result<f64>;

    /// EnergyFullDesign property
    #[dbus_proxy(property)]
    fn energy_full_design(&self) -> zbus::Result<f64>;

    /// EnergyRate property
    #[dbus_proxy(property)]
    fn energy_rate(&self) -> zbus::Result<f64>;

    /// HasHistory property
    #[dbus_proxy(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    /// HasStatistics property
    #[dbus_proxy(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    /// IconName property
    #[dbus_proxy(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    /// IsPresent property
    #[dbus_proxy(property)]
    fn is_present(&self) -> zbus::Result<bool>;

    /// IsRechargeable property
    #[dbus_proxy(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    /// Luminosity property
    #[dbus_proxy(property)]
    fn luminosity(&self) -> zbus::Result<f64>;

    /// Model property
    #[dbus_proxy(property)]
    fn model(&self) -> zbus::Result<String>;

    /// NativePath property
    #[dbus_proxy(property)]
    fn native_path(&self) -> zbus::Result<String>;

    /// Online property
    #[dbus_proxy(property)]
    fn online(&self) -> zbus::Result<bool>;

    /// Percentage property
    #[dbus_proxy(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    /// PowerSupply property
    #[dbus_proxy(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    /// Serial property
    #[dbus_proxy(property)]
    fn serial(&self) -> zbus::Result<String>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// Technology property
    #[dbus_proxy(property)]
    fn technology(&self) -> zbus::Result<u32>;

    /// Temperature property
    #[dbus_proxy(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    /// TimeToEmpty property
    #[dbus_proxy(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    /// TimeToFull property
    #[dbus_proxy(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> zbus::Result<u32>;

    /// UpdateTime property
    #[dbus_proxy(property)]
    fn update_time(&self) -> zbus::Result<u64>;

    /// Vendor property
    #[dbus_proxy(property)]
    fn vendor(&self) -> zbus::Result<String>;

    /// Voltage property
    #[dbus_proxy(property)]
    fn voltage(&self) -> zbus::Result<f64>;

    /// WarningLevel property
    #[dbus_proxy(property)]
    fn warning_level(&self) -> zbus::Result<u32>;
}
