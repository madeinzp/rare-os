strings! {
    // ---- short ages and counts
    /// Short age, seconds; n is a number. Short: table cell
    age_s: "{n}s",
    /// Short age, minutes; n is a number. Short: table cell
    age_m: "{n}m",
    /// Short age, hours; n is a number. Short: table cell
    age_h: "{n}h",
    /// Short age, days; n is a number. Short: table cell
    age_d: "{n}d",
    /// Short age, months; n is a number. Short: table cell
    age_mo: "{n}mo",
    /// Short age, years; n is a number. Short: table cell
    age_y: "{n}y",
    /// Time until the next funding payment; h hours, m minutes. Short
    until_hm: "{h}h {m}m",
    /// Time until the next funding payment; m minutes. Short
    until_m: "{m}m",
    /// Liquidation price when it is effectively unreachable (a 1x long)
    near_zero: "Near $0",
    /// Coin age in a meta line, after the market cap; age is like 52m
    age_old: "{age} old",
    /// Market cap in a coin's meta line; value is a money figure. Short
    mc: "MC {value}",
    /// Volume under a coin card and in the chart readout; value is a figure. Short
    vol: "Vol {value}",
    /// Under a perps market card; value is a leverage like 50x
    up_to: "Up to {value}",
    /// A route through the market: number of swap hops
    hops: "{n} hop|{n} hops",
    /// A route with venue names; venues is a list of exchange names, such as Cetus, Turbos
    hops_via: "{n} hop via {venues}|{n} hops via {venues}",
    /// Trader labels: number of other wallets funded from the same source
    linked_wallets: "Linked to {n} wallet|Linked to {n} wallets",
    /// Column header: holder count; n is a formatted number
    holders_n: "{n} holder|{n} holders",

    // ---- hints, aria labels
    /// Aria/hint on a side switch button; label is the side name
    switch_to: "Switch to {label}",
    /// Aria/hint on a timeframe chip; label is a timeframe such as 1h
    show_label: "Show {label}",
    /// Hint on the minus button of the amount stepper
    less_amount: "Less amount",
    /// Hint on the plus button of the amount stepper
    more_amount: "More amount",
    /// Hint on the minus button of the leverage stepper, and the hint bar for the left bumper
    less_leverage: "Less leverage",
    /// Hint on the plus button of the leverage stepper, and the hint bar for the right bumper
    more_leverage: "More leverage",
    /// Hint bar: decrease a value. Short
    less: "Less",
    /// Hint bar: flip the pair of a swap. Short
    flip: "Flip",
    /// Hint bar: pick the highlighted coin. Short
    choose: "Choose",
    /// Aria label on the coin button of a swap
    change_coin: "Change coin",
    /// Hint bar: take control of the chart. Also the label under the chart
    control_chart: "Control chart",
    /// Hint bar: switch the chart into the mode with cursor and zoom
    chart_mode: "Chart mode",
    /// Hint bar in chart mode: move the cursor
    chart_move: "Move",
    /// Hint bar in chart mode: zoom
    chart_zoom: "Zoom",
    /// Hint bar in chart mode: show fewer bars
    chart_shorter: "Shorter",
    /// Hint bar in chart mode: show more bars
    chart_longer: "Longer",
    /// Hint bar in chart mode: switch between candlesticks and a line
    chart_candles_or_line: "Candles or line",
    /// Hint bar in chart mode: pick indicators
    chart_indicators: "Indicators",
    /// Hint bar in chart mode: go back to the page
    leave_chart: "Leave chart",
    /// Label under the chart: the chart is a candlestick chart
    candles: "Candles",
    /// Label under the chart: the chart is a line chart
    line: "Line",
    /// Label under the chart in chart mode, next to the back button
    leave: "Leave",
    /// Hint bar: 7 or 30 day window. Also a timeframe chip and a column header
    days_30: "30 days",
    /// Hint bar: 7 day window. Also a chip
    days_7: "7 days",
    /// Hint on the price alert button
    set_price_alert: "Set a price alert",
    /// Hint on a perps row to open its market
    open_market: "Open market",
    /// Hint on a trader's row: mirror their buy
    mirror_buy: "Mirror buy",
    /// Hint on a trader's row: mirror their sell
    mirror_sell: "Mirror sell",
    /// Hint on the follow button: stop following. Button label when a trader is followed is `following`
    unfollow: "Unfollow",
    /// Button and hint: follow a trader
    follow: "Follow",
    /// Button label: this trader is followed. Also a section title
    following: "Following",

    // ---- review sheet (shared)
    /// Network gas line in a trade review: Rare pays the gas
    covered_by_rare: "Covered by Rare",
    /// Network gas line in a trade review; amount is a SUI amount such as 0.004
    about_gas: "about {amount} SUI",
    /// Under the review lines while the trade is being checked
    checking_trade: "Checking the trade...",
    /// Under the review lines once the trade checked out
    checked_note: "Checked against the network just now. Rare Wallet shows the final details before anything is signed.",
    /// Review note and toast after a test-only signature
    signed_test_note: "Signed as a test and checked against the network. Nothing was sent.",
    /// Review note and toast after a perps trade is sent
    sent_note: "Sent. It shows in Positions once the network has it.",
    /// Toast after a swap is sent
    swap_sent: "Swap sent. Your balance updates in a moment.",
    /// Review line label: the network fee
    network_gas: "Network gas",
    /// Review sheet: the wallet has no passkey
    add_passkey_text: "Add a passkey to trade.",
    /// Button: add a passkey to the wallet. Short: button
    add_passkey: "Add a passkey",
    /// Review sheet: nobody is signed in
    sign_in_passkey_text: "Sign in with a passkey to trade.",
    /// Review failed to build; error is the reason, already plain text
    build_failed: "Couldn't build this trade. {error}",
    /// The trade was rejected; error is the reason, already plain text
    trade_failed: "The trade didn't go through: {error}",
    /// The signed-in account changed while a review was open
    account_changed: "The active account changed. Close this and start the trade again.",
    /// Review line: the trade's slippage setting
    slippage: "Slippage",
    /// Review line and column: price. Short
    price: "Price",
    /// Review line: the route a swap takes
    route: "Route",
    /// Review line: the swap fee
    fee: "Fee",
    /// Review line: the fewest coins the swap pays out
    minimum_received: "Minimum received",
    /// Review line: what the player pays; the amount follows
    you_pay: "You pay",
    /// Review line and swap label: what the player sells
    you_sell: "You sell",
    /// Review line and swap label: estimated amount received
    you_get_about: "You get about",
    /// Review sheet title for a buy; symbol is a coin ticker
    buy_title: "Buy {symbol}",
    /// Review sheet title for a sell; symbol is a coin ticker
    sell_title: "Sell {symbol}",
    /// Review sheet button for a buy
    confirm_buy: "Confirm buy",
    /// Review sheet button for a sell
    confirm_sell: "Confirm sell",
    /// Review sheet button for a swap
    confirm_swap: "Confirm swap",
    /// Review sheet title of a swap; from and to are coin tickers
    swap_title: "Swap {from} to {to}",
    /// Title of the signing request for a swap; from and to are coin tickers
    swap_for: "Swap {from} for {to}",
    /// Title of the signing request for closing a perps position; symbol is a market ticker
    close_symbol: "Close {symbol}",
    /// Review sheet title for closing a position; symbol is a market ticker, side is long or short
    close_title: "Close {symbol} {side}",
    /// Title of the signing request for mirroring a trader's buy
    mirror_a_buy: "Mirror a buy",
    /// Title of the signing request for mirroring a trader's sell
    mirror_a_sell: "Mirror a sell",
    /// Hint bar and review button for closing a perps position
    close_position: "Close position",

    // ---- coin search
    /// Section title in coin search: recently opened coins
    recent: "Recent",
    /// Coin search failed
    search_failed: "Could not search coins right now.",
    /// Coin search in progress
    searching: "Searching",
    /// Coin search found nothing
    no_coin_match: "No coin matches. Paste a full coin type to open one that is not listed.",
    /// Toast: a typed search found nothing; query is what was typed
    no_coins_found: "No coins found for {query}",
    /// Placeholder text of the search field in a swap picker
    search_any_coin: "Search any coin by name, ticker or coin type",
    /// Search button on the Trade home
    search_any_coin_short: "Search any coin",
    /// Aria label of the search button
    search_coins: "Search coins",

    // ---- swap
    /// Page title and button of the swap screen. Also an action button on the Trade home
    swap: "Swap",
    /// Swap button once both coins are picked
    review_swap: "Review swap",
    /// Hint bar and under-button hint: open the review sheet
    review: "Review",
    /// Swap screen: wallet missing
    add_wallet_first: "Add a wallet first",
    /// Swap error: build failed because no wallet
    add_wallet_first_period: "Add a wallet first.",
    /// Swap screen: the amount is more than the balance
    not_enough_balance: "Not enough balance",
    /// Swap screen: there is no way to swap the amount
    no_route: "No route for this amount",
    /// Swap screen: the balance of the coin that is paid; amount is a number, symbol a ticker
    balance_of: "Balance {amount} {symbol}",
    /// Swap coin button when no coin is picked
    choose_coin: "Choose a coin",
    /// Preset chip: use the whole balance. Short: chip
    max: "Max",
    /// Sheet title: pick the coin to pay with
    pay_with: "Pay with",

    // ---- token page
    /// Sell preset chip text for 100 percent. Short: chip
    all_amount: "All",
    /// Why the order button is off; symbol is a coin ticker
    no_coin_to_sell: "No {symbol} to sell",
    /// Under the amount when selling with no amount picked; symbol is a coin ticker
    of_your: "of your {symbol}",
    /// Under the amount: the coin amount and its value; coin like "5 SUI", usd a money figure
    under_coin_usd: "{coin}, about {usd}",
    /// Under the amount: its value; usd is a money figure
    about_usd: "about {usd}",
    /// While the quote loads
    getting_quote: "Getting a quote",
    /// Balance line when no wallet is connected
    no_wallet: "No wallet",
    /// Buy button with no coin chosen yet
    review_buy: "Review buy",
    /// Buy button; symbol is a coin ticker
    review_buy_coin: "Review buy {symbol}",
    /// Sell button with no coin chosen yet
    review_sell: "Review sell",
    /// Sell button; symbol is a coin ticker
    review_sell_coin: "Review sell {symbol}",
    /// The coin is not in the market data
    coin_not_indexed: "This coin is not in the market data yet.",
    /// The coin could not be loaded
    coin_load_failed: "Could not load this coin.",
    /// The coin is not available
    coin_unavailable: "This coin is not available right now.",
    /// Short label next to a price change: over the last 24 hours. Short
    h24: "24h",
    /// Button: set a price alert for the coin. Short: button
    price_alert: "Price alert",
    /// Stat label
    market_cap: "Market cap",
    /// Stat label: traded volume in 24 hours
    volume_24h: "24h volume",
    /// Stat label
    liquidity: "Liquidity",
    /// Stat label and tab: the wallets that hold a coin
    holders: "Holders",
    /// Stat label: buys and sells in 24 hours
    buys_sells_24h: "Buys / sells 24h",
    /// Stat label and column header: how old a coin or trade is
    age_label: "Age",
    /// Tab: recent trades of a coin
    trades: "Trades",
    /// Column header: buy or sell
    side: "Side",
    /// Column header
    amount: "Amount",
    /// Column header: the money value
    value: "Value",
    /// Column header: the trader
    trader: "Trader",
    /// Trades table when loading failed
    trades_load_failed: "Couldn't load trades.",
    /// Holders table when loading failed
    holders_load_failed: "Couldn't load holders.",
    /// Holders table when there are none
    no_holders: "No holders seen yet.",
    /// Holders column header for one holder
    holder: "Holder",
    /// Holders column header: share of the supply
    share: "Share",
    /// Holders table tag for a liquidity pool
    pool: "Pool",
    /// Review line label: the player's position
    position: "Position",
    /// Review line label: the player's balance
    balance: "Balance",

    // ---- home
    /// Section label, tab and panel title: coins (tokens)
    coins: "Coins",
    /// Section label
    collections: "Collections",
    /// Section label and tab
    perps: "Perps",
    /// Section label and tab
    traders: "Traders",
    /// Summary label above the wallet value
    portfolio: "Portfolio",
    /// Next to today's profit; lowercase, follows a figure
    today_lower: "today",
    /// Portfolio value could not load
    unavailable: "Unavailable",
    /// Wallet label line; who is the account name
    who_watch_only: "{who}, watch only",
    /// Title when no wallet is connected
    trade_any_coin: "Trade any coin on Sui",
    /// Body when no wallet is connected
    add_account_to_trade: "Add a Rare Wallet account to buy, sell and open perps from here.",
    /// Action button: open the positions list. Short: button
    positions: "Positions",
    /// Action button: open the wallet. Short: button
    open_wallet: "Open Wallet",
    /// Rail title
    trending: "Trending",
    /// Trending failed to load
    trending_failed: "Could not load trending coins. This page refreshes on its own.",
    /// Panel title
    top_movers: "Top movers",
    /// Top movers failed to load
    movers_failed: "Could not load movers.",
    /// Panel title
    new_launches: "New launches",
    /// New launches failed to load
    new_launches_failed: "Could not load new launches.",
    /// Last tile of the perps rail
    all_markets: "All markets",
    /// Perps rail subtitle
    perps_tagline: "Long or short with leverage",
    /// Perps failed to load
    perps_unavailable: "Perps markets are unavailable right now.",
    /// Perps failed to load, table
    perps_unavailable_refresh: "Perps markets are unavailable right now. This page refreshes on its own.",
    /// Stat label: money in the perps account
    perps_balance: "Perps balance",
    /// Stat label: margin free to use
    available: "Available",
    /// Stat label: profit or loss on open positions
    unrealized_pnl: "Unrealized P/L",
    /// Stat label: number of open positions
    open_positions: "Open positions",
    /// Column header
    volume: "Volume",
    /// Column header: total open contracts
    open_interest: "Open interest",
    /// Column header and stat: funding rate per hour
    funding_1h: "Funding 1h",
    /// Positions page empty state title
    no_wallet_yet: "No wallet yet",
    /// Positions page empty state body
    no_wallet_yet_body: "Add a Rare Wallet account on the Wallet tab to see your coins and positions here.",
    /// Stat label: today's change
    today: "Today",
    /// Tag next to the title of a watch-only account. Short
    watch_only: "Watch only",
    /// Panel title
    perps_positions: "Perps positions",
    /// Column header
    size: "Size",
    /// Column header and stat
    entry: "Entry",
    /// Column header: the current price of a market
    mark: "Mark",
    /// Column header and stat: the price where a position closes by force
    liquidation: "Liquidation",
    /// Column header and stat: profit or loss. Short
    pnl: "P/L",
    /// No open positions
    no_perps_positions: "No open perps positions.",
    /// Positions failed to load
    perps_positions_failed: "Could not load perps positions.",
    /// Column header: a coin
    coin: "Coin",
    /// No coins in the wallet
    no_coins: "No coins in this wallet yet.",
    /// Wallet coins failed to load
    coins_failed: "Could not load this wallet's coins.",
    /// Review sheet subtitle for closing a position
    market_order_whole: "Market order for the whole position",
    /// Review line: profit or loss if the position closed now
    pnl_if_closed: "P/L if closed now",
    /// Review line: the margin the player gets back
    margin_back: "Margin back",

    // ---- perp market page
    /// Perps market page: error
    market_unavailable: "This market is unavailable right now.",
    /// Under a perps market's ticker; pair is like BTC/USD
    perp_pair: "{pair} perp",
    /// Stat label: when the next funding is paid
    next_funding: "Next funding",
    /// Stat label
    max_leverage: "Max leverage",
    /// Stat and review line: trading fees
    fees: "Fees",
    /// Panel title
    your_position: "Your position",
    /// No open position in this market
    no_open_position: "No open position in this market.",
    /// No wallet to trade perps
    add_wallet_to_trade_perps: "Add a Rare Wallet account to trade perps.",
    /// Order line label: the perps account
    perps_account: "Perps account",
    /// Order line label: where more margin comes from
    from_your_wallet: "From your wallet",
    /// Order line value: the account does not exist yet
    set_up_first: "Set up first",
    /// Order line value: nothing is added
    nothing: "Nothing",
    /// Order line label: the player's USDC balance
    wallet_usdc: "Wallet USDC",
    /// Label above the margin amount
    margin: "Margin",
    /// Label above the leverage stepper
    leverage: "Leverage",
    /// Side of a position or the order switch. Short: switch
    long: "Long",
    /// Side of a position or the order switch. Short: switch
    short: "Short",
    /// The long side inside a sentence ("Open long")
    long_lower: "long",
    /// The short side inside a sentence ("Open short")
    short_lower: "short",
    /// Why the order is blocked
    one_time_setup: "One time, before your first order",
    /// Why the order is blocked
    not_enough_usdc: "Not enough USDC in your wallet",
    /// Why the order is blocked; amount is a money figure
    minimum_order: "Minimum order {amount}",
    /// Order button and review title: create the perps account first
    set_up_perps: "Set up perps",
    /// Order button; side is long or short, leverage like 5x
    review_side_lev: "Review {side} {leverage}",
    /// Review button; side is long or short
    open_side: "Open {side}",
    /// Review sheet button
    create_account: "Create account",
    /// Review line label (account setup)
    what_it_does: "What it does",
    /// Review line value
    creates_account_note: "Creates your perps account for USDC margin",
    /// Review line label
    after_that: "After that",
    /// Review line value
    review_again_note: "Review your order again to open it",
    /// Review line label: estimated entry price
    entry_about: "Entry about",
    /// Review line label: extra funds moved from the wallet to the perps account
    adds_from_wallet: "Adds from your wallet",
    /// Review sheet subtitle
    market_order: "Market order",

    // ---- traders board and trader page
    /// Traders board: sort and column header
    realized_pnl: "Realized P/L",
    /// Traders board: sort and column header
    win_rate: "Win rate",
    /// Traders board note; trades is a count, volume a number of thousands of dollars
    board_note: "At least {trades} trades and ${volume}K volume. Wash and bot-like wallets left out.",
    /// Traders board failed to load
    traders_unavailable: "Top traders are unavailable right now. This page refreshes on its own.",
    /// Traders board empty
    no_traders: "No trader passes the activity minimum yet.",
    /// Following panel empty
    follow_hint: "Follow a trader to get their trades as they happen.",
    /// Trader labels
    smart_money: "Smart money",
    /// Trader labels
    whale: "Whale",
    /// Trader labels: key opinion leader
    kol: "KOL",
    /// Trader page label
    trades_like_bot: "Trades like a bot",
    /// Stat label
    unrealized: "Unrealized",
    /// Stat label and panel title
    holdings: "Holdings",
    /// Small tag while a live feed runs. Short
    live: "Live",
    /// Panel title
    latest_trades: "Latest trades",
    /// Trader's trades failed to load
    trades_failed_refresh: "Could not load trades. This page refreshes on its own.",
    /// No trades
    no_trades: "No trades yet.",
    /// Panel title
    mirror_size: "Mirror size",
    /// Panel text; "A" is the A button
    mirror_explain: "A on a trade builds the same swap at your size. You review it before anything is signed.",
    /// Label above the buy size chips
    each_buy: "Each buy, in SUI",
    /// Label above the sell size chips
    each_sell: "Each sell, of what you hold",
    /// Holdings empty
    holds_nothing: "Holds nothing priced right now.",
    /// Holdings failed
    holdings_unavailable: "Holdings are unavailable right now.",
    /// Review title; symbol is a coin ticker
    mirror_title_buy: "Mirror buy of {symbol}",
    /// Review title; symbol is a coin ticker
    mirror_title_sell: "Mirror sell of {symbol}",
    /// Review subtitle; name is a trader, usd a money figure, size the player's size
    mirror_sub_buy: "{name} bought {usd}. Your size: {size}.",
    /// Review subtitle; name is a trader, usd a money figure, size the player's size
    mirror_sub_sell: "{name} sold {usd}. Your size: {size}.",
    /// The player's buy size; amount is a number
    mirror_size_sui: "{amount} SUI",
    /// The player's sell size; pct a number, symbol a coin ticker
    mirror_size_pct: "{pct}% of your {symbol}",
    /// Review line value; usd is a money figure, symbol a coin ticker
    bought_of: "Bought {usd} of {symbol}",
    /// Review line value; usd is a money figure, symbol a coin ticker
    sold_of: "Sold {usd} of {symbol}",
    /// Mirror plan message
    mirror_choose_size: "Choose a size first.",
    /// Mirror plan message
    mirror_no_sui_price: "No price for SUI right now.",
    /// Mirror plan message
    mirror_checking_balance: "Checking your balance.",
    /// Mirror plan message
    mirror_hold_none: "You hold none of this coin.",
    /// Perps order sheet: the price preview could not load
    price_order_failed: "Couldn't price this order right now.",
    /// Toast: the price alert needs a price
    alert_needs_price: "Price alerts need a price for this coin.",
}
