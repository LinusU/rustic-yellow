pub const W_NEW_SOUND_ID: u16 = 0xc0ee;

pub const W_AUDIO_ROM_BANK: u16 = 0xc0ef;
pub const W_AUDIO_SAVED_ROM_BANK: u16 = 0xc0f0;

pub const W_FREQUENCY_MODIFIER: u16 = 0xc0f1;
pub const W_TEMPO_MODIFIER: u16 = 0xc0f2;

pub const W_SPRITE_DATA_START: u16 = 0xc100;
pub const W_SPRITE01_STATE_DATA1: u16 = 0xc110;
pub const W_SPRITE01_STATE_DATA1_IMAGE_INDEX: u16 = 0xc112;
pub const W_SPRITE_PLAYER_STATE_DATA1_FACING_DIRECTION: u16 = 0xc109;
pub const W_SPRITE01_STATE_DATA2: u16 = 0xc210;
pub const W_SPRITE_DATA_END: u16 = 0xc300;

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
/// menu isn't active. \
/// It is also used to remember the cursor position of mon lists (for the
/// withdraw/deposit/release actions) in Bill's PC so that it doesn't get lost
/// when you choose a mon from the list and a sub-menu is shown. It's reset when
/// you return to the main Bill's PC menu.
pub const W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM: u16 = 0xcc2b;

/// It is used by the bag list to remember the cursor position while the menu
/// isn't active.
pub const W_BAG_SAVED_MENU_ITEM: u16 = 0xcc2c;

/// It is used by the start menu to remember the cursor position while the menu
/// isn't active. \
/// The battle menu uses it so that the cursor position doesn't get lost when
/// a sub-menu is shown. It's reset at the start of each battle.
pub const W_BATTLE_AND_START_SAVED_MENU_ITEM: u16 = 0xcc2d;

pub const W_PLAYER_MOVE_LIST_INDEX: u16 = 0xcc2e;

/// how many times should HandleMenuInput poll the joypad state before it returns?
pub const W_MENU_JOYPAD_POLL_COUNT: u16 = 0xcc34;

/// if non-zero, skip waiting for a button press after displaying text in DisplayTextID
pub const W_DO_NOT_WAIT_FOR_BUTTON_PRESS_AFTER_DISPLAYING_TEXT: u16 = 0xcc3c;

pub const W_PREDEF_HL: u16 = 0xcc4f;
pub const W_PREDEF_DE: u16 = 0xcc51;
pub const W_PREDEF_BC: u16 = 0xcc53;

/// 0 = player's party \
/// 1 = enemy party \
/// 2 = current box \
/// 3 = daycare \
/// 4 = in-battle mon
///
/// AddPartyMon uses it slightly differently.
/// If the lower nybble is 0, the mon is added to the player's party, else the enemy's.
/// If the entire value is 0, then the player is allowed to name the mon.
pub const W_MON_DATA_LOCATION: u16 = 0xcc49;

/// which NPC movement script pointer is being used \
/// 0 if an NPC movement script is not running
pub const W_NPC_MOVEMENT_SCRIPT_POINTER_TABLE_NUM: u16 = 0xcc57;

pub const W_PARENT_MENU_ITEM: u16 = 0xccd3;

/// if [wAILayer2Encouragement] != 1, the second AI layer is not applied
pub const W_AI_LAYER2_ENCOURAGEMENT: u16 = 0xccd5;

/// current HP of player and enemy substitutes
pub const W_PLAYER_SUBSTITUTE_HP: u16 = 0xccd7;
pub const W_ENEMY_SUBSTITUTE_HP: u16 = 0xccd8;

/// The player's selected move during a test battle. \
/// InitBattleVariables sets it to the move Pound.
pub const W_TEST_BATTLE_PLAYER_SELECTED_MOVE: u16 = 0xccd9;

/// 0=regular, 1=mimic, 2=above message box (relearn, heal pp..)
pub const W_MOVE_MENU_TYPE: u16 = 0xccdb;

pub const W_PLAYER_SELECTED_MOVE: u16 = 0xccdc;
pub const W_ENEMY_SELECTED_MOVE: u16 = 0xccdd;

pub const W_LINK_BATTLE_RANDOM_NUMBER_LIST_INDEX: u16 = 0xccde;

// number of times remaining that AI action can occur
pub const W_AI_COUNT: u16 = 0xccdf;

