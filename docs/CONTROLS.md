# Controls

In one sentence: **the Rare button always means Rare, the Quick button always means the device, and everything else belongs to whatever is on screen.**

The SuiPlay0X1's own buttons (Rare, Quick, LC, RC) reach the launcher through InputPlumber's intercept mode (build items 1 and 2 in [UX-BUILD.md](UX-BUILD.md)). Until those land on the device, the launcher's hints show the older chords. Settings > Controls shows the map and remaps LC and RC.

## Rare OS

| Control | Launcher | A game or app in front | Rare menu, Quick Settings or Switcher open | First run | Asleep |
| --- | --- | --- | --- | --- | --- |
| **Rare** press | Rare menu; again: Home | Rare menu over the game (the game gets no input); again: Home, game keeps running | In the Rare menu: Home. In Quick Settings or the Switcher: the Rare menu | Nothing | Nothing |
| **Rare** hold 0.6 s | Switcher | Switcher | Switcher | Nothing | Nothing |
| **Rare** + Y | Screenshot | Screenshot | Screenshot of what is under the panel | Nothing | Nothing |
| **Rare** + View | Save the last 30 s | Save the last 30 s | Same | Nothing | Nothing |
| **Rare** + X | On-screen keyboard | On-screen keyboard (typed into the focused window) | Nothing | Nothing | Nothing |
| **Rare** + B hold 1.5 s | Nothing | Force quit the app in front, no confirm | Nothing | Nothing | Nothing |
| **Rare** + volume | Brightness | Brightness | Brightness | Brightness | Nothing |
| **Quick** press | Quick Settings (again or B closes) | Quick Settings over the game | Swaps to Quick Settings, or closes it | Quick Settings | Nothing |
| **LC** press (default) | Screenshot | Nothing (Settings > Controls > LC in games, or the game's page; LC set to Keyboard opens the keyboard here too) | Screenshot | Nothing | Nothing |
| **LC** hold 1 s | Save the last 30 s | When LC in games is set, or LC is Keyboard | Same | Nothing | Nothing |
| **RC** press (default) | On-screen keyboard | On-screen keyboard over the game or app, typed into it (Settings > Controls > RC in games, or the game's page) | Nothing | On-screen keyboard | Nothing |
| **RC** hold 1 s | Performance overlay: next level | When RC in games is set, or RC is Keyboard (the default) | Same | Nothing | Nothing |
| **Power** press | Sleep | Sleep (game paused in memory) | Sleep | Sleep | Wake |
| **Power** hold 1 s | Power menu | Same | Same | Same | Wake |
| **Power** hold 10 s | Hardware off | Same | Same | Same | Same |
| **Volume** | Volume, Rare's bar | Same | Same | Same | Nothing |
| **A** | Select | Game | Select | Select | Nothing |
| **B** | Back | Game | Close, back to the game | Back one step | Nothing |
| **X** | The screen's extra action | Game | The panel's extra action | Step extra | Nothing |
| **Y** | Search | Game | The panel's second action | Skip setup | Nothing |
| **Menu** | Options for the focused item | Game | Nothing | Nothing | Nothing |
| **View** | The screen's view (sort, filter, list or grid, chart mode) | Game | Nothing | Nothing | Nothing |
| **LB / RB** | Top tabs | Game | Nothing | Nothing | Nothing |
| **LT / RT** | Sections of the screen | Game | The panel's tabs | Nothing | Nothing |
| **D-pad, left stick** | Move focus | Game | Move focus | Move focus | Nothing |
| **Right stick** | Scroll, chart pan and zoom | Game | Scroll | Nothing | Nothing |
| **L3, R3** | Nothing | Game | Nothing | Nothing | Nothing |
| **Touch** | Tap focuses and selects; a game never starts from a touch | Game | Tap selects, tap outside closes | Tap selects | Nothing |
| **View + Menu** hold 0.6 s | Switcher (silent rescue) | Switcher | Switcher | Nothing | Nothing |

- **Remapping.** LC and RC presses can be set to Capture, Keyboard, Performance overlay, Key shortcut or Nothing in Settings > Controls (`lcAction`, `rcAction` in the launcher config). Their holds, Rare and Quick are fixed. The SuiPlay has no back paddles. In games, LC and RC do nothing until picked (`lcGameAction`, `rcGameAction`), except that a button set to Keyboard in the launcher is the keyboard in games too.
- **Key shortcut.** LC or RC set to Key shortcut holds a key combination down for exactly as long as the button is held (key down on press, key up on release, no tap or hold action), so push-to-talk apps such as dictation tools can use it. The keys are picked in a sheet (Settings > Controls > LC / RC key shortcut > Change, or by picking Key shortcut on a row): pressed on any keyboard the launcher's window gets keys from, or picked with the pad (Ctrl, Alt, Shift, Super, then one key, or modifiers alone). Saved as `lcShortcut` / `rcShortcut` (`ctrl+alt+space`, `rare_input::shortcut`). The keys go out on `Rare shortcut keyboard` on /dev/uinput (`rare_launch::apps::pointer::hold_keys`) to the window with keyboard focus: in Rare's session, over games, and in Desktop mode.
- **The keyboard over a game or app.** Rare + X, a Keyboard press of LC or RC, Quick Settings > Keyboard, or Y in an app the pad drives as a mouse. It is drawn in Rare's HUD window, which gamescope paints on the overlay plane like Steam's own keyboard, so the game or app keeps focus, and each key goes out at once on Rare's virtual keyboard (`Rare keyboard` on /dev/uinput, US layout) into whatever has focus: Steam's sign-in and Steam Guard, a game's name field, Chrome. While it is up InputPlumber intercepts the pad, so the app gets no pad input. D-pad moves, A types, X deletes, Y is Shift, Menu is Enter, View is Tab, LB and RB move the cursor, B closes; Rare and Quick close it and do what they always do. LT switches to the symbols (`?123`, then Y for `#+=`; every ASCII symbol is on one of the two pages) and back. A held X keeps deleting and a held A keeps typing the key (420 ms, then 14 a second). In a web address field (the Browser's address bar or a new tab) `.` `/` `:` `-` sit on the letters and there is a `.com` key; email fields get `@` and `.com`. The launcher's own keyboard and the one in web games and Rare's web window have the same pages and keys (`features/kblayout.rs`, checked against `webgame.js` by a test), and a finger held on a key repeats it there too.
- **Intercept.** While a game or app has the screen, InputPlumber's intercept mode is `Pass`: everything goes to the game except Rare and Quick, which go only to Rare over D-Bus. While a Rare surface is open it is `All`, so the game sees nothing. The launcher in front is `Pass`.
- **Stick lights.** Settings > Lighting: Default (the controller's own lights, untouched, what a device shows until someone changes it), On or Off. On has Solid, Breathe (4 s a breath, never fully dark) and Rainbow (each ring turns through the hues, 8 s a turn), a color (nine swatches and a hue slider; Both, Left or Right once `rare-lights-test` has found the two rings apart on the device, DEVICE-PROOF 23a) and brightness. Changes show on the rings as they are made. rare-fand draws them (`org.rare.Lights`, saved in `/var/lib/rare/lights`, so they come back after a restart and in Desktop mode); the driver is `ayaneo_ec_suiplay` (`/sys/class/leds/ayaneo:rgb:joystick_rings`, `rgb_mode`, `rgb_zones`). The rings go dark while the device sleeps and come back on wake; at power off and when rare-fand stops the controller gets them back.
- **Sticks, gyro, rumble.** Deadzones, vertical invert, gyro on or off with its sensitivity, "Gyro in Steam games" and rumble strength are in Settings > Controls. The gyro is read through InputPlumber; Steam games see the pad as a Steam Deck while they run.

## Desktop mode

KDE Plasma has no idea what a gamepad is, so Rare runs its own controller there: `rare-launcher-native --desktop-controls` (`launcher/native/src/features/desktop.rs`), started by Plasma's autostart (`/etc/xdg/autostart/rare-desktop-controls.desktop`, `/usr/libexec/rare/desktop-controls`, log in `~/.local/state/rare/desktop.log`). It puts InputPlumber back to the Xbox pad and intercept mode `Pass` (Rare's session can leave either), listens to Rare, Quick, LC and RC over D-Bus, and reads every pad.

| Control | Desktop controls (default) | Gamepad (a full screen window or a Steam game in front) |
| --- | --- | --- |
| **Right stick** | Pointer | Game |
| **Left stick** | Scroll (both ways) | Game |
| **A / RT** | Left click (held: drag) | Game |
| **B / LT** | Right click | Game |
| **X** | Space | Game |
| **Y** | Plasma's on-screen keyboard, up or down | Game |
| **D-pad** | Arrow keys | Game |
| **LB / RB** | Shift+Tab / Tab | Game |
| **Menu / View** | Enter / Esc | Game |
| **R3** | Middle click | Game |
| **Rare** press | Plasma's app menu | Same |
| **Quick** press | Switch to gamepad, with an on-screen note | Switch to desktop controls |
| **LC / RC** | As Settings > Controls > LC / RC: Keyboard toggles Plasma's on-screen keyboard, Capture takes a screenshot (Spectacle), Key shortcut holds its keys | As LC / RC in games |

- **Which mode.** A KWin script the controller loads (`market.rare.Desktop.Window`) reports the active window: full screen, or a Steam game window (`steam_app_*`), gets the controller as a normal gamepad (the grab is let go); everything else gets desktop controls (every pad is grabbed with EVIOCGRAB, so no app sees it as a gamepad, and it drives `Rare desktop controller` on /dev/uinput). Quick switches by hand until another window comes to the front.
- **Return to Rare** is the desktop's Return to Rare, as before.
- **On-screen keyboard.** Plasma's own (KWin's virtual keyboard). It types into Wayland apps; apps on Xwayland that don't take input methods don't get its text.

