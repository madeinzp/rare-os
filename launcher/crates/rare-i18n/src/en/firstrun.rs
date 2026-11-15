// First-run setup: welcome, Wi-Fi, time zone, sign in, picture, Steam, stores, emulators, controller, moving assets.
strings! {
    /// Welcome step
    welcome_title: "Welcome to Rare OS",
    /// Button on the welcome step; short: button
    get_started: "Get started",
    /// Hint on the language button (controller hint)
    language: "Language",
    /// Footer hint: skip the whole setup (controller hint, Y)
    skip_setup: "Skip setup",
    /// Screen reader label of the step dots; n is the current step, total the number of steps
    step_of: "Step {n} of {total}",
    /// Shown while setup finishes
    getting_ready: "Getting your games ready",
    /// Shared buttons
    later: "Later",
    not_now: "Not now",
    /// Skip button when nothing has been signed in; short: button
    skip_for_now: "Skip for now",
    /// "Signed in as {name}": line with a check mark; name is an account name
    signed_in_as: "Signed in as {name}",

    /// Wi-Fi step: title and button (also the button on the account step when offline)
    connect_wifi: "Connect to Wi-Fi",
    wifi_body: "Games, updates, Rare Marketplace and your wallet need the internet. Pick your network.",
    /// Button when there is no connection; short: button
    continue_offline: "Continue offline",
    /// Button: look for Wi-Fi networks again; short: button
    scan_again: "Scan again",
    /// name is a Wi-Fi network name
    connected_to: "Connected to {name}",
    connected_internet: "Connected to the internet",
    not_connected_yet: "Not connected yet.",
    /// Progress line while joining a network; name is the network name
    joining: "Joining {name}",
    looking_networks: "Looking for networks",
    wifi_managed_title: "Wi-Fi is managed outside Rare",
    wifi_managed_body: "This system doesn't let the launcher change networks. Connect in desktop mode, then come back. If the device is online already, just continue.",
    no_networks_title: "No networks found",
    no_networks_body: "Move closer to your router, or check that Wi-Fi is turned on, then scan again.",
    /// Tag on a network that needs a password
    password: "Password",
    /// Tag on a network without a password
    open_network: "Open",
    /// Controller hint on a network row (verb)
    join: "Join",
    /// Error shown under the list; name is the network name
    join_bad_password: "Couldn't join {name}. Check the password and try again.",
    join_failed: "Couldn't join {name}.",
    scan_failed: "Couldn't look for networks.",
    /// Screen reader label of the signal bars; pct is a number 0-100
    signal_pct: "Signal {pct}%",

    /// Time zone step
    time_zone_title: "Time zone",
    time_zone_body: "So the clock, sleep timers and store sale ends show your time.",
    /// Line with a check mark; zone is a place name, clock the time of day
    zone_now: "{zone}, {clock} now",
    utc_for_now: "Set to UTC for now, so the clock says {clock}.",
    pick_time_zone: "Pick time zone",
    keep_utc: "Keep UTC",

    /// Account step (sign in or create)
    pk_create_your_account: "Create your account",
    pk_create_account: "Create account",
    pk_sign_in_passkey: "Sign in with a passkey",
    pk_start_body: "One account for Rare Wallet, Friends and Rare Marketplace. It is kept on this SuiPlay and signs with A. You write down 12 words once as a backup.",
    pk_pin_body: "This device signs everyday transactions with the PIN. Your passkey can always sign too.",
    pk_create_body: "Make a new wallet protected by a passkey.",
    pk_signin_body: "Use the passkey you made before.",
    pk_no_internet: "No internet. You can create your account now; Friends and the Marketplace need it later.",
    pk_other_ways: "Other ways to sign in",
    /// Title and short description of the three ways to hold an account
    pk_way_device: "This SuiPlay",
    pk_way_device_body: "Signs with A. Back up 12 words once",
    pk_way_phone: "A passkey on your phone",
    pk_way_phone_body: "Scan a code and confirm with your face or fingerprint",
    pk_way_key: "A passkey on a security key",
    pk_way_key_body: "Plug it into the USB-C port and touch it",
    /// Title after signing in; name is a username without the @
    pk_youre_name: "You're @{name}",
    pk_youre_signed_in: "You're signed in",
    pk_how_friends_see: "How friends see you",
    pk_your_username: "Your username",
    pk_username_body: "Friends find you by your username. It is free, and you can change it from your profile.",

    /// Picture step
    picture_title: "Pick a picture",
    picture_body: "Friends see it next to your name. You can change it any time.",
    /// Heading over Rare's default pictures
    default_pictures: "Default",
    nft_picture_note: "An NFT you hold can be your picture too. Pick it from Change picture on your profile.",

    /// Steam step
    steam_ready_title: "Steam is ready",
    steam_ready_body: "Your Steam games show up in Library and on Home, and they start straight from Rare.",
    steam_sign_in_title: "Sign in to Steam",
    steam_sign_in_body: "Steam opens in its big-screen mode. Sign in there (your phone's Steam app can scan the code), then press the Rare button to come back here.",
    open_steam: "Open Steam",
    open_steam_again: "Open Steam again",
    skip_steam: "Skip Steam for now",
    /// Line with a check mark; name is a Steam account name, n a number of games
    steam_signed_in_games: "Signed in as {name}, {n} game found|Signed in as {name}, {n} games found",
    steam_waiting: "Waiting for you to sign in to Steam.",
    steam_offline: "You are offline, so Steam cannot sign in yet. Skip this and sign in to Steam later from Library.",
    steam_guard: "Steam Guard is Steam's own sign-in check. Approve the sign-in in the Steam Mobile app, or type the code Steam emails you. Press A on the code box for Steam's keyboard.",
    steam_tip_installed: "Games you already installed on this device are kept and show up by themselves.",
    steam_tip_heroic: "Games you installed with Heroic (Epic and GOG) or itch.io are listed and start from Rare too.",

    /// Epic Games and GOG step
    stores_title: "Epic Games and GOG",
    stores_body: "Optional. Sign in to either to play those games here too. You can do this later in Settings > Accounts.",
    /// Under a store's name; name is the store (Epic Games, GOG)
    store_games_note: "Your {name} games in the Library, installed and started from Rare.",
    stores_offline: "You are offline. Sign in later from Settings > Accounts.",

    /// Emulators step
    emulators_title: "Emulators",
    emulators_body: "Rare installs the emulators for the systems you pick and sets every button for the SuiPlay. Games you copy into Emulation/roms show up in Library. You can change this any time in Library, Emulation.",
    emu_all_installed: "Every emulator is installed. Controls are set for the SuiPlay's buttons.",
    emu_nothing_ticked: "Nothing ticked. Tick the systems you play, or set up everything.",
    /// Button when nothing is ticked; short: button
    set_up: "Set up",
    /// Button; n is the number of ticked emulators; short: button
    set_up_n: "Set up {n} emulator|Set up {n} emulators",

    /// Controller check
    check_controller_title: "Check your controller",
    check_controller_body: "Press each button. It lights up here when it works. If one stays dark, Settings > Controls can remap it.",
    every_button_works: "Every button works.",
    /// n of total buttons have been pressed
    buttons_pressed: "{n} of {total} pressed. The sticks move the highlight, so smooth moves mean they work. A and B work if they got you here.",
    /// Names of the d-pad directions on the check's button chips; short
    pad_up: "Up",
    pad_down: "Down",
    pad_left: "Left",
    pad_right: "Right",

    /// Move assets from the old SuiPlay wallet
    mv_title: "Move your assets to a wallet you control",
    mv_body: "The wallet your SuiPlay came with signs through its maker's sign-in. If that service stops, what's in it can't move anymore. Moving it now keeps it yours.",
    mv_no_old_wallet: "I don't have an old wallet",
    mv_dont_have_one: "I don't have one",
    mv_set_up_wallet: "Set up your wallet",
    mv_just_look: "Just look at my old wallet",
    mv_new_address: "Your new address",
    mv_new_address_body: "Everything from your old SuiPlay wallet goes here. Next, Rare reads what's in the old wallet and lists exactly what to send, in order.",
    mv_first_wallet_title: "First, a wallet you control",
    /// name is a wallet name
    mv_watch_only: "{name} is watch-only, so it can't receive with a key you hold. Create a passkey wallet to move your assets into.",
    mv_need_wallet: "Your assets need somewhere to go: a Rare Wallet signed in with your passkey. You can still look at what's in your old wallet first.",
    mv_change_address: "Change address",
    mv_enter_address: "Enter address",
    /// Button; addr is a shortened wallet address
    mv_scan_addr: "Scan {addr}",
    mv_address_hint: "A 0x address, or a name ending in .sui if you set one.",
    mv_which_wallet: "Which wallet came with your SuiPlay?",
    mv_which_wallet_body: "Enter its address. On the SuiPlay's original system it's under Wallet, at the top. Rare only reads it: nothing moves until you approve it in that wallet.",
    mv_see_plan: "See the move plan",
    mv_set_up_to_move: "Set up your wallet to move these",
    mv_different_wallet: "Different wallet",
    /// addr is a shortened address, name the wallet's own name
    mv_wallet_read: "Wallet {addr}. Read from the Sui network; nothing has moved.",
    mv_wallet_read_named: "Wallet {addr} ({name}). Read from the Sui network; nothing has moved.",
    mv_reading: "Reading your old wallet",
    mv_whats_in: "What's in your old wallet",
    mv_all_moved: "Everything that can move has moved",
    mv_nothing_to_move: "Nothing to move",
    mv_send_these: "Send these, in this order",
    mv_empty_body: "The old wallet is empty. Rare keeps an eye on it in case something new arrives.",
    mv_left_body: "What's left can't be sent by anyone but its own app. Rare keeps an eye on the old wallet in case something new arrives.",
    mv_plan_body: "Sign these in your old SuiPlay wallet while it still works. Each one sends to your new address. Come back here and check again when you're done.",
    mv_check_again: "Check again",
    mv_finish_later: "Finish later",
    /// addr is a shortened address
    mv_moved_to: "Moved to {addr}",
    mv_fees_covered: "Rare covers the fees for these moves.",
    /// Label over a QR code and address
    mv_send_to: "Send to",
    mv_boot_menu: "Already on Rare OS? The original system stays on this SuiPlay in the boot menu: hold Shift while it powers on (with a USB-C keyboard), pick it, send, then restart into Rare OS.",
    mv_stays_behind: "Stays behind",
    /// "and 3 more" under a list that was cut off; n is a number
    and_more: "and {n} more",
    mv_sent: "Sent",
    mv_sending: "Sending",
    mv_wallet_empty: "This wallet is empty.",
    mv_only_stuck: "Only items that can't be sent are in this wallet.",
    /// Section titles of the scan
    mv_coins: "Coins",
    mv_collectibles: "Collectibles",
    mv_kiosks: "Kiosks",
    mv_staked: "Staked SUI",
    mv_suins_names: "SuiNS names",
    mv_cant_move: "Can't move",
    /// id is a shortened kiosk id
    mv_personal_kiosk: "Personal kiosk {id}",
    mv_kiosk: "Kiosk {id}",
    mv_in_sales: "{amount} in sales",
    mv_listed: "Listed {amount}",
    mv_locked: "Locked",
    /// kiosks is a number of kiosks, n the number of items in them
    mv_kiosks_summary: "{kiosks}, {n} item|{kiosks}, {n} items",
    mv_total_staked: "Total staked",
    mv_staked_note: "Moves as it is and keeps earning.",
    /// date is a month and year
    mv_until: "Until {date}",
    mv_truncated: "This wallet holds more objects than Rare reads at once. Move these first, then check again for the rest.",
    mv_starting: "Starting",
    mv_is_new_wallet: "That's your new wallet. Enter the address of the wallet your SuiPlay came with.",
    mv_open_failed: "Couldn't open that wallet.",
    mv_read_failed: "Couldn't read that wallet. Check the address and try again.",
}