pub const W_ENEMY_MOVE_LIST_INDEX: u16 = 0xcce2;

/// The enemy mon's HP when it was switched in or when the current player mon
/// was switched in, which was more recent.
///
/// It's used to determine the message to print when switching out the player mon.
pub const W_LAST_SWITCH_IN_ENEMY_MON_HP: u16 = 0xcce3;

pub const W_SAFARI_ESCAPE_FACTOR: u16 = 0xcce8;
pub const W_SAFARI_BAIT_FACTOR: u16 = 0xcce9;

pub const W_TRANSFORMED_ENEMY_MON_ORIGINAL_DVS: u16 = 0xcceb;

pub const W_MON_IS_DISOBEDIENT: u16 = 0xcced;

pub const W_PLAYER_DISABLED_MOVE_NUMBER: u16 = 0xccee;
pub const W_ENEMY_DISABLED_MOVE_NUMBER: u16 = 0xccef;

/// When running in the scope of HandlePlayerMonFainted, it equals 1. \
/// When running in the scope of HandleEnemyMonFainted, it equals 0.
pub const W_IN_HANDLE_PLAYER_MON_FAINTED: u16 = 0xccf0;

pub const W_PLAYER_USED_MOVE: u16 = 0xccf1;
pub const W_ENEMY_USED_MOVE: u16 = 0xccf2;

pub const W_ENEMY_MON_MINIMIZED: u16 = 0xccf3;

pub const W_MOVE_DIDNT_MISS: u16 = 0xccf4;

/// Flags that indicate which party members have fought the current enemy mon
pub const W_PARTY_FOUGHT_CURRENT_ENEMY_FLAGS: u16 = 0xccf5;

pub const W_LOW_HEALTH_ALARM_DISABLED: u16 = 0xccf6;

pub const W_PLAYER_MON_MINIMIZED: u16 = 0xccf7;

/// Number of hits by enemy in attacks like Double Slap, etc.
pub const W_ENEMY_NUM_HITS: u16 = 0xcd05;

/// Set buttons are ignored.
pub const W_JOY_IGNORE: u16 = 0xcd6b;

/// Size of downscaled mon pic used in pokeball entering/exiting animation \
/// $00 = 5×5 \
/// $01 = 3×3
pub const W_DOWNSCALED_MON_SIZE: u16 = 0xcd6c;

pub const W_CD6D: u16 = 0xcd6d;

/// if running on SGB or CGB, it's 1, else it's 0
pub const W_ON_SGB: u16 = 0xcf1a;

pub const W_CF91: u16 = 0xcf90;

/// This is used to determine whether the default music is already playing when
/// attempting to play the default music (in order to avoid restarting the same
/// music) and whether the music has already been stopped when attempting to
/// fade out the current music (so that the new music can be begin immediately
/// instead of waiting).
///
/// It sometimes contains the sound ID of the last music played, but it may also
/// contain $ff (if the music has been stopped) or 0 (because some routines zero
/// it in order to prevent assumptions from being made about the current state of
/// the music).
pub const W_LAST_MUSIC_SOUND_ID: u16 = 0xcfc9;

pub const W_ENEMY_MOVE_NUM: u16 = 0xcfcb;
pub const W_ENEMY_MOVE_EFFECT: u16 = 0xcfcc;
pub const W_ENEMY_MOVE_POWER: u16 = 0xcfcd;
pub const W_ENEMY_MOVE_TYPE: u16 = 0xcfce;
pub const W_ENEMY_MOVE_ACCURACY: u16 = 0xcfcf;
pub const W_ENEMY_MOVE_MAX_PP: u16 = 0xcfd0;
pub const W_PLAYER_MOVE_NUM: u16 = 0xcfd1;
pub const W_PLAYER_MOVE_EFFECT: u16 = 0xcfd2;
pub const W_PLAYER_MOVE_POWER: u16 = 0xcfd3;
pub const W_PLAYER_MOVE_TYPE: u16 = 0xcfd4;
pub const W_PLAYER_MOVE_ACCURACY: u16 = 0xcfd5;
pub const W_PLAYER_MOVE_MAX_PP: u16 = 0xcfd6;

pub const W_ENEMY_MON_SPECIES2: u16 = 0xcfd7;
pub const W_BATTLE_MON_SPECIES2: u16 = 0xcfd8;

