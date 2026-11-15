strings! {
    // amount pad
    /// Screen reader label of the amount keypad
    amount_keys: "Amount keys",
    /// Keypad key that deletes the last digit. short: key
    key_del: "Del",
    // connect prompt
    /// Connect sheet title; game is the web game's name
    connect_title: "{game} wants to see your address",
    connect_body: "It can also ask you to sign. Every request still shows up here for you to approve, and nothing is signed without it.",
    /// Heading above the list of accounts to pick from
    account_to_share: "Account to share",
    /// Button: share the address with the game. short: button
    allow: "Allow",
    /// Button: refuse to share. short: button
    dont_allow: "Don't allow",
    // site checks
    site_invalid: "Could not verify this site address.",
    site_insecure: "This site is not using HTTPS.",
    site_lookalike: "This address uses special characters and may imitate another site.",
    // fee line
    /// Label of Rare's own fee line in a transaction review
    rare_fee: "Rare fee",
    // locked sheet (PIN locked after 5 wrong tries)
    /// Headline and sheet-wide status when the PIN is locked
    locked_head: "Locked. Use your phone to unlock",
    locked_body: "5 wrong PINs locked the key on this SuiPlay. Your phone (or security key) unlocks it, then you pick a new PIN.",
    forgot_head: "Use your backup to change the PIN",
    forgot_body: "Your phone or security key signs once, then you pick a new PIN.",
    /// Sheet title when changing a forgotten PIN
    change_pin_title: "Change PIN",
    /// Sheet title when the PIN is locked
    locked_title: "Locked",
    /// Sheet title once the backup has unlocked the key
    unlocked_title: "Unlocked",
    unlock_title: "Unlock this SuiPlay",
    scan_code: "Scan this code with your phone's camera, then approve with Face ID or your fingerprint.",
    keep_bluetooth_both: "Keep Bluetooth on for both.",
    approve_on_phone: "Approve on your phone",
    touch_key: "Touch your security key",
    plug_in_key: "Plug it into the USB-C port on top. Touch it when it blinks.",
    getting_ready: "Getting ready",
    set_new_pin: "Set a new PIN",
    new_pin_body: "Your backup unlocked this SuiPlay. Pick a new PIN for it.",
    /// Buttons when changing a forgotten PIN. short: button
    use_phone: "Use your phone",
    use_key: "Use your security key",
    /// Buttons when the PIN is locked. short: button
    unlock_phone: "Unlock with your phone",
    unlock_key: "Unlock with your security key",
    sign_phone_instead: "Sign this with your phone instead",
    sign_key_instead: "Sign this with your key instead",
    // hardware and phone panels
    connecting_ledger: "Connecting to your Ledger",
    connecting_keystone: "Connecting to your Keystone",
    /// Shown above a command the person runs once; device is Ledger or Keystone
    run_once_plug: "Run this once, then plug the {device} in again:",
    /// Label of the transaction hash grid
    transaction_hash: "Transaction hash",
    check_hash: "Check it matches the hash on your Keystone.",
    sign_with_phone: "Sign with your phone",
    /// Numbered step under the QR code; n is the step number, step the instruction
    phone_step_n: "{n}. {step}",
    phone_step_1: "Open the camera on your phone and point it at this code.",
    phone_step_2: "Tap the passkey prompt and pick your Rare passkey.",
    phone_step_3: "Confirm with your face or fingerprint. Keep Bluetooth on, near this SuiPlay.",
    key_needs_pin: "Your security key needs its PIN",
    /// PIN attempts remaining on the security key
    tries_left: "{n} try left|{n} tries left",
    getting_qr: "Getting a QR code for your phone",
    qr_soon: "It shows here in a moment. Keep Bluetooth on.",
    /// Button: type the security key PIN. short: button
    enter_pin: "Enter PIN",
    // recipient
    your_accounts: "Your accounts",
    /// Heading above recent recipients
    recent: "Recent",
    /// Relative date for something less than a day ago
    since_today: "Today",
    /// Relative date, short: "3d ago"
    since_days_ago: "{n}d ago",
    /// Short date: month abbreviation then day, e.g. "Mar 4"
    since_date: "{month} {day}",
    month_jan: "Jan",
    month_feb: "Feb",
    month_mar: "Mar",
    month_apr: "Apr",
    month_may: "May",
    month_jun: "Jun",
    month_jul: "Jul",
    month_aug: "Aug",
    month_sep: "Sep",
    month_oct: "Oct",
    month_nov: "Nov",
    month_dec: "Dec",
    pick_other_address: "Pick an address other than this account",
    /// Name being resolved (a SuiNS name)
    looking_up: "Looking up {name}",
    // PIN tasks (Settings > Accounts: Change PIN, Show recovery words)
    /// Sheet title; the 12 words that restore the account
    recovery_words_title: "Recovery words",
    recovery_warn: "These 12 words are this SuiPlay's key to your account. With them and your phone or security key, anyone can take everything in it. Rare never asks for them. Write them on paper and keep it safe. Never type them into a website, a message or a photo.",
    recovery_hint: "Check that nobody can see the screen. You need them only to sign with a PIN on another SuiPlay.",
    /// Button. short: button
    show_words: "Show the words",
    enter_current_pin: "Enter your current PIN",
    forgot_use_backup: "Forgot it? Use your backup",
    enter_pin_for_words: "Enter your PIN to see your recovery words",
    pick_new_pin: "Pick a new PIN",
    new_pin_rules: "6 to 12 digits. It signs on this SuiPlay; 5 wrong ones lock it until your backup unlocks it.",
    write_down_order: "Write them down in this order",
    never_share_words: "Never share them. Anyone with these words and your backup can take your account.",
    // PIN prompt
    /// Wrong PIN message; n is the tries left
    wrong_pin: "Wrong PIN. {n} try before a wait.|Wrong PIN. {n} tries before a wait.",
    wrong_passphrase: "Wrong passphrase. {n} try before a wait.|Wrong passphrase. {n} tries before a wait.",
    /// Short wait: seconds
    wait_seconds: "{n} s",
    /// Short wait: minutes
    wait_minutes: "{n} min",
    /// wait is a short time like "12 s" or "2 min"
    too_many_tries: "Too many tries. Try again in {wait}.",
    /// Field label for the vault passphrase
    passphrase: "Passphrase",
    /// Button: sign with the phone passkey instead of the PIN
    use_phone_instead: "Use my phone instead",
    unlock_wallet_title: "Unlock Rare Wallet",
    could_not_unlock: "Could not unlock",
    // review sheet
    /// Price line under a coin when no price is known
    no_price: "No price",
    /// Screen reader label and loading state while the transaction is simulated
    checking: "Checking",
    checking_what: "Checking what this does",
    preview_unavailable: "Could not preview this transaction. Approve only if you trust who is asking.",
    /// Red banner title when the simulated transaction would fail
    would_fail: "This would fail",
    /// Section titles of the review
    you_send: "You send",
    you_receive: "You receive",
    goes_to: "Goes to",
    nothing_moves: "No coins or objects move in or out of your wallet.",
    /// Label of the signing account row, and the fallback name of an unknown account
    account: "Account",
    network_fee: "Network fee",
    /// Fee line label: the fee name, then its share of the amount sent, e.g. "Rare fee 0.2%"
    fee_with_pct: "{label} {pct}",
    /// Toggle the person ticks when the review asks for an address check
    ack_address: "I checked the full address and it is right",
    /// Collapsible section title for the technical call list
    details: "Details",
    /// Heading above the text of a message to sign
    message: "Message",
    message_hex: "Message (hex)",
    message_warn: "Signing proves you own this account. It cannot move funds by itself, but a site can use it to sign you in or approve actions, so only sign messages you expect.",
    // result
    sig_ready: "Your signature is ready.",
    tx_went_through: "Your transaction went through.",
    tx_id: "Transaction ID",
    test_run: "Test run. Nothing was sent to the network.",
    /// Button state after Copy ID was pressed. short: button
    copied: "Copied",
    /// Button: copy the transaction id. short: button
    copy_id: "Copy ID",
    // titles
    /// Sheet title after signing a message or a signature-only transaction
    title_signed: "Signed",
    /// Sheet title after a transaction was sent
    title_sent: "Sent",
    title_sign_message: "Sign message",
    title_approve_tx: "Approve transaction",
    title_sign_tx: "Sign transaction",
    // footer
    /// Disabled button while the wallet is signing. short: button
    signing: "Signing",
    approve_off: "Approve is off because this would fail.",
    approve_anyway: "Approve anyway",
    will_likely_fail: "It will likely fail and still cost the network fee.",
    /// Primary buttons. short: button
    approve: "Approve",
    sign: "Sign",
    network_rejected: "The network rejected this transaction.",
    could_not_sign: "Could not sign. Try again.",
    // review host
    /// Shown in place of Approve when the account lacks a coin; sym is the coin symbol, e.g. WAL
    short_of_coin: "This account doesn't have enough {sym}. Add {sym} to it and try again.",
    short_of_gas: "This account doesn't have enough SUI to pay the network fee. Add SUI to it and try again.",
    /// Title of the parental PIN prompt
    parent_pin: "Parent's PIN",
    parent_pin_tx: "Enter the PIN to approve this transaction.",
    parent_pin_msg: "Enter the PIN to sign this message.",
    /// PIN sheet title
    enter_your_pin: "Enter your PIN",
    pin_reason_sign: "Enter your PIN to sign",
    pin_reason_approve: "Enter your PIN to approve",
    pin_reason_passkey: "Enter your PIN, or sign with the passkey on your phone.",
    /// amount is a dollar amount like 182.40, without the sign
    pin_reason_send_usd: "This sends about ${amount}. Enter your PIN to approve.",
    pin_reason_send: "Enter your PIN to approve this send.",
    /// Buttons on the PIN sheet. short: button
    sign_phone_instead_pin: "Sign with my phone instead",
    sign_key_instead_pin: "Sign with my security key instead",
    /// Title of the keyboard prompt for the security key PIN
    security_key_pin: "Security key PIN",
    no_pin_account: "No PIN account on this SuiPlay",
    unlock_backup_again: "Unlock with your backup again",
    /// Toasts
    tx_id_copied: "Transaction ID copied",
    pin_changed: "PIN changed",
}
