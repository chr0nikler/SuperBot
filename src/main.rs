
extern crate discord; extern crate libc; #[macro_use]
extern crate log;


use discord::Discord;
use discord::State;
use discord::Connection;
use discord::voice::VoiceConnection;
use discord::model::Event;
use discord::model::CurrentUser;
use discord::model::ChannelType;
use discord::model::Channel;
use discord::model::User;
use discord::model::ChannelId;
use discord::model::ServerId;
use std::process::Command;
use libc::*;
use std::str;
use std::str::FromStr;
use std::process;
use std::mem;
use std::ptr;
use std::env;
use std::io;
use std::io::Stdout;
use std::io::Write;
use std::collections::HashMap;
use std::time::Duration;

fn main() {


    let YOUR_CHANNEL: &str = &env::var("DISCORD_CHANNEL").unwrap();
    let YOUR_USERNAME: &str  = &env::var("DISCORD_NAME").unwrap();

    let mut bot_in_channel = false;
    let mut channel_id: Option<ChannelId>= None;
    let mut user_in_channel = false;
    let mut user_in_game = false;
    let mut possible_addrs: HashMap<i32,u32> = HashMap::new();
    let mut start_search = false;
    let mut first_search = true;
    let mut pid: pid_t = -1;

   
    
    println!("{:?}",&env::var("DISCORD_NAME"));
    println!("{:?}",&env::var("DISCORD_CHANNEL"));
    println!("{:?}",&env::var("DISCORD_TOKEN"));

    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").unwrap()
    ).expect("login failed");


    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        println!("looping {}",start_search);

        if(!start_search) { 
        // Match against incoming events
            match connection.recv_event() {
                
                // Event when something happens on a voice channel
                Ok(Event::VoiceStateUpdate(server_opt,voice_state)) => {

                    println!("Got voice update: {:?},{:?}",server_opt,voice_state.channel_id);

                    if server_opt.is_some() && voice_state.channel_id.is_some() {

                        let server = server_opt;
                        let user = discord.get_member(server_opt.expect("No Server"),voice_state.user_id).unwrap();
                        let channel: Channel = discord.get_channel(voice_state.channel_id.expect("No Channel")).unwrap();

                        match channel {

                            Channel::Public(ref voice) if voice.kind == ChannelType::Voice => {

                                // Verify user and channel joined are the ones we desire
                                if user.display_name() == YOUR_USERNAME && voice.name == YOUR_CHANNEL {
                                    user_in_channel = !user_in_channel;
                                    println!("user_in_channel {}",user_in_channel);
                                    println!("user_in_game {}",user_in_game);
                                    channel_id = voice_state.channel_id;
                                }

                                // If conditions are met, bot joines voice channel to
                                if user_in_game && user_in_channel && !bot_in_channel {

                                    let voice = Some(connection.voice(server));

                                    match channel_id {
                                        Some(id) => {
                                            println!("Joining");
                                            voice.map(|v| v.connect(id));
                                            bot_in_channel = true;
                                            start_search = true;
                                        }
                                        None => println!("Never found channel id")
                                    }
                                }
                            }
                            _ => println!("Not a voice channel")
                        }
                    } else {
                        user_in_channel = false;
                    }
                },

                // Presence includes change of game state for users
                Ok(Event::PresenceUpdate{presence,server_id,roles}) => {

                    let user = discord.get_member(server_id.expect("No Server"),presence.user_id).unwrap();
                    let server = server_id;

                    println!("Presence changed of: {}",presence.user_id);

                    if presence.game.is_some() {

                        // Check if user and game are the ones we desire
                        if user.display_name() == YOUR_USERNAME && presence.game.expect("No game").name == "Rocket League" {

                            user_in_game = !user_in_game;
                            println!("user_in_channel {}",user_in_channel);
                            println!("user_in_game {}",user_in_game);

                        }

                        // Same as above
                        if user_in_game && user_in_channel && !bot_in_channel {

                            let voice = Some(connection.voice(server));

                            match channel_id {
                                Some(id) => {
                                    println!("Joining");
                                    voice.map(|v| v.connect(id));
                                    bot_in_channel = true;
                                    start_search = true;

                                    let output = Command::new("pgrep")
                                                          .arg("RocketLeague")
                                                          .output()
                                                          .expect("where's the PID?");

                                    // Grap pid from pgrep
                                    let mut tmp_pid: String = String::from_utf8(output.stdout).unwrap();

                                    let tmp_pid_len = tmp_pid.len();

                                    // Grabbing from stdout introduces a \n character at the end, so truncate
                                    tmp_pid.truncate(tmp_pid_len-1);

                                    // Cast from String to an int (signed, 32 bit)
                                    pid = tmp_pid.parse::<i32>().unwrap();
                                }
                                None => println!("Never found channel id")
                            }
                        }
                    } else {

                        user_in_game = false;

                    }
                },
                Ok(_) => {}
                Err(discord::Error::Closed(code, body)) => {
                    println!("Gateway closed on us with code {:?}: {}", code, body);
                    break
                },
                Err(err) => println!("Receive error: {:?}", err)
            }
        }

        else {
            /* Commented out code below is for reading the value at
             * a certain address to ascertain the number of goals
             * currently scored by your team.
             * Procedure for now will required storing some 100,000 values
             * in a hash, and trimming those down to about 15 (hopefully).
             *
             * If 15 of the values at certain addresses increment by 1, that is
             * (again hopefully) enough to determine that your team scored a goal
             */

            //let mut voice: Option< &mut VoiceConnection> = None;
            //

            /* list of possible address
             *
             * 0x5c45ce
             * 0x5d924b0,
             * 0x201386f4
             * */

            /* Retrieving PID of Rocket League.
             * Technically should be done when discord has detected that Rocket League
             * has started, but here for now for debugging and testing. Still needs
             * Rocket League running first to work though.
             * */


            unsafe{

                // Start address for guessing
                let mut YOUR_ADDR = 0x2a41dc95;

                let mut addr = YOUR_ADDR;

                let mut first_run = true;

                let mut count = 0u32;

                // Loop to check each address
                for x in 0..10000000 {

                    addr = addr + 1;
                    let mut value: u32 = mem::uninitialized();
                    let local_iov = iovec {
                    iov_base: &mut value as *mut _ as *mut c_void,
                    iov_len: mem::size_of::<u32>(),
                    };
                    let remote_iov = iovec {
                    iov_base: addr as *mut c_void,
                    iov_len: mem::size_of::<u32>(),
                    };


                    let read = process_vm_readv(pid, &local_iov, 1, &remote_iov, 1, 0);

                    let &mut val = possible_addrs.entry(addr).or_insert(value);


                    

                    if val != u32::max_value() && val+1 == value{
                        //println!("We have a goal? {}",addr);
                        possible_addrs.entry(addr).or_insert(value);
                        count = count + 1;
                    } 

                    // Comment back in when debugging. Pipe to file for better viewing
                     
                    //println!("addr: {:#x}",addr );
                    //println!("value: {}",value );
                    //println!("read: {}",read );
                    

                } 
                //io::stdout().flush().unwrap();
                println!("addr: {:#x}",addr );
                println!("count: {}",count);

                first_run = false;

            };
            io::stdout().flush();
        }


    }


}
