pub const W_NEW_SOUND_ID: u16 = 0xc0ee;

pub const W_AUDIO_ROM_BANK: u16 = 0xc0ef;

pub const W_TILE_MAP: u16 = 0xc3a0;

pub const W_TOP_MENU_ITEM_Y: u16 = 0xcc24;
pub const W_TOP_MENU_ITEM_X: u16 = 0xcc25;

/// the id of the currently selected menu item \
/// the top item has id 0, the one below that has id 1, etc. \
/// note that the "top item" means the top item currently visible on the screen,
/// add this value to [wListScrollOffset] to get the item's position within the list
pub const W_CURRENT_MENU_ITEM: u16 = 0xcc26;

/// the tile that was behind the menu cursor's current location
pub const W_TILE_BEHIND_CURSOR: u16 = 0xcc27;

/// id of the bottom menu item
pub const W_MAX_MENU_ITEM: u16 = 0xcc28;

/// bit mask of keys that the menu will respond to
pub const W_MENU_WATCHED_KEYS: u16 = 0xcc29;

/// id of previously selected menu item
pub const W_LAST_MENU_ITEM: u16 = 0xcc2a;

/// It is mainly used by the party menu to remember the cursor position while the
/// menu isn't active.
/// It is also used to remember the cursor position of mon lists (for the
/// withdraw/deposit/release actions) in Bill's PC so that it doesn't get lost
/// when you choose a mon from the list and a sub-menu is shown. It's reset when
/// you return to the main Bill's PC menu.
pub const W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM: u16 = 0xcc2b;

/// how many times should HandleMenuInput poll the joypad state before it returns?
pub const W_MENU_JOYPAD_POLL_COUNT: u16 = 0xcc34;

/// if running on SGB or CGB, it's 1, else it's 0
pub const W_ON_SGB: u16 = 0xcf1a;

/// the map you will start at when the debug bit is set
pub const W_DEFAULT_MAP: u16 = 0xd07b;

/// 1 = no save file or save file is corrupted
/// 2 = save file exists and no corruption has been detected
pub const W_SAVE_FILE_STATUS: u16 = 0xd087;

pub const W_OPTIONS_INITIALIZED: u16 = 0xd089;

/// not exactly sure what this is used for, but it seems to be used as a multipurpose temp flag value
pub const W_CURRENT_MAP_SCRIPT_FLAGS: u16 = 0xd125;

pub const W_LINK_STATE: u16 = 0xd12a;

pub const W_POKEDEX_OWNED: u16 = 0xd2f6;
pub const W_POKEDEX_OWNED_END: u16 = 0xd309;

/// bit 7 = battle animation
///   0: On
///   1: Off
/// bit 6 = battle style
///   0: Shift
///   1: Set
/// bits 0-3 = text speed (number of frames to delay after printing a letter)
///   1: Fast
///   3: Medium
///   5: Slow
pub const W_OPTIONS: u16 = 0xd354;

/// bit 0: Boulder \
/// bit 1: Cascade \
/// bit 2: Thunder \
/// bit 3: Rainbow \
/// bit 4: Soul \
/// bit 5: Marsh \
/// bit 6: Volcano \
/// bit 7: Earth
pub const W_OBTAINED_BADGES: u16 = 0xd355;

/// bit 0: If 0, limit the delay to 1 frame. Note that this has no effect if
///        the delay has been disabled entirely through bit 1 of this variable
///        or bit 6 of wd730.
/// bit 1: If 0, no delay.
pub const W_LETTER_PRINTING_DELAY_FLAGS: u16 = 0xd357;

pub const W_PRINTER_SETTINGS: u16 = 0xd497;

/// bit 0: the player has received Lapras in the Silph Co. building
/// bit 1: set in various places, but doesn't appear to have an effect
/// bit 2: the player has healed pokemon at a pokemon center at least once
/// bit 3: the player has a received a pokemon from Prof. Oak
/// bit 4: disable battles
/// bit 5: set when a battle ends and when the player blacks out in the overworld due to poison
/// bit 6: using the link feature
/// bit 7: set if scripted NPC movement has been initialised
pub const W_D72E: u16 = 0xd72d;

/// bit 0: NPC sprite being moved by script \
/// bit 5: ignore joypad input \
/// bit 6: print text with no delay between each letter \
/// bit 7: set if joypad states are being simulated in the overworld or an NPC's movement is being scripted
pub const W_D730: u16 = 0xd730;

pub const W_PLAY_TIME_HOURS: u16 = 0xda40;
pub const W_PLAY_TIME_MAXED: u16 = 0xda41;
pub const W_PLAY_TIME_MINUTES: u16 = 0xda42;
pub const W_PLAY_TIME_SECONDS: u16 = 0xda43;
pub const W_PLAY_TIME_FRAMES: u16 = 0xda44;
