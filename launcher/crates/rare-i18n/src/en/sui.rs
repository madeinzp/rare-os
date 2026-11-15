strings! {
    /// Hint bar (A button) on a Sui game tile in the profile; short
    game_page: "Game page",
    /// Screen reader label on the picture tile that is in use
    current_picture: "Current picture",
    /// Empty state title on the profile when there is no wallet account
    add_account_title: "Add your Sui account",
    /// Empty state body on the profile
    add_account_body: "Your SuiNS name, picture, Sui games and collectibles show up here once Rare Wallet has an account.",
    /// Button
    open_rare_wallet: "Open Rare Wallet",
    /// Small label above the player name on the profile page
    profile: "Profile",
    /// Button on the profile; short
    change_picture: "Change picture",
    /// Hint bar text for the Change picture button; short
    pick_picture_hint: "Pick a picture",
    /// Button; short
    change_username: "Change username",
    /// Button; short
    pick_username: "Pick a username",
    /// Button on the profile and title of the saves sheet
    cloud_saves: "Cloud saves",
    /// Stat label on the profile: SUI plus coins plus collectibles in dollars
    total_value: "Total value",
    /// Stat label on the profile for the value change today; short
    today: "Today",
    /// Stat label on the profile: the value of the player's coins
    coins: "Coins",
    /// Stat label and shelf title on the profile (NFTs the player holds)
    collectibles: "Collectibles",
    /// Stat label: how many games the player played on this device
    games_played: "Games played",
    /// Stat label: total play time on this device
    time_played: "Time played",
    /// Play time under an hour; n is minutes; short
    minutes_short: "{n} min",
    /// Play time in hours; n is a number; short
    hours_short: "{n} h",
    /// Stat label: the value change over the last day; short
    h24: "24h",
    /// Stat label for the value change since a clock time, e.g. Since 3:07 PM
    since_time: "Since {time}",
    /// Shelf title on the profile
    your_sui_games: "Your Sui games",
    /// Shelf note: n is how many Sui games the player owns
    owned_on_sui: "{n} owned on Sui",
    /// Shelf title on the profile
    games_you_played: "Games you played",
    /// Shelf note under Games you played
    on_this_device: "on this device",
    /// Hint bar (A button) on a picture choice; short
    use_this: "Use this",
    /// Shown in the picture picker while NFTs load
    loading_nfts: "Loading your NFTs",
    /// Picture picker empty state
    no_nft_pictures: "Any NFT you hold can be your picture. Buy one in Rare Marketplace and it shows up here.",
    /// Title of the picture picker sheet
    pick_your_picture: "Pick your picture",
    /// Section label in the picture picker: Rare's default pictures
    default_pictures: "Default",
    /// Section label in the picture picker
    your_nfts: "Your NFTs",
    /// Toast after picking a profile picture; name is the picture's or NFT's name
    is_your_picture: "{name} is your picture",
    /// Web game input mode label: the game reads the controller directly
    input_gamepad: "Controller",
    /// Web game input mode label: the controller moves between buttons on the page
    input_focus: "Controller picks buttons",
    /// Web game input mode label: the controller sends keyboard keys
    input_keys: "Controller as keyboard",
    /// How the controller maps to keyboard and mouse in a web game; A, B, X, RB and View are button names
    controls_keys: "D-pad and left stick are the arrow keys, A Enter, B Escape, X Space. The right stick moves a pointer, RB clicks. View opens the keyboard.",
    /// Controller note for a web game; Guide is the controller's home button
    controls_guide: "Guide returns to Rare with the game still running.",
    /// Anti-cheat note on a game page; names is a list of anti-cheat products
    ac_denied: "Online play won't run here: the publisher blocks Linux in its anti-cheat ({names}). Offline modes may still work.",
    /// Anti-cheat note on a game page; names is a list of anti-cheat products
    ac_broken: "Online play won't run here: its anti-cheat ({names}) doesn't support Linux yet. Offline modes may still work.",
    /// Anti-cheat note on a game page; names is a list of anti-cheat products
    ac_supported: "{names} supports Linux and Proton.",
    /// Anti-cheat note on a game page; names is a list of anti-cheat products
    ac_running: "Runs with {names} on Proton, unofficially.",
    /// Status badge on a Sui game: it is playable; short
    status_live: "Live",
    /// Status badge on a Sui game: in beta; short
    status_beta: "Beta",
    /// Status badge on a Sui game; short
    status_coming: "Coming soon",
    /// Status badge on a Sui game: announced, no date; short
    status_announced: "Announced",
    /// Status badge on a Sui game: development paused; short
    status_paused: "Paused",
    /// Platform tag on a Sui game: runs in a browser; short
    platform_web: "Web",
    /// Platform tag on a Sui game: phones; short
    platform_mobile: "Mobile",
    /// Home shelf profile tile subtitle; n is how many Sui games the player owns
    sui_games_owned: "{n} Sui game owned|{n} Sui games owned",
    /// Home shelf profile tile subtitle when no Sui games are owned
    your_profile: "Your profile",
    /// Hint bar (A button); short
    open_wallet: "Open Wallet",
    /// Home shelf tile title when there is no account
    your_sui_profile: "Your Sui profile",
    /// Home shelf tile body when there is no account
    add_account_hint: "Add a Rare Wallet account to see your name, picture, games you own and cloud saves.",
    /// Subtitle on the Cloud saves tile
    emulator_saves: "Emulator saves on Walrus",
    /// Subtitle on the Share tile: new screenshots and clips
    new_captures: "{n} new capture|{n} new captures",
    /// Subtitle on the Share tile
    screenshots_clips: "Screenshots and clips",
    /// Hint bar (A button) on an NFT drop tile; verb; short
    mint: "Mint",
    /// Subtitle on the Launchpad tile when nothing is minting
    no_live_mints: "No live mints",
    /// Subtitle on the Launchpad tile
    new_drops: "New drops",
    /// Tile title: Rare's NFT launchpad
    launchpad: "Launchpad",
    /// Shelf note on the home Sui shelf; n is how many drops are live
    minting_now: "{n} minting now",
    /// Title of the Sui shelf on Home
    on_sui: "On Sui",
    /// Button on a game page while the game runs; also its hint
    quit_game: "Quit game",
    /// Play button label while the game is launching; short
    starting: "Starting...",
    /// Heading of the emulator panel: the consoles with games
    systems: "Systems",
    /// Heading of the emulator panel: recently played ROMs
    played_last: "Played last",
    /// Button on a web game page; browser is the browser's name, e.g. Chrome
    open_in_browser: "Open in {browser}",
    /// Proton version choice: the experimental build; short
    experimental: "Experimental",
    /// Button to install a Proton build; name is the build's name, e.g. GE-Proton
    install_tool: "Install {name}",
    /// Button to remove an out-of-date Proton build; name is the build's name
    remove_old_tool: "Remove old {name}",
    /// Progress text while installing a Proton build
    proton_lookup: "Looking up the release",
    /// Progress text while installing a Proton build; pct is a number 0 to 100
    proton_download_pct: "Downloading {pct}%",
    /// Progress text while installing a Proton build
    proton_verify: "Checking the checksum",
    /// Progress text while installing a Proton build
    proton_unpack: "Unpacking",
    /// Progress text after switching a game's Proton build
    restarting_steam: "Restarting Steam",
    /// Line on a game card: how many items the player owns of this game
    owned_items: "Owned · {n} item|Owned · {n} items",
    /// Button on a game page that opens the player's items in the Marketplace; also its hint
    your_items: "Your items",
    /// Title of the Sui games catalog page
    sui_games: "Sui Games",
    /// Button that opens the catalog on rare.market in the web window
    open_on_rare: "Open on Rare",
    /// Section title in the Sui catalog
    all_games: "All games",
    /// Section title in the Sui catalog: games that are not live yet
    coming: "Coming",
    /// Note under the Coming section of the Sui catalog
    coming_note: "Announced, paused or offline for now. They move up when they're live.",
    /// Title of the sheet that lists a game's NFT collections; also a button on a game page
    collections: "Collections",
    /// Button on a web game's page; opens it in the game window
    play_now: "Play now",
    /// Button on a web game's page
    add_to_library: "Add to library",
    /// Button on a game page; store is a store name such as Steam
    get_it_on: "Get it on {store}",
    /// Button on a game page that opens its page on rare.market
    view_on_rare: "View on Rare",
    /// How long cloud backups are kept (choice on the Cloud saves page); short
    keep_30: "30 days",
    /// How long cloud backups are kept; short
    keep_90: "90 days",
    /// How long cloud backups are kept; short
    keep_180: "6 months",
    /// How long cloud backups are kept; short
    keep_365: "1 year",
    /// Lowercase relative time in the saves table: under an hour ago
    ago_just_now: "just now",
    /// Relative time in the saves table; n is hours; short
    ago_hours: "{n} h ago",
    /// Lowercase relative time in the saves table
    ago_yesterday: "yesterday",
    /// Relative time in the saves table; n is days
    ago_days: "{n} day ago|{n} days ago",
    /// Cloud save progress step: gathering the save files
    step_pack: "Packing",
    /// Cloud save progress step
    step_encode: "Encrypting",
    /// Cloud save progress step: waiting for the player to approve in the wallet
    step_register: "Approve in Rare Wallet",
    /// Cloud save progress step
    step_upload: "Uploading",
    /// Cloud save progress step: second approval in the wallet
    step_certify: "Approve to confirm",
    /// Cloud save progress step: writing the restored files
    step_unpack: "Restoring",
    /// Intro under the Cloud saves page title
    saves_intro: "Emulator and web game saves, encrypted on this device and kept on Walrus. Your wallet pays for the storage and only your account can open them. Steam games use Steam Cloud.",
    /// Cloud saves status title when no wallet account exists
    no_account: "No account",
    /// Cloud saves status body when no wallet account exists
    no_account_body: "Add a Rare Wallet account to back up saves.",
    /// Cloud saves status title: the save key is available
    unlocked: "Unlocked on this device",
    /// Cloud saves status; n is the number of backups, keep is a duration such as 90 days, wal and sui are amounts with their coin names
    unlocked_body: "{n} backup on Walrus. Backing up everything here for {keep} costs about {wal} plus {sui} in fees.|{n} backups on Walrus. Backing up everything here for {keep} costs about {wal} plus {sui} in fees.",
    /// Same as the previous one, for the Walrus test network
    unlocked_body_testnet: "{n} backup on Walrus testnet. Backing up everything here for {keep} costs about {wal} plus {sui} in fees.|{n} backups on Walrus testnet. Backing up everything here for {keep} costs about {wal} plus {sui} in fees.",
    /// Cloud saves status title: no save key yet
    locked: "Locked",
    /// Cloud saves status body
    locked_body: "Approve one signature in Rare Wallet to make your save key. It costs nothing and the key never leaves this device.",
    /// Button that makes the cloud save key; short
    unlock: "Unlock",
    /// Section note: how many save sets were found on this device
    n_found: "{n} found",
    /// Section title on the Cloud saves page
    on_this_device_title: "On this device",
    /// Empty state title on the Cloud saves page
    no_saves_found: "No emulator saves found",
    /// Empty state body on the Cloud saves page
    no_saves_body: "Rare looks in RetroArch, RPCS3, PCSX2, Cemu, DuckStation, Dolphin, PPSSPP and other emulators, EmuDeck's Emulation folder and your web games.",
    /// Table column heading: the save set's name; short
    col_saves: "Saves",
    /// Table column heading: file size; short
    col_size: "Size",
    /// Table column heading: when the saves last changed; short
    col_changed: "Changed",
    /// Table column heading: when the saves were last backed up; short
    col_last_backup: "Last backup",
    /// Section title on the Cloud saves page
    other_devices: "From your other devices",
    /// Section note under From your other devices
    not_here_yet: "not on this device yet",
    /// Button label that asks for a second press to confirm; short
    press_again: "Press again",
    /// Button: restore a backup from another device onto this one; short
    restore_here: "Restore here",
    /// Button: restore a backup; short
    restore: "Restore",
    /// Sentence appended to the automatic cloud saves note; wal and sui are amounts with coin names, n is how many uploads they cover
    auto_funds: "Funds: {wal} and {sui}, about {n} upload.|Funds: {wal} and {sui}, about {n} uploads.",
    /// Title of the automatic cloud saves panel
    auto_on: "Automatic cloud saves are on",
    /// Body of the automatic cloud saves panel when on
    auto_on_body: "Rare checks the cloud before you play and uploads after, for emulators, web games and PC games outside Steam, Epic and GOG. Uploads use a cloud saves address of your account, so nothing asks to approve.",
    /// Title of the automatic cloud saves panel
    auto_off: "Automatic cloud saves are off",
    /// Body of the automatic cloud saves panel when off
    auto_off_body: "Turn on to sync saves before and after every session. One wallet signature on this device; then add a little WAL and SUI for the uploads.",
    /// Button: top up the cloud saves address; short
    add_funds: "Add funds",
    /// Hint bar text for Add funds; WAL and SUI are coin names
    add_funds_hint: "Add 1 WAL and 0.2 SUI",
    /// Disabled button label while the wallet approval is pending
    waiting_wallet: "Waiting for the wallet",
    /// Table cell where the last backup time shows once the key is unlocked
    unlock_to_see: "Unlock to see",
    /// Table cell: when the last backup was made, and how many days it is kept for; short
    backup_age_left: "{when} · {n} d left",
    /// Table cell: this save set has never been backed up; short
    never: "Never",
    /// Hint bar text for the Restore button; short
    restore_this_backup: "Restore this backup",
    /// Button: back this save set up to the cloud (verb); short
    back_up: "Back up",
    /// Hint bar text for the Back up button; short
    back_up_now: "Back up now",
    /// Small note after a save set's name: how many files it has
    n_files: "{n} file|{n} files",
}
