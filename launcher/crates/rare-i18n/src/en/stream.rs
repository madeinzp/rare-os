strings! {
    /// Line under a game cover: the game is already running on your PC
    running_on_pc: "Running on your PC",
    /// Line under a game cover; name is the PC name
    host_asleep: "{name} is asleep",
    /// Line under a game cover; name is the PC name
    host_off: "{name} is off",
    /// Home strip heading for games streamed from your PC
    on_your_pc: "On your PC",
    /// Status next to a PC name: a game is running on it
    playing_on_pc: "Playing on the PC",
    /// Status next to a PC name while it wakes up
    waking_up: "Waking up",
    /// Status next to a PC name: it is asleep
    asleep: "Asleep",
    /// Status next to a PC name: it is switched off
    off: "Off",
    /// Heading of the empty Streaming library card
    play_games_title: "Play the games on your PC",
    /// Body of the empty Streaming library card
    play_games_body: "Stream from a gaming PC running Sunshine or GeForce Experience, over your home Wi-Fi.",
    /// Button and hint: open streaming setup
    set_up_streaming: "Set up streaming",
    /// Heading when a PC lists no games; name is the PC name
    no_games_listed: "No games listed on {name}",
    /// Body when a PC lists no games
    no_games_listed_body: "Add games in Sunshine's Applications tab, then look again.",
    /// Heading when a PC does not answer; name is the PC name
    host_not_answering: "{name} isn't answering",
    /// Body when a PC does not answer
    host_not_answering_body: "Turn it on, or wake it from here if Wake-on-LAN is set up.",
    /// Button: search the network or the game list again
    look_again: "Look again",
    /// Button: wake a PC; name is the PC name
    wake_host: "Wake {name}",
    /// Toast when the typed PC address is invalid
    bad_address: "That doesn't look like an address",
    /// Second sentence of a streaming error toast
    try_again_sentence: "Try again.",
    /// Second sentence of a streaming error toast
    check_wifi_sentence: "Check your Wi-Fi and try again.",
    /// Toast: searching the network for PCs failed
    err_look: "Couldn't look for PCs.",
    /// Toast: adding a PC by address failed
    err_add: "Couldn't add that PC. Check the address and try again.",
    /// Toast: the PC has no known address; name is the PC name
    unknown_address: "Rare doesn't know where {name} is. Add it by address.",
    /// Toast: pairing failed
    err_pair: "Couldn't pair with that PC.",
    /// Toast: pairing worked; name is the PC name
    is_paired: "{name} is paired",
    /// Toast: listing the games on a PC failed
    err_list: "Couldn't list the games on that PC.",
    /// Toast when Play is pressed and the PC does not answer; name is the PC name
    host_not_answering_play: "{name} isn't answering. Turn it on, then try again.",
    /// Toast when a stream starts; game is a title, name is the PC name
    streaming_toast: "Streaming {game} from {name}. Press the Rare button to come back to Rare.",
    /// Toast when a stream starts in guest mode; game is a title, name is the PC name
    streaming_toast_guest: "Streaming {game} from {name}. Hold View to come back to Rare.",
    /// Label on a disabled button while a stream starts (short)
    busy_starting: "Starting",
    /// Label on a disabled button while a stream resumes (short)
    busy_opening: "Opening",
    /// Label on a disabled button while a game on the PC quits (short)
    busy_quitting: "Quitting",
    /// Toast: starting the game failed
    err_start: "Couldn't start the game.",
    /// Toast: resuming the game failed
    err_resume: "Couldn't resume the game.",
    /// Toast while quitting the game on a PC; name is the PC name
    quitting_on: "Quitting the game on {name}",
    /// Toast: closing the game failed
    err_close: "Couldn't close the game.",
    /// Toast: waking the PC failed
    err_wake: "Couldn't wake that PC.",
    /// Toast: saving the stream settings failed
    err_save: "Couldn't save the streaming settings.",
    /// First sentence of the error when installing Moonlight failed
    err_setup: "Couldn't set up streaming.",
    /// Toast: Moonlight finished installing
    moonlight_installed: "Moonlight is installed",
    /// Toast: the PC did not wake; name is the PC name
    wake_failed: "{name} didn't wake up. Check Wake-on-LAN is on in its BIOS and network settings.",
    /// Title of a running stream whose game is unknown; name is the PC name
    game_on_host: "A game on {name}",
    /// Title of a running stream whose game and PC are unknown
    pc_game: "PC game",
    /// Heading while Moonlight installs, and a disabled button (short)
    getting_moonlight: "Getting Moonlight",
    /// Sheet text while Moonlight installs and pairing waits; name is the PC name
    moonlight_why_waiting: "Rare streams with Moonlight, a free open-source app. It takes a minute; pairing with {name} starts once it's in.",
    /// Sheet text while Moonlight installs
    moonlight_why_busy: "Rare streams with Moonlight, a free open-source app. It takes a minute, and you can look for your PC meanwhile.",
    /// Sheet text before Moonlight is installed
    moonlight_why: "Rare streams with Moonlight, a free open-source app. It installs from Flathub for you only, no password needed.",
    /// Label above a terminal command shown when the install failed
    or_desktop_terminal: "Or in Desktop mode, in a terminal:",
    /// Status of a PC row: pairing is done (short)
    paired: "Paired",
    /// Status of a PC row: not paired yet (short)
    not_paired: "Not paired",
    /// Status of a PC row: the PC does not answer (short)
    not_answering: "Not answering",
    /// Button and hint: show the games on a paired PC
    show_games: "Show games",
    /// Hint bar on a PC row: start pairing (verb, short)
    pair: "Pair",
    /// Title of the streaming setup sheet
    stream_from_pc: "Stream from your PC",
    /// Heading above the list of PCs found
    pcs_on_network: "PCs on your network",
    /// Shown next to a heading while searching the network (short)
    looking: "Looking",
    /// Empty state of the list of PCs
    no_pcs_yet: "No PCs found yet.",
    /// Note on the setup sheet; {link} is replaced by the Sunshine web address, keep it
    sunshine_note: "Your PC needs Sunshine (free, for any graphics card) or GeForce Experience, on the same network. Install Sunshine from {link}, start it, then look again.",
    /// Hint bar: install Moonlight
    get_moonlight: "Get Moonlight",
    /// Button and hint: add a PC by typing its address
    add_by_address: "Add by address",
    /// Title of the pairing sheet; name is the PC name
    pair_with: "Pair with {name}",
    /// Subtitle of the pairing sheet
    pairing_once: "Pairing happens once. After that your PC's games show up in your Library.",
    /// Pairing step 1 for GeForce Experience; name is the PC name
    gfe_step1: "GeForce Experience shows a box on {name}.",
    /// Pairing step 2 for GeForce Experience; the PIN is shown on screen
    gfe_step2: "Type this PIN in it.",
    /// Last pairing step
    finishes_on_own: "Rare finishes on its own.",
    /// Pairing step 1 for Sunshine; name is the PC name, {url} is replaced by a web address, keep it
    sunshine_step1: "On {name}, open Sunshine at {url}",
    /// Pairing step 2 for Sunshine
    sunshine_step2: "Go to PIN. Enter this PIN and a name for this SuiPlay.",
    /// Pairing step 3 for Sunshine
    sunshine_step3: "Select Send. Rare finishes on its own.",
    /// Pairing status; name is the PC name
    paired_with: "Paired with {name}",
    /// Pairing status; name is the PC name
    waiting_for: "Waiting for {name}",
    /// Game page status: the game is streaming to this device
    streaming_here: "Streaming to this SuiPlay",
    /// Game page status; name is the PC name
    line_on_pc: "Running on {name}. Resume picks it up where you left it.",
    /// Game page status; game is another game's title, name is the PC name
    line_busy: "{game} is running on {name}. Play quits it first.",
    /// Game page status and button while a PC wakes; name is the PC name
    waking_host: "Waking {name}",
    /// Game page status; name is the PC name
    line_asleep: "{name} is asleep. Play wakes it first.",
    /// Game page status; name is the PC name
    line_offline: "{name} isn't answering. Turn it on, then look again.",
    /// Game page status; name is the PC name
    line_ready: "Ready on {name}",
    /// Name used for a game on the PC whose title is unknown
    another_game: "Another game",
    /// Subtitle of a game page; name is the PC name
    on_host: "On {name}",
    /// Heading of the stream quality settings
    stream_quality: "Stream quality",
    /// Setting row label
    resolution: "Resolution",
    /// Setting row label
    bitrate: "Bitrate",
    /// Setting row label for the video codec
    video: "Video",
    /// Value of the codec setting when Auto picked one; codec is e.g. AV1
    auto_codec: "Auto ({codec})",
    /// Note under the stream settings
    quality_note: "For every PC game, at 60 fps. Lower the bitrate if the picture stutters on busy Wi-Fi. While you play, Moonlight reads your controller. Press the Rare button to come back to Rare and resume or quit.",
    /// Note under the stream settings in guest mode
    quality_note_guest: "For every PC game, at 60 fps. Lower the bitrate if the picture stutters on busy Wi-Fi. While you play, Moonlight reads your controller. Hold View to come back to Rare and resume or quit.",
    /// Button: quit the game running on the PC
    quit_game: "Quit game",
    /// Button: quit the game running on the PC, then play this one; game is the other game's title
    quit_and_play: "Quit {game} and play",
    /// Button: wake the PC and start the game
    wake_and_play: "Wake and play",
}
