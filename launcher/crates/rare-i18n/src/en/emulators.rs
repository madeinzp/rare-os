strings! {
    /// Emulator power profile name, inside "Starts on {power} power."
    power_quiet: "Quiet",
    power_performance: "Performance",
    power_custom: "Custom",
    power_balanced: "Balanced",
    /// Short month names for the install date
    month_jan: "Jan",
    month_feb: "Feb",
    month_mar: "Mar",
    month_apr: "Apr",
    month_may: "May",
    month_jun: "Jun",
    month_jul: "Jul",
    month_aug: "Aug",
    month_sep: "Sep",
    month_oct: "Oct",
    month_nov: "Nov",
    month_dec: "Dec",
    /// Date: month is a short name, day and year are numbers, e.g. "Oct 27, 2026"
    date_fmt: "{month} {day}, {year}",
    /// Row line when an update exists and its checksum is published; tag is a version, date as above
    update_out_checksum: "{tag} is out, checksum published. Installed {date}.",
    /// Row line when an update exists without a published checksum
    update_out_no_checksum: "{tag} is out, no checksum published. Installed {date}.",
    /// Row line for an installed emulator with a version tag; power is the profile name
    installed_tag: "Installed {date}, {tag}. Starts on {power} power.",
    /// Row line for an installed emulator without a version tag
    installed: "Installed {date}. Starts on {power} power.",
    /// Empty state row title in Settings > Emulators
    none_title: "No emulators found",
    /// Empty state row description
    none_body: "Put emulator AppImages in the Applications folder in your home folder and they show up here.",
    /// Button while an emulator update is running (short: button)
    updating: "Updating",
    /// Button while checking for an update (short: button)
    checking: "Checking",
    /// Button: look for a newer emulator release (short: button)
    check_update: "Check for update",
}
