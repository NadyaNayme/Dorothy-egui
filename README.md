[![CI](https://github.com/NadyaNayme/Dorothy-egui/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/NadyaNayme/Dorothy-egui/actions/workflows/rust.yml)

# Dorothy
A gold bar/blue box drop logger for Granblue Fantasy.

![Dorothy UI](https://github.com/NadyaNayme/Dorothy-egui/blob/master/src/images/dorothy_ui.png?)

## Latest Release

You can download the [latest release here](https://github.com/NadyaNayme/Dorothy-egui/releases/latest).

Or, if you prefer, you can use the [webapp](https://nadyanayme.github.io/Dorothy-egui/) - though currently exporting to .csv is not supported.

## How to Use
- Left click an icon or item name to add a drop.
- Shift+Left click to remove a drop.
- Left click an item in Recent Drops to remove that specific drop.

Make sure to go and customize your View and Settings to your liking - the defaults settings are optimized for feature discoverability and are not the recommended settings.

## View

View are specific settings that adjust Dorothy's UI. It allows you to show the side panels, move the right panel to the bottom (useful for mobile), and adjust which features are shown and how they look for the Center panel.

## Settings

- **Auto Update on Startup**: Dorothy will attempt to download the latest release of Dorothy and if a higher version exists will automatically download and replace the existing .exe with the new version. A restart is required after.
- **Dark Mode**: Enables Dark Mode.
- **Always On Top**: Will make the Dorothy window always stay on top of other windows (unless those windows are also set to Always On Top)
- **Reset Counts on Export**: Resets drop counts to "0" on export.
- **Calculate droprates by total kills**: Calculate drop percentages out of all chests instead of only blue chests
