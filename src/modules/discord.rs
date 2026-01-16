use crate::modules::config::{CLIENT_ID, GAME_NAME};

use anyhow::Result;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub struct DiscordClient {
    client: Option<DiscordIpcClient>,
    is_connected: bool,
}

impl DiscordClient {
    pub fn new() -> Self {
        Self {
            client: DiscordIpcClient::new(CLIENT_ID).ok(),
            is_connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        if self.is_connected {
            return Ok(());
        }

        if let Some(client) = &mut self.client {
            client.connect()
                .map_err(|e| anyhow::anyhow!("Failed to connect to Discord: {}", e))?;
            self.is_connected = true;
        } else {
             // Try re-creating if it failed initially (rare)
             self.client = DiscordIpcClient::new(CLIENT_ID).ok();
             if let Some(client) = &mut self.client {
                 client.connect()
                     .map_err(|e| anyhow::anyhow!("Failed to connect to Discord after recreate: {}", e))?;
                 self.is_connected = true;
             }
        }
        Ok(())
    }

    pub fn update_presence(&mut self, start_time: i64) -> Result<()> {
        if !self.is_connected {
            return Ok(());
        }

        if let Some(client) = &mut self.client {
             let details = format!("Playing {}", GAME_NAME);
            // NOTE: If the Application ID does not have these specific assets uploaded in the Discord Developer Portal,
            // the Rich Presence might NOT appear at all.
            // For safety, we will try to set it, but if it fails silently (Discord side), it might be due to missing assets.
            // We use standard keys often present or fallback.
            
            let assets = activity::Assets::new()
                .large_image("motorstorm") // Ensure this key exists in Developer Portal!
                .large_text(GAME_NAME)
                .small_image("rpcs3")      // Ensure this key exists!
                .small_text("RPCS3 Emulator");

            let timestamps = activity::Timestamps::new().start(start_time);

            let payload = activity::Activity::new()
                .details(&details)
                .state("On RPCS3 Emulator")
                .assets(assets)
                .timestamps(timestamps);

            client.set_activity(payload)
                .map_err(|e| anyhow::anyhow!("Failed to set activity: {}", e))?;
             // We can optionally return true/false or something to indicate success if we want generic logging,
             // but caller handles error.
        }
        Ok(())
    }

    pub fn clear_presence(&mut self) -> Result<()> {
        if !self.is_connected {
            return Ok(());
        }
        if let Some(client) = &mut self.client {
            let _ = client.clear_activity();
        }
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}
