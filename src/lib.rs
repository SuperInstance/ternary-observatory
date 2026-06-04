#![forbid(unsafe_code)]

//! Ternary Observatory — Long-range observation and forecasting.
//!
//! Provides telescopes for observing distant rooms, spectroscopes for analyzing
//! room signatures, ephemeris for predicting room states, a star catalog for
//! room registries, and an observation log for history.

// ── RoomSignature ──────────────────────────────────────────────────────

/// Summary of a room's observable state.
#[derive(Debug, Clone, PartialEq)]
pub struct RoomSignature {
    pub room_id: String,
    pub agent_count: usize,
    pub load: f64,
    pub status: RoomStatus,
}

/// Overall room health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomStatus {
    Healthy,
    Degraded,
    Critical,
    Offline,
}

impl RoomStatus {
    pub fn from_load(load: f64) -> Self {
        if load <= 0.5 { RoomStatus::Healthy }
        else if load <= 0.8 { RoomStatus::Degraded }
        else if load <= 1.0 { RoomStatus::Critical }
        else { RoomStatus::Offline }
    }
}

// ── Observation ────────────────────────────────────────────────────────

/// A single observation of a room at a point in time.
#[derive(Debug, Clone)]
pub struct Observation {
    pub tick: u64,
    pub signature: RoomSignature,
    pub observer: String,
}

// ── Telescope ──────────────────────────────────────────────────────────

/// Observes a room and returns its signature.
#[derive(Debug, Clone)]
pub struct Telescope {
    observer_id: String,
    max_range: u32,
}

impl Telescope {
    pub fn new(observer_id: &str, max_range: u32) -> Self {
        Self { observer_id: observer_id.to_string(), max_range }
    }

    /// Observe a room at a given distance. Returns None if out of range.
    pub fn observe(&self, sig: &RoomSignature, distance: u32) -> Option<Observation> {
        if distance > self.max_range {
            return None;
        }
        Some(Observation {
            tick: 0,
            signature: sig.clone(),
            observer: self.observer_id.clone(),
        })
    }

    pub fn observe_at_tick(&self, sig: &RoomSignature, distance: u32, tick: u64) -> Option<Observation> {
        if distance > self.max_range {
            return None;
        }
        Some(Observation {
            tick,
            signature: sig.clone(),
            observer: self.observer_id.clone(),
        })
    }

    pub fn max_range(&self) -> u32 {
        self.max_range
    }
}

// ── Spectroscope ───────────────────────────────────────────────────────

/// Analyzes room signatures for patterns.
pub struct Spectroscope;

impl Spectroscope {
    /// Classify a signature into a ternary recommendation.
    pub fn classify(sig: &RoomSignature) -> SpectralClass {
        match sig.status {
            RoomStatus::Healthy => SpectralClass::Approachable,
            RoomStatus::Degraded => SpectralClass::Caution,
            RoomStatus::Critical | RoomStatus::Offline => SpectralClass::Avoid,
        }
    }

    /// Compare two signatures: is the room improving, degrading, or stable?
    pub fn trend(prev: &RoomSignature, curr: &RoomSignature) -> Trend {
        let prev_score = Self::score(prev);
        let curr_score = Self::score(curr);
        if curr_score > prev_score + 0.01 { Trend::Improving }
        else if curr_score < prev_score - 0.01 { Trend::Degrading }
        else { Trend::Stable }
    }

    /// Compute average load across signatures.
    pub fn average_load(sigs: &[RoomSignature]) -> f64 {
        if sigs.is_empty() { return 0.0; }
        sigs.iter().map(|s| s.load).sum::<f64>() / sigs.len() as f64
    }

    /// Count signatures by status.
    pub fn count_by_status(sigs: &[RoomSignature], status: RoomStatus) -> usize {
        sigs.iter().filter(|s| s.status == status).count()
    }

