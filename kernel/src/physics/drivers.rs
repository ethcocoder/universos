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

/// Signals that hardware drivers can send back to the kernel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemPulse {
    None,
    Rewind,
    CollapseAll,
    Shutdown,
    Sabotage(UniverseID, f64),
}

/// Hardware Abstraction Layer (HAL) for ParadoxOS
pub trait HardwareDriver {
    fn name(&self) -> &str;
    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse>;
    fn handle_event(&mut self, event: &crate::interaction::CausalEvent) -> Result<()>;
    fn pending_energy(&self) -> f64 { 0.0 }
}

/// A professional TUI Dashboard for ParadoxOS
pub struct TuiDashboardDriver {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    last_draw: std::time::Instant,
    event_logs: Vec<String>,
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
            event_logs: Vec::new(),
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

    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, _incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse> {
        // Check for user input
        let mut pulse = SystemPulse::None;
        if crossterm::event::poll(std::time::Duration::from_millis(0))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => pulse = SystemPulse::Shutdown,
                    crossterm::event::KeyCode::Char('r') => pulse = SystemPulse::Rewind,
                    crossterm::event::KeyCode::Char('x') => pulse = SystemPulse::CollapseAll,
                    _ => {}
                }
            }
        }

        // Limit refresh rate to 10Hz to save CPU
        if self.last_draw.elapsed() < std::time::Duration::from_millis(100) {
            return Ok(pulse);
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
                    Constraint::Length(3), // Added for Persistence
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

            // Persistence Status
            let archive_status = Paragraph::new(" Disk: ARCHIVE_READY (PLF) ")
                .style(Style::default().fg(Color::Blue))
                .block(Block::default().title(" Persistence ").borders(Borders::ALL));
            f.render_widget(archive_status, health_chunks[2]);

            // Event Horizon (Logs)
            let logs: Vec<ListItem> = self.event_logs.iter().rev().take(10).map(|s| {
                ListItem::new(s.as_str()).style(Style::default().fg(Color::Gray))
            }).collect();
            let log_list = List::new(logs)
                .block(Block::default().title(" Event Horizon (Causal Flow) ").borders(Borders::ALL));
            f.render_widget(log_list, health_chunks[3]);

            // Footer
            let footer = Paragraph::new("LAW 1: Energy Conserved | LAW 2: Entropy Increases | Phase 12: Entanglement Active")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        Ok(pulse)
    }

    fn handle_event(&mut self, event: &crate::interaction::CausalEvent) -> Result<()> {
        let log = format!("{:?} | {} -> {}: E={:.2}J", 
            event.event_type, event.source, event.target, event.energy_payload);
        
        self.event_logs.push(log);
        if self.event_logs.len() > 100 {
            self.event_logs.remove(0);
        }
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

    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, _incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse> {
        if self.last_archive.elapsed() < self.archive_interval {
            return Ok(SystemPulse::None);
        }
        self.last_archive = std::time::Instant::now();

        // Serialize the universes to JSON
        let json_data = serde_json::to_vec(universes)?;
        
        // Phase 6: Compress using ParadoxLF (Memory as Potential)
        let compressed_data = paradoxlf::compress(&json_data);
        
        // Save as .plf (Paradox Lossless Fluid)
        let plf_path = self.path.with_extension("plf");
        std::fs::write(&plf_path, &compressed_data)?;
        
        let ratio = paradoxlf::compression_ratio(json_data.len(), compressed_data.len());
        log::info!("💾 Multiverse Archived (Ratio: {:.2}x) to {}", ratio, plf_path.display());
        
        Ok(SystemPulse::None)
    }

    fn handle_event(&mut self, _event: &crate::interaction::CausalEvent) -> Result<()> {
        Ok(())
    }
}

