strings! {
    /// Placeholder in the Marketplace home search field
    search_collections: "Search collections",
    /// Section title on Marketplace home: collections moving most
    trending: "Trending",
    /// Empty state title on Marketplace home when the data does not load
    load_failed_title: "Could not load Rare Marketplace",
    /// Empty state body on Marketplace home when the data does not load
    load_failed_body: "Check the connection. This page refreshes on its own.",
    /// Note beside the Your NFTs title: the wallet name or address, then how many NFTs it holds
    who_items: "{who}, {n} item|{who}, {n} items",
    /// Tile at the end of a shelf that opens the full list; short
    see_all: "See all",
    /// Section title on Marketplace home and title of the account page for the player's own wallet
    your_nfts: "Your NFTs",
    /// Section title on Marketplace home: table of collections by volume
    top_collections: "Top collections",
    /// Empty state title on Marketplace home
    no_collections: "No collections yet",
    /// Empty state title when the collections table does not load
    collections_failed: "Could not load collections",
    /// Hint bar (X button): steps the time range of the collections table; range is like 24h or 7d
    range_hint: "Range {range}",
    /// Follows a short wallet address under the account page title: how many NFTs it holds
    items_suffix: ", {n} item|, {n} items",
    /// Button on the account page that opens the wallet on the website
    open_on_rare_market: "Open on rare.market",
    /// Empty state title on the account page
    no_nfts_wallet: "No NFTs in this wallet",
    /// Empty state title on the account page when loading fails
    wallet_failed: "Could not load this wallet",
    /// Error shown on the buy sheet if Buy is pressed before the item has loaded
    item_loading: "This item is still loading. Try again in a moment.",
    /// Error shown on the buy sheet when the listing is gone
    item_unlisted: "This item is not listed anymore.",
    /// Error shown on the buy sheet when the purchase cannot be built
    cant_buy: "This item can't be bought here.",
    /// Title of the approval request in Rare Wallet; name is the item's name
    buy_title: "Buy {name}",
    /// Title of the buy sheet after a purchase went through
    bought: "Bought",
    /// Label before the transaction id on the purchase-complete sheet
    transaction: "Transaction",
    /// Purchase-complete message; name is the item's name, total is a price with its coin, e.g. 12.5 SUI
    is_yours_for: "{name} is yours for {total}",
    /// Purchase-complete note under the message
    in_your_wallet: "It's in your Rare Wallet. It can take a minute to show on rare.market.",
    /// Buy sheet price breakdown line: the seller's asking price
    list_price: "List price",
    /// Buy sheet price breakdown line: the marketplace's fee; venue is the marketplace name
    venue_fee: "{venue} fee",
    /// Buy sheet price breakdown line
    creator_royalty: "Creator royalty",
    /// Buy sheet price breakdown line: Rare's commission
    rare_fee: "Rare fee",
    /// Buy sheet price breakdown: the sum
    total: "Total",
    /// Label before the marketplace an item is listed on
    listed_on: "Listed on",
    /// Label above the price on the buy sheet and item page
    total_price: "Total price",
    /// Buy sheet note under the price
    gas_extra: "Network gas is extra.",
    /// Buy sheet QR heading and button
    buy_on_phone: "Buy on your phone",
    /// Buy sheet QR heading when the player is not in the app
    finish_on_phone: "Finish on your phone",
    /// Buy sheet note under the QR code
    scan_to_buy: "Scan to buy on rare.market.",
    /// Buy sheet note when the account has no passkey
    add_passkey_line: "Add a passkey to buy.",
    /// Button on the buy sheet; short
    add_passkey: "Add a passkey",
    /// Buy sheet note when the player is not signed in
    sign_in_line: "Sign in with a passkey to buy.",
    /// Buy sheet progress message
    getting_purchase_ready: "Getting the purchase ready...",
    /// Buy sheet progress message
    review_in_wallet: "Review and approve in Rare Wallet.",
    /// Buy button label while the purchase is being built; short
    getting_ready: "Getting ready...",
    /// Buy button label while Rare Wallet asks for approval; short
    waiting_approval: "Waiting for approval",
    /// Buy button; total is a price with its coin, e.g. 12.5 SUI
    buy_for: "Buy for {total}",
    /// Title of the on-screen keyboard when searching the Marketplace
    ask_title: "Search Rare Marketplace",
    /// Hint on the on-screen keyboard when searching the Marketplace; name.sui is an example SuiNS name
    ask_hint: "Collection name, wallet or name.sui",
    /// Search empty state; q is what the player typed
    no_results: "No results for \"{q}\"",
    /// Small all-caps heading over wallet search results
    wallets_caps: "WALLETS",
    /// Small all-caps heading over collection search results
    collections_caps: "COLLECTIONS",
    /// Column label: the cheapest listing price in a collection
    floor: "Floor",
    /// Column label: trading volume over the last 7 days; short
    volume_7d: "Volume 7d",
    /// Hint bar (Y button) on search results
    edit_search: "Edit search",
    /// Empty state title on the item page
    item_failed: "Could not load this item",
    /// Label before the owner's address on the item page
    owned_by: "Owned by",
    /// Item page buy box title when there is no listing
    not_listed: "Not listed",
    /// Item page note when there is no listing
    owner_not_listed: "The owner has not listed this item.",
    /// Button on the player's own listed item
    change_or_cancel: "Change price or cancel",
    /// Button on the player's own unlisted item
    list_or_sell: "List or sell",
    /// Button on the item page: buy at the listed price
    buy_now: "Buy now",
    /// Button on the item page when the item is in the cart; n is how many items the cart holds
    in_cart: "In cart ({n})",
    /// Button on the item page
    add_to_cart: "Add to cart",
    /// Stat label on the item page: price with all fees
    all_in_price: "All-in price",
    /// Stat label on the item page: the collection's cheapest listing
    floor_price: "Floor price",
    /// Stat label on the item page: how far above or below the floor this item is priced
    floor_diff: "Floor diff",
    /// Stat label on the item page: the highest offer made
    top_offer: "Top offer",
    /// Empty state on the item page
    no_traits: "No traits",
    /// Empty state on the item page: no sales, listings or offers yet
    no_activity: "No activity yet",
    /// Section title on the item page: the item's attributes
    traits: "Traits",
    /// Section title: recent sales, listings and offers
    activity: "Activity",
    /// Hint bar (Start button) that opens the cart; n is how many items are in it
    cart_hint: "Cart ({n})",
    /// Small all-caps stat label on a collection card
    floor_price_caps: "FLOOR PRICE",
    /// Small all-caps stat label on a collection card: trading volume over one day
    volume_1d_caps: "VOLUME 1D",
    /// Small all-caps stat label on a collection card: change in volume over one day; short
    vol_change_1d_caps: "VOL CHANGE 1D",
    /// Table column heading: the collection's name (follows a # rank column)
    collection: "Collection",
    /// Table column heading: how the floor price changed over a time range; win is like 24h or 7d
    floor_win: "Floor {win}",
    /// Table column heading: trading volume
    volume: "Volume",
    /// Table column heading: number of sales
    sales: "Sales",
    /// Table column heading: how many items are listed for sale
    listed: "Listed",
    /// Item card note: the item is staked and cannot be bought
    staked: "Staked",
    /// Activity table column heading: sale, list, offer...; short
    col_type: "Type",
    /// Activity table column heading; short
    col_item: "Item",
    /// Activity table column heading: which marketplace; short
    col_venue: "Venue",
    /// Activity table column heading; short
    col_price: "Price",
    /// Activity table column heading: the sender; short
    col_from: "From",
    /// Activity table column heading: the receiver; short
    col_to: "To",
    /// Activity table column heading: how long ago; short
    col_time: "Time",
    /// Activity table time cell; when is a terse age such as 5m, 2h or 3d; short
    time_ago: "{when} ago",
    /// Activity type: an item sold; short noun
    ev_sale: "Sale",
    /// Activity type: an item was put up for sale; short
    ev_list: "List",
    /// Activity type: a listing was taken down; short
    ev_delist: "Delist",
    /// Activity type: someone made an offer; short noun
    ev_offer: "Offer",
    /// Activity type: an offer was withdrawn; short
    ev_offer_cancel: "Offer cancel",
    /// Activity type: an item moved between wallets; short noun
    ev_transfer: "Transfer",
    /// Activity type: an item was created; short noun
    ev_mint: "Mint",
    /// Holders table column heading: the wallet; short
    col_holder: "Holder",
    /// Holders table column heading: how many items the wallet holds; short
    col_items: "Items",
    /// Holders table column heading: the percentage of all items the wallet holds
    col_share: "Share of supply",
    /// Hint bar (LT/RT) in the sweep sheet: change how many items to buy; short
    fewer_more: "Fewer / more",
    /// Error in the cart when a purchase cannot be built
    some_cant_buy: "Some of these items can't be bought here.",
    /// Cart error after part of the cart was bought; n is how many items were bought, error is the reason the rest failed
    bought_then_error: "Bought {n}. {error}",
    /// Title of the approval request in Rare Wallet for several items
    buy_n_items: "Buy {n} item|Buy {n} items",
    /// Cart error after part of the cart was bought; n is how many items were bought, error is why the next purchase failed
    bought_then: "Bought {n}. Then: {error}",
    /// Error in the sweep sheet
    sweep_price_failed: "Couldn't price this sweep right now. Try again.",
    /// Sweep sheet mode button: buy a set number of items; short
    count: "Count",
    /// Sweep sheet mode button: buy as many items as a budget allows; short
    spend_up_to: "Spend up to",
    /// Sweep sheet budget; amount is a number; SUI is the coin
    up_to_sui: "Up to {amount} SUI",
    /// Sweep sheet budget when none is set yet
    set_budget: "Set a budget",
    /// Sweep sheet count: how many of the listed items will be bought
    n_of_listed: "{n} of {total} listed",
    /// Sweep sheet max price each when no limit is set
    no_limit: "No limit",
    /// Sweep sheet label: the most to pay per item
    max_price_each: "Max price each",
    /// Sweep sheet explanation
    sweep_note: "The cheapest listings on every marketplace, at their all-in price.",
    /// Cart list footer when more items exist than are shown
    and_n_more: "and {n} more",
    /// Sweep sheet empty state
    pick_budget: "Pick a budget to see what it buys.",
    /// Sweep sheet loading message
    finding_listings: "Finding the cheapest listings...",
    /// Sweep sheet empty state when the collection has no listings
    nothing_to_buy_list: "Nothing listed to buy.",
    /// Cart note under the total
    gas_extra_left_out: "Network gas is extra. Items that sell first are left out.",
    /// Cart progress when the items are bought in several transactions
    getting_purchase_n: "Getting purchase {i} of {n} ready...",
    /// Cart progress when the items are bought in several transactions
    review_purchase_n: "Review and approve purchase {i} of {n} in Rare Wallet.",
    /// Cart message after the purchase went through
    bought_n_items: "Bought {n} item. It's in your Rare Wallet.|Bought {n} items. They're in your Rare Wallet.",
    /// Cart button when the account has no passkey
    add_passkey_to_buy: "Add a passkey to buy",
    /// Cart button label when the cart is empty
    nothing_to_buy: "Nothing to buy",
    /// Cart buy button; total is a price with its coin
    buy_n_for: "Buy {n} item for {total}|Buy {n} items for {total}",
    /// Cart button that empties the cart
    clear_cart: "Clear cart",
    /// Sweep sheet title when the collection's name is not known yet
    sweep_this: "Sweep this collection",
    /// Sweep sheet title; sweep means buy the cheapest listings in a collection in one go; name is the collection's name
    sweep_title: "Sweep {name}",
    /// Title of the cart sheet
    cart: "Cart",
    /// Cart sheet subtitle; A is the controller button
    cart_sub: "A on an item takes it out.",
    /// Tab on a collection page: the items for sale; short
    tab_items: "Items",
    /// Tab on a collection page: the wallets that hold its items; short
    tab_holders: "Holders",
    /// Sort choice for a collection's items
    sort_price_asc: "Price: low to high",
    /// Sort choice for a collection's items
    sort_price_desc: "Price: high to low",
    /// Sort choice for a collection's items
    sort_rarity: "Rarity: rarest first",
    /// Sort choice for a collection's items
    sort_recent: "Recently listed",
    /// Sort button text when trait filters are on; sort is the sort's name, n is how many traits are selected
    sort_with_traits: "{sort}, {n} trait|{sort}, {n} traits",
    /// Collection page line: the creator royalty percentage
    royalty: "Royalty {pct}",
    /// Collection page line: the creator royalty percentage, then the marketplaces it is listed on (names)
    royalty_listed_on: "Royalty {pct}, listed on {venues}",
    /// Collection stat: number of sales in the last 7 days; short
    sales_7d: "Sales 7d",
    /// Collection stat: items listed out of the total supply; short
    listed_supply: "Listed / supply",
    /// Collection stat: how many wallets hold items; short
    owners: "Owners",
    /// Empty state title on a collection page
    collection_failed: "Could not load this collection",
    /// Count above a collection's items: how many are listed for sale
    n_listed: "{n} listed",
    /// Button on a collection page: buy the cheapest listings in one go; short
    sweep: "Sweep",
    /// Empty state on a collection page
    nothing_listed: "Nothing listed right now",
    /// Empty state on a collection page when trait filters are on
    nothing_listed_traits: "Nothing listed with these traits",
    /// Holders figure: percentage of owners that are distinct wallets
    unique_owners: "Unique owners",
    /// Holders figure: the share of items the top 10 holders own
    top_10_hold: "Top 10 hold",
    /// Holders figure: the share of items the top 100 holders own
    top_100_hold: "Top 100 hold",
    /// Holders figure: how many items are held out of the supply
    items_held: "Items held",
    /// Holders section heading: items that are staked or locked
    staked_or_locked: "Staked or locked",
    /// Holders list: how many items sit in a staking or locking contract; name is its name
    n_in_name: "{n} in {name}",
    /// Title of the filters panel on a collection page; also a hint bar label
    filters: "Filters",
    /// Heading in the filters panel (noun)
    sort: "Sort",
    /// Button in the filters panel that removes all trait filters; short
    clear: "Clear",
    /// Filters panel empty state
    no_collection_traits: "No traits for this collection",
    /// Filters panel: the floor price of a trait value; SUI is the coin
    sui_floor: "{price} SUI floor",
    /// Filters panel: a trait value with nothing for sale
    no_listings: "No listings",
    /// Title of a wallet's page opened from the holders list when it has no name
    holder: "Holder",
    /// Hint bar (Select button): switch the items to a grid; short
    grid_view: "Grid view",
    /// Hint bar (Select button): switch the items to a list; short
    list_view: "List view",
    /// Title of the on-screen keyboard when searching coins in Trade
    search_coins: "Search coins",
    /// Hint on the on-screen keyboard when searching coins in Trade
    search_coins_hint: "Name, symbol or coin type",
    /// Market tab section strip: NFT collections (next to Coins, Perps, Traders)
    sec_collections: "Collections",
}
