pub mod configuration;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

// LOGIN SEQUENCE

// LOGIN PHASE:
//D [C2S] Handshake                    | Initiates login; sets State = 2
//D [C2S] Login Start                  | Begins login with username
//D [S2C] Encryption Request           | Response to Login Start (if server uses online-mode)
//D [C2S] Encryption Response          | Response to Encryption Request
//D [S2C] Set Compression              | Response to Encryption Response (optional)
//D [S2C] Login Success                | Response to Login Start (after auth completes)
//D [C2S] Login Acknowledged           | Response to Login Success

// CONFIGURATION PHASE:
//D [C2S] Plugin Message            | Independent request; sends client brand
//D [C2S] Client Information        | Independent request; sends language, view distance, etc.
//  [S2C] Plugin Message            | Not a direct response; server brand info
//  [S2C] Feature Flags             | Informational; enables experimental features
//D [S2C] Known Packs               | Response to Client Info (negotiates resource packs)
//D [C2S] Known Packs               | Response to Known Packs
//- [S2C] Registry Data             | Response to Known Packs; multiple packets
//  [S2C] Update Tags               | Informational; tag definitions
//- [S2C] Finish Configuration      | Response to Known Packs and Registry Data
//  [C2S] Finish Configuration Ack  | Response to Finish Configuration

// LOGIN PHASE:
//  [S2C] Login (Play)              | Response to Finish Config Ack
//  [S2C] Change Difficulty         | Informational
//  [S2C] Player Abilities          | Informational
//  [S2C] Set Held Item             | Informational
//  [S2C] Update Recipes            | Informational
//  [S2C] Entity Event              | Informational
//  [S2C] Commands                  | Informational
//  [S2C] Update Recipe Book        | Informational
//  [S2C] Synchronize Position      | Response to Login (Play)
//  [C2S] Teleport Confirm          | Response to Synchronize Position
//  [C2S] Set Player Position       | Optional; client-side adjustment
//  [S2C] Server Data               | Informational
//  [S2C] Player Info Update        | Response to Login (Play); adds players to tab list