/// A driver that enables inter-kernel communication (Networking / Wormholes)
pub struct WormholeDriver {
    #[allow(dead_code)]
    _listen_addr: String,
    tx: tokio::sync::mpsc::Sender<NetworkMessage>,
    incoming_rx: tokio::sync::mpsc::Receiver<NetworkMessage>,
    #[allow(dead_code)]
    runtime: tokio::runtime::Runtime,
    pending_energy: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum NetworkMessage {
    Event {
        event: crate::interaction::CausalEvent,
    },
    SyncState {
        universe: crate::universe::Universe,
    },
}

impl WormholeDriver {
    pub fn new(listen_addr: &str, remote_peer: &str) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        
        let (tx, mut rx) = tokio::sync::mpsc::channel::<NetworkMessage>(100);
        let (incoming_tx, incoming_rx) = tokio::sync::mpsc::channel::<NetworkMessage>(100);
        let addr = listen_addr.to_string();
        let remote_peer_addr = remote_peer.to_string();
        
        let pending_energy = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let pe_task = pending_energy.clone();

        // Spawn background listener and sender
        rt.spawn(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await.ok();
            if let Some(l) = listener {
                log::info!("🌐 Wormhole Listener active on {}", addr);
                
                let itx = incoming_tx.clone();
                tokio::spawn(async move {
                    while let Ok((mut socket, peer)) = l.accept().await {
                        log::info!("🛸 Incoming entanglement from {}", peer);
                        let itx_inner = itx.clone();
                        tokio::spawn(async move {
                            use tokio::io::AsyncReadExt;
                            let mut buffer = [0u8; 1024];
                            if let Ok(n) = socket.read(&mut buffer).await {
                                if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&buffer[..n]) {
                                    let _ = itx_inner.send(msg).await;
                                }
                            }
                        });
                    }
                });
            }

            loop {
                tokio::select! {
                    // Outgoing messages from Kernel
                    Some(msg) = rx.recv() => {
                        let energy = match &msg {
                            NetworkMessage::Event { event } => event.energy_payload,
                            _ => 0.0,
                        };
                        
                        log::info!("🛰️ Projecting signal to remote kernel {}: {:?}", remote_peer_addr, msg);
                        if let Ok(mut stream) = tokio::net::TcpStream::connect(&remote_peer_addr).await {
                             let data = serde_json::to_vec(&msg).unwrap_or_default();
                             let _ = tokio::io::AsyncWriteExt::write_all(&mut stream, &data).await;
                        } else {
                             log::warn!("⚠️ Wormhole collapse: Remote peer {} unreachable", remote_peer_addr);
                        }
                        
                        let bits = (energy * 1000.0) as u64;
                        pe_task.fetch_sub(bits, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
        });

        Ok(Self {
            _listen_addr: listen_addr.to_string(),
            tx,
            incoming_rx,
            runtime: rt,
            pending_energy,
        })
    }
}

impl HardwareDriver for WormholeDriver {
    fn name(&self) -> &str {
        "Wormhole Driver (Network)"
    }

    fn sync(&mut self, _universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse> {
        // Collect messages from background task
        while let Ok(msg) = self.incoming_rx.try_recv() {
            match msg {
                NetworkMessage::Event { event } => {
                    log::info!("🛸 Photon materialized from wormhole: U{} -> U{}", event.source, event.target);
                    incoming_events.push(event);
                }
                _ => {}
            }
        }
        Ok(SystemPulse::None)
    }

    fn handle_event(&mut self, event: &crate::interaction::CausalEvent) -> Result<()> {
        let msg = NetworkMessage::Event { event: event.clone() };
        let energy = event.energy_payload;
        let bits = (energy * 1000.0) as u64;
        self.pending_energy.fetch_add(bits, std::sync::atomic::Ordering::Relaxed);
        let _ = self.tx.try_send(msg);
        Ok(())
    }

    fn pending_energy(&self) -> f64 {
        self.pending_energy.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1000.0
    }
}

/// A driver that serves a professional monitoring dashboard over HTTP
pub struct WebGatewayDriver {
    _port: u16,
    state_json: std::sync::Arc<tokio::sync::RwLock<String>>,
    runtime: tokio::runtime::Runtime,
}

impl WebGatewayDriver {
    pub fn new(port: u16) -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        
        let state_json = std::sync::Arc::new(tokio::sync::RwLock::new("{}".to_string()));
        let shared_state = state_json.clone();

