strings! {
    /// Settings > System > Language: the row, its line and the list's "this one" mark
    language: "Language",
    language_body: "The language of Rare and its menus.",
    language_current: "Current",

    /// Settings > System: Desktop mode row title
    desktop_mode: "Desktop mode",
    desktop_mode_body: "A full desktop for installing apps and managing files.",
    /// Short: button on the Desktop mode row
    switch: "Switch",

    /// Quick settings upscaling rows: row labels (left of the chips)
    up_upscaling: "Upscaling",
    up_sharpness: "Sharpness",
    up_render_size: "Render size",
    /// Short: chip, sharpness level
    up_soft: "Soft",
    /// Short: chip, sharpness level
    up_medium: "Medium",
    /// Short: chip, sharpness level
    up_sharp: "Sharp",

    /// Settings > Network > Wi-Fi row lines
    net_checking: "Checking",
    net_managed_outside: "Managed outside Rare OS. Change networks in desktop mode.",
    /// {name} is the Wi-Fi network name
    net_connected_to: "Connected to {name}.",
    net_connected_no_internet: "Connected to {name}, no internet.",
    net_connected_internet: "Connected to the internet.",
    net_not_connected: "Not connected.",
    /// Bluetooth row lines
    net_bt_on_body: "On. Pair controllers and headphones below.",
    net_bt_blocked: "Off, and blocked by a switch or airplane mode.",
    net_bt_off: "Off.",
    net_bt_none: "No Bluetooth adapter found on this device.",
    /// Short: button while a Wi-Fi scan runs
    net_scanning: "Scanning",
    /// Short: button, scan for Wi-Fi networks again
    net_scan_again: "Scan again",
    /// Hint bar: start a Wi-Fi scan
    net_scan: "Scan",
    /// Row title when the network wants a web sign-in (hotel, airport)
    net_needs_sign_in: "This network needs a sign-in",
    net_needs_sign_in_body: "Open its sign-in page to get online.",
    net_forget_title: "Forget this network",
    /// {name} is the Wi-Fi network name
    net_forget_body: "{name} is not joined automatically after this.",
    /// Short: button and hint, forget a saved network or Bluetooth device
    net_forget: "Forget",
    /// Bluetooth devices panel
    net_devices: "Devices",
    net_devices_looking: "Looking for controllers and headphones. Put yours in pairing mode.",
    net_devices_idle: "Put a controller or headphones in pairing mode, then look for devices.",
    /// Short: button while looking for Bluetooth devices
    net_looking: "Looking",
    /// Short: button and hint
    net_look_for_devices: "Look for devices",
    /// Bluetooth device state: a pair or connect is in progress
    net_working: "Working",
    net_paired: "Paired",
    net_not_paired: "Not paired",
    /// Short: button, pair with a Bluetooth device (verb)
    net_pair: "Pair",
    net_none_found: "No networks found",
    net_none_found_body: "Move closer to your router, then scan again.",
    /// Short: Wi-Fi list status, a join is in progress (also hint)
    net_joining: "Joining",
    /// Short: Wi-Fi list label, the network needs a password
    net_password: "Password",
    /// Hint bar: join this Wi-Fi network
    net_join: "Join",

    /// Restart-to-update sheet (controller only)
    os_restart_title: "Restart to update",
    os_nothing_title: "Nothing to install",
    /// {version} is the version number
    os_version: "Version {version}",
    os_up_to_date: "Rare OS is up to date.",
    os_restart_body: "The update is downloaded and installs while the device restarts. It takes a minute or two. Your games and files stay as they are.",
    /// {notes} is the release notes text, several lines
    os_whats_new: "What's new\n{notes}",
    os_game_running: "A game is running. Close it first so nothing is lost.",
    os_later_body: "Later keeps the update waiting. It installs the next time the device restarts, and the launcher reminds you again next time it opens.",
    os_power_off_warn: "Hold the power button to switch off only if you have to. A forced power-off can lose a waiting update, though it downloads again by itself.",
    /// Short: button while restarting
    os_restarting: "Restarting",
    /// Short: button and hint
    os_restart_now: "Restart now",
    /// Short: button and hint, postpone the update
    os_later: "Later",
    /// Restore previous system sheet
    os_restore_title: "Restore previous system",
    os_restore_sub: "Go back to the system this device ran before Rare OS.",
    os_restore_body: "The next start is the previous system. Rare OS stays installed, and your files and games are shared by both.",
    os_restore_back: "To come back to Rare OS, pick it in the boot menu.",
    /// Short: button and hint
    os_restore_restart: "Restore and restart",
    /// Short: button and hint
    os_restore_later: "Restore, restart later",

    /// Settings > Parental controls
    pc_title: "Parental controls",
    pc_body: "Set a 4 to 6 digit PIN to turn them on. The PIN is then needed for purchases, approving in Rare Wallet, and changing System, Network, Accounts and Developer settings.",
    /// Short: button
    pc_set_pin: "Set PIN",
    /// Row title: the PIN itself
    pc_pin: "PIN",
    pc_pin_body: "Needed for purchases, approving in Rare Wallet, these rules and locked settings. It stays unlocked for 5 minutes once entered.",
    /// Short: button
    pc_change_pin: "Change PIN",
    /// Section label; {name} is the profile name
    pc_rules_for: "Rules for {name}",
    pc_played_today: "Played today",
    pc_daily: "Daily play time",
    pc_daily_body: "When it runs out, a parent's PIN is needed to start a game.",
    pc_hours: "Allowed hours",
    pc_hours_body: "Outside these hours, a parent's PIN is needed to start a game.",
    /// Short: choice chip, no hour limit
    pc_any_time: "Any time",
    /// Short: choice chip, limit play to a time window
    pc_set_hours: "Set hours",
    /// Slider label: start hour
    pc_from: "From",
    /// Slider label: end hour
    pc_to: "To",
    pc_age: "Age rating",
    pc_age_body: "From the Steam store's ESRB and PEGI ratings. Games without a rating stay allowed.",
    pc_off_toast: "Parental controls are off",
    /// PIN prompt title
    pc_parent_pin: "Parent's PIN",
    /// PIN prompt line; {label} is a settings section name
    pc_enter_pin_to_open: "Enter the PIN to open {label}.",

    /// Settings > Power. Idle times: {n} is a number
    pw_never: "Never",
    pw_hours: "{n} h",
    pw_minutes: "{n} min",
    /// Battery care lines; {limit}, {low}, {floor} are percentages (numbers, no % sign)
    pw_care_off: "Charging stops at {limit}% while plugged in and starts again at {low}%, so the battery ages more slowly.",
    pw_care_unplugged: "Charging stops at {limit}% the next time you plug in.",
    pw_care_low: "Charging normally until the battery is above {floor}%.",
    pw_care_hot: "Charging normally while the battery is warm.",
    pw_care_unknown: "Charging normally while the battery level can't be read.",
    pw_care_charging: "Plugged in and charging to {limit}%.",
    pw_care_holding: "Plugged in and holding at {limit}%.",
    /// Battery health line, first sentence; {n} is a percentage number
    pw_wear_none: "As much charge as when it was new.",
    pw_wear: "Holds {n}% less charge than when it was new.",
    /// Battery health line, second sentence (follows the first after a space)
    pw_cycles_counted: "{n} charge cycle since Rare OS was installed.|{n} charge cycles since Rare OS was installed.",
    pw_cycles: "{n} charge cycle.|{n} charge cycles.",
    /// Short: choice chips (performance profile and fan)
    pw_quiet: "Quiet",
    pw_balanced: "Balanced",
    pw_performance: "Performance",
    pw_max: "Max",
    pw_custom: "Custom",
    pw_profile: "Performance profile",
    /// {watts} is a number of watts
    pw_profile_body: "{watts} W. Kept after a restart. Each game starts on Balanced unless you save settings for it in Quick settings.",
    pw_limit: "Power limit",
    pw_limit_body: "A limit of your own instead of a profile.",
    pw_cpu_boost: "CPU boost",
    pw_cpu_boost_body: "Faster CPU, more heat and battery.",
    pw_fan: "Fan",
    pw_fan_body: "Quiet stays low to about 65 C. Max spins up early. Custom opens the curve editor.",
    /// {now} is the live reading like "54 C, 2,898 rpm"; {what} is the pw_fan_body sentence
    pw_fan_now: "{now} now. {what}",
    pw_performance_title: "Performance",
    pw_no_control: "This device does not let Rare OS change its power limits.",
    pw_sleep: "Sleep when idle",
    pw_sleep_body: "Sleeps after this long with no input. Never while a game runs or sound plays. Press the power button to wake.",
    /// Short: button; {value} is an idle time like "15 min" or "Never"
    pw_on_battery: "On battery: {value}",
    pw_charging: "Charging: {value}",
    /// {limit} is a percentage number
    pw_care_title: "Battery care (stop at {limit}%)",
    pw_stop_at: "Stop charging at",
    pw_stop_at_body: "Lower keeps the battery healthy for longer. Higher lasts longer away from the charger.",
    /// {n} is a percentage number
    pw_health: "Battery health {n}%",

    /// Settings > Controls > Button layout sheet
    rm_title: "Button layout",
    rm_sub: "Applies to all games and the launcher. Rare and Quick always stay as they are.",
    rm_hint_list: "Pick a button to change what it does.",
    /// Short: footer button; A and B are pad button names
    rm_swap_ab: "Swap A and B",
    rm_swap_ab_back: "Swap A and B back",
    rm_swap_xy: "Swap X and Y",
    rm_swap_xy_back: "Swap X and Y back",
    rm_reset: "Reset to default",
    /// Heading in pick mode; {name} is a pad button name (A, Left bumper...)
    rm_what_does: "What {name} does",
    /// Choice chip: the button mapped to itself; {name} is a pad button name
    rm_own: "{name} (its own)",
    /// Hint bar: choose this target for the button
    rm_pick: "Pick",

    /// Settings > Developer / System: remote access (SSH) and network apps
    rem_less_than_minute: "less than a minute left",
    /// {n} is a number of minutes (2 or more)
    rem_minutes_left: "{n} minute left|{n} minutes left",
    rem_title: "Remote access (SSH)",
    rem_working: "Working on it.",
    rem_intro_on: "Computers on your network can sign in to this device.",
    rem_intro_off: "Sign in to this device from a computer to install things, edit files and run tools. Off until you turn it on.",
    rem_before: "Before you turn it on",
    rem_warning: "Anyone on your network with the password can control this device.",
    /// {host} is the device's network name, {ip} an IP address, {user} the sign-in user name
    rem_addr_ip: "{host}.local or {ip}, user {user}",
    rem_addr: "{host}.local, user {user}",
    rem_starting: "Starting",
    rem_starting_body: "The SSH server is starting. Check again in a moment.",
    rem_password: "Password",
    rem_pw_default: "The password is still the default one from before Rare OS, so password sign-in is off. Set a new one to sign in with a password.",
    rem_pw_on: "A password is set. Password sign-in is on.",
    rem_pw_off_key: "A password is set. Password sign-in is off because a key is added.",
    rem_pw_none: "No password yet. Set one, or add a key.",
    /// Short: button
    rem_set_new_password: "Set a new password",
    /// Short: button
    rem_set_password: "Set a password",
    rem_keys_none: "No keys yet. Keys are safer than a password: password sign-in turns off by itself once you add one.",
    /// {n} is a number of SSH keys
    rem_keys_n: "{n} key. Anyone with a matching private key can sign in.|{n} keys. Anyone with a matching private key can sign in.",
    rem_add_key: "Add a key",
    /// Short: button, add an SSH key by pasting or typing it
    rem_paste_or_type: "Paste or type",
    /// Short: button, add an SSH key from a USB stick
    rem_usb_stick: "USB stick",
    /// Key row title when the key has no comment; {kind} is a key type like ed25519
    rem_key_kind: "{kind} key",
    /// Key row title; {comment} is the key's comment, {kind} its type
    rem_key_comment: "{comment} ({kind})",
    rem_keep_pw: "Keep password sign-in on",
    rem_keep_pw_body: "Off by default once a key is added.",
    /// {left} is a time left like "42 minutes left"
    rem_auto_off_in: "SSH turns off by itself, {left}.",
    rem_auto_off_idle: "Switch SSH off again an hour from now.",
    rem_auto_off: "Turn off after 1 hour",
    rem_dev_tools: "Developer tools",
    rem_dev_tools_body: "Drive this device from your own tools and agents. The Local API and the SDK are at rare.market/developers.",
    /// {host} is the device's network name, {port} a port number
    rem_netapi_on: "Tools on your network can call the Local API at http://{host}.local:{port}. They need the token below.",
    rem_netapi_off: "Let tools and agents on a laptop call the Local API. Off by default.",
    rem_netapi: "Allow network apps",
    /// Row title: an access token
    rem_token: "Token",
    /// Short: button, make a new access token
    rem_new_token: "New token",
    rem_caps: "What network apps can do",
    rem_caps_body: "Each one is a separate switch. A request to buy or sell always opens a review sheet here, and nothing is signed until you press confirm on this device.",
    rem_activity: "Recent network activity",
    rem_activity_body: "The latest calls from tools on your network, also kept in the log.",
    rem_activity_none: "Nothing yet",
    rem_activity_none_body: "Calls from network apps show up here.",
    /// {cap} is a capability name, {from} the caller's IP address
    rem_call_allowed: "{cap} - from {from} - allowed",
    rem_call_refused: "{cap} - from {from} - refused",
    /// Relative times in the activity list, lower case; {n} is a number
    rem_just_now: "just now",
    rem_s_ago: "{n} s ago",
    rem_min_ago: "{n} min ago",
    rem_h_ago: "{n} h ago",
    /// System section SSH row; {command} is a shell command like "ssh user@host.local"
    rem_sys_on: "On. {command}. Keys and passwords are in Developer.",
    rem_sys_off: "Off. Turn it on to sign in from a computer. More options in Developer.",
    /// On-screen keyboard titles for the SSH password, public key and GitHub user name
    rem_ask_password: "Password for remote sign-in",
    rem_ask_key: "Public key (starts with ssh-ed25519 or ssh-rsa)",
    rem_ask_github: "GitHub user name",
    /// Confirm sheet title; {n} is a number of keys
    rem_gh_title: "Add {n} key from GitHub?|Add {n} keys from GitHub?",
    /// {user} is a GitHub user name
    rem_gh_body: "Anyone who controls the GitHub account {user} will be able to sign in to this device. Only continue if that is you.",

    /// Settings > Controls > LC / RC key shortcut sheet; {name} is the button, LC or RC
    sc_title: "{name} key shortcut",
    sc_sub: "{name} holds these keys down for as long as you hold it. Dictation apps and other push-to-talk apps listen for a key like this.",
    sc_hint: "Press the keys on a keyboard, or pick them below.",
    sc_none_picked: "No keys picked",
    /// Small heading above the modifier key chips (Ctrl, Alt...)
    sc_hold_with: "Hold with",
    /// Small heading above the key chips
    sc_key: "Key",
    sc_pick_one: "Pick at least one key.",

    /// Settings > Sound
    snd_volume: "Volume",
    snd_volume_body: "Left and right to change.",
    snd_no_output: "No sound output found.",
    /// {name} is the output device's name
    snd_playing_on: "Playing on {name}.",
    snd_pick_output: "Pick where sound plays.",
    snd_output: "Output",
    snd_louder: "Louder speakers",
    snd_louder_body: "More volume from the built-in speakers. A limiter keeps loud parts clean.",
    snd_microphone: "Microphone",
    /// {name} is the microphone's name
    snd_mic_body: "{name}. Off mutes it for every app.",
    snd_mic_level: "Mic level",
    snd_mic_level_body: "How loud you sound to others.",
    snd_no_mic: "No microphone found.",
    snd_sounds: "Sounds",
    snd_sounds_body: "Soft clicks when you move, select and go back.",

    /// Settings > Storage. Steam start choices: button label, then the row's line
    sto_steam_boot: "After startup",
    sto_steam_boot_body: "Steam starts quietly a little after the launcher. Steam games start fastest. Keeps Steam running, which costs some battery.",
    sto_steam_focus: "When I pick a game",
    sto_steam_focus_body: "Steam starts quietly when you open a Steam game, so Play starts fast. Uses nothing until then.",
    sto_steam_demand: "On Play",
    sto_steam_demand_body: "Steam starts only when you press Play. Uses nothing, and the first game takes a few seconds longer.",
    /// Steam Cloud line pieces, joined with ", "; {n} is a number of games
    sto_cloud_synced: "{n} game synced|{n} games synced",
    sto_cloud_behind: "{n} not synced yet",
    sto_cloud_conflict: "{n} with a conflict",
    /// Sort choices: short word on the Sort button, then the row's line
    sto_sort_size: "Size",
    sto_sort_size_body: "Biggest first",
    sto_sort_name: "Name",
    sto_sort_name_body: "By name",
    sto_sort_drive: "Drive",
    sto_sort_drive_body: "By drive, biggest first",
    /// {free} and {total} are sizes like "120 GB"
    sto_free_of: "{free} free of {total}",
    sto_fmt_testing: "Testing the card. This takes a minute or two.",
    sto_fmt_formatting: "Formatting",
    sto_fmt_almost: "Almost done",
    sto_this_device: "This device",
    /// Lower case, inside "Move to {name}" when the target is the internal drive
    sto_this_device_lc: "this device",
    sto_reading: "Reading the drives",
    sto_adding: "Adding it to Steam and Rare",
    /// {free} is a line like "120 GB free of 256 GB"
    sto_not_set_up: "{free}. Not set up for games yet.",
    sto_sd_card: "SD card",
    /// Short: button
    sto_use_for_games: "Use for games",
    /// Hint bar
    sto_add_hint: "Add to Steam and Rare",
    /// Short: button, format the SD card
    sto_format: "Format",
    /// Hint bar
    sto_format_hint: "Erase and format",
    sto_cant_hold: "This card can't hold games yet. Format it to use it.",
    sto_no_card: "No card in the slot. Put one in to get more space for games.",
    sto_games_apps: "Games and apps",
    sto_measuring: "Measuring",
    /// {sort} is a sort line like "Biggest first"
    sto_measuring_again: "{sort}. Measuring again",
    /// Short: button; {by} is a word like "Size" or "Name"
    sto_sort: "Sort: {by}",
    /// Hint bar
    sto_sort_hint: "Sort",
    /// Short: button
    sto_measure_again: "Measure again",
    sto_closing_steam: "Closing Steam",
    /// Move progress in the size column; {n} is a percentage number
    sto_copying: "Copying {n}%",
    sto_checking: "Checking {n}%",
    sto_finishing: "Finishing",
    /// Short: button and hint, move a game to another drive
    sto_move: "Move",
    /// Short: button; {name} is a drive name
    sto_move_to: "Move to {name}",
    /// Toast text
    sto_put_card: "Put an SD card in to move games to it.",
    /// Hint bar
    sto_cancel_move: "Cancel the move",
    /// Short: button, empty a cache
    sto_clear: "Clear",
    sto_nothing: "Nothing installed yet",
    sto_nothing_body: "Games and apps you install show up here with their size.",
    sto_showing_all: "Showing everything",
    sto_showing_biggest: "Showing the biggest",
    /// {shown} and {total} are numbers
    sto_x_of_y: "{shown} of {total}",
    sto_show_fewer: "Show fewer",
    sto_show_all: "Show all",
    /// Heading above the per-store game counts
    sto_sources: "Sources",
    sto_scan: "Scan for games",
    sto_scan_body: "Steam, Epic, GOG, itch.io and your Games folders.",
    sto_folder: "Games folder",
    sto_folder_body: "A folder with one game per subfolder, on this device or an SD card.",
    /// Short: button
    sto_add_folder: "Add folder",
    /// Line under a game folder
    sto_scanned: "Scanned for games.",
    sto_steam_start: "When Steam starts",
    sto_steam_cloud: "Steam Cloud",
    sto_grid: "SteamGridDB cover art",
    sto_grid_added: "API key added",
    sto_grid_body: "Optional. A free API key adds covers for games outside Steam.",
    /// Short: button
    sto_change_key: "Change key",
    /// Short: button
    sto_add_key: "Add key",
    sto_shots: "Screenshots and clips",
    sto_shots_body: "Take a screenshot or keep the last 30 seconds, and share them as a link on rare.market.",
    /// Short: button
    sto_open_share: "Open Share",
    /// Toasts from the storage buttons
    sto_wait_card: "Wait for the SD card to finish.",
    sto_wait_move_current: "Wait for the current move to finish.",
    sto_wait_move: "Wait for the move to finish.",
    sto_android: "Android's system and data. Uninstall Android apps one by one to free space.",
    sto_saves: "Saves live here. They go when you uninstall the game.",
    sto_roms: "Delete ROMs from the game page of a recent game, or in Desktop mode.",
    sto_uninstall_desktop: "Uninstall this in Desktop mode.",

    /// Settings > System > Support (time zone, logs, factory reset)
    sup_time_zone: "Time zone",
    /// Row line; {city} {region} are place names like "New York" and "America", {time} the clock
    sup_zone: "{city}, {region}. {time}",
    sup_zone_utc: "UTC. {time}",
    sup_not_set: "Not set",
    sup_auto_time: "Automatic time",
    sup_auto_time_body: "Sets the clock from the internet.",
    sup_logs: "Logs for a bug report",
    sup_logs_body: "The launcher and system logs, in Documents. Keys, passwords and wallets are never in them.",
    /// Short: button while the logs are being made
    sup_saving: "Saving",
    /// Short: button while the logs upload
    sup_uploading: "Uploading",
    /// Short: button, upload the logs and get a web link
    sup_get_link: "Get a link",
    sup_reset: "Factory reset",
    sup_reset_body: "Sign out and clear Rare, or erase everything on this device.",
    sup_needs_update: "Needs the next system update.",
    /// Short: button
    sup_reset_btn: "Reset",
    /// Used when a USB drive has no name
    sup_the_drive: "the drive",
    /// Toast; {zone} is a city name or UTC
    sup_zone_set: "Time zone set to {zone}",
    /// Error toast starts (the reason follows)
    sup_err_zone: "Couldn't change the time zone.",
    sup_err_ntp: "Couldn't change automatic time.",
    sup_err_save: "Couldn't save the logs.",
    sup_err_copy: "Couldn't copy the logs.",
    sup_err_upload: "Couldn't upload the logs.",
    sup_err_upload_hint: "Save them instead and copy them to a USB drive.",
    sup_err_reset: "Couldn't start the reset.",
    sup_err_reboot: "The reset is set for the next start. Restart to finish.",
    /// Toast; {path} is a folder path
    sup_saved_to: "Saved to {path}.",
    sup_saved_to_usb_hint: "Saved to {path}. To put them on a USB drive, plug one in and save again.",
    sup_copy_title: "Copy the logs to the USB drive?",
    sup_copy_body: "A copy goes on the drive, so you can attach it to a bug report from a computer.",
    /// Toast; {name} is the USB drive's name
    sup_copied: "Copied to {name}. You can remove the drive.",
    /// PIN prompt line for the factory reset
    sup_reset_pin: "Enter the parent's PIN to reset this device.",
    sup_not_reset: "The device was not reset.",
    sup_restarting: "Restarting to reset this device.",

    /// Time zone picker sheet
    tz_sub: "Pick your region, then the city nearest you.",
    /// Hint bar: choose this region or city
    tz_choose: "Choose",
    tz_cant_read: "Couldn't read the list of time zones.",
    tz_common: "Common time zones",
    /// Hint line; LB and RB are the pad's shoulder buttons
    tz_jump: "LB and RB jump a letter",
    /// Factory reset sheet. Choice titles (also buttons) and what each does
    rs_keep: "Keep games",
    rs_keep_body: "Signs out and clears Rare on this device: settings, accounts, wallets, friends, notices, Wi-Fi networks and Bluetooth pairings. Installed games and saves stay.",
    rs_erase: "Erase everything",
    rs_erase_body: "Removes everything you added: games, saves, screenshots, files, accounts, wallets and Wi-Fi networks. Rare OS stays installed.",
    rs_wallets: "Wallets on this device are removed. Have the recovery phrase for each one before you go on, or they can't be brought back.",
    rs_sub: "Pick what to keep. The device restarts to reset.",
    /// {word} is the word the player types to confirm; it stays RESET in every language
    rs_confirm: "The device restarts and resets before Rare opens again. This can't be undone. To go on, type {word}.",
    /// Short: button and hint that opens the keyboard
    rs_type: "Type {word}",
    /// Toast when the typed word is wrong
    rs_mismatch: "That didn't match. Type {word} to go on.",
    /// Logs link sheet
    lk_title: "Link to your logs",
    lk_sub: "Anyone with the link can download them for a month. It is copied already.",
    lk_scan: "Scan the code with a phone, or paste the link into your bug report.",
    /// Short: button
    lk_copy: "Copy link",
    /// Short: button after the link was copied
    lk_copied: "Copied",

    /// Settings > System > updates (rows under the OS version)
    upd_not_yet: "not yet",
    upd_yesterday: "yesterday",
    upd_check_failed: "Rare OS could not check for updates. It tries again by itself.",
    /// {error} is an English sentence from the updater
    upd_retry_suffix: "{error} It tries again by itself.",
    upd_offline: "No internet connection. It tries again by itself.",
    /// Row title; {version} is a version number
    upd_ready_to: "Ready. Restart to update to {version}",
    upd_ready: "Ready. Restart to update",
    upd_ready_body: "It is downloaded and installs while the device restarts. Restart now, or keep playing and do it later.",
    /// {done} and {total} are sizes like "2.1 GB"; {pct} a percentage number
    upd_progress: "{done} of {total} ({pct}%). It downloads in the background, so you can keep playing.",
    upd_downloads_bg: "It downloads in the background, so you can keep playing.",
    /// Row title; {version} is a version number
    upd_downloading: "Downloading Rare OS {version}",
    upd_downloading_generic: "Downloading the update",
    upd_checking: "Checking for updates",
    upd_checking_body: "Looking for a new version. A download goes on in the background.",
    upd_wait_battery: "It downloads by itself on the charger, or when the battery is at 30% or more. Update now downloads it at 15% or more.",
    upd_wait_game: "It downloads by itself after you finish playing. Update now downloads it during the game.",
    upd_wait_metered: "It downloads by itself on a network that is not metered. Update now downloads it on this one.",
    upd_wait_soon: "It downloads by itself soon.",
    upd_waiting: "Update waiting to download",
    upd_stopped: "The update stopped before it finished",
    upd_stopped_body: "It goes on from where it stopped within the hour, or now with Update now.",
    /// {done} and {total} are sizes like "2.1 GB"
    upd_stopped_partial: "It goes on from where it stopped within the hour, or now with Update now. {done} of {total} is already on the device.",
    upd_up_to_date: "Rare OS is up to date",
    upd_skipped: "A version that did not work on this device is skipped.",
    upd_unsupported: "Updates are not set up on this device",
    upd_no_source: "This build has no update source.",
    upd_could_not: "Could not update",
    /// {ago} is a relative time like "2 hours ago"
    upd_uptodate_checked: "Last checked {ago}. Updates download by themselves and install at the next restart.",
    upd_uptodate_unchecked: "Not checked yet. Updates download by themselves and install at the next restart.",
    upd_running: "The version running now.",
    upd_fell_back: "Back on the previous version",
    /// {version} is the version that did not start
    upd_fell_back_body: "Rare OS {version} did not start properly, so the device went back to this version by itself. {version} does not install again; the next version does.",
    /// Short: button that shows the updater's own error text
    upd_details: "Details",
    upd_hide_details: "Hide details",
    upd_show_details: "Show details",
    /// Hint bar
    upd_restart_hint: "Restart to update",
    /// Short: button, the update is downloading / a check is running
    upd_checking_btn: "Checking",
    /// Short: button, check for and download an update
    upd_now: "Update now",
    /// Hint bar
    upd_check_hint: "Check for updates",
    upd_whats_new: "What's new",
    upd_prev: "Previous version",
    /// {version} is a version number
    upd_prev_body: "Go back to Rare OS {version} if this version has a problem. Your games and files stay as they are.",
    /// Short: button
    upd_go_back: "Go back",
    /// Hint bar
    upd_go_back_hint: "Go back to the previous version",
    upd_restore_body: "Start the system this device ran before Rare OS. Rare OS stays installed and your files and games are shared.",
    /// Short: button
    upd_restore: "Restore",
    upd_boot: "Boot menu",
    upd_boot_set: "The boot menu shows on the next start, once. Pick an older version there if an update misbehaves.",
    upd_boot_body: "Show the boot menu on the next start, once, to pick an older version or another system.",
    /// Short: button
    upd_boot_btn: "Show next start",
    upd_boot_btn_set: "Set for next start",
    /// Hint bar
    upd_boot_hint: "Boot menu next start",

    /// Toasts and errors from Settings actions (the device runs them on workers). Errors are the start of a toast; the reason from the system follows
    lv_couldnt_look_for_networks: "Couldn't look for networks.",
    lv_couldnt_switch_the_output: "Couldn't switch the output.",
    lv_couldnt_change_the_speakers: "Couldn't change the speakers.",
    lv_couldnt_change_the_mic_level: "Couldn't change the mic level.",
    lv_couldnt_add_the_key: "Couldn't add the key.",
    lv_password_set: "Password set",
    lv_couldnt_set_the_password: "Couldn't set the password.",
    lv_that_isnt_a_public_key: "That isn't a public key. It starts with ssh-ed25519 or ssh-rsa.",
    lv_no_keys_found_put_a: "No keys found. Put a file called authorized_keys on a USB stick and plug it in.",
    lv_that_isnt_a_github_user: "That isn't a GitHub user name.",
    lv_key_removed: "Key removed",
    lv_couldnt_remove_the_key: "Couldn't remove the key.",
    lv_couldnt_change_that: "Couldn't change that.",
    lv_remote_access_turns_off_in: "Remote access turns off in an hour",
    lv_couldnt_set_the_timer: "Couldn't set the timer.",
    lv_copied: "Copied",
    lv_couldnt_change_network_apps: "Couldn't change network apps.",
    lv_couldnt_save_that: "Couldn't save that.",
    lv_new_token_made_the_old: "New token made. The old one stopped working.",
    lv_couldnt_make_a_new_token: "Couldn't make a new token.",
    lv_couldnt_change_the_volume: "Couldn't change the volume.",
    lv_couldnt_change_the_brightness: "Couldn't change the brightness.",
    lv_couldnt_change_night_light: "Couldn't change Night light.",
    lv_couldnt_change_the_tvs_picture: "Couldn't change the TV's picture.",
    lv_couldnt_change_the_performance_profile: "Couldn't change the performance profile.",
    lv_couldnt_change_cpu_boost: "Couldn't change CPU boost.",
    lv_couldnt_change_the_fan: "Couldn't change the fan.",
    lv_couldnt_change_the_charge_limit: "Couldn't change the charge limit.",
    lv_couldnt_change_battery_care: "Couldn't change battery care.",
    lv_that_didnt_work: "That didn't work.",
    lv_couldnt_check_for_updates: "Couldn't check for updates.",
    lv_the_update_check_did_not: "The update check did not start. Try again in a moment.",
    lv_couldnt_go_back_to_the: "Couldn't go back to the previous version.",
    lv_the_boot_menu_shows_on: "The boot menu shows on the next start",
    lv_couldnt_set_the_boot_menu: "Couldn't set the boot menu.",
    lv_couldnt_restore_the_previous_system: "Couldn't restore the previous system.",
    lv_couldnt_forget_that_network: "Couldn't forget that network.",
    lv_couldnt_turn_bluetooth_on: "Couldn't turn Bluetooth on.",
    lv_couldnt_forget_that_device: "Couldn't forget that device.",
    lv_mute_mic_failed: "Couldn't mute the mic.",
    lv_mic_on_failed: "Couldn't turn the mic on.",
    lv_gh_unreachable: "Couldn't reach GitHub. Check your connection.",
    /// {user} is a GitHub user name
    lv_gh_no_user: "GitHub has no user called {user}.",
    lv_gh_no_answer: "GitHub didn't answer. Try again in a moment.",
    lv_gh_too_large: "GitHub's answer was too large.",
    lv_gh_no_keys: "{user} has no public keys on GitHub.",
    lv_keys_already: "Those keys were already added",
    /// Toast; {n} is a number of keys
    lv_keys_added: "Added {n} key. Password sign-in turns off once a key is added.|Added {n} keys. Password sign-in turns off once a key is added.",
    lv_remote_on_setup: "Remote access is on. Set a password or add a key to sign in.",
    lv_remote_on: "Remote access is on",
    lv_remote_off: "Remote access is off",
    lv_remote_on_failed: "Couldn't turn on remote access.",
    lv_remote_off_failed: "Couldn't turn off remote access.",
    /// Confirm sheet title; {n} is a number of keys
    lv_usb_title: "Add {n} key from the USB stick?|Add {n} keys from the USB stick?",
    lv_usb_body: "Anyone with a matching private key will be able to sign in to this device.",
    /// Short: confirm button
    lv_add_keys: "Add keys",
    /// Used as the subtitle when the key has no name
    lv_this_key: "this key",
    lv_remove_title: "Remove this key?",
    /// {fingerprint} is the key's fingerprint, shown on its own line
    lv_remove_body: "The computer that uses it can no longer sign in.\n{fingerprint}",
    lv_netapps_on: "Network apps are allowed",
    lv_netapps_off: "Network apps are off",
    /// Used when a capability has no name
    lv_cap_that: "That",
    /// Toast; {name} is a capability name like "Library"
    lv_cap_on: "{name} is on",
    lv_cap_off: "{name} is off",
    /// {version} is a version number
    lv_os_version: "Rare OS {version}",
    lv_previous_version: "the previous version",
    /// {name} is "Rare OS 0.1.40" or "the previous version"
    lv_rollback_body: "The device restarts into {name}. Your games and files stay as they are.\nThis version stays on the device but does not install again by itself. Update now in Settings installs it again.",
    lv_rollback_title: "Go back to the previous version",
    /// Short: confirm button
    lv_rollback_yes: "Go back and restart",
    /// Toasts about an emulator; {name} is its name, {tag} a version tag
    lv_emu_current: "{name} is up to date",
    lv_emu_check_failed: "Couldn't check {name}.",
    lv_emu_updated: "{name} updated to {tag}",
    lv_emu_update_failed: "Couldn't update {name}.",
    /// Wi-Fi join errors; {ssid} is the network name
    lv_join_password: "Couldn't join {ssid}. Check the password and try again.",
    lv_join_failed: "Couldn't join {ssid}.",
    /// Used when a Bluetooth device has no name
    lv_the_device: "the device",
    /// Bluetooth toasts; {name} is the device's name
    lv_bt_forgot: "Forgot {name}",
    lv_bt_disconnected: "Disconnected {name}",
    lv_bt_connected: "Connected to {name}",
    lv_bt_connect_failed: "Couldn't connect to {name}.",
    lv_bt_pairing_hint: "Put it in pairing mode and try again.",

    lv_check_wifi_hint: "Check your Wi-Fi and try again.",
    /// Sound: the built-in output device
    snd_dev_speaker: "Speaker",
    /// Sound: the built-in input device
    snd_dev_builtin_mic: "Built-in mic",
    /// Sound: wired headphones as output
    snd_dev_headphones: "Headphones",
    /// Sound: a wired headset's microphone
    snd_dev_headset_mic: "Headset mic",
    /// Storage: the source line of files a removed game left behind
    sto_left_over: "Left over",
    /// Storage: a removed Steam game's leftover saves; appid is its Steam app id
    sto_leftover_steam: "Leftover saves, Steam game {appid}",
    /// Storage: leftover saves of something named; what is a folder name
    sto_leftover_named: "Leftover saves, {what}",
}
