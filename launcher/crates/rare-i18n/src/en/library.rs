strings! {
    /// Sort order in the View sheet
    sort_recent: "Recently played",
    /// Sort order in the View sheet (by game name)
    sort_name: "Name",
    /// Sort order in the View sheet
    sort_played: "Most played",
    /// Filter tab (games marked as favourites; short)
    tab_favourites: "Favourites",
    /// Filter tab (games you hid; short)
    tab_hidden: "Hidden",
    /// Filter tab for the player's collections, also the hint for B inside one (short)
    collections: "Collections",
    /// Filter tab for streaming from a PC (short)
    tab_streaming: "Streaming",
    /// Library header count; n is a number of apps
    apps_n: "{n} app|{n} apps",
    /// Library header count for Streaming; n is a number of games on the PC
    n_on_your_pc: "{n} on your PC",
    /// Library header count while looking for games
    scanning: "Scanning",
    /// Library header count in Collections; n is a number
    collections_n: "{n} collection|{n} collections",
    /// Button: open the Steam app
    open_steam: "Open Steam",
    /// Library note title while Steam waits for you to sign in
    steam_needs_sign_in: "Steam needs you to sign in",
    /// Library note body while Steam waits for you to sign in
    steam_sign_in_line: "Steam games start once you sign in. Steam may ask for a Steam Guard code from your email or the Steam Mobile app.",
    /// Button: add a folder of games to Steam's own library
    add_to_steam: "Add to Steam",
    /// Library note title: games found in a Steam library from an earlier install
    foreign_title: "Games found from your previous system",
    /// Library note body; n is a number of games, paths is a list of folders
    foreign_body: "{n} games in {paths}. They play from here; add the folder to Steam so Steam keeps them updated.",
    /// Streaming tab: PC status (a game is running on it)
    playing_on_pc: "Playing on the PC",
    /// Streaming tab: PC status (sleeping, can be woken)
    asleep: "Asleep",
    /// Button and hint on the Streaming tab with no PC (short)
    set_up_streaming: "Set up streaming",
    /// Streaming tab empty state title
    play_pc_title: "Play the games on your PC",
    /// Streaming tab empty state body
    play_pc_body: "Stream from a gaming PC running Sunshine or GeForce Experience, over your home Wi-Fi. Rare gets Moonlight, the app that streams, the first time.",
    /// Favourites tab empty state
    no_favourites_title: "No favourites yet",
    /// Favourites tab empty state body; Menu is the controller button
    no_favourites_body: "Press Menu on a game and pick Add to favourites, or press X on its page.",
    /// Not installed tab empty state
    all_installed_title: "Every game you own is installed",
    /// Not installed tab empty state body
    all_installed_body: "Steam games your account owns show here when they are not on this device.",
    /// Hidden tab empty state
    nothing_hidden_title: "Nothing hidden",
    /// Hidden tab empty state body
    nothing_hidden_body: "Games you hide leave the library and Home. They show here so you can bring them back.",
    /// Collection with no games, title
    empty_collection_title: "Nothing in this collection yet",
    /// Body for empty collections; Menu is the controller button
    add_to_collection_body: "Press Menu on a game and pick Add to collection.",
    /// Button: look for games again
    scan_again: "Scan again",
    /// Library empty state title
    no_games_title: "No games here yet",
    /// Library empty state body
    no_games_body: "Install a game in Steam, or put a game folder in Games on this device or your SD card.",
    /// View sheet row; name is a sort order such as "Name"
    sort_by: "Sort: {name}",
    /// View sheet row; name is a filter such as "Favourites"
    show_filter: "Show: {name}",
    /// Button, sheet row and card (verb phrase): make a collection of games
    new_collection: "New collection",
    /// Sheet row
    rename_collection: "Rename collection",
    /// Sheet row and button in the confirm sheet
    delete_collection: "Delete collection",
    /// Sheet title (sort, filter and collections)
    library_view: "Library view",
    /// Sheet subtitle
    library_view_sub: "Sort, filter and collections",
    /// Add sheet row
    add_file_game: "A game from a file or folder",
    /// Add sheet row
    add_web_game: "A web game",
    /// Add sheet row
    add_pc_stream: "A PC to stream from",
    /// Add sheet title
    add_title: "Add to your library",
    /// Keyboard title when adding a web game
    web_game_address: "Web game address",
    /// Confirm sheet title; name is a collection name
    delete_title: "Delete {name}?",
    /// Confirm sheet body
    games_stay: "The games stay in your library.",
    /// Confirm sheet button: do not delete
    keep_it: "Keep it",
    /// Toast; name is a collection name
    deleted_toast: "{name} deleted",
    /// Sheet title (game consoles and computers for emulation)
    systems: "Systems",
    /// Sheet subtitle
    systems_sub: "Show one system's games",
    /// Android loading line before any progress text
    starting: "Starting",
    /// Android tab empty state title
    android_unavailable_title: "Android isn't available here",
    /// Android tab empty state body
    android_unavailable_body: "Android apps need Rare OS on this device.",
    /// Android tab empty state title
    android_update_title: "Android comes with the next Rare OS update",
    /// Android tab empty state body
    android_update_body: "Update Rare OS in Settings > System. Then set up Android here with one press.",
    /// Android setup in progress, note
    android_wait_note: "Getting Android takes a few minutes. You can keep using Rare.",
    /// Android setup in progress, title
    setting_up_android: "Setting up Android",
    /// Button: install Android
    set_up_android: "Set up Android",
    /// Controller hint on the Set up Android button (short)
    set_up: "Set up",
    /// Android tab empty state title
    play_android_title: "Play Android games",
    /// Android tab empty state body
    play_android_body: "Rare gets Android without Google apps (about 1 GB) and adds F-Droid and Aurora Store, so you can install games with no Google account.",
    /// Button: search F-Droid (an app store)
    find_fdroid: "Find on F-Droid",
    /// Controller hint
    search_fdroid: "Search F-Droid",
    /// Android tab note
    android_find_note: "Find a game on F-Droid, or open F-Droid or Aurora Store. Games you install show up here.",
    /// State of an app you can open now
    ready: "Ready",
    /// State of an app that is installing
    installing: "Installing",
    /// Hint on a not installed app: A installs it (short)
    get: "Get",
    /// Section title under Streaming
    more_ways: "More ways to stream",
    /// Section note
    more_ways_sub: "From a PlayStation, or a PC with Steam",
    /// Button: choose folders that hold ROM files (game files for emulators)
    rom_folders: "ROM folders",
    /// Emulation tab empty state body
    emu_empty_body: "Copy games you own into Emulation/roms on this device or your SD card, in the folder for their system (snes, gba, psx...). Rare finds them on the next scan.",
    /// First system chip; n is a number of games
    chip_all: "All \u{b7} {n}",
    /// Controller hint on a system chip (short)
    show: "Show",
    /// Chip that opens the rest of the systems; n is how many chips are hidden (short)
    more_n: "More {n}",
    /// Controller hint on the More chip
    all_systems: "All systems",
    /// Collections tab empty state
    no_collections_title: "No collections yet",
    /// Controller hint on the New collection card (short)
    create: "Create",
    /// Section title in the Emulation tab
    emulators: "Emulators",
    /// Under a focused game never played
    not_played_yet: "Not played yet",
}