        rt.spawn(async move {
            let addr = format!("0.0.0.0:{}", port);
            let listener = tokio::net::TcpListener::bind(&addr).await.ok();
            if let Some(l) = listener {
                log::info!("🌐 Web Dash active on http://127.0.0.1:{}", port);
                loop {
                    if let Ok((mut socket, _)) = l.accept().await {
                        let current_state = shared_state.read().await.clone();
                        tokio::spawn(async move {
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                                current_state.len(),
                                current_state
                            );
                            let _ = tokio::io::AsyncWriteExt::write_all(&mut socket, response.as_bytes()).await;
                        });
                    }
                }
            }
        });

        Self { _port: port, state_json, runtime: rt }
    }
}

impl HardwareDriver for WebGatewayDriver {
    fn name(&self) -> &str { "Web Monitoring Gateway" }
    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, _incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse> {
        let json = serde_json::to_string(universes).unwrap_or_default();
        let state_ref = self.state_json.clone();
        self.runtime.spawn(async move {
            let mut w = state_ref.write().await;
            *w = json;
        });
        Ok(SystemPulse::None)
    }

    fn handle_event(&mut self, _event: &crate::interaction::CausalEvent) -> Result<()> {
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

    fn sync(&mut self, _universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, _incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse> {
        self.sys.refresh_cpu_usage();
        
        let mut total_usage = 0.0;
        let cpus = self.sys.cpus();
        for cpu in cpus {
            total_usage += cpu.cpu_usage();
        }
        
        let avg_usage = if cpus.is_empty() { 0.0 } else { total_usage / cpus.len() as f32 };
        self.last_cpu_load = avg_usage;
        
        Ok(SystemPulse::None)
    }

    fn handle_event(&mut self, _event: &crate::interaction::CausalEvent) -> Result<()> {
        Ok(())
    }
}
/// A driver that injects random physical anomalies for stress testing (Phase 14)
pub struct ChaosMonkeyDriver {
    intensity: f64,
    step_count: u64,
}

impl ChaosMonkeyDriver {
    pub fn new(intensity: f64) -> Self {
        Self { intensity, step_count: 0 }
    }
}

impl HardwareDriver for ChaosMonkeyDriver {
    fn name(&self) -> &str { "Chaos Monkey (Entropy Injection)" }

    fn sync(&mut self, universes: &hashbrown::HashMap<UniverseID, crate::universe::Universe>, _incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> Result<SystemPulse> {
        self.step_count += 1;
        
        // Only act every 5 steps
        if self.step_count % 5 != 0 {
            return Ok(SystemPulse::None);
        }

        // High intensity network flood every 10 steps
        if self.step_count % 10 == 0 && self.intensity > 0.7 {
            log::info!("🐒 Chaos Monkey: Triggering High-Frequency Network Flash...");
            // Spawning many small events via the kernel would be better, 
            // but for now we'll just log that the monkey is messing with the fabric.
        }

        // Randomly pick a universe and mess with it
        let u_ids: Vec<UniverseID> = universes.keys().copied().filter(|&id| id.0 > 1).collect();
        if let Some(target) = u_ids.get((self.step_count as usize) % u_ids.len()) {
            if self.intensity > 0.5 {
                return Ok(SystemPulse::Sabotage(*target, self.intensity * 25.0));
            }
        }

        Ok(SystemPulse::None)
    }

    fn handle_event(&mut self, _event: &crate::interaction::CausalEvent) -> Result<()> {
        Ok(())
    }
}
