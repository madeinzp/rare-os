strings! {
    /// Item sheet error: rare.market could not be reached
    unreachable: "Couldn't reach rare.market. Check the connection and try again.",
    /// Item sheet error when no message came back
    didnt_go_through: "That didn't go through. Try again.",
    /// Item sheet title while loading or when it failed (a collectible)
    item: "Item",
    /// Item sheet error
    couldnt_load_item: "Couldn't load this item.",
    /// Item sheet status; venue is a marketplace name
    listed_on: "Listed on {venue}",
    /// Item status: locked in a game's staking
    staked: "Staked",
    /// Item status
    not_listed: "Not listed",
    /// Stat label and price button (short): the lowest listed price of the collection
    floor: "Floor",
    /// Price button: floor price plus ten percent (short)
    floor_plus_10: "Floor +10%",
    /// Stat label: the highest offer on the item
    best_offer: "Best offer",
    /// Stat label: price of the last sale
    last_sale: "Last sale",
    /// Shown while offers load
    looking_for_offers: "Looking for offers...",
    /// Offer cannot be taken; price is a number of SUI, reason a sentence
    best_offer_blocked: "Best offer {price} SUI. {reason}",
    /// Button: sell into the best offer; price is a number
    sell_for: "Sell for {price} SUI",
    /// Note under Sell; price is a number, venue a marketplace name
    offer_after_fees: "Best offer {price} SUI on {venue}. You get this after fees and royalty.",
    /// Note under Sell; venue is a marketplace name
    offer_full_price: "Best offer on {venue}. You get the full price.",
    /// Note under Sell when Rare pays the network fee; note is the next sentence
    fee_paid_note: "Network fee paid by Rare. {note}",
    /// Shown when nobody offers
    no_offers: "No offers on this item right now.",
    /// Section title
    sell_now: "Sell now",
    /// Note when listed on another marketplace; venue is its name
    listed_elsewhere: "Listed on {venue}. Cancel it there to list or send it here.",
    /// Button (short)
    other_price: "Other price",
    /// Button: list again at a price; price is a number
    relist_for: "Relist for {price} SUI",
    /// Button: list for sale at a price; price is a number
    list_for: "List for {price} SUI",
    /// Button when no price is chosen
    pick_price: "Pick a price",
    /// Section title when already listed
    change_price: "Change the price",
    /// Section title
    list_for_sale: "List for sale",
    /// Button: take the listing down
    cancel_listing: "Cancel listing",
    /// Shown while a Rare Wallet approval is pending
    working: "Review and approve in Rare Wallet.",
    /// Toast after a sale; name is the item, price a number
    sold_for: "Sold {name} for {price} SUI",
    /// Toast after listing; name is the item, price a number
    listed_for: "Listed {name} for {price} SUI",
    /// Toast after taking a listing down; name is the item
    cancelled_listing: "Cancelled the listing of {name}",
    /// Sheet subtitle; game is the game's title
    asked_rare: "{game} asked Rare",
    /// Claim sheet line: a trophy voucher; id is a number
    voucher_trophy: "Trophy {id}",
    /// Claim sheet line: an item voucher; kind is a number
    voucher_item: "Item kind {kind}",
    /// Claim sheet line; list is e.g. "level 2, hp 10"
    voucher_item_with: "Item kind {kind} with {list}",
    /// Claim sheet line: a game stat changes; key is the stat name
    stat_becomes: "{key} becomes {value}",
    /// Claim button while waiting for the wallet
    waiting_approval: "Waiting for approval",
    /// Claim button and hint (verb)
    claim: "Claim",
    /// Claim sheet secondary button: dismiss
    not_now: "Not now",
    /// Claim sheet body for a trophy
    goes_to_profile: "Goes to your Rare profile. Rare covers the network fee when your wallet is short.",
    /// Claim sheet body for an item
    goes_to_wallet: "Goes to your Rare Wallet. You review it before anything is signed.",
    /// Claim sheet success line (starts with a space)
    claim_done: " Done. It can take a minute to show.",
    /// Claim sheet note when there is no wallet account
    add_account_claim: "Add a Rare Wallet account to claim.",
    /// Claim sheet title after success
    claimed: "Claimed",
    /// Claim sheet subtitle when no game asked
    from_link: "From a Rare link",
    /// Claim error
    claim_failed: "The claim didn't go through",
    /// Sell sheet title; name is a short item name like #1204
    sell_title: "Sell {name}",
    /// Inventory total; ends with a space, the amount in SUI follows. n is a number of items
    items_floor_value: "{n} item, floor value |{n} items, floor value ",
    /// Inventory group title for items without a game
    other_collectibles: "Other collectibles",
    /// Under an item; price is a number of SUI or --
    floor_price: "Floor {price}",
    /// Inventory empty state title
    no_collectibles: "No collectibles in this wallet",
    /// Inventory empty state body
    no_collectibles_body: "Items from Sui games show up here, grouped by game.",
    /// Inventory error title
    load_failed: "Could not load your inventory",
    /// Inventory error body
    load_failed_body: "Check the connection and try again.",
}
