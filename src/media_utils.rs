use std::sync::Arc;

use futures::{SinkExt, Stream, StreamExt, pin_mut};
use iced::stream;
use mpris_client_async::{Mpris, Playback, Player, PlayerEvent, properties::PlaybackStatus};

#[derive(Debug, Clone)]
pub enum MprisEvent {
    /// A new instance of MPRIS was created
    NewInstance(Arc<Mpris<'static>>),
    /// The MPRIS stream ended. The mpris instance may or may not be invalid at this point, but it's recommended to not use it
    StreamEnded,
    /// A player event
    Event(PlayerEvent)
}

pub fn mpris_subscription() -> impl Stream<Item = MprisEvent> {
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

        let _ = output.send(MprisEvent::NewInstance(Arc::clone(&mpris))).await;
        while let Some(event) = stream.next().await {
            let _ = output.send(MprisEvent::Event(event)).await;
        }
        let _ = output.send(MprisEvent::StreamEnded).await;
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