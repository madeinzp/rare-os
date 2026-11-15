strings! {
    // Section names in the left list of Settings
    sec_system: "System",
    sec_display: "Display",
    sec_sound: "Sound",
    sec_controls: "Controls",
    sec_power: "Power",
    sec_network: "Network",
    sec_accounts: "Accounts",
    sec_devices: "Devices",
    sec_parental: "Parental controls",
    sec_storage: "Storage",
    sec_capture: "Capture and streaming",
    sec_notifications: "Notifications",
    sec_ask: "Ask Rare",
    sec_emulators: "Emulators",
    sec_developer: "Developer",
    sec_about: "About",
    sec_licences: "Credits and licences",
    /// Search field placeholder in the Settings section list
    search_placeholder: "Search settings",
    /// Hint on the credits rows: A opens the licence text
    read_licence: "Read the licence",
    /// Intro above the Credits and licences list
    credits_intro: "Rare OS is built from our own code and these open-source projects. Thank you to everyone who made them.",
    // About
    /// Title of the Rare Device Pass panel
    pass_title: "Rare Pass",
    /// Label in the pass panel: the pass number. short
    pass_label: "Pass",
    /// Label: the device model. short
    device_label: "Device",
    /// Label: when the pass was first seen on this device. short
    first_seen: "First seen",
    pass_kept: "Kept by this account. It stays with this device: if the device changes hands, its new owner can claim it.",
    pass_moved: "This device's Rare Pass moved to another account. Claim it again to bring it back here.",
    pass_intro: "One per device. Rare checks this SuiPlay's security chip, then adds a Rare Pass to your account that shows you own a genuine SuiPlay.",
    /// Button label while the claim is checking the device
    pass_checking: "Checking this device",
    /// Button label while the claim waits for the owner's approval
    pass_waiting: "Waiting for your approval",
    pass_claim: "Claim your Rare Pass",
    /// Hint on the claim button
    claim: "Claim",
    /// About rows: label for the operating system. short
    system_label: "System",
    /// About rows: the device's network name. short
    hostname_label: "Hostname",
    /// About rows: label for how the launcher is shown. short
    display_label: "Display",
    /// Value of the Display row when running full screen on the device
    game_mode: "Game mode",
    /// Value of the Display row when running in a window
    window_mode: "Window",
    serial_label: "Serial number",
    /// BIOS version label. short
    bios_label: "BIOS",
    /// Label for the launcher's own version. short
    launcher_label: "Launcher",
    /// Button on the About panel that opens Credits and licences
    credits_button: "Credits and licences",
    // Accounts
    /// Username row body when set; {username} is the name
    username_set: "{username}. You can change it once every 30 days.",
    /// Username row body when set and a SuiNS name exists; {name} is the SuiNS name
    username_set_suins: "{username}, SuiNS {name}. You can change it once every 30 days.",
    username_none: "Friends find you by it. Free, and nobody needs a SuiNS name.",
    /// {who} is the account's name or short address
    rare_signed_in: "Signed in as {who}. One sign-in for Rare Wallet and Rare Marketplace.",
    rare_signed_out: "One sign-in for Rare Wallet and Rare Marketplace. Your games work without an account.",
    /// {name} is the Steam profile name
    steam_signed_in: "Signed in as {name}. Sign out or switch accounts in Steam's settings.",
    steam_signed_out: "Sign in to see your Steam games in Library and start them from Rare.",
    acct_recovery_device: "Recovery phrase on this SuiPlay, signs with A",
    acct_key_device: "Private key on this SuiPlay, signs with A",
    acct_recovery_pin: "Recovery phrase, signs with your PIN",
    acct_key_pin: "Private key, signs with your PIN",
    acct_pin_account: "PIN account, signs here with your PIN, your phone or key is the backup",
    acct_passkey: "Passkey, signs with your phone or PIN",
    /// Kind of wallet account (hardware wallet), a product name
    acct_ledger: "Ledger",
    /// Kind of wallet account (hardware wallet), a product name
    acct_keystone: "Keystone",
    acct_watch: "Watch only, cannot sign",
    /// Kind of wallet account, fallback
    acct_generic: "Account",
    /// {kind} is the account kind text; the sentence ends the row's line
    session_signed_in: "{kind}. Signed in to rare.market.",
    /// {kind} is the account kind text; {date} like "Oct 27, 2026"
    session_since: "{kind}. Signed in to rare.market since {date}.",
    session_not: "{kind}. Not signed in to rare.market.",
    /// Account row line when it cannot sign; {kind} is the account kind text
    session_kind_only: "{kind}.",
    /// Twelve short month names separated by spaces, January first
    months_short: "Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec",
    /// A date; {month} is a short month name, {day} and {year} numbers, like "Oct 27, 2026"
    date_short: "{month} {day}, {year}",
    /// {amount} is a dollar number without the sign
    send_pin_on: "On. Sends over ${amount} ask for it. Everything else signs with A.",
    send_pin_off: "Off. Accounts on this SuiPlay sign with A alone. Turn it on to ask for a PIN before sends over ${amount}.",
    device_pin_body: "PIN accounts sign on this SuiPlay with it. 5 wrong PINs lock it until your phone or security key unlocks it.",
    /// Title of a setting row
    row_rare_account: "Rare account",
    /// Title of a setting row; Steam is a product name
    row_steam: "Steam",
    row_rare_wallet: "Rare Wallet",
    row_setup: "Setup",
    wallet_body: "Balances, collectibles and moving assets from your old SuiPlay wallet.",
    setup_body: "Go through the first-run steps again: your account, moving assets, Steam and favourites.",
    /// Button: short
    open_steam: "Open Steam",
    /// Button: short
    run_setup: "Run setup",
    /// Title of a setting row
    row_username: "Username",
    /// Button: short
    pick_username: "Pick a username",
    /// Account row title; {label} is the account's name
    acct_main_title: "{label} (main)",
    make_main: "Make this the main account",
    row_device_pin: "PIN on this SuiPlay",
    change_pin: "Change PIN",
    recovery_words: "Recovery words",
    row_big_sends: "PIN for big sends",
    /// Button: short
    set_pin: "Set a PIN",
    // Settings shell (mod.rs)
    /// Search results empty state; {query} is what the player typed
    no_match: "No settings match \"{query}\".",
    /// Hint on a search result row: A jumps to the setting
    go_to_setting: "Go to setting",
    locked_title: "Locked by parental controls",
    /// {section} is a section name
    locked_body: "Enter the parent's PIN to open {section}.",
    /// Hint on the locked row: A asks for the parent's PIN
    enter_pin: "Enter PIN",
    /// Toast when a setting could not be written to the config file
    save_failed: "Couldn't save that setting.",
    /// Toast: an address the player typed
    url_http: "That address needs to start with http:// or https://.",
    /// Toast: an address the player typed
    url_https: "That address needs to start with https://.",
    /// Hint bar: B clears the search
    clear_search: "Clear search",
    // Ask Rare
    ask_title: "Ask Rare",
    ask_intro: "Talk or type in the Rare menu's Ask tab, even over a game. Rare uses a model you bring and pays nothing. Your key is sealed in Rare Wallet and goes only to the provider you pick.",
    /// Model provider choice: any other OpenAI-compatible service. short
    provider_other: "Other",
    ask_pick: "Pick where the answers come from.",
    ask_claude: "Your Anthropic API key. Rare never registers with Anthropic.",
    ask_openai: "Your OpenAI API key. Sign in with ChatGPT is not offered.",
    ask_compat: "Any service that speaks the OpenAI API: a router or your own server.",
    ask_ollama: "Ollama on this device or a PC on your network. Works offline.",
    row_provider: "Model provider",
    /// Value when a setting has no value yet
    not_set: "Not set",
    /// {value} is the default model name or address
    default_value: "{value} (default)",
    row_model: "Model",
    row_address: "Address",
    key_saved: "Saved in Rare Wallet",
    key_optional: "Not set (only if your service needs one)",
    row_api_key: "API key",
    /// Button: short
    replace: "Replace",
    row_check: "Check it works",
    check_body: "Sends one short question to your model.",
    /// Button: short
    test: "Test",
    voice_title: "Speak to Rare",
    voice_body: "Hold X on the Ask tab and talk. Speech turns into text on this device; the speech model (190 MB) downloads the first time.",
    hold_title: "Hold RC to talk",
    hold_body: "Hold RC anywhere, even in a game, and let go to send. RC's own hold action is off while this is on.",
    shots_title: "Pictures of the game",
    shots_body: "Over a game, a screenshot can go to your model with the question, for help with what is on screen. This is the default; the Ask tab has a switch each time.",
    agents_title: "Agents on your network",
    agents_body: "Tools on a laptop can use the same actions over MCP: Developer > Allow network apps, one switch per kind of action. Buying and sending always wait for you here.",
    // Capture
    cap_no_recorder: "Needs gpu-screen-recorder, or GStreamer with VA-API. Neither is on this system.",
    cap_replay_off: "Off. Turn it on to save the last moments of a game.",
    /// {last} is a phrase like "the last 30 seconds"
    cap_replay_on: "Keeps {last} while a game runs, on the GPU's video encoder. Uses a little battery.",
    /// Appended after the replay line, a separate sentence
    cap_recording: "Recording now.",
    cap_presence_off: "Off. Friends on Discord don't see what you play on Rare.",
    cap_discord_closed: "Open Discord from Apps. Friends then see the game you play on Rare.",
    cap_discord_arrpc: "Discord on the web needs arRPC, which comes with the next Rare OS update.",
    /// {game} is a game title
    cap_friends_see: "Friends see: Playing {game}.",
    cap_discord_web: "Discord on the web is open. Friends see every game you play.",
    cap_discord_app: "The Discord app is open. Friends see games Discord knows.",
    cap_no_sunshine: "Needs Sunshine, which comes with the next Rare OS update.",
    /// {name} is the device name, like SuiPlay0X1
    cap_host_off: "Play this {name}'s games on a TV, a PC or a phone with Moonlight. Off until you turn it on.",
    /// {error} is a short error from Sunshine, in English
    cap_host_failed_why: "Sunshine couldn't start ({error}). Turn this off and on to try again.",
    cap_host_failed: "Sunshine couldn't start. Turn this off and on to try again.",
    cap_host_starting: "Starting.",
    /// {name} is the device name, {ip} its address
    cap_host_on_ip: "On. Moonlight on your network finds it as {name}, or add {ip}.",
    cap_host_on: "On. Moonlight on your network finds it as {name}.",
    row_replay: "Instant replay",
    row_length: "Length",
    length_body: "How much a saved clip keeps.",
    /// {used} is a size like "1.2 GB", {cap} a number of GB
    cap_used: "Clips use {used} of {cap} GB. The oldest go first.",
    row_clip_space: "Space for clips",
    row_save_clip: "Save a clip",
    save_clip_body: "Rare and View in a game, or Save in Quick settings and the Rare menu. Clips go to Share with the game's name.",
    /// Button: short
    open_share: "Open Share",
    row_presence: "Show what you play on Discord",
    voice_deafened: "Deafened: you hear nothing from Discord and your microphone is muted.",
    voice_muted: "Your microphone is muted in Discord.",
    voice_running: "Discord keeps running while you play. Mute and deafen are here and in the Rare menu.",
    voice_closed: "Discord keeps running while you play. Mute and deafen show here and in the Rare menu when it is open.",
    row_voice: "Voice",
    /// Button: short
    unmute: "Unmute",
    /// Button: short
    mute: "Mute",
    /// Button: short
    undeafen: "Undeafen",
    /// Button: short
    deafen: "Deafen",
    /// {name} is the device name
    row_stream: "Stream this {name}",
    host_pin_waiting: "A device is waiting for its PIN. Type the four digits it shows.",
    /// {name} is the device name
    host_pin_how: "In Moonlight, pick {name}. It shows a PIN: type it here.",
    row_pair: "Pair a device",
    /// Button: short
    enter_pin_btn: "Enter PIN",
    no_paired: "No devices paired yet",
    no_paired_body: "Paired devices show up here. Remove one to stop it from streaming.",
    unnamed_device: "Unnamed device",
    paired_body: "Paired. It can stream this device.",
    nearby_none: "No other Rare devices found on this network.",
    nearby_wait: "Rare devices on the same network show up here.",
    nearby_some: "On the same network. One with streaming on can be played from Stream in Library.",
    row_nearby: "Rare devices nearby",
    /// Button: short
    look_again: "Look again",
    /// {address} is the device's address, then a sentence
    nearby_line_on: "{address}. Streaming is on.",
    nearby_line_off: "{address}. Streaming is off.",
    // Controls
    /// Choice for what the LC or RC button does: take a screenshot or clip. short
    act_capture: "Capture",
    /// Choice for what the LC or RC button does: open the on-screen keyboard. short
    act_keyboard: "Keyboard",
    act_perf: "Performance overlay",
    act_shortcut: "Key shortcut",
    /// Choice for what the LC or RC button does: no action. short
    act_nothing: "Nothing",
    /// {keys} are the shortcut's keys, {button} is LC or RC
    shortcut_held: "{keys}. Held down for as long as you hold {button}, in Rare, games and Desktop mode.",
    shortcut_none: "No keys picked yet.",
    /// Row title; {button} is LC or RC
    shortcut_title: "{button} key shortcut",
    row_buttons: "Buttons",
    buttons_body: "The SuiPlay's own buttons. Rare and Quick always do the same thing, so no game or app can take them.",
    /// Row title: the button named Rare
    row_rare_btn: "Rare",
    rare_btn_body: "Press for the Rare menu, press again for Home, hold for the Switcher. Fixed.",
    /// Row title: the button named Quick
    row_quick_btn: "Quick",
    quick_btn_body: "Quick Settings. Fixed.",
    lc_body: "The small button at the top left. Press.",
    rc_body: "The small button at the top right. Press.",
    row_paddles: "Back paddles",
    paddles_body: "This device has none.",
    row_holds: "Holds",
    holds_body: "Holding LC saves the last 30 seconds and holding RC changes the performance overlay, unless the button is set to Nothing or a key shortcut. A key shortcut holds its keys down while you hold the button, for push-to-talk apps like dictation. In games both do nothing until you pick something below, except the keyboard: a button set to Keyboard opens it over games and apps too. Inside Steam Game Mode Steam keeps Rare and Quick.",
    row_in_games: "In games",
    in_games_body: "Menu is Start, View is Select, A B X Y are the console's buttons by where they sit. Rare and Quick never reach the game. Every emulator follows it.",
    /// Button: short
    show: "Show",
    row_lc_game: "LC in games",
    lc_game_body: "Press, while a game has the screen. A game's own page can pick its own.",
    row_rc_game: "RC in games",
    rc_game_body: "Press, while a game or app has the screen. Keyboard types into it, Steam's sign-in too.",
    row_desktop: "In Desktop mode",
    desktop_body: "The right stick moves the pointer and the left stick scrolls. A clicks, B right-clicks, Y opens the keyboard, the d-pad is the arrow keys. Rare opens the app menu. LC and RC do what you picked above.",
    row_games_desktop: "Games in Desktop mode",
    games_desktop_body: "A game or app in full screen gets the controller as a normal controller. Press Quick to switch between the two yourself.",
    row_deadzone_left: "Left stick deadzone",
    row_deadzone_right: "Right stick deadzone",
    row_invert_left: "Invert left stick, up and down",
    invert_left_body: "Pushing up moves down.",
    row_invert_right: "Invert right stick, up and down",
    invert_right_body: "For looking around in games.",
    gyro_none: "Motion controls.",
    gyro_active: "On. The controller carries motion right now.",
    gyro_tilt: "Tilt the device to aim in games that support it.",
    gyro_down: "The controller service isn't running. Restart the device and try again.",
    row_gyro: "Gyro",
    row_gyro_sens: "Gyro sensitivity",
    row_gyro_steam: "Gyro in Steam games",
    gyro_steam_body: "Motion controls in Steam games. The controller shows up as a Steam Deck while a Steam game runs. Set the sensitivity in the game's Steam controller settings.",
    row_rumble: "Rumble",
    rumble_body: "A light buzz on the same moves, on controllers that support it.",
    row_rumble_strength: "Rumble strength",
    /// {summary} is the current button layout, a sentence
    layout_applies: "{summary} Applies to all games and the launcher.",
    /// {summary} is the current button layout, a sentence
    layout_after_update: "{summary} Takes effect after the next system update.",
    /// {summary} is the current button layout, a sentence
    layout_too_old: "{summary} This version of Rare OS can't change buttons yet. An update brings it.",
    row_layout: "Button layout",
    // Developer
    dev_testnet_on: "Testnet: Sui reads, Rare Wallet and the Rare API use testnet. Nothing here costs real SUI.",
    dev_mainnet: "Mainnet. Switch to testnet to try a game with test SUI.",
    row_network: "Network",
    /// Choice: Sui mainnet. short
    mainnet: "Mainnet",
    /// Choice: Sui testnet. short
    testnet: "Testnet",
    /// {url} is the default address
    dev_default_rpc: "{url} (default)",
    row_testnet_rpc: "Testnet RPC",
    row_testnet_api: "Testnet market API",
    testnet_api_default: "rare.market with ?network=testnet (default)",
    row_testnet_sponsor: "Testnet gas sponsor",
    testnet_sponsor_default: "Same as the testnet market API (default)",
    /// Button: short; sets an override
    set: "Set",
    /// {url} {slug} are the dev game's address and name, {n} a number of collections
    dev_game_line: "{url}, {slug}, {n} collection|{url}, {slug}, {n} collections",
    dev_game_none: "None. Start the launcher with --dev-game <url> to run a local web game with the Local API.",
    /// {url} is the local API address
    dev_api_on: "Listening at {url}. Games started from Rare get RARE_API and RARE_TOKEN.",
    dev_api_off: "Not running.",
    row_local_api: "Local API",
    row_dev_game: "Developer game",
    // Devices (phone control)
    row_phone_control: "Phone control",
    phone_control_body: "Use rare.market on your phone to see what this device is doing and to control it: install games, type, open links, send pictures and files, and get notified when downloads finish.",
    row_sign_in_first: "Sign in first",
    sign_in_first_body: "Sign in with Rare Wallet here, then sign in to rare.market on your phone with the same account.",
    row_paired: "Paired",
    /// {who} is a short wallet address
    paired_body_phone: "Any phone signed in to {who} on rare.market/devices controls this device.",
    /// Button: short
    unpair: "Unpair",
    row_add_phone: "Add another phone",
    add_phone_body: "Sign in on rare.market with the same account. No new code needed.",
    /// {code} is the pairing code; {n} minutes left
    pair_scan_body: "Scan with your phone's camera, or open rare.market/devices/pair and type {code}. Sign in with the same account. The code works for {n} more minute.|Scan with your phone's camera, or open rare.market/devices/pair and type {code}. Sign in with the same account. The code works for {n} more minutes.",
    /// Row title; {code} is the pairing code
    pair_code_title: "Code {code}",
    row_waiting_phone: "Waiting for your phone",
    waiting_phone_body: "This page updates by itself once the phone confirms.",
    /// Button: short
    new_code: "New code",
    pair_working: "Working on it.",
    pair_intro: "Shows a code to scan with your phone. The phone needs the same account signed in at rare.market.",
    row_pair_phone: "Pair a phone",
    row_didnt_work: "Didn't work",
    row_recent_phone: "Recent from your phone",
    row_nothing_yet: "Nothing yet",
    nothing_yet_body: "Actions from your phone show up here.",
    /// Relative time under a phone action
    ago_now: "just now",
    /// {n} minutes ago, short
    ago_min: "{n} min ago",
    /// {n} hours ago, short
    ago_hour: "{n} h ago",
    /// {n} days ago, short
    ago_day: "{n} d ago",
    /// {when} is a relative time like "5 min ago"; the action did not work
    recent_failed: "{when}, didn't work",
    // Display
    ui_auto: "Automatic",
    ui_large: "Large",
    ui_larger: "Larger",
    built_in_screen: "Built-in screen",
    /// Fallback name of the connected display
    external_display: "external display",
    /// {name} is the display's name, {mode} its mode like 3840x2160
    docked_to_mode: "Docked to {name} {mode}",
    docked_to: "Docked to {name}",
    /// Toast when the TV or monitor is unplugged
    back_on_built_in: "Back on the built-in screen",
    /// {size} is the resolution, like 3840x2160
    docked_body: "The picture and the sound are on the display. Showing at {size}. Unplug the cable to come back to the built-in screen.",
    display_coming: "A display is plugged in and will take over in a moment.",
    display_plug: "Plug a TV or monitor into the USB-C port and the picture and the sound move to it.",
    row_ui_size: "UI size",
    ui_size_body: "Makes text and buttons bigger on top of the automatic size. Kept after a restart.",
    night_dim: "Night dim. Darker to look at, but it does not save battery.",
    left_right: "Left and right to change.",
    row_brightness: "Brightness",
    row_dim_idle: "Dim after idle",
    dim_idle_body: "Dims the screen to 30% after 60 seconds with no input in the launcher. Never in a game.",
    row_screen: "Screen",
    no_brightness: "This device has no screen brightness to change.",
    row_night_light: "Night light",
    night_light_body: "Warmer colours that are easier on the eyes at night. Games too.",
    row_warmth: "Warmth",
    row_night_only: "Only at night",
    night_only_body: "On from 9 PM to 7 AM. Off: on all the time while Night light is on.",
    /// A TV mode choice: the size the TV itself picks
    tv_own: "TV's own",
    row_resolution: "Resolution",
    resolution_body: "What games and Rare draw at. The TV shows it full screen. Lower is smoother in heavy games.",
    row_refresh: "Refresh rate",
    refresh_body: "How many times a second the TV draws a new picture.",
    /// Hint on a choice chip: A picks it
    choose: "Choose",
    // Settings search index: each row's title, and its search words (space separated, lowercase, in the language's own words; multi-word phrases may be split)
    ix_devices_pair: "Pair a phone",
    ixk_devices_pair: "phone remote control qr rare.market companion keyboard send install",
    ix_ssh_system: "Remote access (SSH)",
    ixk_ssh_system: "ssh remote terminal sign in password key mod",
    ix_language: "Language",
    ixk_language: "english locale text",
    ix_updates: "Updates",
    ixk_updates: "update upgrade version rollback boot menu system image restart install check",
    ix_restore_system: "Restore previous system",
    ixk_restore_system: "undo rollback gameos old system go back",
    ix_time_zone: "Time zone",
    ixk_time_zone: "timezone clock time date zone city",
    ix_time_auto: "Automatic time",
    ixk_time_auto: "ntp clock time sync",
    ix_logs_save: "Logs for a bug report",
    ixk_logs_save: "logs bug report support debug export usb",
    ix_factory_reset: "Factory reset",
    ixk_factory_reset: "reset erase wipe factory sell clear",
    ix_boot_menu: "Boot menu",
    ixk_boot_menu: "grub startup older version next start",
    ix_desktop_mode: "Desktop mode",
    ixk_desktop_mode: "kde desktop files install apps",
    ix_docked: "Docked",
    ixk_docked: "tv monitor hdmi usb-c external display resolution 4k 1080p",
    ix_ui_size: "UI size",
    ixk_ui_size: "scale bigger larger text size zoom tv",
    ix_brightness: "Brightness",
    ixk_brightness: "screen backlight night dim darker brighter",
    ix_dim_idle: "Dim after idle",
    ixk_dim_idle: "screen dim timeout battery",
    ix_night_light: "Night light",
    ixk_night_light: "warm blue light night eyes colour temperature color",
    ix_tv_size: "TV resolution",
    ixk_tv_size: "tv 4k 1080p resolution docked monitor",
    ix_tv_refresh: "TV refresh rate",
    ixk_tv_refresh: "hz 120 refresh tv",
    ix_volume: "Volume",
    ixk_volume: "audio speaker loud quiet headphones",
    ix_output: "Output",
    ixk_output: "audio speaker headphones bluetooth hdmi tv usb sound device switch",
    ix_mic: "Microphone",
    ixk_mic: "mic mute voice chat headset input",
    ix_mic_level: "Mic level",
    ixk_mic_level: "microphone input volume gain voice",
    ix_sounds: "Sounds",
    ixk_sounds: "clicks ui sounds sound effects mute",
    ix_buttons: "Buttons",
    ixk_buttons: "remap rare quick lc rc paddles map",
    ix_btn_lc: "LC button",
    ixk_btn_lc: "remap capture screenshot left",
    ix_btn_rc: "RC button",
    ixk_btn_rc: "remap keyboard overlay right",
    ix_remap: "Button layout",
    ixk_remap: "remap swap nintendo a and b x and y mapping buttons",
    ix_deadzone_left: "Left stick deadzone",
    ixk_deadzone_left: "drift stick analog",
    ix_deadzone_right: "Right stick deadzone",
    ixk_deadzone_right: "drift stick analog",
    ix_invert_left: "Invert left stick",
    ixk_invert_left: "flip stick vertical",
    ix_invert_right: "Invert right stick",
    ixk_invert_right: "flip stick look vertical",
    ix_gyro_on: "Gyro",
    ixk_gyro_on: "motion imu aim inputplumber",
    ix_gyro_sens: "Gyro sensitivity",
    ixk_gyro_sens: "motion aim speed",
    ix_rumble_strength: "Rumble strength",
    ixk_rumble_strength: "vibration haptics buzz strength",
    ix_rumble: "Rumble",
    ixk_rumble: "vibration haptics controller buzz gamepad",
    ix_gyro: "Gyro in Steam games",
    ixk_gyro: "motion steam deck controller aim gamepad",
    ix_profile: "Performance profile",
    ixk_profile: "quiet balanced performance watts tdp speed",
    ix_tdp: "Power limit",
    ixk_tdp: "watts custom slider processor cpu",
    ix_cpu_boost: "CPU boost",
    ixk_cpu_boost: "turbo processor heat faster",
    ix_fan: "Fan",
    ixk_fan: "cool quiet noise temperature heat",
    ix_sleep_idle: "Sleep when idle",
    ixk_sleep_idle: "suspend auto sleep timeout battery charging",
    ix_battery_care: "Battery care",
    ixk_battery_care: "80% charge limit charging battery life stop at 80",
    ix_battery_health: "Battery health",
    ixk_battery_health: "capacity wear battery",
    ix_wifi: "Wi-Fi",
    ixk_wifi: "wifi wireless internet network connect password scan",
    ix_wifi_forget: "Forget this network",
    ixk_wifi_forget: "wifi remove network saved network",
    ix_bluetooth: "Bluetooth",
    ixk_bluetooth: "wireless pair headphones controller",
    ix_username: "Username",
    ixk_username: "name friends profile suins",
    ix_rare_account: "Rare account",
    ixk_rare_account: "sign in login wallet google passkey",
    ix_setup: "Setup",
    ixk_setup: "first run onboarding passkey run again",
    ix_wallet: "Rare Wallet",
    ixk_wallet: "sui balances collectibles assets",
    ix_parental_pin: "Parental controls",
    ixk_parental_pin: "parental pin kids child family lock",
    ix_parental_daily: "Daily play time",
    ixk_parental_daily: "play time limit screen time kids",
    ix_parental_hours: "Allowed hours",
    ixk_parental_hours: "bedtime hours time kids",
    ix_parental_age: "Age rating",
    ixk_parental_age: "esrb pegi rating mature age",
    ix_scan: "Scan for games",
    ixk_scan: "library refresh steam epic gog itch",
    ix_games_folder: "Games folder",
    ixk_games_folder: "add folder sd card remove folder library path",
    ix_steam_start: "When Steam starts",
    ixk_steam_start: "steam background boot launch play",
    ix_steamgriddb: "SteamGridDB cover art",
    ixk_steamgriddb: "covers api key artwork images",
    ix_share: "Screenshots and clips",
    ixk_share: "capture record share video",
    ix_replay_toggle: "Instant replay",
    ixk_replay_toggle: "clip record replay save last video capture gpu-screen-recorder",
    ix_replay_length: "Replay length",
    ixk_replay_length: "clip seconds minutes replay",
    ix_replay_cap: "Space for clips",
    ixk_replay_cap: "clips storage limit space delete",
    ix_discord_presence: "Show what you play on Discord",
    ixk_discord_presence: "discord rich presence status playing arrpc",
    ix_discord_mute: "Discord voice",
    ixk_discord_mute: "discord mute deafen voice call microphone",
    ix_host_toggle: "Stream this device",
    ixk_host_toggle: "stream sunshine moonlight tv remote play host pair pin",
    ix_nearby_scan: "Rare devices nearby",
    ixk_nearby_scan: "network devices nearby lan find",
    ix_notif_dnd: "Do not disturb",
    ixk_notif_dnd: "quiet silence notices",
    ix_notif_toasts: "Show toasts over a game",
    ixk_notif_toasts: "popups in game notices",
    ix_notif_desktop: "Desktop notifications",
    ixk_notif_desktop: "background notices",
    ix_notif_alerts: "Your price alerts",
    ixk_notif_alerts: "price alert coin delete",
    ix_dev_ssh: "Remote access (SSH)",
    ixk_dev_ssh: "ssh remote terminal password key github usb qr mod login",
    ix_ask_provider: "Ask Rare",
    ixk_ask_provider: "ai assistant agent claude openai chatgpt ollama model llm voice talk api key",
    ix_ask_voice: "Speak to Rare",
    ixk_ask_voice: "voice microphone talk speech whisper push to talk",
    ix_ask_hold: "Hold RC to talk",
    ixk_ask_hold: "rc button voice assistant",
    ix_ask_shots: "Pictures of the game",
    ixk_ask_shots: "screenshot vision help boss privacy",
    ix_dev_netapi: "Allow network apps",
    ixk_dev_netapi: "local api lan agent token sdk tools laptop",
    ix_dev_netapi_caps: "What network apps can do",
    ixk_dev_netapi_caps: "permissions switches profile inventory library launch controller input purchase screenshot agent",
    ix_dev_netapi_activity: "Recent network activity",
    ixk_dev_netapi_activity: "log audit requests agent history",
    ix_dev_network: "Network",
    ixk_dev_network: "testnet mainnet sui developer switch",
    ix_dev_testnet_rpc: "Testnet RPC",
    ixk_dev_testnet_rpc: "fullnode endpoint sui",
    ix_dev_testnet_api: "Testnet market API",
    ixk_dev_testnet_api: "rare.market endpoint",
    ix_dev_testnet_sponsor: "Testnet gas sponsor",
    ixk_dev_testnet_sponsor: "endpoint gas",
    ix_dev_game: "Developer game",
    ixk_dev_game: "dev-game local api web game studio",
    ix_emulators: "Emulators",
    ixk_emulators: "update appimage retro",
    ix_about: "About Rare OS",
    ixk_about: "version device hostname system",
    ix_licences: "Credits and licences",
    ixk_licences: "open source thanks licenses",
    /// Search words every notification switch has
    ixk_notif: "notification notice",
    /// Notification switch title (Settings > Notifications)
    nk_sold: "Sold",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_sold: "One of your items sold, or someone took your listing.",
    /// Notification switch title (Settings > Notifications)
    nk_bought: "Bought",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_bought: "You bought an item, or your offer was accepted.",
    /// Notification switch title (Settings > Notifications)
    nk_offer_received: "Offers",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_offer_received: "A new offer on an item or collection you hold.",
    /// Notification switch title (Settings > Notifications)
    nk_coin_received: "Coins received",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_coin_received: "Someone sent you coins, above $1.",
    /// Notification switch title (Settings > Notifications)
    nk_nft_received: "NFTs received",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_nft_received: "Someone sent you an item from a collection on rare.market.",
    /// Notification switch title (Settings > Notifications)
    nk_airdrop: "Airdrops",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_airdrop: "An item minted straight to you.",
    /// Notification switch title (Settings > Notifications)
    nk_perp_liquidation: "Perps liquidation",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_perp_liquidation: "A perps position is near its liquidation price.",
    /// Notification switch title (Settings > Notifications)
    nk_price_alert: "Price alerts",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_price_alert: "A price alert you set was hit.",
    /// Notification switch title (Settings > Notifications)
    nk_followed_trade: "Followed traders",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_followed_trade: "A trader you follow made a trade.",
    /// Notification switch title (Settings > Notifications)
    nk_friend_request: "Friend requests",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_friend_request: "Someone wants to be your friend.",
    /// Notification switch title (Settings > Notifications)
    nk_friend_online: "Friends online",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_friend_online: "A friend came online or started a game.",
    /// Notification switch title (Settings > Notifications)
    nk_party_invite: "Party invites",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_party_invite: "A friend invited you to their party.",
    /// Notification switch title (Settings > Notifications)
    nk_direct_message: "Messages",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_direct_message: "A friend sent you a message.",
    /// Notification switch title (Settings > Notifications)
    nk_party_message: "Party messages",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_party_message: "A new message in your party chat.",
    /// Notification switch title (Settings > Notifications)
    nk_os_update: "Rare OS updates",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_os_update: "An update is ready and installs when you restart.",
    /// Notification switch title (Settings > Notifications)
    nk_download_done: "Downloads",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_download_done: "A Steam download finished.",
    /// Notification switch title (Settings > Notifications)
    nk_emu_update: "Emulator updates",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_emu_update: "A newer version of an emulator is available.",
    /// Notification switch title (Settings > Notifications)
    nk_trophy: "Trophies",
    /// What the notification kind is for, under its switch (search reads its words too)
    nkb_trophy: "A trophy was earned.",
}
