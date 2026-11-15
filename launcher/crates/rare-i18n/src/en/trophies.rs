strings! {
    /// Trophy tier name (lowest)
    bronze: "Bronze",
    /// Trophy tier name
    silver: "Silver",
    /// Trophy tier name
    gold: "Gold",
    /// Trophy tier name (highest)
    platinum: "Platinum",
    /// Where a minted trophy came from: the game's studio issued it
    studio_issued: "Studio issued",
    /// Where a minted trophy came from: Rare attested the player's own local data
    self_reported: "Self-reported",
    /// Source name for achievements earned in emulated retro games
    retro_trophies: "Retro trophies",
    /// Tier line: points after the tier name; n is a number
    dot_points: "\u{b7} {n} points",
    /// Game card note; n is a number of trophies ready to claim
    n_to_claim: "{n} to claim",
    /// Game card note; n is a number of minted trophies
    n_minted: "{n} minted",
    /// A number of Rare score points; n is a formatted number
    points_n: "{n} points",
    /// Game card note; date is a short date like "Mar 4"
    last_on: "Last on {date}",
    /// Game card line; source is e.g. Steam, unlocked and total are numbers
    card_line: "{source} \u{b7} {unlocked} of {total}",
    /// Tier count on the profile; n is a number, tier is a tier name (e.g. "3 Gold")
    count_tier: "{n} {tier}",
    /// Hint on the profile trophies card (opens the full trophy case)
    trophy_case: "Trophy case",
    /// Player level; n is a number
    level_n: "Level {n}",
    /// Profile trophies note; unlocked is a number of trophies, n a number of games
    unlocked_in_games: "{unlocked} unlocked in {n} game|{unlocked} unlocked in {n} games",
    /// Under the level bar; points is a number, level the next level
    to_level: "{points} to level {level}",
    /// Hint on a showcase trophy card
    open_game: "Open the game",
    /// Sort order of games in the case (short)
    sort_recent: "Recent",
    /// Sort order and stat label (short)
    completion: "Completion",
    /// Sort order (by game name, short)
    name: "Name",
    /// Filter and status (short)
    unlocked: "Unlocked",
    /// Filter and status (short)
    locked: "Locked",
    /// Controller hint for X: show every trophy
    show_all: "Show all",
    /// Controller hint for X
    show_unlocked: "Show unlocked",
    /// Controller hint for X
    show_locked: "Show locked",
    /// Controller hint for X: order games by recent play
    sort_by_recent: "Sort by recent",
    /// Controller hint for X
    sort_by_completion: "Sort by completion",
    /// Controller hint for X
    sort_by_name: "Sort by name",
    /// Empty state title for a game with no achievements
    no_achievements_title: "No achievements for this game here",
    /// Empty state body
    no_achievements_body: "Steam keeps achievements on this device once the game has run here at least once.",
    /// Case page title when not signed in
    your_trophies: "Your trophies",
    /// Stat label: the player's Rare score
    rare_score: "Rare score",
    /// "{a} of {b}", e.g. unlocked of total
    x_of_y: "{a} of {b}",
    /// Stat label: points that do not count toward the Rare score yet
    not_counted: "Not counted",
    /// Stat label and status: trophies recorded on chain
    minted: "Minted",
    /// Note under the case header when rare.market said why this device does not count; reason is a sentence
    case_note_device: "Every unlock shows here. The Rare score counts verified ones, and this device's aren't verified yet: {reason}",
    /// Note under the case header (checked); TPM is a security chip
    case_note_checked: "Every unlock shows here. The Rare score counts verified ones: unlocks on a SuiPlay that proves it runs Rare OS (its TPM) and match your play sessions, and retro trophies in hardcore.",
    /// Note under the case header (not checked yet)
    case_note_unchecked: "Every unlock shows here. The Rare score counts verified ones once rare.market has checked them (turn on friends to send them).",
    /// Showcase slot with nothing pinned
    empty_slot: "Empty slot",
    /// Showcase slot hint
    empty_slot_hint: "Press A on an unlocked trophy to show it here",
    /// Section title: trophies the player shows
    showcase: "Showcase",
    /// Section note
    showcase_note: "On your profile and rare.market",
    /// Section note; n is a number of games
    n_with_trophies: "{n} with trophies",
    /// Empty state title
    no_trophies_title: "No trophies yet",
    /// Empty state body
    no_trophies_body: "Play any game and Rare's trophies start with the first launch. Steam, RetroAchievements, Epic and GOG achievements show up here too.",
    /// Stat label
    players_on_rare: "Players on Rare",
    /// Button while claiming
    claiming: "Claiming",
    /// Button: claim trophies (put them on chain); n is a number
    claim_n: "Claim {n} trophy|Claim {n} trophies",
    /// Controller hint and hint for a claimable trophy (verb, short)
    claim: "Claim",
    /// Button when the game is marked complete
    marked_complete: "Marked complete",
    /// Button and hint: mark the game complete
    mark_complete: "Mark complete",
    /// Hint: undo marking the game complete
    not_complete: "Not complete",
    /// Game page label; source is e.g. Steam
    source_hardcore: "{source} \u{b7} Hardcore",
    /// Game page label
    steam_library_source: "Steam · from your Steam library",
    /// Table header (short)
    col_trophy: "Trophy",
    /// Table header (short)
    col_tier: "Tier",
    /// Table header: how rare the trophy is (short)
    col_rarity: "Rarity",
    /// Table header (short)
    col_status: "Status",
    /// Empty text while loading
    loading_achievements: "Loading the achievements.",
    /// Empty text in the Locked filter
    nothing_left: "Nothing left to unlock.",
    /// Empty text in the Unlocked filter
    nothing_unlocked: "Nothing unlocked yet.",
    /// Section title; source is e.g. Steam
    source_achievements: "{source} achievements",
    /// Section note; n is a number of rows shown
    n_shown: "{n} shown",
    /// Section title for Rare's own trophies
    rare_trophies: "Rare trophies",
    /// Tooltip on the check mark
    verified_suiplay: "Verified on SuiPlay",
    /// Minted row line; source is Studio issued or Self-reported, number is the trophy number
    source_number: "{source} \u{b7} #{number}",
    /// Trophy status
    ready_to_claim: "Ready to claim",
    /// Trophy status while putting it on chain
    pinning: "Pinning",
    /// Trophy status
    on_chain: "On chain",
    /// Trophy line; date is a short date
    date_hardcore: "{date} \u{b7} Hardcore",
    /// Trophy line (earned in hardcore mode: no save states, no cheats)
    hardcore: "Hardcore",
    /// Progress of a trophy that counts days
    progress_days: "{have} of {need} days",
    /// Progress of a trophy that counts hours
    progress_hours: "{have} of {need} hours",
    /// Rarity source: percent of players on Rare
    on_rare: "on Rare",
    /// Rarity source; source is Steam or GOG
    on_source: "on {source}",
    /// Rarity source: percent of players
    of_players: "of players",
    /// Name of a hidden trophy
    hidden_trophy: "Hidden trophy",
    /// Description of a hidden trophy
    keep_playing: "Keep playing to reveal it.",
    /// Toast on a locked trophy; desc is its description
    locked_how: "Locked. {desc}",
    /// Toast on a locked streak trophy
    locked_how_days: "Locked. {desc} {have} of {need} days so far.",
    /// Toast on a locked playtime trophy
    locked_how_hours: "Locked. {desc} {have} of {need} hours so far.",
    /// Hint on a locked trophy
    how_to_unlock: "How to unlock",
    /// Tag on a trophy shown on the player's profile
    on_profile: "On profile",
    /// Pop-up title
    popup_minted: "Trophy minted",
    /// Pop-up title
    popup_trophy: "Trophy unlocked",
    /// Pop-up title
    popup_achievement: "Achievement unlocked",
    /// Pop-up second line when more unlock; meta is the game, tier and points, n is how many more
    meta_more: "{meta} and {n} more",
}
