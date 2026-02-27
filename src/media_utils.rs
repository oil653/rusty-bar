use std::sync::Arc;

use futures::{SinkExt, Stream, StreamExt, pin_mut};
use iced::stream;
use mpris_client_async::{Mpris, Playback, Player, PlayerEvent, properties::PlaybackStatus};

pub fn mpris_subscription() -> impl Stream<Item = (Option<Arc<Mpris<'static>>>, Option<PlayerEvent>)> {
    stream::channel(32, async |mut output| {
        let mpris = Arc::new(match Mpris::new().await {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to initialise MPRIS listener: {e}");
                return; // Stream ends, iced will attempt to restart it
            }
        });

        let stream = match mpris.player_stream().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to subscribe to MPRIS proxy events: {e}");
                return; // Stream ends again, iced will attempt to restart it
            }
        };
        pin_mut!(stream);

        // Send an empty event, so the state will have mpris, and it can fetch currently active players
        let _ = output.send((Some(Arc::clone(&mpris)), None)).await;
        while let Some(value) = stream.next().await {
            let _ = output.send((Some(Arc::clone(&mpris)), Some(value))).await;
        }
        let _ = output.send((None, None)).await;
    })
}

pub async fn get_tracked_player(players: Vec<Arc<Player>>) -> Option<Arc<Player>> {
    for player in players {
        if let Ok(Playback::Playing) = player.get(PlaybackStatus).await {
            return Some(player.clone());
        }
    }

    None
}