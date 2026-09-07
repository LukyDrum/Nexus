use std::time::Duration;

use crossbeam::channel::Receiver;
use iced::futures::{StreamExt, executor::block_on};
use mpris::{PlaybackStatus, Player, PlayerFinder};
use zbus::{Connection, MatchRule, fdo::DBusProxy};

use crate::language::{
    DuplexChannel, Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value,
};

const MICROS_IN_SECOND: i64 = 1_000_000;
const MPRIS_PLAYER_PREFIX: &str = "org.mpris.MediaPlayer2.";

pub(super) fn mpris_library() -> Library {
    Library::from([
        ("player", player_function()),
        ("track", track_function()),
        ("play", play_function()),
        ("pause", pause_function()),
        ("playPause", play_pause_function()),
        ("next", next_function()),
        ("previous", previous_function()),
        ("events", events_function()),
    ])
}

/// Returns the name of the current player (eg. Spotify)
fn player_function() -> Function {
    let params = FunctionParams::default();
    let player_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };

        Value::new_string(player.identity())
    };

    Function {
        params,
        code: FunctionCode::new_host(player_impl),
        closure: None,
    }
}

/// Returns metadata about the current track:
///     - title
///     - album
///     - artist
///     - length
///     - timestamp
///     - artUrl
///     - isPlaying
fn track_function() -> Function {
    let params = FunctionParams::default();
    let track_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };

        collect_track_info(&player).unwrap_or_default()
    };

    Function {
        params,
        code: FunctionCode::new_host(track_impl),
        closure: None,
    }
}

fn play_function() -> Function {
    let params = FunctionParams::default();
    let play_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };
        let _ = player.play();

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(play_impl),
        closure: None,
    }
}

fn pause_function() -> Function {
    let params = FunctionParams::default();
    let pause_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };
        let _ = player.pause();

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(pause_impl),
        closure: None,
    }
}

fn play_pause_function() -> Function {
    let params = FunctionParams::default();
    let play_pause_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };
        let _ = player.play_pause();

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(play_pause_impl),
        closure: None,
    }
}

fn next_function() -> Function {
    let params = FunctionParams::default();
    let next_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };
        let _ = player.next();

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(next_impl),
        closure: None,
    }
}

fn previous_function() -> Function {
    let params = FunctionParams::default();
    let previous_impl = |_environment: &SharedEnvironment| -> Value {
        let Some(player) = active_player() else {
            return Value::Null;
        };
        let _ = player.previous();

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(previous_impl),
        closure: None,
    }
}

/// Returns a Channel to which information about the track will be send every time a track/track-state changes.
fn events_function() -> Function {
    let params = FunctionParams::default();
    let events_impl = |_environment: &SharedEnvironment| -> Value {
        let events_channel = DuplexChannel::default();
        let events_channel_value = Value::Channel(events_channel.clone());

        let _handle = std::thread::spawn(move || {
            let player_change_receiver = watch_player_changes();

            if let Some(player) = active_player() {
                let _ = events_channel.send(collect_track_info(&player).unwrap_or_default());
            }

            while let Ok(change) = player_change_receiver.recv() {
                match change {
                    PlayerChange::Change => {
                        if let Some(player) = active_player() {
                            let _ = events_channel
                                .send(collect_track_info(&player).unwrap_or_default());
                        } else {
                            let _ = events_channel.send(Value::new_hash_map(Default::default()));
                        }
                    }
                    PlayerChange::Error => {
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        });

        events_channel_value
    };

    Function {
        params,
        code: FunctionCode::new_host(events_impl),
        closure: None,
    }
}

#[derive(Clone, Copy, Debug)]
enum PlayerChange {
    Change,
    Error,
}

fn watch_player_changes() -> Receiver<PlayerChange> {
    let (sender, receiver) = crossbeam::channel::unbounded();

    // Watch for player being created/removed
    let sender_names = sender.clone();
    let _handle = std::thread::spawn(move || {
        block_on(async {
            let Ok(conn) = Connection::session().await else {
                let _ = sender_names.send(PlayerChange::Error);
                return;
            };
            let Ok(dbus) = DBusProxy::new(&conn).await else {
                return;
            };
            let Ok(mut stream) = dbus.receive_name_owner_changed().await else {
                return;
            };

            while let Some(signal) = stream.next().await {
                if let Ok(args) = signal.args() {
                    let name = args.name();
                    if name.starts_with(MPRIS_PLAYER_PREFIX) {
                        let _ = sender_names.send(PlayerChange::Change);
                    }
                }
            }
        })
    });

    // Watch for all changes in properties
    let sender_props = sender.clone();
    let _handle = std::thread::spawn(move || {
        block_on(async {
            let Ok(conn) = Connection::session().await else {
                return;
            };

            let rule = MatchRule::builder()
                .msg_type(zbus::message::Type::Signal)
                .interface("org.freedesktop.DBus.Properties")
                .expect("Valid interface")
                .member("PropertiesChanged")
                .expect("Valid member")
                .path("/org/mpris/MediaPlayer2")
                .expect("Valid path")
                .build();

            let Ok(mut stream) = zbus::MessageStream::for_match_rule(rule, &conn, None).await
            else {
                return;
            };

            while let Some(_msg) = stream.next().await {
                let _ = sender_props.send(PlayerChange::Change);
            }
        })
    });

    receiver
}

fn active_player() -> Option<Player> {
    let finder = PlayerFinder::new().ok()?;
    finder.find_active().ok()
}

fn collect_track_info(player: &Player) -> Option<Value> {
    let track = player.get_metadata().ok()?;

    let title = track.title().map(Value::new_string).unwrap_or_default();
    let album = track
        .album_name()
        .map(Value::new_string)
        .unwrap_or_default();
    let artist = track
        .artists()
        .map(|artists| Value::new_string(artists.join(", ")))
        .unwrap_or_default();
    let length = track
        .length()
        .map(|length| Value::Int(length.as_secs() as i64))
        .unwrap_or_default();
    let timestamp = player
        .get_position_in_microseconds()
        .map(|micros| Value::Int(micros as i64 * MICROS_IN_SECOND))
        .unwrap_or_default();
    let art_url = track.art_url().map(Value::new_string).unwrap_or_default();
    let is_playing = player
        .get_playback_status()
        .map(|status| Value::Bool(matches!(status, PlaybackStatus::Playing)))
        .unwrap_or_default();

    let map = [
        ("title", title),
        ("album", album),
        ("artist", artist),
        ("length", length),
        ("timestamp", timestamp),
        ("artUrl", art_url),
        ("isPlaying", is_playing),
    ]
    .into_iter()
    .map(|(key, value)| (Value::new_string(key), value))
    .collect();

    Some(Value::new_hash_map(map))
}
