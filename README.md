# ternary-observatory

Long-range observation and forecasting of room states in ternary fleets.

## Why This Exists

When you manage a fleet of agents across many rooms, you need to know what's happening everywhere — not just where you are. You also need to predict what will happen. Ternary-observatory provides telescopes (observe distant rooms), spectroscopes (analyze patterns), ephemeris tables (predict future states), and a star catalog (registry of known rooms).

## Core Concepts

- **Telescope**: Observes a room at a given distance, returning its signature. Has a maximum observation range.
- **Room signature**: A summary of a room's state — agent count, load factor, and health status (Healthy/Degraded/Critical/Offline).
- **Spectroscope**: Analyzes signatures to classify rooms (Approachable/Caution/Avoid) and detect trends (Improving/Stable/Degrading).
- **Ephemeris**: Records observation time series and predicts future load via linear extrapolation.
- **Star catalog**: Registry of known rooms with distance and category metadata.
- **Observation log**: Append-only history of all observations with time-range queries.

## Quick Start

```toml
[dependencies]
ternary-observatory = "0.1"
```

```rust
use ternary_observatory::*;

let mut obs = Observatory::new("control-tower", 100);
obs.catalog.register("engine-room", 10, RoomCategory::Core);

let sig = RoomSignature {
    room_id: "engine-room".into(),
    agent_count: 5,
    load: 0.3,
    status: RoomStatus::Healthy,
};
obs.observe_room(&sig, 10, 0);

let class = Spectroscope::classify(&sig);
assert_eq!(class, SpectralClass::Approachable);
```

## API Overview

| Type | Description |
|------|-------------|
| `RoomSignature` | Observable state summary of a room |
| `RoomStatus` | Health enum: Healthy, Degraded, Critical, Offline |
| `Observation` | A timestamped observation of a room |
| `Telescope` | Observe rooms at distance |
| `Spectroscope` | Classify and trend-analyze signatures |
| `SpectralClass` | Ternary classification: Approachable, Caution, Avoid |
| `Trend` | Direction: Improving, Stable, Degrading |
| `Ephemeris` | Time-series prediction of room states |
| `StarCatalog` | Registry of known rooms with metadata |
| `ObservatoryLog` | Append-only observation history |
| `Observatory` | Top-level coordinator for all tools |

## How It Works

`Telescope` is simple: given a room signature and a distance, it returns an `Observation` if the distance is within range. No network calls — it operates on data you provide.

`Spectroscope` is stateless. `classify()` maps room status to a ternary spectral class. `trend()` compares two signatures by computing an internal health score and checking the delta. Load averages and status counts are straightforward aggregations.

`Ephemeris` stores observations in a Vec and predicts future load by linear extrapolation from the two most recent observations for a given room. Predictions are clamped to [0.0, 1.0]. With fewer than two observations, it returns the most recent load or None.

`Observatory` ties it all together: `observe_room()` runs the telescope, records to both the log and ephemeris, and returns the observation.

## Known Limitations

- Prediction is purely linear — no seasonal patterns, no exponential smoothing, no anomaly detection.
- No concept of observation uncertainty or confidence intervals.
- `StarCatalog` doesn't support updates — entries must be removed and re-registered.
- Telescope range is static; no adaptive range based on conditions.
- All data is in-memory; no persistence or serialization.

## Use Cases

- **Datacenter monitoring**: Observe server rooms, predict load spikes, classify which rooms are safe to route traffic to.
- **Game world oversight**: A master server watches all zone servers, predicting which will hit capacity.
- **Fleet health dashboard**: Track agent counts and load across rooms, detect degrading trends before they become critical.

## Ecosystem Context

Part of the SuperInstance ternary fleet. `ternary-beacon` feeds room discovery data into the star catalog. `ternary-navigator` uses observation data to adjust pathfinding weights. `ternary-shipyard` agents report health data that the observatory consumes.

## License

MIT

## See Also
- **ternary-beacon** — related
- **ternary-navigator** — related
- **ternary-compass** — related
- **ternary-constellation** — related
- **ternary-event** — related

