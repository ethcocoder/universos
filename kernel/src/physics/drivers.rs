use anyhow::Result;
use crate::types::UniverseID;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, List, ListItem, Gauge},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
    Terminal,
};
use std::io::{stdout, Stdout};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

/// Hardware Abstraction Layer (HAL) for ParadoxOS
pub trait HardwareDriver {
    fn name(&self) -> &str;
    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>) -> Result<()>;
    fn handle_signal(&mut self, source: UniverseID, data: &str) -> Result<()>;
}

/// A professional TUI Dashboard for ParadoxOS
pub struct TuiDashboardDriver {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    last_draw: std::time::Instant,
}

impl TuiDashboardDriver {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal,
            last_draw: std::time::Instant::now(),
        })
    }
}

impl Drop for TuiDashboardDriver {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
    }
}

impl HardwareDriver for TuiDashboardDriver {
    fn name(&self) -> &str {
        "Paradox Dashboard"
    }

    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>) -> Result<()> {
        // Limit refresh rate to 10Hz to save CPU
        if self.last_draw.elapsed() < std::time::Duration::from_millis(100) {
            return Ok(());
        }
        self.last_draw = std::time::Instant::now();

        self.terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(size);

            // Header
            let header = Paragraph::new("🌌 ParadoxOS Kernel v0.6.0 - Physical Intelligence Dashboard")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, chunks[0]);

            // Main View
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ])
                .split(chunks[1]);

            // Universes List
            let mut items = Vec::new();
            for (id, u) in universes {
                let stability_color = if u.stability_score > 0.7 { Color::Green } else if u.stability_score > 0.3 { Color::Yellow } else { Color::Red };
                items.push(ListItem::new(format!(
                    "U{} | E: {:.2}J | S: {:.2} | Entropy: {:.2} | Time: {}",
                    id, u.energy, u.stability_score, u.entropy, u.timeline_index
                )).style(Style::default().fg(stability_color)));
            }
            let universes_list = List::new(items)
                .block(Block::default().title(" Active Universes ").borders(Borders::ALL));
            f.render_widget(universes_list, main_chunks[0]);

            // System Health
            let health_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ])
                .split(main_chunks[1]);

            // Total Energy Gauge
            let total_energy: f64 = universes.values().map(|u| u.energy).sum();
            let energy_gauge = Gauge::default()
                .block(Block::default().title(" Total Universe Energy ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Yellow))
                .ratio((total_energy / 10000.0).min(1.0));
            f.render_widget(energy_gauge, health_chunks[0]);

            // Entropy Gauge
            let avg_entropy: f64 = if universes.is_empty() { 0.0 } else { universes.values().map(|u| u.entropy).sum::<f64>() / universes.len() as f64 };
            let entropy_gauge = Gauge::default()
                .block(Block::default().title(" Avg Entropy Level ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Magenta))
                .ratio((avg_entropy / 50.0).min(1.0));
            f.render_widget(entropy_gauge, health_chunks[1]);

            // Footer
            let footer = Paragraph::new("LAW 1: Energy Conserved | LAW 2: Entropy Increases | Press Ctrl+C to exit")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        Ok(())
    }

    fn handle_signal(&mut self, _source: UniverseID, _data: &str) -> Result<()> {
        // In a full TUI, this would add to a log buffer on the screen
        Ok(())
    }
}

/// A driver that persists the Multiverse state to disk
pub struct ArchiveDriver {
    path: std::path::PathBuf,
    last_archive: std::time::Instant,
    archive_interval: std::time::Duration,
}

impl ArchiveDriver {
    pub fn new<P: Into<std::path::PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            last_archive: std::time::Instant::now(),
            archive_interval: std::time::Duration::from_secs(5), // Save every 5 seconds
        }
    }
}

