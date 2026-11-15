strings! {
    /// Share row in the Library/Home side list: "{n} new" at its end, n is how many new captures
    fresh_count: "{n} new",
    /// Hint-bar text for the Share row (A button)
    open_share: "Open Share",
    /// Kind of a capture, shown on tiles and the detail page
    kind_clip: "Clip",
    /// Kind of a capture, shown on tiles and the detail page
    kind_screenshot: "Screenshot",
    /// Share page intro under the title
    intro: "Shared links open on rare.market for anyone. Files are stored on Walrus and your wallet pays for the storage.",
    /// Button label while a capture is being saved (short: button)
    saving: "Saving",
    /// Button label (short: button)
    take_screenshot: "Take screenshot",
    /// Hint-bar text for the Take screenshot button
    take_screenshot_hint: "Take a screenshot",
    /// Hint-bar text for the Save clip button
    save_clip_hint: "Save the clip",
    /// Heading of the capture panel (noun)
    capture: "Capture",
    /// Help line under Capture; {last} is a phrase like "the last 30 seconds"
    capture_keys: "In a game: Guide and Y for a screenshot, Guide and View for {last}.",
    /// Title of the instant replay row; {last} is a phrase like "the last 30 seconds"
    replay_title: "Instant replay: keep {last} while playing",
    /// Instant replay status; {method} is the recorder's name
    replay_recording_with: "Recording in the background with {method}. Uses a little battery.",
    /// Instant replay status
    replay_recording: "Recording in the background. Uses a little battery.",
    /// Instant replay status
    replay_on: "On. Recording starts with the next game.",
    /// Instant replay status
    replay_off: "Off. Turn it on to save clips.",
    /// Small label next to a dot while instant replay is recording
    recording: "Recording",
    /// Note on the capture panel
    no_screenshots: "Screenshots aren't available in this session.",
    /// Section heading above the captures grid
    recent: "Recent",
    /// Count next to Recent; n is a number
    on_device: "{n} on this device",
    /// Empty state title
    empty_title: "No screenshots or clips yet",
    /// Empty state body
    empty_body: "Press Guide and Y in a game, or take one here.",
    /// Shown on a tile whose picture could not be loaded
    cant_read: "Can't read this file",
    /// Badge on a capture taken since the last visit (short)
    new_badge: "New",
    /// Hint for a capture tile (A button)
    open_tile: "Open",
    /// Hint for a duration button; {duration} is "1 month" or "1 year"
    keep_for: "Keep for {duration}",
    /// Label of a duration choice
    duration_month: "1 month",
    /// Label of a duration choice
    duration_year: "1 year",
    /// Label above the duration choices
    keep_it_for: "Keep it for",
    /// Payer choice (short: button)
    rare_covers: "Rare covers it",
    /// Payer choice (short: button)
    my_wallet: "My wallet",
    /// Label above the payer choices
    who_pays: "Who pays",
    /// Price line when Rare pays
    free: "Free",
    /// Default note when Rare pays; days is a number. {note} is that sentence or a note from the server
    rare_pays_days: "Rare pays for {days} days on Walrus.",
    /// {note} is a sentence, then the privacy line
    note_anyone: "{note} Anyone with the link can see it.",
    /// Price, first part of a line; {wal} is an amount like "0.019 WAL"
    about_wal: "About {wal}",
    /// Price, second part, shown after "About X" in a muted style; keep the leading space; {fee} is an amount
    plus_fees: " plus {fee} in fees",
    /// Under the price on mainnet; days is a number, {size} like "14.4 MB"
    stored_mainnet: "Stored on Walrus for {days} days, {size}. Anyone with the link can see it.",
    /// Under the price on the test network; days is a number, {size} like "14.4 MB"
    stored_testnet: "Stored on Walrus testnet for {days} days, {size}. Anyone with the link can see it.",
    /// Shown when the price lookup failed
    price_error: "Couldn't get the Walrus price. Try again in a moment.",
    /// Shown while the price is loading
    getting_price: "Getting the price",
    /// Shown when no wallet account exists
    add_wallet: "Add a Rare Wallet account to share.",
    /// Delete button when pressed once (short: button)
    press_again: "Press again",
    /// Hint-bar text for Delete
    delete_hint: "Delete from this device",
    /// Upload step names (short), then what it says while it is the current one
    step_prepare: "Prepare",
    step_preparing: "Preparing the files",
    step_pay: "Pay for storage",
    step_pay_doing: "Approve in Rare Wallet",
    step_upload: "Upload",
    step_uploading: "Uploading to Walrus",
    step_confirm: "Confirm",
    step_confirm_doing: "Approve to confirm",
    /// Note while uploading when Rare pays
    rare_storing: "Rare is storing it.",
    /// Note while uploading when the player pays
    two_approvals: "Two approvals in Rare Wallet: one pays for the storage, one confirms it.",
    /// Status next to the QR after the link was copied
    copied: "Copied",
    /// Status next to the QR
    scan_or_copy: "Scan the code or copy the link",
    /// Button and hint (short: button)
    copy_link: "Copy link",
    /// Button; {name} is a friend's name
    send_to: "Send to {name}",
    /// Button and hint
    send_to_friend: "Send to a friend",
    /// Button and hint
    post_telegram: "Post to Telegram",
    /// Hint-bar text for the Done button
    back_to_captures: "Back to your captures",
    /// Why clips are not possible on this machine
    clips_unavailable: "Clips need gpu-screen-recorder or GStreamer with VA-API",
    /// Why clips are not possible: instant replay is off
    replay_is_off: "Instant replay is off. Turn it on below.",
    /// Error; {wal} is an amount like "0.019"
    not_enough_wal_need: "Not enough WAL in this account. Sharing this needs about {wal} WAL.",
    /// Error
    not_enough_wal: "Not enough WAL in this account.",
    /// Error
    not_enough_sui: "Not enough SUI in this account for the fees.",
    /// Error
    walrus_unreachable: "Couldn't reach Walrus. Check the connection and try again.",
    /// Error
    didnt_go_through: "That didn't go through. Try again.",
    /// Relative time for a recent capture
    ago_just_now: "just now",
    /// Relative time; n is a number of hours
    ago_hours: "{n} h ago",
    /// Relative time
    ago_yesterday: "yesterday",
    /// Relative time; n is a number of days (2 or more)
    ago_days: "{n} day ago|{n} days ago",
}