pub const W_ENEMY_MON_NICK: u16 = 0xcfd9;

pub const W_ENEMY_MON_PARTY_POS: u16 = 0xcfe7;

pub const W_BATTLE_MON_PP: u16 = 0xd02c;

pub const W_TRAINER_CLASS: u16 = 0xd030;

pub const W_TRAINER_PIC_POINTER: u16 = 0xd032;

pub const W_IS_IN_BATTLE: u16 = 0xd056;

// which entry in LoneAttacks to use
// it's actually the same thing as ^
pub const W_LONE_ATTACK_NO: u16 = 0xd05b;
pub const W_GYM_LEADER_NO: u16 = 0xd05b;

/// in a wild battle, this is the species of pokemon \
/// in a trainer battle, this is the trainer class + OPP_ID_OFFSET
pub const W_CUR_OPPONENT: u16 = 0xd058;

/// in normal battle, this is 0 \
/// in old man battle, this is 1 \
/// in safari battle, this is 2
pub const W_BATTLE_TYPE: u16 = 0xd059;

/// the map you will start at when the debug bit is set
pub const W_DEFAULT_MAP: u16 = 0xd07b;

/// 1 = no save file or save file is corrupted
/// 2 = save file exists and no corruption has been detected
pub const W_SAVE_FILE_STATUS: u16 = 0xd087;

pub const W_OPTIONS_INITIALIZED: u16 = 0xd089;

pub const W_SPRITE_FLIPPED: u16 = 0xd0a9;

pub const W_NAME_LIST_TYPE: u16 = 0xd0b5;

pub const W_MON_HEADER: u16 = 0xd0b7;
pub const W_MON_H_INDEX: u16 = 0xd0b7;
pub const W_MON_H_SPRITE_DIM: u16 = 0xd0c1;
pub const W_MON_H_FRONT_SPRITE: u16 = 0xd0c2;

pub const W_SAVED_TILE_ANIMATIONS: u16 = 0xd0d3;

pub const W_REPEL_REMAINING_STEPS: u16 = 0xd0da;

/// used as a Pokemon and Item storage value. Also used as an output value for CountSetBits
pub const W_D11E: u16 = 0xd11e;

/// not exactly sure what this is used for, but it seems to be used as a multipurpose temp flag value
pub const W_CURRENT_MAP_SCRIPT_FLAGS: u16 = 0xd125;

pub const W_CUR_ENEMY_LVL: u16 = 0xd126;

pub const W_LINK_STATE: u16 = 0xd12a;

// after a battle, you have at least 3 steps before a random battle can occur
pub const W_NUMBER_OF_NO_RANDOM_BATTLE_STEPS_LEFT: u16 = 0xd13b;

pub const W_PLAYER_NAME: u16 = 0xd157;

pub const W_PARTY_DATA_START: u16 = 0xd162;
pub const W_PARTY_MON1: u16 = 0xd16a;
pub const W_PARTY_MON1_PP: u16 = 0xd187;
pub const W_PARTY_MON1_LEVEL: u16 = 0xd18b;
pub const W_PARTY_MON2: u16 = 0xd196;
pub const W_PARTY_DATA_END: u16 = 0xd2f6;

pub const W_MAIN_DATA_START: u16 = 0xd2f6;

pub const W_POKEDEX_OWNED: u16 = 0xd2f6;
pub const W_POKEDEX_OWNED_END: u16 = 0xd309;
pub const W_POKEDEX_SEEN_END: u16 = 0xd31c;

pub const W_NUM_BAG_ITEMS: u16 = 0xd31c;

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

pub const W_PLAYER_ID: u16 = 0xd358;

pub const W_CUR_MAP: u16 = 0xd35d;

pub const W_CUR_MAP_TILESET: u16 = 0xd366;

pub const W_D436: u16 = 0xd435;

pub const W_PRINTER_SETTINGS: u16 = 0xd497;

pub const W_SIGN_COORDS: u16 = 0xd4b0;
pub const W_SIGN_TEXT_IDS: u16 = 0xd4d0;

/// movement byte 2, text ID
pub const W_MAP_SPRITE_DATA: u16 = 0xd4e3;

/// trainer class/item ID, trainer set ID
pub const W_MAP_SPRITE_EXTRA_DATA: u16 = 0xd503;

