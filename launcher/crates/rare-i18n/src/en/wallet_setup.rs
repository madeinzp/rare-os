strings! {
    /// Sheet title: the PIN pad that asks for the PIN of a security key (FIDO key)
    key_pin_title: "Security key PIN",
    /// Sheet subtitle under the security key PIN title; n is a number
    tries_left: "{n} tries left",
    /// Sheet subtitle: asks for the PIN that was set on the security key
    key_pin_prompt: "Enter the PIN you set on your key",
    /// Passkey sign-in: instruction under a QR code, scanned with the phone camera
    scan_qr: "Scan this code with your phone's camera, then approve with Face ID or your fingerprint.",
    /// Passkey: note under the QR code
    phone_bluetooth_note: "Your phone connects to this SuiPlay over Bluetooth, so keep Bluetooth on for both.",
    /// Passkey: heading once the phone has scanned the code
    approve_on_phone: "Approve on your phone",
    /// Passkey: heading before the QR code shows
    getting_ready: "Getting ready",
    /// Passkey: status line while the connection starts
    starting_connection: "Starting the connection",
    /// Passkey with a USB security key: heading
    insert_key: "Insert your security key and touch it",
    /// Passkey with a USB security key: status line under the heading
    plug_key: "Plug it into the USB port. Touch it when it blinks.",
    /// Heading of the Bluetooth step
    turn_on_bluetooth_title: "Turn on Bluetooth",
    /// Bluetooth step: explanation under the heading
    bluetooth_note: "Your phone and this SuiPlay talk over Bluetooth to set up your passkey.",
    /// Button label while Bluetooth is being switched on (short: button)
    turning_on: "Turning on",
    /// Button: switch Bluetooth on (short: button)
    turn_on_bluetooth: "Turn on Bluetooth",
    /// Error: Bluetooth cannot be turned on because of a hardware or software switch
    bluetooth_blocked: "Bluetooth is blocked by a switch on this device.",
    /// Error: Bluetooth could not be turned on
    bluetooth_failed: "Could not turn Bluetooth on. Try again.",
    /// Page title when adding a recovery phrase from first-run setup
    your_recovery_words: "Your recovery words",
    /// Page title: create a new wallet account
    create_account: "Create your account",
    /// Page title: write down the recovery phrase
    backup_title: "Back up your recovery phrase",
    /// Page title: quiz that checks the person wrote the phrase down
    check_backup: "Check your backup",
    /// Intro on the create account page; "A" is the controller button name
    phrase_intro: "It is kept on this SuiPlay and signs with A, no PIN. You get a list of secret words to write down once. They are the only way to get your account back if you lose this device.",
    /// Label above the 12 / 24 words choice
    length: "Length",
    /// Segment option: a 12 word recovery phrase (short)
    words_12: "12 words",
    /// Segment option: a 24 word recovery phrase (short)
    words_24: "24 words",
    /// Button: reveal the recovery phrase
    show_my_words: "Show my words",
    /// Warning above the recovery phrase
    write_down_warning: "Write these words down in order and keep them private. They never leave this SuiPlay. Anyone with them can take your money, and Rare cannot recover them for you.",
    /// Button: the person has written the phrase down
    i_wrote_it_down: "I wrote it down",
    /// Backup quiz question; n is the word position, q the question number, total the number of questions
    pick_word: "Pick word #{n} of your phrase. Question {q} of {total}.",
    /// Backup quiz: shown after a wrong pick; n is the word position
    wrong_word: "That is not word #{n}. Look at your backup and try again.",
    /// Title of the PIN setup form
    set_a_pin: "Set a PIN",
    /// Note under the PIN setup form for a recovery phrase
    phrase_pin_note: "It unlocks your recovery phrase on this device. It never leaves it.",
    /// Error: saving the recovery phrase failed
    save_phrase_failed: "Could not save the phrase",
    /// Hint bar label: type a character (short)
    hint_type: "Type",
    /// Error: the action was cancelled
    err_cancelled: "Cancelled.",
    /// Error: something took too long
    err_timeout: "That took too long. Try again.",
    /// Error: Bluetooth is off
    err_ble_off: "Bluetooth is off. Turn it on and try again.",
    /// Error: no hardware device found
    err_notfound: "No device found. Check that it is connected and try again.",
    /// Error: the hardware device is busy
    err_busy: "The device is busy. Finish what it is showing, then try again.",
    /// Error: no network
    err_network: "Couldn't reach the network. Check your connection and try again.",
    /// Error: wrong PIN
    err_wrong_pin: "That PIN is wrong. Try again.",
    /// Error: feature not supported on this device
    err_unsupported: "This is not supported on this device.",
    /// Generic fallback error
    err_default: "That didn't go through. Try again.",
    /// Page title: choose Ledger accounts to add
    pick_ledger: "Pick Ledger accounts",
    /// Page title: connect a Ledger hardware wallet
    connect_ledger: "Connect your Ledger",
    /// Page subtitle on the Ledger connect page
    unlock_ledger: "Unlock your Ledger and open the Sui app.",
    /// Page title: choose Keystone accounts to add
    pick_keystone: "Pick Keystone accounts",
    /// Page title: connect a Keystone hardware wallet
    connect_keystone: "Connect your Keystone",
    /// Page subtitle on the Keystone connect page; the menu path is shown on the device
    keystone_connect_note: "Connect your Keystone 3 Pro with a USB-C cable. On the Keystone: Settings > Connection > USB.",
    /// Label above the USB / Bluetooth choice for a Ledger
    how_connected: "How is it connected",
    /// Segment option (short)
    usb_cable: "USB cable",
    /// Segment option, also a label under a device (short)
    bluetooth: "Bluetooth",
    /// Note under the transport choice when Bluetooth is picked
    ledger_ble_note: "Nano X, Stax and Flex. Turn Bluetooth on from the Ledger's settings.",
    /// Note under the transport choice when USB is picked
    ledger_usb_note: "Plug it in with a USB-C or USB cable.",
    /// Label above a pairing code shown for the Ledger Bluetooth pairing
    pairing_code: "Pairing code",
    /// Under the Ledger pairing code
    pairing_check: "Check it matches your Ledger and confirm there.",
    /// Status while connecting to the Ledger
    connecting_ledger: "Connecting to your Ledger",
    /// Hint under the status while connecting to the Ledger
    keep_ledger_unlocked: "Keep it unlocked with the Sui app open.",
    /// Status while reading the Keystone
    reading_keystone: "Reading your Keystone",
    /// Hint under the status while reading the Keystone
    keep_keystone_unlocked: "Keep it unlocked and on the USB screen.",
    /// Bluetooth step note when Bluetooth is off and a Ledger is connected over it
    ledger_ble_off: "Turn Bluetooth on to connect your Ledger over Bluetooth.",
    /// Label of a Bluetooth device that is already paired (short)
    bluetooth_paired: "Bluetooth, paired",
    /// Empty state title: no Ledger device found
    no_ledger: "No Ledger found",
    /// Empty state title: no Keystone device found
    no_keystone: "No Keystone found",
    /// Empty state text: no Keystone found
    keystone_not_found_hint: "Plug it in, unlock it and turn on USB in its connection settings.",
    /// Empty state text: no Ledger found over Bluetooth
    ledger_ble_not_found_hint: "Turn on Bluetooth on your Ledger, unlock it and open the Sui app.",
    /// Empty state text: no Ledger found over USB
    ledger_usb_not_found_hint: "Plug it in, unlock it and open the Sui app.",
    /// Button: search for devices again (short)
    search_again: "Search again",
    /// Note on the Keystone connect page
    keystone_hash_note: "The Keystone signs a transaction hash over USB. Rare shows you the decoded transaction first, so check it before you approve on the device.",
    /// Error: Linux needs a udev rule to use the USB device
    udev_permission: "Rare OS needs permission to use this device.",
    /// Instruction above a terminal command to copy
    udev_run_once: "Run this once in Konsole, then plug your device in again.",
    /// Button after the command was copied (short)
    copied: "Copied",
    /// Button: copy the terminal command (short)
    copy_command: "Copy command",
    /// Tag on a hardware account row that is already in the wallet (short)
    added: "Added",
    /// Row label of a hardware account; n is its number
    account_n: "Account {n}",
    /// Button state while the device shows the address to check (short)
    check_device: "Check device",
    /// Button: the address on the device matches (short)
    verified: "Verified",
    /// Button: the address on the device differs (short)
    does_not_match: "Does not match",
    /// Button: show the address on the hardware device to compare (short)
    verify_on_device: "Verify on device",
    /// Button state while loading more accounts (short)
    reading: "Reading",
    /// Button: load more accounts from the device (short)
    show_more: "Show more",
    /// Button state while accounts are being added (short)
    adding: "Adding",
    /// Button: add one account
    add_account: "Add account",
    /// Button: add several accounts; n is the number (2 or more)
    add_accounts: "Add {n} accounts",
    /// Import option title
    way_phrase_title: "Recovery phrase",
    /// Import option description
    way_phrase_body: "Use the 12 or 24 words from another wallet",
    /// Import option title
    way_key_title: "Private key",
    /// Import option description; suiprivkey is the key prefix, keep it
    way_key_body: "Paste a suiprivkey... key",
    /// Import option description for Ledger
    way_ledger_body: "Nano S Plus, Nano X, Stax and Flex over USB or Bluetooth",
    /// Import option description for Keystone
    way_keystone_body: "Keystone 3 Pro over a USB-C cable",
    /// Import option title: watch-only address
    way_watch_title: "Watch an address",
    /// Import option description for watch-only
    way_watch_body: "Follow any wallet. You cannot send from it",
    /// Placeholder shown in an empty recovery word slot (short)
    word_placeholder: "word",
    /// Subtitle on the Restore PIN account page
    restore_sub: "Type the 12 recovery words of your PIN account. Your phone or security key signs in once, then this SuiPlay signs with a new PIN. The address stays the same.",
    /// Error under the recovery word grid
    phrase_invalid: "That phrase is not valid. Check each word and the order.",
    /// Error under the private key field; suiprivkey1 is the key prefix, keep it
    key_invalid: "That is not a valid private key. It starts with suiprivkey1.",
    /// Note under the PIN setup form when importing keys
    keys_pin_note: "It unlocks your keys on this device. It never leaves it.",
    /// Button: import the phrase or key (short)
    import: "Import",
    /// Button: paste all recovery words from the clipboard (short)
    paste_all: "Paste all words",
    /// Sheet title: choose one recovery word; n is the word position
    word_n: "Word {n}",
    /// Sheet subtitle under the word title
    pick_the_word: "Pick the word",
    /// Page title: import a private key
    import_key_title: "Import a private key",
    /// Page subtitle: import a private key; suiprivkey1 is the prefix, keep it
    import_key_sub: "Paste a key that starts with suiprivkey1. It is stored encrypted on this device.",
    /// Page title: restore a PIN account from its recovery words
    restore_title: "Restore your PIN account",
    /// Page title: import a recovery phrase
    import_phrase_title: "Import a recovery phrase",
    /// Page subtitle: import a recovery phrase
    import_phrase_sub: "Type the first letters of each word and pick it from the list.",
    /// Error: pasted text has the wrong number of words; n is the count
    wrong_count: "That is {n} words. A recovery phrase has 12 or 24.",
    /// Error: a typed word is not in the recovery word list; word is what was typed
    not_a_word: "\"{word}\" is not a recovery word. Check the spelling.",
    /// Error: adding the wallet failed
    add_wallet_failed: "Could not add the wallet",
    /// Passkey choice title when creating an account
    use_phone: "Use your phone",
    /// Passkey choice title when signing in
    sign_in_phone: "Sign in with your phone",
    /// Passkey choice description for the phone
    phone_body: "Scan a code with your phone's camera, then use Face ID or your fingerprint",
    /// Passkey choice title for a hardware security key
    use_key: "Use a security key",
    /// Passkey choice description for a security key
    key_body: "A USB or NFC key such as a YubiKey. No camera needed.",
    /// Note under the passkey choices when creating an account
    passkey_always_signs: "Your passkey can always sign. This device signs everyday transactions with your PIN.",
    /// Title of the PIN setup form for this device
    set_device_pin: "Set a device PIN",
    /// Note under the device PIN setup form
    device_pin_note: "This device signs everyday transactions with it. Your passkey can always sign too.",
    /// Heading after signing in with a passkey
    signed_in: "You are signed in",
    /// Text after signing in with a passkey
    device_asks_phone: "This device will ask your phone to approve each transaction.",
    /// Bluetooth step note when Bluetooth went off during the passkey step
    ble_back_on: "Turn Bluetooth back on to finish with your phone.",
    /// Error: no USB security key found
    no_key_found: "No security key found. Insert it and try again.",
    /// Error: the phone has no Rare passkey
    no_passkey_on_phone: "No Rare passkey found on that phone.",
    /// Error: the passkey step failed
    passkey_failed: "Could not use the passkey. Try again.",
    /// Page and sheet title: create a new account (short)
    create_account_short: "Create account",
    /// Page title: sign in with a passkey
    sign_in_passkey: "Sign in with a passkey",
    /// Page subtitle when creating an account with a passkey
    create_sub_long: "Make a new wallet protected by a passkey. There are no words to write down.",
    /// Page subtitle when signing in with a passkey
    signin_sub_long: "Use the passkey you made before to get your wallet back.",
    /// Prompt: a passkey is needed to sell
    need_sell: "Add a passkey to sell.",
    /// Prompt: a passkey is needed to buy
    need_buy: "Add a passkey to buy.",
    /// Prompt: a passkey is needed to trade
    need_trade: "Add a passkey to trade.",
    /// Prompt when signed out: sign in to sell
    need_sell_signed_out: "Sign in with a passkey to sell.",
    /// Prompt when signed out: sign in to buy
    need_buy_signed_out: "Sign in with a passkey to buy.",
    /// Prompt when signed out: sign in to trade
    need_trade_signed_out: "Sign in with a passkey to trade.",
    /// Prompt: sign in to see the person's items
    need_items: "Sign in with a passkey to see your items.",
    /// Button and sheet title: add a passkey
    add_passkey: "Add a passkey",
    /// Sheet subtitle when creating an account
    create_sub: "Make a new wallet protected by a passkey.",
    /// Sheet subtitle when adding an existing passkey
    signin_sub: "Use the passkey you made before.",
    /// Sheet footer button: switch to using an existing passkey
    use_existing_passkey: "Use a passkey I already have",
    /// PIN pad heading when confirming a new PIN
    enter_again: "Enter it again",
    /// Heading of the passphrase form
    choose_passphrase: "Choose a passphrase",
    /// Note under the PIN pad when confirming a 6 digit PIN
    confirm_digits: "Type the same 6 digits to confirm.",
    /// Note under the passphrase heading
    passphrase_note: "At least 8 characters. It unlocks your keys on this device.",
    /// Field label
    passphrase: "Passphrase",
    /// Field placeholder (short)
    at_least_8: "At least 8 characters",
    /// Field label
    confirm_passphrase: "Confirm passphrase",
    /// Field placeholder (short)
    type_again: "Type it again",
    /// Field error: the two passphrases differ
    do_not_match: "These do not match",
    /// Button: switch to a passphrase instead of a PIN
    use_passphrase: "Use a passphrase instead",
    /// Button: switch to a 6 digit PIN instead of a passphrase
    use_pin: "Use a 6 digit PIN instead",
    /// Error: the two PINs differ
    pins_no_match: "Those PINs did not match. Start again.",
    /// Note under the PIN setup when creating a PIN account
    pin_body: "You sign on this SuiPlay with this PIN. After 5 wrong PINs it locks until your backup unlocks it.",
    /// Note under the PIN setup when restoring a PIN account
    restore_pin_body: "Your account signs on this SuiPlay with this PIN. After 5 wrong PINs it locks until your backup unlocks it.",
    /// Note on the restore page, above the phone / key choices
    restore_backup_body: "Your phone or security key signs in once, so this SuiPlay knows the account. The address stays the same.",
    /// Note above the backup choices
    backup_body: "Your backup signs on rare.market and anywhere you are away from this SuiPlay, and unlocks it if you forget the PIN.",
    /// Page title when restoring a PIN account with a backup
    sign_in_backup: "Sign in with your backup",
    /// Page title: add a phone or security key as backup
    add_backup: "Add a backup",
    /// Heading on the PIN pad for an existing PIN
    enter_pin: "Enter your PIN",
    /// Note under the heading when the device already has a PIN
    has_pin_note: "This SuiPlay already has a PIN. Your new account uses it too.",
    /// PIN setup title when restoring a PIN account
    set_pin_for_suiplay: "Set a PIN for this SuiPlay",
    /// PIN setup title when creating a PIN account
    create_with_pin: "Create account with PIN",
    /// Choice title: use the security key (restore)
    use_your_key: "Use your security key",
    /// Choice description: security key in the USB-C port (restore)
    use_key_body: "Plug it into the USB-C port on top and touch it",
    /// Choice title: add a phone as backup
    add_phone: "Add your phone",
    /// Choice title: add a security key as backup
    add_key: "Add a security key",
    /// Choice description for adding a security key
    add_key_body: "A USB key such as a YubiKey, in the USB-C port on top",
    /// Choice title: add both phone and key
    add_both: "Add both",
    /// Choice description for adding both
    add_both_body: "Your phone and a security key. Either one can sign and unlock.",
    /// Heading: account restored
    account_back: "Your account is back",
    /// Text under the restored heading
    account_back_note: "This SuiPlay signs with your PIN again. Same address, same backups.",
    /// Heading: account created
    account_ready: "Your account is ready",
    /// Text under the account ready heading when the backup is a phone
    ready_note_phone: "This SuiPlay signs with your PIN. On rare.market and anywhere else, your phone signs.",
    /// Text under the account ready heading when the backup is a security key
    ready_note_key: "This SuiPlay signs with your PIN. On rare.market and anywhere else, your security key signs.",
    /// Text under the account ready heading when both are set
    ready_note_both: "This SuiPlay signs with your PIN. On rare.market and anywhere else, your phone or your security key signs.",
    /// Label above the account address
    address: "Address",
    /// Button: move funds to the new account
    move_everything: "Move everything here",
    /// Bluetooth step note when adding a phone as backup
    bluetooth_add_phone: "Your phone and this SuiPlay talk over Bluetooth to add your phone.",
    /// Error: wrong PIN
    wrong_pin: "Wrong PIN",
    /// Error: no security key found while adding a backup
    no_key_usbc: "No security key found. Plug it into the USB-C port on top and try again.",
    /// Error: adding the backup failed
    backup_failed: "Could not add the backup. Try again.",
    /// Toast: accepting an offer needs an account that can sign
    toast_add_signer: "Add an account that can sign to accept offers",
    /// Toast: the offer was withdrawn or filled
    toast_offer_closed: "That offer is no longer open",
    /// Toast: a higher offer appeared while selling
    toast_better_offer: "A better offer is open now. Check it before selling",
    /// Text prompt title: the price of an item, in SUI
    ask_price: "Price in SUI",
    /// Text prompt placeholder: example price
    price_example: "e.g. 12.5",
    /// Text prompt title: the recipient
    ask_send_to: "Send to",
    /// Text prompt placeholder: a Sui address or a SuiNS name
    send_to_example: "0x address or name.sui",
    /// Error: the Sui network refused the transaction
    network_rejected: "The network rejected this transaction.",
    /// Error under an item that cannot be sent: the send check failed
    item_check_failed: "Couldn't check this item. Try again.",
    /// Toast after sending several collectibles
    toast_collectibles_sent: "Collectibles sent",
    /// Toast after sending
    toast_sent: "Sent",
    /// Fallback name of the account that receives everything, in "Move everything to {label}"
    this_account: "this account",
    /// Move everything: how an account signs (PIN account)
    signs_pin: "Signs with your PIN",
    /// Move everything: how a passkey account signs
    signs_phone_or_key: "Signs with your phone or security key",
    /// Move everything: how a Ledger account signs
    signs_ledger: "Signs on your Ledger",
    /// Move everything: how a Keystone account signs
    signs_keystone: "Signs on your Keystone",
    /// Move everything: how this device's account signs; A is the controller button
    signs_a: "Signs with A on this SuiPlay",
    /// Move everything: how a Rare Wallet PIN account signs
    signs_wallet_pin: "Signs with your Rare Wallet PIN",
    /// Signing sheet title when moving everything in several transactions; label is the account name, i the step, n the total
    move_title_n: "Move everything to {label} ({i} of {n})",
    /// Signing sheet title when moving everything; label is the account name
    move_title: "Move everything to {label}",
    /// Error: the Sui network refused the move transaction
    move_rejected: "The network rejected the move. Nothing left the account.",
    /// Error: scanning the account failed; error is the technical reason
    read_account_failed: "Could not read that account: {error}",
    /// Unlock prompt reason: adding a new recovery phrase
    gate_add_phrase: "Enter your PIN to add this phrase",
    /// Unlock prompt reason: importing a wallet
    gate_add_wallet: "Enter your PIN to add this wallet",
    /// Unlock prompt reason: adding the next account of a phrase
    gate_add_account: "Enter your PIN to add an account",
    /// Text prompt placeholder: the first letters of a recovery word
    first_letters: "First letters",
    /// Text prompt placeholder: pasting a whole recovery phrase
    paste_placeholder: "12 or 24 words",
    /// Error under the watch address field
    enter_address_or_name: "Enter a 0x address or a .sui name",
    /// Toast: an account was added
    toast_account_added: "Account added",
    /// Error: adding an account failed
    add_account_failed: "Could not add the account",
    /// Unlock prompt reason: adding a passkey account
    gate_add_this_account: "Enter your PIN to add this account",
    /// Error: no Ledger found (long form: USB or Bluetooth)
    ledger_not_found_long: "No Ledger found. Connect it with USB or turn on its Bluetooth, unlock it and open the Sui app.",
    /// Text prompt title: name for a hardware account; n is the account number
    name_for_account: "Name for account {n}",
    /// Error: nothing was ticked on the hardware accounts page
    pick_account_to_add: "Pick an account to add.",
    /// Error: the wallet background worker stopped
    wallet_stopped: "The wallet stopped. Try again.",
    /// Note on the account created screen: where the PIN key is kept (fallback when unknown)
    kept_chip: "Checked by this SuiPlay's security chip",
    /// Passkey step note while adding several backups; i is the step, n the total
    backup_n_phone: "Backup {i} of {n}: your phone",
    /// Passkey step note while adding several backups; i is the step, n the total
    backup_n_key: "Backup {i} of {n}: your security key",
    /// Text prompt title: the address of the old SuiPlay wallet
    ask_old_address: "Your old SuiPlay wallet address",
    /// Text prompt placeholder: a Sui address or a SuiNS name
    old_address_example: "0x... or name.sui",
    /// Progress line when scanning the old wallet starts
    stage_starting: "Starting",
    /// The word for a passphrase, inside a sentence like "Wrong passphrase. Try again."
    noun_passphrase: "passphrase",
    /// The word for a PIN, inside a sentence like "Wrong PIN. Try again."
    noun_pin: "PIN",
    /// Text prompt title and placeholder: the name of an account
    account_name: "Account name",
    /// Toast: an address was copied
    toast_address_copied: "Address copied",
    /// Unlock prompt reason: removing an account
    gate_remove_account: "Enter your PIN to remove this account",
    /// Text prompt title: confirm removal by typing the word REMOVE; keep REMOVE exactly, it is what the person types
    type_remove: "Type REMOVE to confirm",
    /// Toast: renaming an account failed
    toast_rename_failed: "Couldn't rename the account.",
    /// Toast: adding an account failed
    toast_add_failed: "Couldn't add the account.",
    /// Toast: signed out of the wallet
    toast_signed_out: "Signed out",
    /// Toast: an account was removed
    toast_account_removed: "Account removed",
    /// Error: removing an account failed
    remove_failed: "Could not remove it",
    /// Toast: every wallet was removed from the device
    toast_all_removed: "All wallets removed from this device",
    /// Error: removing all wallets failed
    remove_all_failed: "Could not remove the wallets",
    /// Toast: the wallet was locked
    toast_wallet_locked: "Wallet locked",
    /// Error: no Ledger found (short form)
    ledger_not_found: "No Ledger found. Plug it in and unlock it.",
}