impl HardwareDriver for ArchiveDriver {
    fn name(&self) -> &str {
        "Multiverse Archive (Disk)"
    }

    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>) -> Result<()> {
        if self.last_archive.elapsed() < self.archive_interval {
            return Ok(());
        }
        self.last_archive = std::time::Instant::now();

        // Serialize the universes
        let data = serde_json::to_string_pretty(universes)?;
        
        // Save to file (in a real OS, we'd use ParadoxLF compression here)
        std::fs::write(&self.path, data)?;
        
        // Note: In high-performance mode, we'd do this on a background thread
        Ok(())
    }

    fn handle_signal(&mut self, _source: UniverseID, _data: &str) -> Result<()> {
        Ok(())
    }
}

/// A driver that enables inter-kernel communication (Networking / Wormholes)
pub struct WormholeDriver {
    #[allow(dead_code)]
    listen_addr: String,
    tx: tokio::sync::mpsc::Sender<NetworkMessage>,
    #[allow(dead_code)]
    runtime: tokio::runtime::Runtime,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum NetworkMessage {
    Signal {
        source: UniverseID,
        target: UniverseID,
        data: String,
    },
    SyncState {
        universe: crate::universe::Universe,
    },
}

impl WormholeDriver {
    pub fn new(listen_addr: &str) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        
        let (tx, mut rx) = tokio::sync::mpsc::channel::<NetworkMessage>(100);
        let addr = listen_addr.to_string();

        // Spawn background listener and sender
        rt.spawn(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind network port");
            log::info!("🌐 Wormhole Driver listening on {}", addr);

            loop {
                tokio::select! {
                    // Incoming connections
                    Ok((_socket, peer)) = listener.accept() => {
                        log::info!("🛸 New entanglement from {}", peer);
                        tokio::spawn(async move {
                            // In a real OS, we'd handle handshake and stream here
                            // For demo, we just log connection
                        });
                    }
                    // Outgoing messages from Kernel
                    Some(msg) = rx.recv() => {
                        // In a professional implementation, we'd route this to the correct peer
                        log::info!("🛰️ Projecting signal across wormhole: {:?}", msg);
                    }
                }
            }
        });

        Ok(Self {
            listen_addr: listen_addr.to_string(),
            tx,
            runtime: rt,
        })
    }
}

impl HardwareDriver for WormholeDriver {
    fn name(&self) -> &str {
        "Wormhole Driver (Network)"
    }

    fn sync(&mut self, _universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>) -> Result<()> {
        // Here we could sync state vectors to remote peers
        Ok(())
    }

    fn handle_signal(&mut self, source: UniverseID, data: &str) -> Result<()> {
        // Send signal across the network
        let msg = NetworkMessage::Signal {
            source,
            target: UniverseID(999), // Representing a remote target
            data: data.to_string(),
        };
        
        let _ = self.tx.try_send(msg);
        Ok(())
    }
}

use sysinfo::System;

/// A driver that maps physical host CPU load to Kernel Kinetic Energy
pub struct KineticEnergyDriver {
    sys: System,
    #[allow(dead_code)]
    last_cpu_load: f32,
}

impl KineticEnergyDriver {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_usage();
        Self {
            sys,
            last_cpu_load: 0.0,
        }
    }
}

impl HardwareDriver for KineticEnergyDriver {
    fn name(&self) -> &str {
        "Kinetic Energy Channel (CPU)"
    }

    fn sync(&mut self, _universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>) -> Result<()> {
        self.sys.refresh_cpu_usage();
        
        let mut total_usage = 0.0;
        let cpus = self.sys.cpus();
        for cpu in cpus {
            total_usage += cpu.cpu_usage();
        }
        
        let avg_usage = if cpus.is_empty() { 0.0 } else { total_usage / cpus.len() as f32 };
        self.last_cpu_load = avg_usage;
        
        Ok(())
    }

    fn handle_signal(&mut self, _source: UniverseID, _data: &str) -> Result<()> {
        Ok(())
    }
}

