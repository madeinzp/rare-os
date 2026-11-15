strings! {
    /// A date then a clock time, e.g. "Mar 4, 2025, 3:07 PM"
    date_time: "{date}, {time}",
    /// Error toast; what is a sentence like "Couldn't load activity."
    failed_try_again: "{what} Try again.",
    /// Error shown when the network client is missing
    sui_unreachable: "Sui is not reachable",
    /// Hint bar: press A to type into a text field
    hint_type: "Type",
    /// Screen reader label of the PIN dots; n is how many digits are entered
    digits: "{n} digit|{n} digits",
    /// Number pad key that deletes the last digit. Short: button
    key_delete: "Del",
    /// Item rarity tier
    tier_common: "Common",
    tier_uncommon: "Uncommon",
    /// Item rarity tier (the word, not the brand)
    tier_rare: "Rare",
    tier_epic: "Epic",
    tier_legendary: "Legendary",
    tier_mythic: "Mythic",
    /// Tooltip on a rarity badge; rank is a number, tier a rarity name
    rank_title: "Rank {rank}, {tier}",
    /// Label of the verified-collection check badge
    verified: "Verified",
    /// Tooltip on a marketplace logo; venue is the marketplace name
    listed_on: "Listed on {venue}",
    /// Under an item card: the item is staked
    staked: "Staked",
    /// Under an item card: the item has no sale listing
    not_listed: "Not listed",
    /// Error shown when a signing request cannot start
    not_available: "Rare Wallet is not available",
    /// Error shown when the wallet needs the network and has none
    no_network: "Rare Wallet can't reach the network right now",
    /// Error under a recipient field when SuiNS lookup is down
    name_lookup_unavailable: "Name lookup is not available right now",
    /// Error shown when the wallet task died
    stopped_try_again: "The wallet stopped. Try again.",
    /// How an account approves: no prompt
    method_silent: "This account is on this SuiPlay, so it signs in by itself. Nothing to approve.",
    method_pin: "You approve with your Rare Wallet PIN.",
    method_passkey_both: "You approve with your PIN on this SuiPlay, or with the passkey on your phone: Rare shows a QR code to scan.",
    method_passkey_phone: "You approve with your passkey: Rare shows a QR code, you scan it with your phone and confirm with your face or fingerprint. Keep Bluetooth on.",
    method_pin_account: "You approve with your PIN on this SuiPlay. Your backup (phone or security key) can always sign too.",
    method_ledger: "You approve on your Ledger. Connect it and open the Sui app.",
    method_keystone: "You approve on your Keystone over a USB-C cable.",
    /// Toast
    not_native_yet: "This part of the wallet is not in the native launcher yet",
    /// Toast after copying a wallet address
    address_copied: "Address copied",
    no_ledger_path: "This account has no Ledger path. Remove it and add it again.",
    ledger_check_stopped: "The Ledger check stopped. Try again.",
    /// Screen reader label of the copy-address button
    copy_address: "Copy address",
    /// Shown on a copy button for a moment after copying. Short: button
    copied: "Copied",
    /// Column and label for the 24-hour price change. Short
    h24: "24h",
    /// Under the account total when no USD prices could be loaded
    prices_unavailable: "Prices unavailable",
    /// Coin table column header
    col_coin: "Coin",
    /// Coin table column header
    col_amount: "Amount",
    /// Coin table column header, the USD value
    col_value: "Value",
    /// Activity row title for a transaction the account sent
    act_sent: "Sent",
    /// Activity row title
    act_received: "Received",
    /// Activity row title
    act_swapped: "Swapped",
    /// Activity row title for any other transaction
    act_transaction: "Transaction",
    /// Activity row subtitle; an address follows. summary is like "0.5 SUI"
    summary_from: "{summary} from",
    /// Activity row subtitle; an address follows. summary is like "0.5 SUI"
    summary_to: "{summary} to",
    /// Activity row: the transaction failed
    failed: "Failed",
    /// Account kind label: an address that is only followed
    kind_watch: "Watch only",
    /// Account kind label
    kind_passkey: "Passkey",
    /// Account kind label: signed in with a social login
    kind_social: "Social",
    /// Account kind label: made from a 12-word recovery phrase
    kind_phrase: "Recovery phrase",
    /// Account kind label: imported private key
    kind_key: "Private key",
    /// Activity day heading
    today: "Today",
    /// Activity day heading
    yesterday: "Yesterday",
    /// noun is PIN or passphrase; n is seconds
    wrong_wait: "Wrong {noun}. Wait {n} second and try again.|Wrong {noun}. Wait {n} seconds and try again.",
    /// noun is PIN or passphrase
    wrong_try_again: "Wrong {noun}. Try again.",
    /// n is seconds
    too_many_tries: "Too many tries. Wait {n} second.|Too many tries. Wait {n} seconds.",
    wallet_locked: "The wallet is locked.",
    didnt_work: "That didn't work. Try again.",
    /// Button on a watch-only account: add an account that can send
    add_signing_account: "Add a signing account",
    /// Button, page title and hint for the account list
    accounts: "Accounts",
    /// Button; moves assets from the previous wallet app
    move_from_old: "Move from your old SuiPlay wallet",
    /// Wallet home tab
    coins: "Coins",
    /// Wallet home tab and section title (NFTs)
    collectibles: "Collectibles",
    /// Wallet home tab: transaction history
    activity: "Activity",
    /// Button: send n selected collectibles. Short: button
    send_n: "Send {n}",
    /// Button shown while selecting collectibles and none is picked. Short: button
    pick_items_to_send: "Pick items to send",
    /// Empty state title
    no_coins_yet: "No coins yet",
    /// Empty state body
    no_coins_body: "Send Sui assets to this address and they show up here.",
    /// Empty state title
    balances_failed: "Could not load balances",
    check_connection: "Check the connection and try again.",
    /// Button below the activity list
    show_more: "Show more",
    /// Empty state title
    no_activity_yet: "No activity yet",
    no_activity_body: "Your transactions show up here.",
    older_activity_failed: "Couldn't load older activity",
    activity_failed: "Couldn't load activity",
    /// Hint bar: A toggles the selection of a collectible
    hint_toggle: "Toggle",
    /// Placeholder of the passphrase field and a name for the secret. Short
    passphrase: "Passphrase",
    /// Button: copy the private key. Short: button
    copy_key: "Copy key",
    enter_passphrase_continue: "Enter your passphrase to continue",
    enter_pin_continue: "Enter your PIN to continue",
    /// Warning above the 12 recovery words
    reveal_warn_phrase: "Anyone with these words can take everything in this account. Write them down and keep them offline. Never type them into a website or share them in chat.",
    /// Warning above the private key
    reveal_warn_key: "Anyone with this key can take everything in this account. Write it down and keep it offline. Never type it into a website or share it in chat.",
    /// Lower-case noun used inside "Wrong {noun}." messages
    noun_passphrase: "passphrase",
    /// Noun used inside "Wrong {noun}." messages
    noun_pin: "PIN",
    /// Passphrase rule
    use_8_chars: "Use at least 8 characters",
    /// PIN rule
    use_6_digits: "Use at least 6 digits",
    enter_current_passphrase: "Enter your current passphrase",
    enter_current_pin: "Enter your current PIN",
    choose_new_passphrase: "Choose a new passphrase",
    choose_new_pin: "Choose a new PIN (6 digits or more)",
    enter_new_passphrase_again: "Enter the new passphrase again",
    enter_new_pin_again: "Enter the new PIN again",
    passphrase_changed: "Your passphrase is changed.",
    pin_changed: "Your PIN is changed.",
    /// Sheet title
    change_passphrase: "Change passphrase",
    /// Sheet title
    change_pin: "Change PIN",
    mismatch_passphrase: "Those did not match. Choose the new passphrase again.",
    mismatch_pin: "Those did not match. Choose the new PIN again.",
    /// Remove-account sheet body for a watch-only account
    remove_watch: "Rare Wallet stops watching this address. Nothing moves and nothing is lost.",
    /// Remove-account sheet body for a passkey account
    remove_passkey: "The account leaves this device. Your passkey signs you back in any time, here or on another device.",
    /// Remove-account sheet body for a Ledger or Keystone account
    remove_hardware: "The account is removed from this device. Your hardware wallet still holds the key, so you can add it again later.",
    /// Remove-account sheet body for a phrase or key account
    remove_native: "The account and its key are removed from this device. Make sure you saved the recovery phrase or key first. Without it the funds in this account cannot be recovered.",
    /// Sheet title
    sign_out_title: "Sign out of Rare?",
    /// Sheet title
    remove_account_title: "Remove this account?",
    /// Sheet subtitle; name is the account name, kind its lower-case kind (e.g. "passkey")
    account_subtitle: "{name}, {kind}",
    /// Button while signing out. Short: button
    signing_out: "Signing out",
    /// Button while removing. Short: button
    removing: "Removing",
    /// Sheet title
    remove_all_title: "Remove all wallets?",
    /// Button. Short: button
    remove_all: "Remove all",
    remove_all_body: "Every account, recovery phrase and key is erased from this device, and the PIN is cleared. Funds stay on the network, but without your recovery phrases or hardware wallets you cannot reach them again.",
    /// Field label; word is the literal word REMOVE the player must type (not translated)
    type_remove_confirm: "Type {word} to confirm",
    hw_help_usb: "Ledger and Keystone connect over USB. Plug the device in, unlock it, and for Ledger open the Sui app. Ledger also works over Bluetooth.",
    hw_help_permission: "If Rare OS says it needs permission to use the device, run one command in a terminal, then unplug and plug the device back in. The check below shows the exact command for this system.",
    /// found is a device name
    hw_found: "Found {found}. Nothing is in the way.",
    /// Button. Short: button
    copy_command: "Copy command",
    /// Sheet title and settings row
    hardware_wallets: "Hardware wallets",
    /// Button while checking for the device. Short: button
    looking: "Looking",
    /// Button. Short: button
    check_my_ledger: "Check my Ledger",
    /// Accounts page group heading: accounts that sign with Rare (phrase, key, passkey, PIN)
    group_rare: "Rare accounts",
    /// Accounts page group heading
    group_hardware: "Hardware",
    /// Accounts page group heading: watch-only addresses
    group_watch: "Watch",
    /// Screen reader label of the check mark on the selected account
    current_account: "Current account",
    /// Button while adding an account. Short: button
    adding: "Adding",
    /// Button: derive the next account from the recovery phrase this device holds
    add_another_from_phrase: "Add another from this phrase",
    /// Button. Short: button
    add_account: "Add account",
    /// Empty state
    no_accounts_yet: "No accounts yet",
    /// Menu row
    use_this_account: "Use this account",
    /// Menu row and hint
    rename: "Rename",
    /// Menu row and page title
    move_everything_here: "Move everything to this account",
    /// Menu row description
    from_another_account: "From another account on this SuiPlay",
    /// Menu row
    remove_from_device: "Remove from this device",
    /// Page subtitle
    accounts_on_device: "{n} account on this device|{n} accounts on this device",
    /// Hint bar: A selects the account
    hint_use: "Use",
    /// Hint bar: Y opens the account actions
    hint_actions: "Actions",
    /// Under a coin: no USD price known
    no_price: "No price",
    /// Row label in transaction details
    status: "Status",
    /// Transaction status
    success: "Success",
    details_gone: "The network no longer keeps the full details of this transaction. This is what the activity list recorded.",
    /// Panel title in transaction details
    you_sent: "You sent",
    /// Panel title in transaction details
    you_received: "You received",
    /// Row label: the sender address
    from: "From",
    /// Row label: the recipient address
    to: "To",
    /// Network fee row: the sponsor paid the fee
    paid_by_sponsor: "Paid by sponsor",
    /// Network fee row when the fee was refunded; amount is a number
    fee_refund: "Refund {amount} SUI",
    /// Row label
    network_fee: "Network fee",
    /// Row label: the transaction id
    digest: "Digest",
    /// Screen reader label
    copy_digest: "Copy digest",
    /// Row label
    sender: "Sender",
    /// Object line in transaction details; an object id follows
    obj_sent: "Sent",
    /// Object line; an object id follows
    obj_created: "Created",
    /// Object line; an object id follows
    obj_deleted: "Deleted",
    /// Object line; an object id follows
    obj_received: "Received",
    /// Heading above a QR code
    open_on_phone: "Open on your phone",
    scan_suiscan: "Scan to see this transaction on Suiscan.",
    /// Button
    view_on_suiscan: "View on Suiscan",
    /// Label above the amount of one coin the account holds
    your_balance: "Your balance",
    /// Row label: the coin price in USD
    price: "Price",
    /// Heading
    recent_activity: "Recent activity",
    /// Empty state; symbol is a coin ticker like SUI
    no_coin_activity: "No {symbol} activity yet",
    could_not_load_activity: "Could not load activity",
    /// Page subtitle; name is the receiving account name, addr its short address
    move_to: "To {name} ({addr})",
    /// "Add an account" is the name of a screen
    move_no_source: "There is no other account on this SuiPlay to move from. Add it first under Add an account, then come back here.",
    move_pick: "Pick the account to move from. It approves once, the way it always does.",
    /// Progress text; name is an account name
    move_reading: "Reading what {name} holds",
    /// name is an account name
    move_empty: "{name} is empty",
    move_nothing: "There is nothing to move.",
    move_sui_less_fee: "{amount} SUI, less the fee",
    /// Row value: n further items not listed
    n_more: "{n} more",
    /// Summary row (Sui kiosks)
    kiosks: "Kiosks",
    /// Summary row value; kiosks is a count, items a phrase like "3 items"
    kiosks_items: "{kiosks}, {items}",
    items_n: "{n} item|{n} items",
    /// Summary row
    kiosk_items: "Kiosk items",
    /// Summary row
    staked_sui: "Staked SUI",
    stakes_n: "{n} stake|{n} stakes",
    /// Summary row
    suins_names: "SuiNS names",
    move_from_note: "From {name} ({addr}). Listings and locked items move with their kiosk.",
    /// Heading of the list of things that cannot be moved
    stays_behind: "Stays behind",
    /// Button; n is how many approvals are needed
    move_everything_approvals: "Move everything ({n} approvals)",
    /// Button
    move_everything: "Move everything",
    /// Progress title; name is an account name
    approve_with: "Approve with {name}",
    move_check_sheet: "Check the review sheet, then approve the way this account signs.",
    move_on_its_way: "Everything is on its way",
    /// from and to are account names; count is like "2 transactions"
    move_sent_all: "{from} sent everything it could to {to}. {count}",
    transactions_n: "{n} transaction|{n} transactions",
    /// Where a collectible is
    held_your_kiosk: "In your kiosk",
    held_your_wallet: "In your wallet",
    held_a_kiosk: "In a kiosk",
    held_object: "Held inside another object",
    held_shared: "Shared object",
    held_immutable: "Immutable",
    item_load_failed: "Could not load this item",
    /// price is a number
    listed_for: "Listed for {price} SUI",
    /// Disabled button. Short: button
    cant_send_this: "Can't send this one",
    /// Button
    view_in_marketplace: "View in Marketplace",
    /// why is a reason sentence without the final period, e.g. "It is staked in ACT"
    cant_send_reason: "{why}.",
    /// Heading above the item traits
    traits: "Traits",
    /// Under a trait; pct is a percentage like 2.4%
    trait_share: "{pct} have this",
    /// Row label: where the item is held
    held: "Held",
    /// Row label
    kiosk: "Kiosk",
    /// Row label
    object: "Object",
    /// Screen reader label
    copy_kiosk_id: "Copy kiosk id",
    /// Screen reader label
    copy_object_id: "Copy object id",
    /// Button label while waiting for the player to compare the address on the device. Short: button
    check_your_ledger: "Check your Ledger",
    /// Button. Short: button
    verify_on_ledger: "Verify on Ledger",
    /// Label above the wallet address
    your_address: "Your address",
    ledger_same: "Your Ledger shows the same address.",
    ledger_different: "Your Ledger shows a different address. Don't use this one.",
    only_sui_assets: "Only send Sui network assets to this address.",
    note_watch: "This wallet is watch only. You can receive here, but you can't send from it.",
    /// "Verify on Ledger" is the button name
    note_ledger: "Verify on Ledger shows this address on the device so you can match it before sharing.",
    note_keystone: "This address belongs to your Keystone. Sending needs the device.",
    /// Main button text while items are checked. Short: button
    cta_checking_items: "Checking items",
    /// Disabled button. Short: button
    cta_nothing_sendable: "Nothing here can be sent",
    /// Disabled button. Short: button
    cta_enter_recipient: "Enter a recipient",
    /// Disabled button while a SuiNS name resolves. Short: button
    cta_looking_up: "Looking up name",
    /// Disabled button. Short: button
    cta_check_recipient: "Check the recipient",
    /// Disabled button: the recipient is the sender. Short: button
    cta_pick_another: "Pick another address",
    /// Button that opens the review sheet. Short: button
    cta_review: "Review",
    watch_only_title: "This account is watch only",
    watch_only_body: "Add an account that can sign to send from this wallet.",
    items_load_failed: "These items could not be loaded.",
    /// Placeholder of the recipient field; keep ".sui"
    recipient_placeholder: "0x address or name.sui",
    /// Button while the review sheet opens. Short: button
    opening_review: "Opening review",
    /// Page title
    send_collectibles: "Send collectibles",
    /// Page subtitle; name is an account name
    items_from: "{n} item from {name}|{n} items from {name}",
    /// Review sheet title for one collectible; name is its name
    send_named: "Send {name}",
    /// Review sheet title
    send_collectibles_n: "Send {n} collectibles",
    /// Review sheet title when the send is split in batches
    send_collectibles_batch: "Send {n} collectibles ({batch} of {batches})",
    /// Disabled button. Short: button
    cta_enter_amount: "Enter an amount",
    /// Disabled button; symbol is a coin ticker. Short: button
    cta_not_enough: "Not enough {symbol}",
    /// Disabled button. Short: button
    cta_leave_gas: "Leave some SUI for gas",
    /// Button; also the name of a screen
    add_an_account: "Add an account",
    /// Under the coin picker; amount is a number
    balance_of: "Balance {amount}",
    /// Screen reader label of the switch between coin and USD entry
    type_in_coins: "Type in coins",
    type_in_usd: "Type in USD",
    /// Small label next to the converted amount; switches between coin and USD entry. Short
    switch: "Switch",
    /// Button: use the whole balance. Short: button
    max: "Max",
    max_note: "Max sends everything. The network fee comes out of it.",
    no_coins_in_account: "No coins in this account yet.",
    /// Sheet title
    send_which_coin: "Send which coin",
    /// Page subtitle; name is an account name
    from_account: "From {name}",
    /// Review sheet title; amount is a number, symbol a coin ticker
    send_amount: "Send {amount} {symbol}",
    /// Toast once a send went through; amount is a number, symbol a coin ticker
    sent_amount: "Sent {amount} {symbol}",
    /// Settings row: lock the wallet
    lock_now: "Lock now",
    /// Settings row description
    lock_now_body: "Ask for your PIN next time",
    /// Settings row title
    auto_lock: "Auto-lock",
    auto_lock_body: "Lock after being idle",
    /// Auto-lock choice; n is minutes. Short: button
    n_min: "{n} min",
    /// Row description
    needs_passphrase: "Needs your passphrase",
    /// Row description
    needs_pin: "Needs your PIN",
    /// Settings row
    show_phrase: "Show recovery phrase",
    /// Settings row
    show_key: "Show private key",
    keys_keyring: "Keys are stored in your system keyring.",
    keys_encrypted: "Keys are stored encrypted on this device.",
    keys_none: "No keys are stored on this device yet.",
    /// Panel title
    security: "Security",
    /// Empty state
    no_games_connected: "No games are connected to this wallet.",
    /// Panel title
    connected_games: "Connected games",
    /// Settings row
    hw_help_row: "Ledger and Keystone help",
    /// Settings row description
    hw_help_row_body: "Fix USB permission and check your device",
    /// Settings row
    remove_this_account: "Remove this account",
    /// Settings row
    remove_all_row: "Remove all wallets from this device",
    /// Settings row description
    remove_all_row_body: "Erases every account and key here",
    /// Page title
    watch_title: "Watch an address",
    watch_body: "Follow any wallet's coins and collectibles. You cannot send from it.",
    /// Field label and keyboard prompt
    address_or_name: "Address or SuiNS name",
    /// Placeholder; keep "0x..." and ".sui"
    address_placeholder: "0x... or name.sui",
    /// Field label
    name_optional: "Name (optional)",
    /// Placeholder: an example account name
    watch_name_placeholder: "Watch 1",
    /// Button. Short: button
    start_watching: "Start watching",
    /// Error under the field
    could_not_add_address: "Could not add that address",
    /// Page title of the wallet with no accounts (product name)
    rare_wallet: "Rare Wallet",
    /// Heading
    other_ways_sign_in: "Other ways to sign in",
    /// Heading
    other_ways_add: "Other ways to add",
    /// Ghost button. Short: button
    move_assets: "Move your assets from your old SuiPlay wallet",
    /// List row
    create_pin_account: "Create account with PIN",
    /// List row
    restore_pin_account: "Restore an account with PIN",
    /// List row
    create_passkey_account: "Create a passkey account",
    /// Row description
    passkey_way: "Signs with your phone or a security key. No words to write down",
    /// A is the A button of the controller
    create_line: "Your account is kept on this SuiPlay and signs with A. You write down 12 words once as a backup.",
    /// Row description
    pin_way: "Signs on this SuiPlay with a PIN. Your phone or a security key is the backup",
    /// Row description
    restore_way: "Your 12 recovery words and your phone or security key. Same address",
    /// List row
    add_from_phrase: "Add an account from your recovery phrase",
    add_from_phrase_body: "The next account of the phrase already on this device",
    /// Page subtitle
    pick_how_to_add: "Pick how you want to add it.",
    /// Button. Short: button
    create_account: "Create account",
    /// Button. Short: button
    sign_in_passkey: "Sign in with a passkey",
}
