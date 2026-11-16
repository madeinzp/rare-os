strings! {
    /// Title of the page listing the button map of games and emulators
    buttons_in_games: "Buttons in games",
    /// Intro under the "Buttons in games" title
    buttons_in_games_intro: "The same in every game and emulator. Rare sets each emulator up this way and keeps your own changes.",
    /// Card title: the button map shared by all games
    every_game: "Every game",
    /// Toast after the player picked a cover for a game
    art_kept_cover: "New cover kept for this game",
    /// Toast after the player picked a hero image (wide banner) for a game
    art_kept_hero: "New hero image kept for this game",
    /// Toast after the player picked a logo for a game
    art_kept_logo: "New logo kept for this game",
    /// Toast after the player went back to the original cover
    art_back_cover: "The original cover is back",
    /// Toast after the player went back to the original hero image
    art_back_hero: "The original hero image is back",
    /// Toast after the player went back to the original logo
    art_back_logo: "The original logo is back",
    /// Error toast when saving a picked picture failed
    art_keep_failed: "Couldn't keep that picture.",
    /// Title of the art page; {title} is the game's name
    art_title: "{title} art",
    /// Subtitle of the art page
    art_subtitle: "Pick what shows in your library and on the game's page",
    /// Note on the art page while searching
    art_looking: "Looking for art",
    /// Note on the art page when the search failed; {error} is the reason
    art_lookup_failed: "Couldn't look for art: {error}",
    /// Note on the art page when the search found nothing
    art_none_found: "Nothing found for this name. Pick a picture from this device.",
    /// Note on the art page: the pictures come from SteamGridDB
    art_from_sgdb: "From SteamGridDB",
    /// Note on the art page: the pictures come from Steam's store
    art_from_steam: "From Steam's store. Add a SteamGridDB key in Settings > Storage for more choices.",
    /// Small label on the picture that is currently used
    in_use: "In use",
    /// Controller hint on a picture or file: pick it (short)
    pick: "Pick",
    /// Tile: choose a picture from a file on this device
    art_from_device: "From this device",
    /// Tile: go back to the game's original art
    art_use_original: "Use the original",
    /// Hint bar: LB and RB switch between art types
    art_tabs_hint: "Cover, hero, logo",
    /// Tab on the art page: the game's cover image
    art_cover: "Cover",
    /// Tab on the art page: the game's wide banner image
    art_hero: "Hero",
    /// Tab on the art page: the game's logo
    art_logo: "Logo",
    /// File picker: kind label of a Windows program (.exe), shown at the right of the row (short)
    pick_windows_game: "Windows game",
    /// File picker: kind label of an AppImage file (short)
    pick_appimage: "AppImage",
    /// File picker: kind label of a shell script (short)
    pick_script: "Script",
    /// File picker: kind label of a native Linux program (short)
    pick_linux_game: "Linux game",
    /// File picker: kind label of an image file (short)
    pick_picture: "Picture",
    /// Name of the SD card drive
    sd_card: "SD card",
    /// File picker: a place that does not exist yet; {path} is the folder path
    pick_made_when_picked: "{path} (made when you pick it)",
    /// File picker place: the Games folder in the device's home
    pick_games_on_device: "Games on this device",
    /// File picker place: Games folder on the SD card
    pick_games_on_sd: "Games on the SD card",
    /// File picker place: Games folder on an external drive; {drive} is the drive's name
    pick_games_on_drive: "Games on {drive}",
    /// File picker place: the user's home folder
    pick_home_folder: "Home folder",
    /// File picker place: the Pictures folder
    pick_pictures: "Pictures",
    /// File picker place: the Desktop folder
    pick_desktop: "Desktop",
    /// File picker first row: add the folder that is open as a game
    pick_add_folder: "Add this folder as a game",
    /// File picker first row: use the folder that is open as the games folder
    pick_use_folder: "Use this folder",
    /// File picker title when adding a game
    pick_title_add: "Add a game",
    /// File picker title when choosing a cover
    pick_title_cover: "Pick a cover",
    /// File picker title when choosing a hero image
    pick_title_hero: "Pick a hero image",
    /// File picker title when choosing a logo
    pick_title_logo: "Pick a logo",
    /// File picker title when choosing a games folder
    pick_title_folder: "Pick a games folder",
    /// Toast while a game is being added from the file picker
    pick_adding: "Adding the game and looking for its art",
    /// Toast after a game was added; {title} is the game's name
    pick_added: "Added {title}",
    /// File picker subtitle at the places list when adding a game
    pick_sub_add: "Pick a Windows .exe, an AppImage, a .sh script, or a game's folder",
    /// File picker subtitle at the places list when choosing a picture
    pick_sub_image: "Pick a PNG, JPG or WebP picture",
    /// File picker subtitle at the places list when choosing a games folder
    pick_sub_folder: "Rare looks for one game in each folder inside it",
    /// File picker: kind label of a folder row (short, noun)
    pick_folder: "Folder",
    /// File picker empty folder, adding a game. B is the controller's B button
    pick_empty_add: "Nothing here starts a game. B goes back a folder.",
    /// File picker empty folder, choosing a picture. B is the controller's B button
    pick_empty_image: "No pictures here. B goes back a folder.",
    /// File picker empty folder, choosing a games folder. B is the controller's B button
    pick_empty_folder: "No folders in here. B goes back a folder.",
    /// Hint bar in the file picker: B goes up one folder
    pick_up_folder: "Up a folder",
    /// Check files status when Steam stopped; {reason} is Steam's own text
    verify_steam_stopped: "Steam stopped the check: {reason}",
    /// Check files status: Steam replaced {n} files that were missing
    verify_replaced_missing: "Steam replaced {n} missing file|Steam replaced {n} missing files",
    /// Check files status: Steam replaced {n} files that were damaged
    verify_replaced_damaged: "Steam replaced {n} damaged file|Steam replaced {n} damaged files",
    /// Check files status: {m} missing and {c} damaged files replaced; n is m plus c (plural only)
    verify_replaced_both: "Steam replaced {m} missing and {c} damaged file|Steam replaced {m} missing and {c} damaged files",
    /// Check files status when all files were fine
    verify_all_fine: "All files are fine",
    /// Check files status when Steam replaced files and gave no count
    verify_replaced_some: "Steam replaced missing or damaged files",
    /// Check files status while Steam repairs files
    verify_replacing: "Steam is replacing missing or damaged files",
    /// Check files status with progress; {done} and {total} are sizes like "809 MB" and "9.8 GB"
    verify_checking_progress: "Checking files: {done} of {total}",
    /// Check files status while Rare makes the first file list; {done} and {total} are sizes
    verify_listing_progress: "Making a list of the files: {done} of {total}",
    /// Check files status without progress
    verify_checking: "Checking files",
    /// Check files status while waiting for Steam to begin
    verify_waiting: "Waiting for Steam to start the check",
    /// Error shown when Check files needs Steam and it is missing
    verify_no_steam: "Steam is not installed.",
    /// Check files status when Steam never began the check
    verify_steam_no_start: "Steam didn't start the check. Try again, or use Open in Steam.",
    /// Check files result: all {n} files are the same as the saved list; n is a formatted number
    verify_all_match: "All {n} file matches the saved list|All {n} files match the saved list",
    /// Part of the file-check result: {n} files changed; n is a formatted number
    verify_files_changed: "{n} file changed|{n} files changed",
    /// Part of the file-check result: {n} files missing; n is a formatted number
    verify_files_missing: "{n} file missing|{n} files missing",
    /// File-check result; {changed} and {missing} are texts like "1 file changed" and "2 files missing"; {names} is up to two file names
    verify_differs_both: "{changed}, {missing} ({names}). Reinstall or copy the game again to repair it",
    /// File-check result; {part} is a text like "1 file changed" or "2 files missing"; {names} is up to two file names
    verify_differs_one: "{part} ({names}). Reinstall or copy the game again to repair it",
    /// Error when the game's folder no longer exists
    verify_folder_gone: "The game's folder isn't there any more.",
    /// File-check status after saving the current files as the good copy; n is a number
    verify_saved_good: "Saved {n} file as the good copy|Saved {n} files as the good copy",
    /// File-check status after the first list was saved; n is a number
    verify_saved_list: "Saved a list of {n} file. The next check compares against it|Saved a list of {n} files. The next check compares against it",
    /// File-check status when saving the list failed; {error} is the reason
    verify_save_failed: "Couldn't save the file list: {error}",
    /// Name used for a Wi-Fi network without a name, in "Sign in to {network}" and similar (lower case, mid-sentence)
    portal_this_network: "this network",
    /// Network name used in "Connected to {network}" when the Wi-Fi name is unknown (lower case, mid-sentence)
    portal_the_internet: "the internet",
    /// On-screen keyboard: the Shift key cap (short)
    key_shift: "Shift",
    /// On-screen keyboard: the space bar cap (short)
    key_space: "Space",
    /// On-screen keyboard: the paste key cap (short)
    key_paste: "Paste",
    /// On-screen keyboard: the Enter key cap inside games (short)
    key_enter: "Enter",
    /// On-screen keyboard controller hint line when typing in the launcher. A, X, Y, LT, Menu and B are controller button names and stay; {shift} and {layer} are key caps like "Shift" and "?123"
    kb_hint_launcher: "A Type    X Delete    Y {shift}    LT {layer}    Menu Done    B Cancel",
    /// On-screen keyboard controller hint line when typing over a game. A, X, Y, LT, Menu, View, LB, RB and B are controller button names and stay; {shift} and {layer} are key caps
    kb_hint_over: "A Type    X Delete    Y {shift}    LT {layer}    Menu Enter    View Tab    LB RB Cursor    B Close",
    /// Toast when first-run setup finishes. "Rare button" is the physical button on the device
    firstrun_done_toast: "Press the Rare button in any game for the Rare menu",
    /// Prompt title asking for a Wi-Fi password during setup; {ssid} is the network name
    firstrun_wifi_password: "Password for {ssid}",
    /// Setup step title for emulators while the list is loading
    firstrun_emulators: "Emulators",
    /// Setup emulators step loading line
    firstrun_looking_games: "Looking for your games",
    /// Controller hint during setup: Y skips the rest of setup
    firstrun_skip_setup: "Skip setup",
    /// Parent's PIN sheet title when a PIN already exists
    pin_change_title: "Change the PIN",
    /// Parent's PIN sheet title when none exists yet
    pin_set_title: "Set a parent's PIN",
    /// PIN sheet text asking to type the new PIN a second time
    pin_enter_again: "Enter the same PIN again.",
    /// PIN sheet text when choosing a new PIN; OK is the pad's OK key
    pin_pick_digits: "Pick 4 to 6 digits, then press OK.",
    /// PIN sheet status after too many wrong tries; {wait} is a duration like "30 seconds"
    pin_too_many: "Too many tries. Try again in {wait}.",
    /// PIN sheet error after a wrong PIN; {n} is the tries left before a wait
    pin_wrong: "Wrong PIN. {n} try before a wait.|Wrong PIN. {n} tries before a wait.",
    /// PIN sheet error when the new PIN has the wrong length
    pin_use_digits: "Use 4 to 6 digits.",
    /// Toast after the PIN was changed
    pin_changed: "PIN changed",
    /// Toast after the first PIN was set
    pin_set: "PIN set. Parental controls are on.",
    /// PIN sheet error when the second entry differs
    pin_mismatch: "The PINs didn't match. Try again.",
    /// Parental controls: minutes, e.g. "30 min" (short, unit)
    parental_min: "{n} min",
    /// Parental controls: hours, e.g. "1.5 h" (short, unit); n may have a decimal
    parental_hours: "{n} h",
    /// Parental controls: hours and minutes, e.g. "1 h 20 min" (short)
    parental_hours_min: "{h} h {m} min",
    /// Parental controls age limit choice, e.g. "Up to 12" (the game's age rating may be at most n)
    parental_up_to: "Up to {n}",
    /// Parental controls profile name used without a signed-in account
    parental_this_device: "This device",
    /// Blocked launch title: the daily limit is spent
    parental_used_up: "Today's play time is used up",
    /// Blocked launch title; {from} and {to} are hours like "8:00"
    parental_allowed_from: "Playing is allowed from {from} to {to}",
    /// Blocked launch title when the game's age rating is too high
    parental_needs_pin: "This game needs a parent's PIN",
    /// Blocked launch reason; {age} is the game's age rating
    parental_rated: "It's rated for ages {age} and up. A parent's PIN lets it start.",
    /// Blocked launch reason for time limits
    parental_pin_lets: "A parent's PIN lets this game start.",
    /// In-game notice body when the allowed hours are over; {from} and {to} are hours like "8:00"
    parental_ends_window_up: "Playing is allowed from {from} to {to}. Save your game and take a break.",
    /// In-game notice body before the allowed hours end; {time} is an hour like "20:00"
    parental_ends_at: "Playing time ends at {time}. Save your game.",
    /// In-game notice body when the daily limit is spent
    parental_ends_used_up: "Today's play time is used up. Save your game and take a break.",
    /// In-game notice body shortly before the daily limit is spent
    parental_ends_soon: "Today's play time ends soon. Save your game.",
    /// In-game notice title when play time ran out
    parental_time_up: "Time is up",
    /// In-game notice title; {n} is the minutes left
    parental_minutes_left: "{n} minute of play time left|{n} minutes of play time left",
    /// Cloud saves error: no wallet account yet. "Rare Wallet" is a product name, "Wallet" the app's Wallet tab
    saves_add_account: "Add a Rare Wallet account first, in Wallet.",
    /// Text on the wallet's approval screen when unlocking cloud saves
    saves_unlock_prompt: "Unlock cloud saves",
    /// Cloud saves error when the wallet service stopped
    saves_wallet_stopped: "The wallet stopped. Try again.",
    /// Cloud saves error when the save to back up is gone
    saves_gone: "That save isn't on this device anymore.",
    /// Cloud saves error when the Sui network is unreachable
    saves_no_sui: "Couldn't reach Sui. Check the connection and try again.",
    /// Cloud saves error when the wallet account cannot sign
    saves_add_signing_account: "Add a Rare Wallet account you can sign with first",
    /// Toast after a cloud backup; {what} is the name of the saves, e.g. "RetroArch Save files"
    saves_backed_up: "{what} backed up",
    /// Toast after restoring saves when the old saves were moved aside; {folder} is a folder name
    saves_restored_kept: "Restored. Your previous saves are kept in {folder}",
    /// Toast after restoring saves; {emulator} is the emulator's name
    saves_restored: "{emulator} saves restored",
    /// Month name, full
    stats_month_january: "January",
    /// Month name, full
    stats_month_february: "February",
    /// Month name, full
    stats_month_march: "March",
    /// Month name, full
    stats_month_april: "April",
    /// Month name, full
    stats_month_may: "May",
    /// Month name, full
    stats_month_june: "June",
    /// Month name, full
    stats_month_july: "July",
    /// Month name, full
    stats_month_august: "August",
    /// Month name, full
    stats_month_september: "September",
    /// Month name, full
    stats_month_october: "October",
    /// Month name, full
    stats_month_november: "November",
    /// Month name, full
    stats_month_december: "December",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_jan_short: "Jan",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_feb_short: "Feb",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_mar_short: "Mar",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_apr_short: "Apr",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_may_short: "May",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_jun_short: "Jun",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_jul_short: "Jul",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_aug_short: "Aug",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_sep_short: "Sep",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_oct_short: "Oct",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_nov_short: "Nov",
    /// Month name, abbreviated to fit a small chart label or date (3 letters in English)
    stats_month_dec_short: "Dec",
    /// Day of the week, full
    stats_day_monday: "Monday",
    /// Day of the week, full
    stats_day_tuesday: "Tuesday",
    /// Day of the week, full
    stats_day_wednesday: "Wednesday",
    /// Day of the week, full
    stats_day_thursday: "Thursday",
    /// Day of the week, full
    stats_day_friday: "Friday",
    /// Day of the week, full
    stats_day_saturday: "Saturday",
    /// Day of the week, full
    stats_day_sunday: "Sunday",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_mon_short: "Mon",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_tue_short: "Tue",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_wed_short: "Wed",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_thu_short: "Thu",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_fri_short: "Fri",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_sat_short: "Sat",
    /// Day of the week, abbreviated to fit a small chart label (3 letters in English)
    stats_day_sun_short: "Sun",
    /// Stats tab and card caption for the current week (the span chosen)
    stats_this_week: "This week",
    /// Stats tab and card caption for the current month
    stats_this_month: "This month",
    /// Stats tab and card caption for the current year
    stats_this_year: "This year",
    /// Play time in minutes, e.g. "35 min" (short)
    stats_min: "{n} min",
    /// Play time in hours, e.g. "12.4 h" (short); n may have a decimal
    stats_hours: "{n} h",
    /// A number of days, e.g. "3 days" (streak)
    stats_days_n: "{n} day|{n} days",
    /// Stats header, right side; {month} is an abbreviated month and {day} the day number, e.g. "Counted by Rare since Mar 4"
    stats_since: "Counted by Rare since {month} {day}",
    /// Same as stats_since for an earlier year; {year} is e.g. 2025
    stats_since_year: "Counted by Rare since {month} {day}, {year}",
    /// Stats page title
    stats_title: "Your stats",
    /// Button on the stats empty state: opens the Library
    stats_go_library: "Go to Library",
    /// Empty state title on the stats page
    stats_empty_title: "No play counted yet",
    /// Empty state text on the stats page
    stats_empty_body: "Rare counts the time you spend in games it starts. Play something and your week shows here.",
    /// Used in comparisons like "2 h more than last week" (lower case, mid-sentence)
    stats_last_week: "last week",
    /// Used in comparisons like "2 h more than last month" (lower case, mid-sentence)
    stats_last_month: "last month",
    /// Used in comparisons like "2 h more than last year" (lower case, mid-sentence)
    stats_last_year: "last year",
    /// Under the total play time when nothing was played yet
    stats_nothing_yet: "Nothing played yet",
    /// Under the total; {when} is "last week", "last month" or "last year"
    stats_nothing_counted: "Nothing counted {when}",
    /// Under the total; {when} is "last week", "last month" or "last year"
    stats_same_as: "The same as {when}",
    /// Under the total; {time} is a duration like "2.0 h", {when} is "last week" and similar
    stats_more_than: "{time} more than {when}",
    /// Under the total; {time} is a duration like "2.0 h", {when} is "last week" and similar
    stats_less_than: "{time} less than {when}",
    /// Stats card title: number of play sessions
    stats_sessions: "Sessions",
    /// Stats card title: number of different games played
    stats_games_played: "Games played",
    /// Stats card title: current streak of days with play
    stats_streak: "Streak",
    /// Stats streak card note when a streak is running
    stats_streak_on: "Days in a row, up to today",
    /// Stats streak card note when there is no streak
    stats_streak_off: "Play today to start one",
    /// Stats card title and year-card label: the longest streak of days
    stats_longest_streak: "Longest streak",
    /// Stats card title and year-card label: the longest single play session
    stats_longest_session: "Longest session",
    /// Heading of the list of most played games
    stats_top_games: "Top games",
    /// Button on the Year span: opens the shareable year card
    stats_your_year: "Your year on Rare",
    /// Hint bar: LB and RB switch between the three spans
    stats_span_hint: "Week, month, year",
    /// Chart caption on the Year span; {time} is a duration
    stats_most_month: "Most in a month: {time}",
    /// Chart caption on Week and Month spans; {time} is a duration
    stats_most_day: "Most in a day: {time}",
    /// Toast when the shareable year picture could not be made
    stats_card_failed: "Couldn't make the picture. Try again.",
    /// Share button label while the year picture is being made
    stats_making_picture: "Making the picture",
    /// Small caps label at the top of the year card; {year} is e.g. 2026. Shown upper case
    stats_year_label: "YOUR {year} ON RARE",
    /// Year card text when nothing was played this year
    stats_year_empty: "No play counted yet this year",
    /// Year card big number; {n} is a formatted number of hours
    stats_hours_n: "{n} hour|{n} hours",
    /// Year card line; {games} is like "3 games" and {sessions} like "12 sessions"
    stats_across: "across {games} and {sessions}",
    /// Count of play sessions; n is a formatted number
    stats_sessions_n: "{n} session|{n} sessions",
    /// Year card fact label: the month with the most play
    stats_busiest_month: "Busiest month",
    /// Year card fact label: the weekday with the most play
    stats_favourite_day: "Favourite day",
    /// Year card list heading: the most played games
    stats_most_played: "Most played",
    /// Toast after Measure again finishes; {n} is the item count, {size} the total size
    storage_measured: "Measured again: {n} item, {size} in all.|Measured again: {n} items, {size} in all.",
    /// Toast when an SD card is inserted and already set up
    storage_sd_ready: "SD card ready",
    /// Toast when an SD card is inserted and not set up for games yet
    storage_sd_ready_setup: "SD card ready. Set it up for games in Settings, Storage.",
    /// Toast when an inserted SD card needs formatting
    storage_sd_format: "This SD card needs formatting before it can hold games. Settings, Storage.",
    /// Toast when the SD card was taken out
    storage_sd_removed: "SD card removed",
    /// Error when Steam would not close for a move
    storage_steam_open: "Steam didn't close. Close it and try again.",
    /// A place games live, used mid-sentence: "{game} is on this device now." (lower case)
    storage_this_device: "this device",
    /// A place games live, used mid-sentence: "{game} is on the SD card now." (lower case)
    storage_the_sd: "the SD card",
    /// Toast when a game move is already running
    storage_wait_move: "Wait for the current move to finish.",
    /// Toast when the game to move has no files here
    storage_no_files: "Couldn't find that game's files on this device.",
    /// Toast when Move is pressed without an SD card
    storage_insert_to_move: "Insert an SD card to move games to it.",
    /// Toast when the library could not be read
    storage_no_library: "Couldn't read your library. Scan again from the Library tab.",
    /// Toast when the game to move is not in the library
    storage_not_in_library: "That game isn't in your library anymore.",
    /// Toast after a move; {game} is the game's title, {place} is "this device" or "the SD card" or a drive name
    storage_moved: "{game} is on {place} now.",
    /// Toast when the player cancels a move; {game} is the game's title
    storage_move_cancelled: "Move cancelled. {game} stays where it was.",
    /// Toast when a move failed; {game} is the title and {error} the reason (a sentence)
    storage_move_failed: "Couldn't move {game}. {error} It stays where it was.",
    /// Toast when Set up is pressed without an SD card
    storage_insert_first: "Insert an SD card first.",
    /// Toast after the SD card was set up for games
    storage_sd_set_up: "The SD card is ready for games. Steam can install to it, and games you put in its Games folder show up in Rare.",
    /// Toast when SD card setup failed; {error} is the reason
    storage_setup_failed: "Couldn't set up the SD card. {error}",
    /// Toast after clearing the image cache when the size is unknown
    storage_cleared_cache: "Cleared the image cache.",
    /// Toast after clearing a cache; {size} is a size like "120 MB"
    storage_cleared: "Cleared {size}.",
    /// Toast when Measure again is pressed while measuring
    storage_still_measuring: "Still measuring.",
    /// Toast when Measure again starts
    storage_measuring: "Measuring again.",
    /// Toast after deleting leftover files; {size} is a size like "1.2 GB"
    storage_deleted: "Deleted {size}.",
    /// Notice when an Epic or GOG game finished downloading; {title} is the game's name
    stores_finished_downloading: "{title} finished downloading",
    /// Notice after an update finished; {title} is the game's name
    stores_up_to_date: "{title} is up to date",
    /// Notice after Check files finished; {title} is the game's name
    stores_files_fixed: "{title} files are checked and fixed",
    /// Notice after cloud saves synced; {title} is the game's name (a subject, e.g. "Hades saves")
    stores_saves_synced: "{title} saves are synced",
    /// Notice after a game's first-time setup finished; {title} is the game's name
    stores_is_set_up: "{title} is set up",
    /// Download row detail while a game's required installers run
    stores_installing_needs: "Installing what the game needs",
    /// Download row detail; {done} and {total} are sizes like "1.2 GB" and "12.3 GB"
    stores_bytes_of: "{done} of {total}",
    /// Download row detail: time left over an hour; {h} hours and {m} minutes (short)
    stores_eta_hours: "{h} h {m} min left",
    /// Download row detail: time left in minutes (short)
    stores_eta_min: "{n} min left",
    /// Notice before a game's first start; {title} is the game's name
    stores_first_start: "Setting up {title} for its first start. This happens once.",
    /// Notice when cloud saves could not be fetched before launch
    stores_cloud_fetch_failed: "Couldn't fetch cloud saves. Playing with the saves on this device.",
    /// Notice when saves could not be uploaded after a game
    stores_cloud_send_failed: "Couldn't send your saves to the cloud. They are safe on this device.",
    /// Install drive choice with enough room; {drive} is the drive's name, {free} like "120 GB"
    stores_drive_free: "{drive}, {free} free",
    /// Install drive choice without enough room; {drive} is the drive's name, {free} like "4 GB"
    stores_drive_short: "{drive}, {free} free, not enough space",
    /// Subtitle of the install drive sheet; {size} is the install size like "12 GB"
    stores_install_where_needs: "Install where? It needs {size}.",
    /// Subtitle of the install drive sheet when the size is unknown
    stores_install_where: "Install where?",
    /// Downloads: status word while an Epic or GOG game's broken files are fetched again
    stores_repairing: "Repairing",
    /// Downloads: status word while cloud saves sync
    stores_syncing_saves: "Syncing saves",
    /// Downloads: an Epic or GOG download stopped, the drive is full
    stores_err_space: "Not enough space on that drive.",
    /// Downloads: an Epic or GOG download stopped, the store sign-in expired
    stores_err_sign_in: "Sign in again in Settings > Accounts.",
    /// Downloads: an Epic or GOG download stopped, no network
    stores_err_offline: "No connection. Check Wi-Fi and try again.",
    /// Downloads: an Epic or GOG download stopped for another reason
    stores_err_generic: "It stopped with an error. Try again.",
    /// Toast after queueing an install; {title} is the game's name
    stores_in_downloads: "{title} is in your downloads",
    /// Toast when a cloud sync starts; {title} is the game's name
    stores_syncing: "Syncing {title} saves",
    /// Notice when a manual cloud sync failed
    stores_sync_failed: "Couldn't sync saves. Try again.",
    /// Account row text when signed in; {name} is the account, {store} is "Epic" or "GOG"
    stores_signed_in_as: "Signed in as {name}. Your {store} games are in the Library.",
    /// Account row text when signed out; {store} is "Epic" or "GOG"
    stores_sign_in_to_see: "Sign in to see your {store} games in the Library and install them.",
    /// Download options sheet: pause the download (verb)
    stores_pause: "Pause",
    /// Download options sheet: cancel an install in progress
    stores_cancel_install: "Cancel the install",
    /// Download options sheet: open the game's page
    stores_game_page: "Game page",
    /// Rare menu line for several downloads; {n} is how many, {pct} the progress number without the percent sign
    stores_n_downloading: "{n} downloading, {pct}%",
    /// Name used for a game whose title is unknown, in "{title} finished downloading"
    stores_a_game: "A game",
    /// Trophy claim note: no wallet account yet. "Rare Wallet" is a product name
    trophies_add_account: "Add a Rare Wallet account to claim",
    /// Trophy claim note: claiming is paused for everyone
    trophies_claims_paused: "Claims are paused",
    /// Trophy claim note while the game's status loads. "Rare Trophies" is a product name
    trophies_checking: "Checking Rare Trophies",
    /// Trophy claim note: the game is not registered in Rare Trophies
    trophies_game_not_listed: "This game is not on Rare Trophies yet",
    /// Trophy claim note: the game's own studio gives out its trophies
    trophies_studio_own: "This game's studio issues its own trophies",
    /// Trophy claim note: claiming is paused for this game
    trophies_game_paused: "Claims for this game are paused",
    /// Trophy claim error: the claim service says too many tries
    trophies_too_many: "Too many claims right now. Try again in a minute.",
    /// Trophy claim error; {status} is an HTTP status number
    trophies_att_status: "The attestation service answered {status}",
    /// Trophy claim error: the service's signed approval is for another registry
    trophies_att_registry: "The attestation is for a different registry",
    /// Trophy claim error: the signed approval is for another game
    trophies_att_game: "The attestation is for a different game",
    /// Trophy claim error: the signed approval is for another trophy
    trophies_att_trophy: "The attestation is for a different trophy",
    /// Trophy claim error: the signed approval ran out
    trophies_att_expired: "The attestation has expired. Try again.",
    /// Trophy claim error: the signature of the approval cannot be read
    trophies_att_signature: "The attestation signature is not readable",
    /// Trophy claim error; {what} is one of the next three words
    trophies_att_invalid: "The attestation has no valid {what}",
    /// Fills {what} in "The attestation has no valid {what}" (lower case noun)
    trophies_what_id: "trophy id",
    /// Fills {what} in "The attestation has no valid {what}" (lower case noun)
    trophies_what_expiry: "expiry",
    /// Fills {what} in "The attestation has no valid {what}" (lower case noun)
    trophies_what_nonce: "nonce",
    /// Note when some selected trophies were skipped; {n} is how many, {why} the reason in lower case
    trophies_not_claimed: "{n} not claimed: {why}",
    /// Note when the player cancelled a claim
    trophies_claim_cancelled: "Claim cancelled",
    /// Note when a claim failed; {error} is the reason
    trophies_claim_failed: "Couldn't claim the trophies. {error}",
    /// Trophy claim error: no claim setup in this app build
    trophies_claim_not_set_up: "Claiming isn't set up in this build.",
    /// Trophy claim error: the game does not accept player-reported trophies
    trophies_no_self_reported: "This game does not take self-reported trophies",
    /// Trophy claim error: the Steam files are missing
    trophies_files_gone: "Steam's files for this game are gone from this device",
    /// Reason a trophy was skipped in a claim
    trophies_no_longer: "No longer unlocked on this device",
    /// Reason a trophy was skipped in a claim
    trophies_not_on_rare: "Not a trophy on Rare yet",
    /// Reason a trophy was skipped in a claim (the platinum is the all-trophies award)
    trophies_platinum_auto: "The platinum unlocks by itself",
    /// Error when no trophy could be claimed
    trophies_nothing_to_claim: "Nothing to claim",
    /// Title on the wallet approval for claiming one trophy; {name} is the trophy
    trophies_claim_one: "Claim {name}",
    /// Title on the wallet approval for claiming several trophies; n is how many
    trophies_claim_n: "Claim {n} trophy|Claim {n} trophies",
    /// Trophy pop-up detail; n is the trophy points
    trophies_points_n: "{n} point|{n} points",
    /// Trophy pop-up title for a trophy pinned on chain; {name} is the trophy
    trophies_on_chain: "On chain: {name}",
    /// Trophy options sheet: stop showing this trophy on the profile showcase
    trophies_remove_showcase: "Remove from your showcase",
    /// Trophy options sheet: show this trophy on the profile
    trophies_show_profile: "Show on your profile",
    /// Trophy options sheet: record the trophy on the blockchain
    trophies_pin_to_chain: "Pin to chain",
    /// Trophy options sheet note: trusted trophy
    trophies_trust_verified: "Verified on SuiPlay: it counts toward your Rare score.",
    /// Trophy options sheet note; {why} is the reason text
    trophies_trust_flagged: "Not counted: {why}.",
    /// Trophy options sheet note: unverified trophy
    trophies_trust_unverified: "Not verified, so it doesn't count toward your Rare score.",
    /// Trophy options sheet note: not checked yet
    trophies_trust_pending: "Not checked by rare.market yet.",
    /// Trophy options sheet subtitle; {game} the game, {tier} the tier name (e.g. Gold), {trust} one of the trust notes
    trophies_sheet_plain: "{game} · {tier}. {trust}",
    /// Trophy options sheet subtitle for a pinned trophy; same placeholders
    trophies_sheet_on_chain: "{game} · {tier}. {trust} It is on chain.",
    /// Trophy options sheet subtitle when it can be pinned; same placeholders
    trophies_sheet_can_pin: "{game} · {tier}. {trust} Pin to chain mints it in your wallet as a self-reported trophy; Rare covers the fee within its limits.",
    /// Note after showing a trophy on the profile
    trophies_on_profile: "On your profile",
    /// Note after hiding a trophy from the profile
    trophies_off_profile: "Removed from your profile",
    /// Note after marking a game as completed
    trophies_marked_complete: "Marked complete",
    /// Note after removing the completed mark
    trophies_unmarked_complete: "No longer marked complete",
    /// Game name when a Steam game's title is unknown; {appid} is a number
    trophies_steam_app: "Steam app {appid}",
    /// Trophy trust note when a trophy failed the checks and no reason was given
    trophies_failed_checks: "Didn't pass Rare's checks",
    /// Note when the player cancelled pinning a trophy to chain
    trophies_pin_cancelled: "Pin cancelled",
    /// Note when pinning failed; {error} is the reason
    trophies_pin_failed: "Couldn't pin it to chain. {error}",
    /// Pin error: not available in this app build
    trophies_pin_not_set_up: "Pin to chain isn't set up in this build.",
    /// Pin error: the friends service is off. "friends" is the Rare setting name
    trophies_pin_need_friends: "Turn on friends to pin trophies",
    /// Title on the wallet approval when pinning a trophy; {name} is the trophy
    trophies_pin_title: "Pin {name} to chain",
    /// Toast when the on-screen keyboard cannot type into games on this device
    typekb_unavailable: "The keyboard can't type here: no virtual keyboard on this device",
    /// Uninstall button on the game page; {size} is the game's size like "12.3 GB"
    uninstall_button_size: "Uninstall ({size})",
    /// Uninstall sheet note about web game saves; {size} is a size
    uninstall_saves_web: "Its saves on this device ({size}). Keep them to pick up where you left off if you add it again.",
    /// Uninstall sheet note about an app's data; {size} is a size
    uninstall_saves_app: "Its settings and data ({size}). Keep them to have them back if you install it again.",
    /// Uninstall sheet note about a Windows game's saves; {size} is a size
    uninstall_saves_windows: "Its saves and settings live in its Windows folder ({size}). Keep them to pick up where you left off if you install it again.",
    /// Uninstall sheet title; {title} is the game's name
    uninstall_title: "Uninstall {title}?",
    /// Uninstall sheet body while the size is measured
    uninstall_checking: "Checking its size",
    /// Uninstall sheet title when it failed; {title} is the game's name
    uninstall_failed_title: "Couldn't uninstall {title}",
    /// Uninstall sheet subtitle; {size} is free disk space
    uninstall_free: "{size} free on this device",
    /// Uninstall sheet text for a Steam game; {size} is the space freed. Steam Cloud is Steam's online save storage
    uninstall_frees_steam: "This frees {size}. Saves in Steam Cloud are kept. Steam asks you to confirm.",
    /// Uninstall sheet text; {size} is the space freed
    uninstall_frees: "This frees {size}.",
    /// Uninstall sheet text when nothing is freed, the entry only goes away
    uninstall_leaves: "It leaves your library.",
    /// Uninstall sheet line; {path} is the install folder
    uninstall_installed_in: "Installed in {path}",
    /// Uninstall sheet checkbox: keep the game's saves
    uninstall_keep_saves: "Keep saves",
    /// Controller hint on the Keep saves checkbox when it is on: pressing deletes the saves too
    uninstall_delete_them: "Delete them too",
    /// Controller hint on the Keep saves checkbox when it is off: pressing keeps the saves
    uninstall_keep_them: "Keep them",
    /// Uninstall sheet text while Steam asks the player to confirm. "Rare comes back" means the launcher returns to the front
    uninstall_confirm_steam: "Confirm in Steam. Rare comes back when Steam is done.",
    /// Uninstall progress; {done} and {total} are sizes
    uninstall_removed: "Removed {done} of {total}",
    /// Uninstall progress with no size known
    uninstall_removing: "Removing",
    /// Button and hint while uninstalling: hide the sheet and keep working (short)
    uninstall_hide: "Hide",
    /// Uninstall sheet title while working; {title} is the game's name
    uninstall_working_title: "Uninstalling {title}",
    /// Toast after uninstalling; {title} is the game's name, {size} the freed space
    uninstall_done_freed: "{title} uninstalled. Freed {size}.",
    /// Toast after uninstalling; {title} is the game's name
    uninstall_done: "{title} uninstalled.",
    /// Toast when uninstalling failed; {title} is the game's name, {error} the reason
    uninstall_failed_toast: "Couldn't uninstall {title}. {error}",
    /// Error when this build cannot uninstall yet
    uninstall_not_wired: "Uninstalling isn't available on this device yet.",
    /// Wishlist row tag: the game is already in the library
    wishlist_in_library: "In your library",
    /// Wishlist row tag: the game is on both lists
    wishlist_steam_and_rare: "Steam and Rare",
    /// Wishlist row tag: the game comes from the Steam wishlist
    wishlist_from_steam: "From your Steam wishlist",
    /// Wishlist row tag: the player added it in Rare
    wishlist_added_in_rare: "Added in Rare",
    /// Wishlist row tag; {when} is a day like "today" or "3 days ago"
    wishlist_added: "added {when}",
    /// Wishlist row: the store with the best price; {store} is the store's name
    wishlist_at_store: "at {store}",
    /// Wishlist row: the game's all-time low price; {price} is money like "$4.99"
    wishlist_lowest_ever: "Lowest ever {price}",
    /// Wishlist row when no store sells the game
    wishlist_no_store: "Not in any store yet",
    /// Wishlist row while prices load
    wishlist_checking_prices: "Checking prices",
    /// Wishlist row price note; {cut} is the discount percent number, {price} the old price
    wishlist_cut_was: "-{cut}% · was {price}",
    /// Controller hint on a wishlist row with a store link: open the deal
    wishlist_open_deal: "Open deal",
    /// Wishlist subtitle; {games} is like "5 games", {sale} how many are on sale
    wishlist_games_on_sale: "{games}, {sale} on sale",
    /// Wishlist page title
    wishlist_title: "Wishlist",
    /// Wishlist search heading; {query} is what the player typed
    wishlist_results_for: "Results for “{query}”",
    /// Wishlist search note while searching
    wishlist_searching: "Searching the stores",
    /// Wishlist search button
    wishlist_search_again: "Search again",
    /// Wishlist search error title
    wishlist_search_failed: "Couldn't search right now",
    /// Wishlist search error text
    wishlist_search_failed_body: "Check the connection and try again.",
    /// Wishlist search empty title
    wishlist_none_found: "No games found",
    /// Wishlist search empty text
    wishlist_none_found_body: "Try a shorter name, like the first word of the title.",
    /// Wishlist search result tag and hint: the game is already wished for
    wishlist_on_it: "On your wishlist",
    /// Wishlist button and hint: add a game by name
    wishlist_add_game: "Add a game",
    /// Wishlist empty text without Steam
    wishlist_empty_body: "Add games you want to keep an eye on. Their lowest price across stores shows here, and Rare tells you when one drops.",
    /// Wishlist empty text when Steam's wishlist could also show
    wishlist_empty_body_steam: "Add games you want to keep an eye on, or sign in to Steam on this device: your Steam wishlist shows here when your profile's game details are public.",
    /// Wishlist empty title
    wishlist_empty: "Your wishlist is empty",
    /// Wishlist header note
    wishlist_prices: "Lowest price now across stores, in USD",
    /// Wishlist header note; {when} is a lower-case time like "3 hours ago"
    wishlist_prices_checked: "Lowest price now across stores, in USD, checked {when}",
    /// Wishlist footer note about the Steam wishlist
    wishlist_steam_note: "Your Steam wishlist shows here too when this device is signed in to Steam and your profile's game details are public.",
    /// Text prompt title for adding a wishlist game
    wishlist_ask_title: "Add a game to your wishlist",
    /// Text prompt placeholder for the game's name
    wishlist_ask_placeholder: "Game name",
    /// Toast after adding a game; {title} is the game's name
    wishlist_added_toast: "Added {title} to your wishlist",
    /// Toast when opening a store page
    wishlist_opening_deal: "Opening the deal",
    /// Toast when no store has the game
    wishlist_no_store_toast: "No store sells this one yet",
    /// Toast after removing a game; {title} is the game's name
    wishlist_removed: "Removed {title}",
    /// Hint bar in wishlist search results: B goes back to the wishlist
    wishlist_back: "Back to wishlist",
    /// Error when adding a web game with an address that is not https
    webgame_need_https: "Web games need an https address (http works only for localhost)",
    /// Error when adding a web game with something that is not an address
    webgame_not_address: "That isn't a web address. Check it and try again.",
    /// Name used on the wallet sheet for a game that gave no name
    webgame_this_game: "This game",
    /// Toast after a web game connected to the wallet; {game} is the game's name. "Rare Wallet" is a product name
    webgame_connected: "{game} connected to Rare Wallet",
    /// Toast when a link points to a collection Rare does not list
    localapi_no_collection: "That collection isn't on Rare.",
    /// Toast and sheet text when an item cannot be sold or bought and no reason was given
    localapi_unavailable: "That isn't available",
    /// Sheet title after a claim worked
    localapi_claimed: "Claimed",
    /// Sheet subtitle: the offer to claim came from a link
    localapi_from_link: "From a Rare link",
    /// Claim sheet text for a trophy claim. "Rare profile" is the player's profile
    localapi_goes_profile: "Goes to your Rare profile. Rare covers the network fee when your wallet is short.",
    /// Claim sheet text for an item claim. "Rare Wallet" is a product name
    localapi_goes_wallet: "Goes to your Rare Wallet. You review it before anything is signed.",
    /// Claim sheet main button (short)
    localapi_claim: "Claim",
    /// Sell sheet title; {name} is the item
    localapi_sell_title: "Sell {name}",
    /// Sell sheet text and secondary button; {price} is an amount like "2.5 SUI"
    localapi_list_for: "List for {price}",
    /// Sell sheet text when no price is set
    localapi_set_price: "Set a price in the marketplace to list this item.",
    /// Sell sheet secondary button when a listing is the main action (short)
    localapi_sell_best: "Sell to the best offer",
    /// Sell sheet line; {price} is an amount like "2.5 SUI"
    localapi_best_offer: "Best offer {price}",
    /// Sell sheet line; {price} is the amount the player gets, like "2.4 SUI"
    localapi_you_receive: "You receive {price}",
    /// Sheet note: the Rare service pays the network fee
    localapi_fee_covered: "Rare covers the network fee.",
    /// Sell sheet text when nobody bid on the item
    localapi_no_offers: "No offers on this item right now.",
    /// Sell sheet main button; {price} is the amount received, like "2.4 SUI" (short)
    localapi_sell_for: "Sell for {price}",
    /// Buy sheet title; {name} is the item
    localapi_buy_title: "Buy {name}",
    /// Buy sheet line; {price} is an amount like "2.5 SUI"
    localapi_price: "Price {price}",
    /// Sheet note about the wallet review step
    localapi_you_review: "You review it before anything is signed.",
    /// Sheet result line after a sale (starts with a space in code, do not add one)
    localapi_done_sold: "Sold. It can take a minute to show.",
    /// Sheet result line after listing an item
    localapi_done_listed: "Listed. It can take a minute to show.",
    /// Sheet result line after a claim
    localapi_done_claimed: "Done. It can take a minute to show.",
    /// Sheet result line after a purchase
    localapi_done_bought: "Bought. It can take a minute to show.",
    /// Sheet main button while the wallet waits for the player's approval
    localapi_waiting: "Waiting for approval",
    /// Sheet close button (decline for now)
    localapi_not_now: "Not now",
    /// Art lookup error: SteamGridDB (an art website) refused the player's key
    art_key_rejected: "SteamGridDB didn't accept the key. Check it in Settings > Storage.",
}
