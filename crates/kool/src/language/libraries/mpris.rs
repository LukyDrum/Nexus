use std::time::Duration;

use mpris::{PlaybackStatus, Player, PlayerFinder};

use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

const MICROS_IN_SECOND: i64 = 1_000_000;

pub(super) fn mpris_library() -> Library {
    Library::from([
        ("player", player_function()),
        ("track", track_function()),
        ("play", play_function()),
        ("pause", pause_function()),
        ("playPause", play_pause_function()),
        ("next", next_function()),
        ("previous", previous_function()),
        ("onChange", on_change_function()),
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

fn on_change_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let on_change_impl = |environment: &SharedEnvironment| -> Value {
        let Some(Value::Function(callback)) = environment.get_variable(TAIL_PARAM) else {
            return Value::Null;
        };

        std::thread::spawn(move || {
            loop {
                let Some(player) = active_player() else {
                    // Lets wait a little and try again
                    std::thread::sleep(Duration::from_secs(1));
                    continue;
                };
                let Ok(mut events) = player.events() else {
                    return;
                };

                let call_callback = || {
                    let track_info = collect_track_info(&player).unwrap_or_default();

                    let mut args = callback.default_args();
                    let _ = args.set_singular_arg(track_info);

                    let _ = callback.call(args);
                };

                // Init before the first event
                call_callback();
                while let Some(Ok(_event)) = events.next() {
                    call_callback();
                }
            }
        });

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(on_change_impl),
        closure: None,
    }
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