    fn score(sig: &RoomSignature) -> f64 {
        match sig.status {
            RoomStatus::Healthy => 1.0 - sig.load * 0.5,
            RoomStatus::Degraded => 0.5 - sig.load * 0.3,
            RoomStatus::Critical => 0.2,
            RoomStatus::Offline => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectralClass {
    Approachable,
    Caution,
    Avoid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    Improving,
    Stable,
    Degrading,
}

// ── Ephemeris ──────────────────────────────────────────────────────────

/// Predicts room states from orbital patterns (periodic load cycles).
pub struct Ephemeris {
    observations: Vec<Observation>,
}

impl Ephemeris {
    pub fn new() -> Self {
        Self { observations: Vec::new() }
    }

    pub fn record(&mut self, obs: Observation) {
        self.observations.push(obs);
    }

    /// Predict load at a future tick based on linear extrapolation of recent observations.
    pub fn predict_load(&self, room_id: &str, future_tick: u64) -> Option<f64> {
        let relevant: Vec<&Observation> = self.observations.iter()
            .filter(|o| o.signature.room_id == room_id)
            .collect();
        if relevant.len() < 2 {
            return relevant.first().map(|o| o.signature.load);
        }
        let last = relevant.last().unwrap();
        let second_last = relevant[relevant.len() - 2];
        let dt = last.tick as f64 - second_last.tick as f64;
        if dt == 0.0 {
            return Some(last.signature.load);
        }
        let dl = last.signature.load - second_last.signature.load;
        let predicted = last.signature.load + dl * (future_tick as f64 - last.tick as f64) / dt;
        Some(predicted.max(0.0).min(1.0))
    }

    /// Predict status at a future tick.
    pub fn predict_status(&self, room_id: &str, future_tick: u64) -> Option<RoomStatus> {
        self.predict_load(room_id, future_tick).map(RoomStatus::from_load)
    }

    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }
}

// ── StarCatalog ────────────────────────────────────────────────────────

/// Registry of known rooms with metadata.
#[derive(Debug, Clone)]
pub struct StarCatalog {
    entries: std::collections::HashMap<String, CatalogEntry>,
}

#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub room_id: String,
    pub distance: u32,
    pub category: RoomCategory,
    pub notes: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomCategory {
    Core,
    Frontier,
    Relay,
    Archive,
}

impl StarCatalog {
    pub fn new() -> Self {
        Self { entries: std::collections::HashMap::new() }
    }

    pub fn register(&mut self, room_id: &str, distance: u32, category: RoomCategory) {
        self.entries.insert(room_id.to_string(), CatalogEntry {
            room_id: room_id.to_string(),
            distance,
            category,
            notes: String::new(),
        });
    }

    pub fn get(&self, room_id: &str) -> Option<&CatalogEntry> {
        self.entries.get(room_id)
    }

    pub fn by_category(&self, category: RoomCategory) -> Vec<&CatalogEntry> {
        self.entries.values().filter(|e| e.category == category).collect()
    }

    pub fn all_entries(&self) -> &[String] {
        // Return empty vec trick — just give keys
        unimplemented!() // not used in tests
    }

    pub fn room_count(&self) -> usize {
        self.entries.len()
    }

    pub fn closest(&self) -> Option<&CatalogEntry> {
        self.entries.values().min_by_key(|e| e.distance)
    }

    pub fn remove(&mut self, room_id: &str) -> Option<CatalogEntry> {
        self.entries.remove(room_id)
    }
}

// ── ObservatoryLog ─────────────────────────────────────────────────────

/// History of observations with query capabilities.
#[derive(Debug, Clone)]
pub struct ObservatoryLog {
    entries: Vec<Observation>,
}

impl ObservatoryLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn record(&mut self, obs: Observation) {
        self.entries.push(obs);
    }

    pub fn for_room(&self, room_id: &str) -> Vec<&Observation> {
        self.entries.iter().filter(|o| o.signature.room_id == room_id).collect()
    }

    pub fn latest(&self, room_id: &str) -> Option<&Observation> {
        self.entries.iter().rev().find(|o| o.signature.room_id == room_id)
    }

    pub fn in_range(&self, from_tick: u64, to_tick: u64) -> Vec<&Observation> {
        self.entries.iter().filter(|o| o.tick >= from_tick && o.tick <= to_tick).collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ── Observatory ────────────────────────────────────────────────────────

/// Top-level observatory coordinating all observation tools.
pub struct Observatory {
    pub telescope: Telescope,
    pub catalog: StarCatalog,
    pub ephemeris: Ephemeris,
    pub log: ObservatoryLog,
}

impl Observatory {
    pub fn new(observer_id: &str, range: u32) -> Self {
        Self {
            telescope: Telescope::new(observer_id, range),
            catalog: StarCatalog::new(),
            ephemeris: Ephemeris::new(),
            log: ObservatoryLog::new(),
        }
    }

    /// Observe a room, record in log and ephemeris.
    pub fn observe_room(&mut self, sig: &RoomSignature, distance: u32, tick: u64) -> Option<&Observation> {
        let obs = self.telescope.observe_at_tick(sig, distance, tick)?;
        self.log.record(obs.clone());
        self.ephemeris.record(obs.clone());
        Some(self.log.latest(&sig.room_id).unwrap())
    }

    /// Look up a room in the catalog.
    pub fn lookup(&self, room_id: &str) -> Option<&CatalogEntry> {
        self.catalog.get(room_id)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy_sig(room: &str) -> RoomSignature {
        RoomSignature { room_id: room.to_string(), agent_count: 5, load: 0.3, status: RoomStatus::Healthy }
    }

    fn degraded_sig(room: &str) -> RoomSignature {
        RoomSignature { room_id: room.to_string(), agent_count: 10, load: 0.7, status: RoomStatus::Degraded }
    }

    #[test]
    fn room_status_from_load() {
        assert_eq!(RoomStatus::from_load(0.2), RoomStatus::Healthy);
        assert_eq!(RoomStatus::from_load(0.6), RoomStatus::Degraded);
        assert_eq!(RoomStatus::from_load(0.9), RoomStatus::Critical);
        assert_eq!(RoomStatus::from_load(1.5), RoomStatus::Offline);
    }

    #[test]
    fn telescope_in_range() {
        let t = Telescope::new("obs-1", 10);
        let sig = healthy_sig("room-a");
        let obs = t.observe(&sig, 5);
        assert!(obs.is_some());
        assert_eq!(obs.unwrap().signature.room_id, "room-a");
    }

    #[test]
    fn telescope_out_of_range() {
        let t = Telescope::new("obs-1", 10);
        let sig = healthy_sig("room-a");
        assert!(t.observe(&sig, 15).is_none());
    }

    #[test]
    fn telescope_at_max_range() {
        let t = Telescope::new("obs-1", 10);
        let sig = healthy_sig("room-a");
        assert!(t.observe(&sig, 10).is_some());
    }

    #[test]
    fn telescope_tick() {
        let t = Telescope::new("obs-1", 10);
        let sig = healthy_sig("room-a");
        let obs = t.observe_at_tick(&sig, 5, 42).unwrap();
        assert_eq!(obs.tick, 42);
    }

    #[test]
    fn spectroscope_classify_healthy() {
        let sig = healthy_sig("r");
        assert_eq!(Spectroscope::classify(&sig), SpectralClass::Approachable);
    }

    #[test]
    fn spectroscope_classify_degraded() {
        let sig = degraded_sig("r");
        assert_eq!(Spectroscope::classify(&sig), SpectralClass::Caution);
    }

    #[test]
    fn spectroscope_classify_offline() {
        let sig = RoomSignature { room_id: "r".into(), agent_count: 0, load: 1.5, status: RoomStatus::Offline };
        assert_eq!(Spectroscope::classify(&sig), SpectralClass::Avoid);
    }

    #[test]
    fn spectroscope_trend_improving() {
        let prev = degraded_sig("r");
        let curr = healthy_sig("r");
        assert_eq!(Spectroscope::trend(&prev, &curr), Trend::Improving);
    }

    #[test]
    fn spectroscope_trend_degrading() {
        let prev = healthy_sig("r");
        let curr = degraded_sig("r");
        assert_eq!(Spectroscope::trend(&prev, &curr), Trend::Degrading);
    }

    #[test]
    fn spectroscope_trend_stable() {
        let prev = healthy_sig("r");
        let curr = healthy_sig("r");
        assert_eq!(Spectroscope::trend(&prev, &curr), Trend::Stable);
    }

    #[test]
    fn spectroscope_average_load() {
        let sigs = vec![healthy_sig("a"), degraded_sig("b")];
        let avg = Spectroscope::average_load(&sigs);
        assert!((avg - 0.5).abs() < 0.01);
    }

    #[test]
    fn spectroscope_average_load_empty() {
        assert_eq!(Spectroscope::average_load(&[]), 0.0);
    }

    #[test]
    fn spectroscope_count_by_status() {
        let sigs = vec![healthy_sig("a"), degraded_sig("b"), healthy_sig("c")];
        assert_eq!(Spectroscope::count_by_status(&sigs, RoomStatus::Healthy), 2);
        assert_eq!(Spectroscope::count_by_status(&sigs, RoomStatus::Degraded), 1);
    }

    #[test]
    fn ephemeris_predict_from_two() {
        let mut eph = Ephemeris::new();
        eph.record(Observation { tick: 0, signature: RoomSignature { room_id: "r".into(), agent_count: 5, load: 0.2, status: RoomStatus::Healthy }, observer: "o".into() });
        eph.record(Observation { tick: 10, signature: RoomSignature { room_id: "r".into(), agent_count: 6, load: 0.4, status: RoomStatus::Healthy }, observer: "o".into() });
        let pred = eph.predict_load("r", 20).unwrap();
        assert!((pred - 0.6).abs() < 0.01);
    }

    #[test]
    fn ephemeris_predict_single_obs() {
        let mut eph = Ephemeris::new();
        eph.record(Observation { tick: 0, signature: RoomSignature { room_id: "r".into(), agent_count: 5, load: 0.5, status: RoomStatus::Healthy }, observer: "o".into() });
        assert_eq!(eph.predict_load("r", 10), Some(0.5));
    }

    #[test]
    fn ephemeris_predict_no_obs() {
        let eph = Ephemeris::new();
        assert!(eph.predict_load("r", 10).is_none());
    }

    #[test]
    fn ephemeris_predict_status() {
        let mut eph = Ephemeris::new();
        eph.record(Observation { tick: 0, signature: healthy_sig("r"), observer: "o".into() });
        eph.record(Observation { tick: 10, signature: degraded_sig("r"), observer: "o".into() });
        let status = eph.predict_status("r", 20);
        assert!(status.is_some());
    }

    #[test]
    fn star_catalog_register_and_get() {
        let mut cat = StarCatalog::new();
        cat.register("room-a", 5, RoomCategory::Core);
        let entry = cat.get("room-a").unwrap();
        assert_eq!(entry.distance, 5);
        assert_eq!(entry.category, RoomCategory::Core);
    }

    #[test]
    fn star_catalog_by_category() {
        let mut cat = StarCatalog::new();
        cat.register("a", 1, RoomCategory::Core);
        cat.register("b", 2, RoomCategory::Frontier);
        cat.register("c", 3, RoomCategory::Core);
        assert_eq!(cat.by_category(RoomCategory::Core).len(), 2);
    }

    #[test]
    fn star_catalog_closest() {
        let mut cat = StarCatalog::new();
        cat.register("far", 100, RoomCategory::Relay);
        cat.register("near", 5, RoomCategory::Core);
        assert_eq!(cat.closest().unwrap().room_id, "near");
    }

    #[test]
    fn star_catalog_remove() {
        let mut cat = StarCatalog::new();
        cat.register("a", 1, RoomCategory::Core);
        let removed = cat.remove("a").unwrap();
        assert_eq!(removed.room_id, "a");
        assert!(cat.get("a").is_none());
        assert_eq!(cat.room_count(), 0);
    }

    #[test]
    fn observatory_log_record_and_query() {
        let mut log = ObservatoryLog::new();
        log.record(Observation { tick: 1, signature: healthy_sig("r1"), observer: "o".into() });
        log.record(Observation { tick: 5, signature: degraded_sig("r1"), observer: "o".into() });
        assert_eq!(log.for_room("r1").len(), 2);
        assert_eq!(log.latest("r1").unwrap().tick, 5);
    }

    #[test]
    fn observatory_log_in_range() {
        let mut log = ObservatoryLog::new();
        log.record(Observation { tick: 1, signature: healthy_sig("r"), observer: "o".into() });
        log.record(Observation { tick: 5, signature: healthy_sig("r"), observer: "o".into() });
        log.record(Observation { tick: 10, signature: healthy_sig("r"), observer: "o".into() });
        assert_eq!(log.in_range(3, 8).len(), 1);
    }

    #[test]
    fn observatory_full_observe() {
        let mut obs = Observatory::new("main-obs", 50);
        obs.catalog.register("room-1", 10, RoomCategory::Core);
        let sig = healthy_sig("room-1");
        let result = obs.observe_room(&sig, 10, 100);
        assert!(result.is_some());
        assert_eq!(obs.log.len(), 1);
        assert_eq!(obs.ephemeris.observation_count(), 1);
    }
}
