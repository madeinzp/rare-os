strings! {
    /// Apps rail heading and tile hint prefix: heading of the Apps section on Home (noun)
    apps_title: "Apps",
    /// Count next to the Apps heading; n is a number
    n_running: "{n} running",
    /// Small state line under an app tile (short)
    tile_installing: "Installing",
    /// Small state line under an app tile (short)
    tile_running: "Running",
    /// Small state line under an app tile (short)
    tile_not_installed: "Not installed",
    /// Hint-bar text for an app tile that is not installed (verb: get the app)
    get: "Get",
    /// App page status line when the app is running
    status_running: "Running",
    /// App page status line; {browser} is the app or browser it opens in, like "Google Chrome"
    opens_in: "Opens in {browser}",
    /// App page status line; {name} is what must be installed first, like "Google Chrome"
    needs: "Needs {name}",
    /// App page status line
    cant_open: "Can't open on this device",
    /// App page button (short)
    close_app: "Close",
    /// Hint-bar text for the Close button on an app page
    close_app_hint: "Close app",
    /// App page button; {name} is the app or way it opens, like "Google Chrome"
    open_in: "Open in {name}",
    /// App page button while installing (short)
    installing: "Installing",
    /// App page button and hint; {name} is the thing to install
    get_name: "Get {name}",
    /// App page note for Battle.net; {name} is Battle.net
    install_blizzard: "Rare gets {name} from Blizzard and sets it up for you, no password needed. Then sign in with your Blizzard account in its window.",
    /// App page note; {name} is an app like "Telegram Desktop"
    install_flathub_full: "{name} is the full desktop app. It installs from Flathub for you only, no password needed.",
    /// App page note; {app} is the app, {browser} the browser it opens in
    install_flathub_in: "{app} opens in {browser}. It installs from Flathub for you only, no password needed.",
    /// App page note under the Battle.net install bar
    proton_note: "The first time, Rare also downloads Proton (up to 1 GB). Blizzard's window opens when it's ready.",
    /// Label above a terminal command
    desktop_mode_terminal: "Or in Desktop mode, in a terminal:",
    /// Heading above the controller layout (noun)
    controls: "Controls",
    /// Note under the controller layout (Rare button)
    come_back_note: "Press the Rare button to come back to Rare, then Resume or Close it.",
    /// Same note in guest mode (View button held)
    come_back_note_guest: "Hold View to come back to Rare, then Resume or Close it.",
    /// Running list: row note while an app is quitting
    quitting: "Quitting...",
    /// Running list: row note after the first press of quit
    unsaved_lost: "Unsaved progress is lost. Press A again to quit.",
    /// Running list button when armed (short: button)
    quit_now: "Quit now",
    /// Running list button for a game (short: button)
    quit_game: "Quit game",
    /// Running list sheet title
    running_title: "Running",
    /// Running list subtitle
    resume_or_quit: "Resume one, or quit it.",
    /// Running list empty state
    nothing_running: "Nothing is running.",
    /// Running list footer, text before a button icon: "Hold [View icon] in any game to come back here." Keep the word order so the icon fits after it
    footer_hold: "Hold",
    /// Running list footer, text before a button icon (the Rare button)
    footer_press: "Press",
    /// Running list footer, text after the button icon
    footer_after: "in any game to come back here.",
    /// Toast after opening a pointer app; {name} is the app
    pointer_toast: "{name}: the right stick moves the pointer. Press the Rare button to come back to Rare.",
    /// Same toast in guest mode
    pointer_toast_guest: "{name}: the right stick moves the pointer. Hold View to come back to Rare.",
    /// Toast; {name} is the app
    opening: "Opening {name}",
    /// Toast (error); {name} is the app
    didnt_start: "{name} didn't start. Try again.",
    /// Toast (error); {name} is the app
    cant_switch_back: "Couldn't switch back to {name}. Try again.",
    /// Toast; {name} is the app
    closing: "Closing {name}",
    /// Toast (error); {name} is the app
    cant_close: "Couldn't close {name}. Try again.",
    /// Toast (error) for an install that failed without a reason; {name} is the app
    cant_install: "Couldn't install {name}. Check your Wi-Fi and try again.",
    /// Toast (error) for an install that failed with a reason; {name} is the app, {reason} a sentence without its final period
    cant_install_why: "Couldn't install {name}. {reason}.",
    /// Toast; {name} is the app
    installed_toast: "{name} is installed",
    /// Name of a running PC game streamed from another computer when it has no title
    pc_game: "PC game",
    /// Name of a running game that has no title
    game_fallback: "Game",
    /// Toast (error)
    cant_switch_back_generic: "Couldn't switch back. Try again.",
    /// Toast (error)
    cant_close_generic: "Couldn't close it. Try again.",
    /// Way of opening Steam's library, shown as "Open in ..."
    steam_library: "Steam's library",
    /// Controller layout, one line each: what the button does
    ctl_pointer: "Moves the pointer",
    ctl_click: "Click (RT too)",
    ctl_right_click: "Right click",
    ctl_scrolls: "Scrolls",
    ctl_back_page: "Back a page",
    ctl_dpad_back_forward: "Left and right: back and forward",
    ctl_prev_tab: "Previous tab",
    ctl_next_tab: "Next tab",
    ctl_new_tab: "New tab",
    ctl_address_bar: "Address bar",
    ctl_close_tab: "Close tab",
    ctl_keyboard: "On-screen keyboard",
    ctl_arrow_keys: "Arrow keys",
    ctl_back_escape: "Back (Escape)",
    ctl_space: "Space: play or pause",
    ctl_enter: "Enter",
    ctl_moves_buttons: "Moves between buttons",
    ctl_select: "Select",
    ctl_moves_a_pointer: "Moves a pointer",
    ctl_app_reads: "The app reads your controller itself",
    /// Controller layout key names shown as text
    key_right_stick: "Right stick",
    key_left_stick: "Left stick",
    key_dpad: "D-pad",
    /// Steam app page text
    about_steam: "Steam's store, library, friends and settings. Press the Rare button to come back to Rare.",
    about_battlenet: "Blizzard's app for World of Warcraft, Diablo, Overwatch, Hearthstone, StarCraft and Call of Duty. Sign in with your Blizzard account there. The games you install show in Library > Battle.net and start from Rare.",
    plan_battlenet: "Playing needs a Blizzard account. Free games like Hearthstone and Overwatch 2 need nothing else.",
    about_telegram: "Chats and channels.",
    about_x: "Posts, news and the Sui timeline.",
    about_netflix: "Films and series. Plays in Google Chrome, which has the video protection Netflix needs.",
    about_youtube: "YouTube's TV layout, made for a controller.",
    about_spotify: "Music and podcasts. Music keeps playing while you play a game, and the Rare menu and Quick settings have play, pause and skip.",
    about_twitch: "Live streams and chat. A stream keeps playing while you play a game, and the Rare menu and Quick settings can pause it.",
    about_discord: "Voice and text chat with your friends and servers.",
    about_xcloud: "Xbox games streamed from the cloud. Sign in with your Microsoft account once; the controller works as on an Xbox. The Rare button opens Rare's menu; hold View for Xbox's own menu.",
    plan_xcloud: "Playing needs a Game Pass plan with cloud gaming. Free-to-play games like Fortnite only need a Microsoft account.",
    about_geforcenow: "Your PC games from Steam, Epic and other stores, streamed from NVIDIA's servers. Sign in with your NVIDIA account once and link your stores there. The Rare button opens Rare's menu; hold View for GeForce NOW's own menu.",
    plan_geforcenow: "The free plan has ads, a queue and one-hour sessions. Paid plans remove them. You also need to own the game in your linked store.",
    about_chiaki: "Play your PS5 or PS4 on this screen over your home network, with chiaki-ng, a free open-source app.",
    plan_chiaki: "Playing needs a PS5 or PS4 on the same network and your PlayStation account.",
    chiaki_step_1: "On your PlayStation, go to Settings > System > Remote Play and turn on Enable Remote Play.",
    chiaki_step_2: "Open PlayStation Remote Play here, sign in with your PlayStation account and pick your console.",
    chiaki_step_3: "On the console, choose Pair Device and type the code it shows here.",
    about_steamlink: "Play the games on a PC running Steam, streamed to this screen over your home network.",
    plan_steamlink: "Playing needs a PC with Steam open on the same network.",
    steamlink_step_1: "On your PC, open Steam and sign in. Leave it running.",
    steamlink_step_2: "Open Steam Link here and pick your PC.",
    steamlink_step_3: "Type the code Steam Link shows into Steam on your PC.",
    steamlink_step_4: "Signed in to the same Steam account here? Your PC's games also show in Steam's library with Stream.",
    about_chrome: "The full web, with tabs.",
}
