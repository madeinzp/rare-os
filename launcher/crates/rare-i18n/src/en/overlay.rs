strings! {
    /// Title of the power sheet (Sleep, Restart, Turn off)
    power: "Power",
    /// Power sheet button: put the device to sleep
    power_sleep: "Sleep",
    /// Power sheet button: leave to the desktop session
    power_desktop_mode: "Desktop mode",
    /// Under the background switch when the game pauses on switch away
    bg_pauses: "Pauses when you switch away, and picks up where you left it.",
    /// Under the background switch, online games
    bg_online: "On for online games, so you stay connected.",
    /// Under the background switch, apps
    bg_app: "On for apps, so calls, messages and music carry on.",
    /// Under the background switch, web games
    bg_web: "On for web games, so you stay connected.",
    /// Under the background switch, default
    bg_none: "Keeps running when you switch away.",
    /// Switch label on a game's page
    bg_title: "Keep running in the background",
    /// Ask Rare status line; pct is a whole number percent
    ask_status_download: "Downloading the speech model, {pct}%. This happens once.",
    /// Ask Rare status line; X and A are controller buttons
    ask_status_listening: "Listening. Let go of X, or press A, to send.",
    /// Ask Rare status line
    ask_status_hearing: "Turning speech into text...",
    /// Ask Rare status line
    ask_status_busy: "Working on it...",
    /// Title of the Ask tab card
    ask_title: "Ask Rare",
    /// Ask tab explainer before it is set up
    ask_intro: "Talk or type, and Rare does it: start a game, change a setting, get a purchase ready for your review. It uses your own model: a Claude or OpenAI key, any OpenAI-compatible service, or Ollama on your network. Rare pays nothing and keeps your key sealed in Rare Wallet.",
    /// Button on the Ask tab; opens Settings
    ask_setup: "Set up in Settings",
    /// Ask tab button: start voice input (verb). Short: button
    ask_talk: "Talk",
    /// Ask tab button while listening: send the question. Short: button
    ask_send: "Send",
    /// Ask tab button: type the question with the keyboard. Short: button
    ask_type: "Type",
    /// Hint bar for the Type button
    ask_type_hint: "Type a question",
    /// Ask tab button: stop the running answer. Short: button
    ask_stop: "Stop",
    /// Switch label on the Ask tab
    ask_picture: "Include a picture of the game",
    /// Under the picture switch
    ask_picture_sub: "Sent to your model with the question",
    /// Heading above example questions to ask
    ask_try: "Try",
    /// Example question shown while over a game
    ask_try_boss: "\"How do I beat this boss?\"",
    /// Example question shown while over a game
    ask_try_brightness: "\"Turn the brightness down\"",
    /// Example question shown while over a game
    ask_try_invite: "\"Invite Sam to play\"",
    /// Example question (launcher); Hades is a game title
    ask_try_start: "\"Start Hades\"",
    /// Example question (launcher)
    ask_try_dnd: "\"Turn on Do not disturb\"",
    /// Example question (launcher)
    ask_try_space: "\"What is taking up space?\"",
    /// Label above older exchanges in the Ask tab
    ask_earlier: "Earlier",
    /// Label above the player's own question, when a picture was sent
    ask_you_picture: "You, with a picture of the game",
    /// Label above the player's own question
    ask_you: "You",
    /// Button on the Ask tab: clear the conversation
    ask_new_chat: "New chat",
    /// Hint bar for the New chat button
    ask_start_over: "Start over",
    /// Rare menu tab over a running game. Short: tab
    tab_game: "Game",
    /// Rare menu tab when no game is under it. Short: tab; product name
    tab_rare: "Rare",
    /// Rare menu tab: your items from this game. Short: tab
    tab_items: "Items",
    /// Count of your items, in the menu line "22 items · worth 98.60 SUI · 9 offers"
    items_count: "{n} item|{n} items",
    /// Part of the items line; amount is a number like 98.60
    items_worth: "worth {amount} SUI",
    /// Part of the items line: offers on your items
    offers_count: "{n} offer|{n} offers",
    /// Under the game name in the Rare menu
    playing_just_started: "Just started",
    /// Under the game name; m is minutes
    playing_min: "Playing for {m} min",
    /// Under the game name; h hours, m minutes
    playing_h_min: "Playing for {h} h {m} min",
    /// Under the game name; h hours
    playing_h: "Playing for {h} h",
    /// Performance profile name
    profile_quiet: "Quiet",
    /// Performance profile name
    profile_balanced: "Balanced",
    /// Performance profile name; also the row title
    profile_performance: "Performance",
    /// Performance profile name
    profile_custom: "Custom",
    /// Performance row sub line: the power profile was saved for this game
    perf_used_for: "Used for {game}",
    /// Quit row title when Steam's own UI is open
    close_steam: "Close Steam",
    /// Quit row title for an app
    close_app: "Close app",
    /// Quit row title for a game
    quit_game: "Quit game",
    /// Quit row sub line for Steam
    quit_steam_what: "Steam closes, with any game it runs, and Rare has the screen.",
    /// Quit row sub line for a streamed game; title is the game name
    quit_stream_what: "Ends {title} on your PC and goes back to Rare.",
    /// Quit row sub line; title is the game name
    quit_what: "Ends {title} and goes back to Rare.",
    /// Menu row title
    your_items: "Your items",
    /// Menu row title while a screenshot or clip is being saved
    saving: "Saving...",
    /// Menu row title (noun)
    screenshot: "Screenshot",
    /// Hint bar for the Screenshot row
    screenshot_hint: "Take a screenshot",
    /// Menu row title: save the instant replay
    save_clip: "Save clip",
    /// Sub line under Save clip
    replay_off: "Instant replay is off. Turn it on in Quick settings.",
    /// Hint bar for the Save clip row
    save_clip_hint: "Save the clip",
    /// Menu row title
    discord_unmute: "Unmute Discord",
    /// Menu row title
    discord_mute: "Mute Discord",
    /// Sub line of the Discord mute row
    discord_mic_muted: "Your microphone is muted",
    /// Sub line of the Discord mute row
    discord_mic_on: "Your microphone is on",
    /// Hint bar (verb)
    mute: "Mute",
    /// Hint bar (verb)
    unmute: "Unmute",
    /// Menu row title
    discord_undeafen: "Undeafen Discord",
    /// Menu row title
    discord_deafen: "Deafen Discord",
    /// Sub line of the Discord deafen row
    discord_hear_nothing: "You hear nothing from Discord",
    /// Sub line of the Discord deafen row
    discord_hear: "You hear Discord",
    /// Hint bar (verb)
    deafen: "Deafen",
    /// Hint bar (verb)
    undeafen: "Undeafen",
    /// Sub line of the Steam row
    steam_overlay_sub: "Steam's overlay: friends, guides, controller layout",
    /// Hint bar
    steam_overlay_hint: "Open Steam's overlay",
    /// Menu row title and hint bar
    open_steam: "Open Steam",
    /// Steam page row (Steam's own library)
    steam_library: "Library",
    /// Steam page row (Steam's own store)
    steam_store: "Store",
    /// Steam page row
    steam_downloads: "Downloads",
    /// Steam page row
    steam_settings: "Steam settings",
    /// Hint bar for the Steam page rows
    open_in_steam: "Open in Steam",
    /// Sub line under Save state in an emulator game
    no_saved_states: "No saved states yet",
    /// Menu row title
    app_details: "App details",
    /// Menu row title
    game_details: "Game details",
    /// On a saved-state card without a thumbnail
    no_picture: "No picture",
    /// Hint bar for a saved-state card (verb)
    load: "Load",
    /// Media controls button (previous track)
    media_previous: "Previous",
    /// Media controls button
    media_pause: "Pause",
    /// Media controls button (next track)
    media_next: "Next",
    /// Error shown when an item can't be loaded for a trade
    deal_read_failed: "Couldn't read that item. Try again.",
    /// Error shown when listing an item with no price
    deal_price_zero: "Price must be above zero.",
    /// Under the app name in the Rare menu header: the app is open
    open_under_rare: "Open under Rare",
    /// Under the game name in the Rare menu header: the game is running
    running_under_rare: "Running under Rare",
    /// Hint bar: close the menu and return to the app
    back_to_app: "Back to the app",
    /// Hint bar: close the menu and return to the game
    back_to_game: "Back to game",
    /// Hint bar, before the Rare button glyph: "Hold [Rare] to switch games". Fragment around a button icon
    hold_before: "Hold",
    /// Hint bar, after the Rare button glyph: "Hold [Rare] to switch games". Fragment after a button icon
    hold_after: "to switch games",
    /// Beside the Rare title in the menu header; game is the game name
    menu_in_game: "in {game}",
    /// Label of the items summary strip (count of your items)
    strip_items: "Items",
    /// Label of the items summary strip: total value at floor price
    strip_floor: "At floor",
    /// Label of the items summary strip: total of best offers
    strip_offers: "Best offers",
    /// Hint bar for the X button on the Ask tab
    hold_to_talk: "Hold to talk",
    /// Hint bar for the Y button
    search_settings: "Search settings",
    /// Shown on the Items and Market tabs when signed out
    sign_in_passkey: "Sign in with a passkey to see your items.",
    /// Shown while a sale is being prepared
    building_sale: "Getting the sale ready...",
    /// Shown while a listing is being prepared
    building_listing: "Getting the listing ready...",
    /// Confirmation after a sale; amount is a number
    sold_for: "Sold for {amount} SUI. It can take a minute to show.",
    /// Confirmation after listing an item; amount is a number
    listed_for: "Listed for {amount} SUI.",
    /// Quit row title once armed, and its hint bar. Short
    quit_now: "Quit now",
    /// Quit row sub line once armed; A is a controller button
    quit_armed_sub: "Unsaved progress is lost. Press A again to quit.",
    /// Quit row title while the game is closing
    quitting: "Quitting...",
    /// Row on the Rare tab; title is the game or app name
    resume_title: "Resume {title}",
    /// Sub line of the Resume row on the Rare tab
    back_to_your_game: "Back to your game",
    /// Rare tab row title: your screenshots and clips
    share: "Share",
    /// Sub line of the Share row
    share_sub: "Your screenshots and clips",
    /// Sub line of the Quick settings row
    quick_settings_sub: "Brightness, volume, Wi-Fi, performance",
    /// Sub line of the Downloads row when nothing downloads
    nothing_downloading: "Nothing downloading",
    /// Rare tab row title
    your_profile: "Your profile",
    /// Sub line of the Power row
    power_sub: "Sleep, restart, turn off, desktop mode",
    /// Name shown when a person has no name
    someone: "Someone",
    /// Short duration; m is minutes
    min_short: "{m} min",
    /// Short duration; h hours, m minutes
    h_min_short: "{h} h {m} min",
    /// Short duration; h is hours
    h_short: "{h} h",
    /// Relative time, lowercase, inside a sentence like "Last seen just now"
    ago_just_now: "just now",
    /// Relative time; n is minutes
    ago_min: "{n} min ago",
    /// Relative time; n is hours
    ago_h: "{n} h ago",
    /// Relative time, lowercase, inside a sentence like "Last seen yesterday"
    ago_yesterday: "yesterday",
    /// Relative time; n is days
    ago_days: "{n} day ago|{n} days ago",
    /// Friend presence line; when is a relative time such as "3 h ago"
    last_seen: "Last seen {when}",
    /// Friend presence line; game is the game name
    away_in: "Away, in {game}",
    /// Friend presence line
    away: "Away",
    /// Friend presence line; time is a duration such as "42 min"
    playing_for: "Playing {game} for {time}",
    /// Friend presence line
    playing_game: "Playing {game}",
    /// Friends tab, signed out
    friends_sign_in: "Sign in with a passkey to see your friends.",
    /// Friends tab, no passkey yet
    friends_add_passkey: "Add a passkey to see your friends.",
    /// Friends tab error
    friends_failed: "Friends didn't load. Try again in a moment.",
    /// Friends tab before friends is turned on
    friends_turn_on_intro: "Turn on friends to see who is playing and join their games.",
    /// Button label while friends is turning on
    turning_on: "Turning on...",
    /// Button on the Friends tab
    turn_on_friends: "Turn on friends",
    /// Section heading on the Friends tab
    party: "Party",
    /// Sub line of a party invite; n is a number
    party_members_count: "{n} in the party",
    /// Party invite row title; name is a friend
    party_invited_you: "{name} invited you",
    /// Right-hand label on a party invite row (verb). Short
    join: "Join",
    /// Hint bar
    join_party: "Join party",
    /// Row title: the party you host
    your_party: "Your party",
    /// Row title; name is the friend who hosts
    party_of: "{name}'s party",
    /// Lowercase "you" in a list of party members
    you_lower: "you",
    /// Right-hand label on the party row (verb). Short
    invite: "Invite",
    /// Hint bar
    invite_friends: "Invite friends",
    /// Row title and hint bar
    leave_party: "Leave party",
    /// Row title and hint bar
    start_party: "Start a party",
    /// Sub line under Start a party
    start_party_sub: "Then invite friends to this game",
    /// Note beside the Messages heading; n unread messages
    new_count: "{n} new",
    /// Section heading on the Friends tab
    messages: "Messages",
    /// Label before the player's own message line: "You: text"
    say_you: "You",
    /// A message line; who is a name or You, text is the message
    say_line: "{who}: {text}",
    /// Hint bar for a message
    reply: "Reply",
    /// Section heading: friends who are playing
    playing_section: "Playing",
    /// Hint bar for a friend who is playing
    join_game: "Join game",
    /// Section heading: friends who are online
    online_section: "Online",
    /// Section heading: friend requests
    requests: "Requests",
    /// Row title
    friend_requests: "{n} friend request|{n} friend requests",
    /// Friends tab empty state
    friends_none_online: "None of your friends are online right now.",
    /// Hint bar for an offer notification
    act_accept_offer: "Accept offer",
    /// Hint bar for a notification
    act_open_item: "Open item",
    /// Hint bar for a notification
    act_open_collection: "Open collection",
    /// Hint bar for a notification (a trading position)
    act_open_position: "Open position",
    /// Hint bar for a notification
    act_open_coin: "Open coin",
    /// Hint bar for a notification
    act_view_tx: "View transaction",
    /// Hint bar for a notification (a trader profile)
    act_open_trader: "Open trader",
    /// Hint bar for a notification
    act_open_friends: "Open friends",
    /// Hint bar for a notification
    act_open_party: "Open party",
    /// Hint bar for an update notification
    act_restart_now: "Restart now",
    /// Hint bar for a notification
    act_open_library: "Open library",
    /// Hint bar for a notification
    act_open_emulators: "Open emulators",
    /// Hint bar for a notification
    act_open_trophies: "Open trophies",
    /// Hint bar for a battery notification
    act_open_power: "Open power",
    /// Compact time ago in the notifications list; n seconds
    time_s: "{n}s",
    /// Compact time ago; n minutes
    time_m: "{n}m",
    /// Compact time ago; n hours
    time_h: "{n}h",
    /// Compact time ago; n days
    time_d: "{n}d",
    /// Compact time ago; n months
    time_mo: "{n}mo",
    /// Compact time ago; n years
    time_y: "{n}y",
    /// Notifications tab empty state
    news_empty: "Nothing new. Sales, offers, invites and trophies show up here.",
    /// Button and hint bar on the Notifications tab
    mark_all_read: "Mark all read",
    /// Fallback name for a game whose name is unknown
    a_game: "A game",
    /// Sub line of the paused-games sheet; name is the game you are opening
    paused_sub: "Two games are paused. Close one and {name} pauses too, or keep it running.",
    /// Sheet title
    paused_title: "Close a paused game?",
    /// Status under a paused game
    paused: "Paused",
    /// Button on the paused-games sheet; name is a game
    keep_running: "Keep {name} running",
    /// Hint bar for the Keep running button
    keep_running_hint: "Keep it running",
    /// Status under an app that is open (adjective)
    status_open: "Open",
    /// Status under a game that is running
    status_running: "Running",
    /// Hint bar on a game card in the switcher (verb)
    switch: "Switch",
    /// Label above recently played games in the switcher
    recent: "Recent",
    /// Switcher footer label next to the Rare+Select chord
    save_clip_short: "Save clip",
    /// Status under a game card after one X press; X is a controller button
    press_x_again: "Press X again to close",
    /// Trophy tier name
    tier_bronze: "Bronze",
    /// Trophy tier name
    tier_silver: "Silver",
    /// Trophy tier name
    tier_gold: "Gold",
    /// Trophy tier name
    tier_platinum: "Platinum",
    /// Trophy row state: saved on chain (adjective). Short
    trophy_minted: "Minted",
    /// Trophy row state (adjective). Short
    trophy_unlocked: "Unlocked",
    /// Trophy row state (adjective). Short
    trophy_locked: "Locked",
    /// Trade error; venue is a marketplace name
    delist_first: "Delist it on {venue} first.",
    /// Trade error when listing
    err_price_zero: "Price must be above zero.",
    /// Trade error
    err_indexing: "This item is still being indexed. Try again in a minute.",
    /// Trade error
    err_offer_gone: "This offer can't be filled any more.",
    /// Trade error
    err_offer_collection: "This offer is for another collection.",
    /// Trade error
    err_offer_item: "This offer is for another item.",
    /// Trade error
    err_own_offer: "You can't accept your own offer.",
    /// Trade error when the market server is unreachable; rare.market is a web address
    err_unreachable: "Couldn't reach rare.market. Check the connection and try again.",
    /// Trade error fallback
    err_generic: "Something went wrong. Try again.",
    /// Items tab empty state
    items_empty: "You don't hold any items from this game yet. The Market tab has some for sale.",
    /// Column heading in the items list. Short
    col_item: "Item",
    /// Column heading: floor price. Short
    col_floor: "Floor",
    /// Column heading: best offer. Short
    col_offer: "Offer",
    /// Column heading: last sale price. Short
    col_last: "Last",
    /// Sub line of an item row; amount is a number
    listed_for_row: "Listed for {amount} SUI",
    /// Hint bar for an item row
    sell_or_list: "Sell or list",
    /// Note under the items list
    prices_in_sui: "Prices in SUI.",
    /// Market tab empty state
    market_empty: "Nothing from this game is listed right now.",
    /// Sub line of a market row; venue is a marketplace name
    coll_on_venue: "{collection} on {venue}",
    /// Sub line of a market row with no collection name; venue is a marketplace name
    on_venue: "on {venue}",
    /// Hint bar for a market row
    buy_hint: "Buy",
    /// Trophies tab empty state; game is the game name
    trophies_empty: "No trophies from {game} yet. Achievements and rewards the game gives you show up here.",
    /// Title of a trophy that is not revealed yet
    hidden_trophy: "Hidden trophy",
    /// Heading above trophies the game itself grants
    from_the_game: "From the game",
    /// Trophies summary; both numbers
    trophy_head: "{unlocked} of {total} unlocked",
    /// Trophies summary; all numbers
    trophy_head_minted: "{unlocked} of {total} unlocked, {minted} minted",
    /// Price choice chip (the floor price)
    price_floor: "Floor",
    /// Price choice chip
    price_floor_plus: "Floor +10%",
    /// Price choice chip (the current listing price)
    price_current: "Current",
    /// Under the Sell button; amounts are numbers, venue a marketplace
    sell_note_fees: "Best offer {amount} SUI on {venue}. You get {net} SUI after fees and royalty.",
    /// Under the Sell button; venue is a marketplace
    sell_note_full: "Best offer on {venue}. You get the full price.",
    /// Item detail line
    listed_for_on: "Listed for {amount} SUI on {venue}",
    /// Item detail stat label
    stat_floor: "Floor",
    /// Item detail stat label
    stat_best_offer: "Best offer",
    /// Item detail stat label
    stat_last_sale: "Last sale",
    /// Item detail, signed out
    sign_in_to_sell: "Sign in with a passkey to sell.",
    /// Item detail, no passkey
    add_passkey_to_sell: "Add a passkey to sell.",
    /// Button and hint bar
    add_passkey: "Add a passkey",
    /// Item detail; venue is a marketplace
    listed_elsewhere: "Listed on {venue}. Delist it there to sell or relist it here.",
    /// Section heading on item detail
    sell_now: "Sell now",
    /// Item detail
    looking_offers: "Looking for offers...",
    /// Item detail; why is a short reason sentence
    best_offer_blocked: "Best offer {amount} SUI on {venue}. {why}",
    /// Button; amount is a number
    sell_for: "Sell for {amount} SUI",
    /// Hint bar
    sell: "Sell",
    /// Item detail
    no_offers: "No offers on this item right now.",
    /// Section heading on item detail
    change_price: "Change the price",
    /// Section heading on item detail
    list_for_sale: "List for sale",
    /// Price chip; label is Floor, Floor +10% or Current; amount is a number
    price_chip: "{label} {amount}",
    /// Hint bar for a price chip
    pick: "Pick",
    /// Price chip: type your own price
    other_price: "Other price",
    /// Hint bar
    type_price: "Type a price",
    /// Button; amount is a number
    relist_for: "Relist for {amount} SUI",
    /// Button; amount is a number
    list_for: "List for {amount} SUI",
    /// Button label before a price is chosen
    pick_price: "Pick a price",
    /// Hint bar (verb)
    list: "List",
    /// Note under the list button
    own_kiosk: "Listed in your own kiosk. Buyers pay the creator royalty on top.",
    /// Status while the wallet asks for approval
    review_approve: "Review and approve in Rare Wallet.",
    /// Emulator action row in the Rare menu (short)
    emu_save_state: "Save state",
    /// Emulator action row (short)
    emu_load_state: "Load state",
    /// Emulator action row; n is the save slot number (short)
    emu_slot: "Slot {n}",
    /// Emulator action row (short)
    emu_fast_forward: "Fast forward",
    /// Emulator action row (short)
    emu_rewind: "Rewind 5 seconds",
    /// Emulator action row: switch to the next disc (short)
    emu_next_disc: "Next disc",
    /// Emulator action row: opens the emulator's own settings (short)
    emu_settings: "Emulator settings",
    /// Emulator action row: opens the emulator's own menu (short)
    emu_menu: "Emulator menu",
    /// Note on a load row when hardcore mode is on
    emu_off_hardcore: "Off in hardcore",
    /// Note on an emulator row that cannot work yet
    emu_no_settings: "The emulator has no settings file yet. Start it once from Rare first.",
    /// Note on an emulator row that cannot work yet
    emu_binding_next: "Rare adds this binding the next time it starts the game.",
    /// Note on a PPSSPP row; action is the PPSSPP action name, key a keyboard key
    emu_bind_ppsspp: "Bind {action} to {key} once in PPSSPP, Settings, Controls.",
}