## Shoulder buttons in the launcher

| Button | Always |
| --- | --- |
| **LB / RB** | The five tabs (Home, Library, Sui, Market, Wallet), wrapping. A page or sheet over the tab closes first. Only first run and sign-in hold them. |
| **LT / RT** | The current screen's sections: Library filters, Sui Featured / All games / Store / Coming soon, Market Collections / Coins / Perps / Traders, a collection's Items / Activity / Holders, a coin's Trades / Holders, Wallet Coins / Collectibles / Activity, the Rare menu's tabs. Without sections they jump through lists. |

**Where LT / RT mean something else.** Only while that panel or mode has focus: the trade order panel (less or more amount or leverage), chart mode (shorter or longer timeframe), the price alert sheet (lower or higher). Modal sheets (a purchase, a wallet approval) hold the shoulder buttons until they close with B.

A starts a game or app only on a deliberate press (or a click), never from a touch, a held button or focus moving.

## Guest mode

When Rare runs as a shortcut inside Steam Game Mode, Steam owns Guide and QAM and Rare cannot intercept them.

| Control | Does |
| --- | --- |
| Guide, QAM | Steam's |
| Hold View 0.5 s | Rare menu over a game or app Rare started |
| Hold View + Menu 0.6 s | Switcher |
| Everything else | As Rare OS in the launcher; games get everything |

## Hint bar

Hints name the SuiPlay's own buttons with device glyphs (Rare, Quick, LC, RC), never "Hold Select + Start". Every screen shows LB RB Tabs, and LT RT Sections when it has sections.

In games every button follows one map, per console too: [EMULATION.md](EMULATION.md#the-standard-in-game-button-map).

Trade's layout: [CONTROLS-TRADE.md](CONTROLS-TRADE.md). The Rare menu: [OVERLAY.md](OVERLAY.md).