pub const W_GRASS_TILE: u16 = 0xd534;

/// bits 0-6: box number \
/// bit 7: whether the player has changed boxes before
pub const W_CURRENT_BOX_NUM: u16 = 0xd59f;

pub const W_PALLET_TOWN_CUR_SCRIPT: u16 = 0xd5f0;

pub const W_RIVAL_STARTER: u16 = 0xd714;
pub const W_PLAYER_STARTER: u16 = 0xd716;

/// bit 0: using Strength outside of battle \
/// bit 1: set by IsSurfingAllowed when surfing's allowed, but the caller resets it after checking the result \
/// bit 3: received Old Rod \
/// bit 4: received Good Rod \
/// bit 5: received Super Rod \
/// bit 6: gave one of the Saffron guards a drink \
/// bit 7: set by ItemUseCardKey, which is leftover code from a previous implementation of the Card Key
pub const W_D728: u16 = 0xd727;

/// bit 0: if not set, the 3 minimum steps between random battles have passed \
/// bit 1: prevent audio fade out
pub const W_D72C: u16 = 0xd72b;

/// This variable is used for temporary flags and as the destination map when
/// warping to the Trade Center or Colosseum.
///
/// bit 0: sprite facing directions have been initialised in the Trade Center \
/// bit 3: do scripted warp (used to warp back to Lavender Town from the top of the pokemon tower) \
/// bit 4: on a dungeon warp \
/// bit 5: don't make NPCs face the player when spoken to
///
/// Bits 6 and 7 are set by scripts when starting major battles in the storyline,
/// but they do not appear to affect anything. Bit 6 is reset after all battles
/// and bit 7 is reset after trainer battles (but it's only set before trainer
/// battles anyway).
pub const W_D72D: u16 = 0xd72c;

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

/// bit 0: check if the player is standing on a door and make him walk down a step if so \
/// bit 1: the player is currently stepping down from a door \
/// bit 2: standing on a warp \
/// bit 6: jumping down a ledge / fishing animation \
/// bit 7: player sprite spinning due to spin tiles (Rocket hideout / Viridian Gym)
pub const W_D736: u16 = 0xd735;

// bit 0: play time being counted
// bit 1: remnant of debug mode; only set by the debug build.
// if it is set:
// 1. skips most of Prof. Oak's speech, and uses NINTEN as the player's name and SONY as the rival's name
// 2. does not have the player start in floor two of the player's house (instead sending them to [wLastMap])
// 3. allows wild battles to be avoided by holding down B
// furthermore, in the debug build:
// 4. allows trainers to be avoided by holding down B
// 5. skips Safari Zone step counter by holding down B
// 6. skips the NPC who blocks Route 3 before beating Brock by holding down B
// 7. skips Cerulean City rival battle by holding down B
// 8. skips Pokémon Tower rival battle by holding down B
// bit 2: the target warp is a fly warp (bit 3 set or blacked out) or a dungeon warp (bit 4 set)
// bit 3: used warp pad, escape rope, dig, teleport, or fly, so the target warp is a "fly warp"
// bit 4: jumped into hole (Pokemon Mansion, Seafoam Islands, Victory Road) or went down waterfall (Seafoam Islands), so the target warp is a "dungeon warp"
// bit 5: currently being forced to ride bike (cycling road)
// bit 6: map destination is [wLastBlackoutMap] (usually the last used pokemon center, but could be the player's house)
pub const W_D732: u16 = 0xd731;

pub const W_EVENT_FLAGS: u16 = 0xd746;

pub const W_GRASS_RATE: u16 = 0xd886;
pub const W_GRASS_MONS: u16 = 0xd887;

pub const W_WATER_RATE: u16 = 0xd8a3;
pub const W_WATER_MONS: u16 = 0xd8a4;

pub const W_PLAY_TIME_HOURS: u16 = 0xda40;
pub const W_PLAY_TIME_MAXED: u16 = 0xda41;
pub const W_PLAY_TIME_MINUTES: u16 = 0xda42;
pub const W_PLAY_TIME_SECONDS: u16 = 0xda43;
pub const W_PLAY_TIME_FRAMES: u16 = 0xda44;

pub const W_MAIN_DATA_END: u16 = 0xda7f;

pub const W_BOX_DATA_START: u16 = 0xda7f;
pub const W_BOX_DATA_END: u16 = 0xdee1;
