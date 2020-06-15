use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Users
{
    users: HashMap<String, String>,
}

fn load_user_data(file_path: String) -> HashMap<String, String> 
{
    let yaml_string: String= fs::read_to_string(file_path).expect("Couldn't open file");
    let yaml: Users = serde_yaml::from_str(&yaml_string).unwrap();
    yaml.users
}

struct DynamicBot
{
    current_users: Vec<String>,
    registered_users: HashMap<String, String>,
}

fn main() 
{
    let users = load_user_data("users.yml".to_string());
    println!("{}", users["neji49"]);
}


// use discord::Discord;
// use discord::State;
// use discord::Connection;
// use discord::voice::VoiceConnection;
// use discord::model::Event;
// use discord::model::CurrentUser;
// use discord::model::ChannelType;
// use discord::model::Channel;
// use discord::model::User;
// use discord::model::ChannelId;
// use discord::model::ServerId;
// use discord::model::VoiceState;
// use discord::model::Presence;
// use discord::model::RoleId;
// use discord::model::PublicChannel;
// use std::process::Command;
// use std::str;
// use std::str::FromStr;
// use std::process;
// use std::mem;
// use std::thread;
// use std::ptr;
// use std::env;
// use std::io;
// use std::io::Stdout;
// use std::io::Write;
// use std::collections::HashMap;
// use std::time::Duration;

// //Environment variables to look up
// static DISCORD_NAME: &'static str = "DISCORD_NAME";
// static DISCORD_CHANNEL: &'static str = "DISCORD_CHANNEL";
// static DISCORD_TOKEN: &'static str = "DISCORD_TOKEN";

// //Discord Info struct
// struct DiscordInfo<'a> {
//     username: &'a str,
//     channel: &'a str,
// }

// //State struct
// struct DiscordState<'a> {
//     bot_in_channel: &'a mut bool,
//     channel_id: &'a mut Option<ChannelId>,
//     user_in_channel: &'a mut bool,
//     user_in_game: &'a mut bool,
// }

            

// fn main() {

//     //Setting token from environment variable
//     let YOUR_TOKEN: &str = &env::var(DISCORD_TOKEN).unwrap();

//     let discord = log_into_discord(YOUR_TOKEN);
//     let (mut connection, _) = discord.connect().expect("connect failed");

//     let mut discord_closure  = move || dispatch_on_event(&discord, &mut connection);

//     let discord_handle = thread::spawn(discord_closure);

//     discord_handle.join().unwrap();
// }

// /// Function takes in bot token and logs into discord; returning a session object.
// fn log_into_discord(token: &str) -> Discord {
//     Discord::from_bot_token(token).expect("login failed")
// }

// /// Function listens for events on the voice channel and runs functions in response.
// fn dispatch_on_event(discord: &Discord, connection: &mut Connection) {
//     loop{
//         println!("Running dispatch thread.");

//         //Initialzing username and channel from environment variables
//         let username: &str  = &env::var(DISCORD_NAME).unwrap();
//         let channel_name: &str = &env::var(DISCORD_CHANNEL).unwrap();
//         let discord_info = DiscordInfo { username: username, channel: channel_name};

//         //Print statements for debugging
//         println!("{:?}", username);
//         println!("{:?}", channel_name);

//         //Initializing state variables
//         let mut discord_state = DiscordState { bot_in_channel: &mut false, channel_id: &mut None, user_in_channel: &mut false, user_in_game: &mut false};

//         match connection.recv_event() {
            
//             Ok(Event::VoiceStateUpdate(server_opt, voice_state)) => voice_channel_update_event(discord, connection, &server_opt, &voice_state, &discord_info, &mut discord_state),

//             Ok(Event::PresenceUpdate{presence, server_id, roles}) => game_state_update_event(discord, connection, presence, &server_id, &roles, &discord_info, &mut discord_state),

//             Ok(_) => {}
            
//             Err(discord::Error::Closed(code, body)) => {
//                 println!("Gateway closed on us with code {:?}: {}", code, body);
//                 process::exit(1);
//             },

//             Err(err) => println!("Receive error: {:?}", err)
//         }
//     }
// }

// /// Event runs when an occurence happens on the voice channel.
// fn voice_channel_update_event(discord: &Discord, connection: &mut Connection, server_opt: &Option<ServerId>, voice_state: &VoiceState, info: &DiscordInfo, state: &mut DiscordState) {
//     println!("Got voice update: {:?},{:?}",server_opt,voice_state.channel_id);

//     if server_opt.is_some() && voice_state.channel_id.is_some() {

//         let server = server_opt;
//         let user = discord.get_member(server_opt.expect("No Server"),voice_state.user_id).unwrap();
//         let channel: Channel = discord.get_channel(voice_state.channel_id.expect("No Channel")).unwrap();

//         match channel {
//             Channel::Public(ref voice) if voice.kind == ChannelType::Voice => channel_is_voice(discord, connection, server, voice, voice_state, info, state),

//             _ => println!("Not a voice channel"),
//         }
//     } else {
//         *state.user_in_channel = false;
//     }
// }

// /// Event dispatch for when state of game changes.
// fn game_state_update_event(discord: &Discord, connection: &mut Connection, presence: Presence, server_id: &Option<ServerId>, roles: &Option<Vec<RoleId>>, info: &DiscordInfo, state: &mut DiscordState) {
//     let user = discord.get_member(server_id.expect("No Server"), presence.user_id).unwrap();
//     let server = server_id;

//     println!("Presence changed of: {}", presence.user_id);

//     if presence.game.is_some() {

//         let name_match_and_playing_game = user.display_name() == info.username && presence.game.expect("No game").name == "Rocket League";

//         if  name_match_and_playing_game {

//             *state.user_in_game = !*state.user_in_game;
//             println!("user_in_channel {}", *state.user_in_channel);
//             println!("user_in_game {}", *state.user_in_game);

//         }

//         check_state_and_join_channel(connection, server, state);

//     } else {

//         *state.user_in_game = false;

//     }
// }

// /// Update state channel id if voice channel.
// fn channel_is_voice(discord: &Discord, connection: &mut Connection, server: &Option<ServerId>, voice: &PublicChannel, voice_state: &VoiceState, info: &DiscordInfo, state: &mut DiscordState) {
//     let user = discord.get_member(server.expect("No Server"),voice_state.user_id).unwrap();
//     let channel: Channel = discord.get_channel(voice_state.channel_id.expect("No Channel")).unwrap();

//     let name_and_channel_match = user.display_name() == info.username && voice.name == info.channel;

//     if  name_and_channel_match {
//         *state.user_in_channel = !*state.user_in_channel;
//         println!("user_in_channel {}", state.user_in_channel);
//         println!("user_in_game {}", state.user_in_game);
//         *state.channel_id = voice_state.channel_id;
//     }

//     check_state_and_join_channel(connection, server, state);
// }

// /// Verifies desired state and has bot take action.
// fn check_state_and_join_channel(connection: &mut Connection, server: &Option<ServerId>, state: &mut DiscordState) {
//     let in_game_in_channel_bot_not_in_channel = *state.user_in_game && *state.user_in_channel && !*state.bot_in_channel;

//     if in_game_in_channel_bot_not_in_channel {

//         let voice = Some(connection.voice(*server));
//         match *state.channel_id {

//             Some(id) => {
//                 println!("Joining");
//                 voice.map(|v| v.connect(id));
//                 *state.bot_in_channel = true;
//             }

//             None => println!("Never found channel id")
//         }
//     }
// }
