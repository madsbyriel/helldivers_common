use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Binding {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stratagem {
    name: String,
    binding: Vec<Binding>
}

impl Stratagem {
    pub fn binding(&self) -> &[Binding] {
        return &self.binding
    }

    pub fn name(&self) -> &str {
        return &self.name
    }

    pub fn new(name: String, binding: Vec<Binding>) -> Self {
        return Self { name, binding }
    }

    pub fn all_stratagems() -> Vec<Stratagem> {
        vec![
            Stratagem::new("MG-43 Machine Gun".to_string(), vec![Binding::Down,Binding::Left,Binding::Down,Binding::Up,Binding::Right]),
            Stratagem::new("APW-1 Anti-Materiel Rifle".to_string(), vec![Binding::Down,Binding::Left,Binding::Right,Binding::Up,Binding::Down]),
            Stratagem::new("M-105 Stalwart".to_string(), vec![Binding::Down,Binding::Left,Binding::Down,Binding::Up,Binding::Up,Binding::Left]),
            Stratagem::new("EAT-17 Expendable Anti-Tank".to_string(), vec![Binding::Down,Binding::Down,Binding::Left,Binding::Up,Binding::Right]),
            Stratagem::new("GR-8 Recoilless Rifle".to_string(), vec![Binding::Down,Binding::Left,Binding::Right,Binding::Right,Binding::Left]),
            Stratagem::new("FLAM-40 Flamethrower".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Down,Binding::Up]),
            Stratagem::new("AC-8 Autocannon".to_string(), vec![Binding::Down,Binding::Left,Binding::Down,Binding::Up,Binding::Up,Binding::Right]),
            Stratagem::new("MG-206 Heavy Machine Gun".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Down,Binding::Down]),
            Stratagem::new("RL-77 Airburst Rocket Launcher".to_string(), vec![Binding::Down,Binding::Up,Binding::Up,Binding::Left,Binding::Right]),
            Stratagem::new("MLS-4X Commando".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Down,Binding::Right]),
            Stratagem::new("RS-422 Railgun".to_string(), vec![Binding::Down,Binding::Right,Binding::Down,Binding::Up,Binding::Left,Binding::Right]),
            Stratagem::new("FAF-14 Spear".to_string(), vec![Binding::Down,Binding::Down,Binding::Up,Binding::Down,Binding::Down]),
            Stratagem::new("StA-X3 W.A.S.P. Launcher".to_string(), vec![Binding::Down,Binding::Down,Binding::Up,Binding::Down,Binding::Right]),
            Stratagem::new("Orbital Gatling Barrage".to_string(), vec![Binding::Right,Binding::Down,Binding::Left,Binding::Up,Binding::Up]),
            Stratagem::new("Orbital Airburst Strike".to_string(), vec![Binding::Right,Binding::Right,Binding::Right]),
            Stratagem::new("Orbital 120mm HE Barrage".to_string(), vec![Binding::Right,Binding::Right,Binding::Down,Binding::Left,Binding::Right,Binding::Down]),
            Stratagem::new("Orbital 380mm HE Barrage".to_string(), vec![Binding::Right,Binding::Down,Binding::Up,Binding::Up,Binding::Left,Binding::Down,Binding::Down]),
            Stratagem::new("Orbital Walking Barrage".to_string(), vec![Binding::Right,Binding::Down,Binding::Right,Binding::Down,Binding::Right,Binding::Down]),
            Stratagem::new("Orbital Laser".to_string(), vec![Binding::Right,Binding::Down,Binding::Up,Binding::Right,Binding::Down]),
            Stratagem::new("Orbital Napalm Barrage".to_string(), vec![Binding::Right,Binding::Right,Binding::Down,Binding::Left,Binding::Right,Binding::Up]),
            Stratagem::new("Orbital Railcannon Strike".to_string(), vec![Binding::Right,Binding::Up,Binding::Down,Binding::Down,Binding::Right]),
            Stratagem::new("Eagle Strafing Run".to_string(), vec![Binding::Up,Binding::Right,Binding::Right]),
            Stratagem::new("Eagle Airstrike".to_string(), vec![Binding::Up,Binding::Right,Binding::Down,Binding::Right]),
            Stratagem::new("Eagle Cluster Bomb".to_string(), vec![Binding::Up,Binding::Right,Binding::Down,Binding::Down,Binding::Right]),
            Stratagem::new("Eagle Napalm Airstrike".to_string(), vec![Binding::Up,Binding::Right,Binding::Down,Binding::Up]),
            Stratagem::new("LIFT-850 Jump Pack".to_string(), vec![Binding::Down,Binding::Up,Binding::Up,Binding::Down,Binding::Up]),
            Stratagem::new("Eagle Smoke Strike".to_string(), vec![Binding::Up,Binding::Right,Binding::Up,Binding::Down]),
            Stratagem::new("Eagle 110mm Rocket Pods".to_string(), vec![Binding::Up,Binding::Right,Binding::Up,Binding::Left]),
            Stratagem::new("Eagle 500kg Bomb".to_string(), vec![Binding::Up,Binding::Right,Binding::Down,Binding::Down,Binding::Down]),
            Stratagem::new("M-102 Fast Recon Vehicle".to_string(), vec![Binding::Left,Binding::Down,Binding::Right,Binding::Down,Binding::Right,Binding::Down,Binding::Up]),
            Stratagem::new("Orbital Precision Strike".to_string(), vec![Binding::Right,Binding::Right,Binding::Up]),
            Stratagem::new("Orbital Gas Strike".to_string(), vec![Binding::Right,Binding::Right,Binding::Down,Binding::Right]),
            Stratagem::new("Orbital EMS Strike".to_string(), vec![Binding::Right,Binding::Right,Binding::Left,Binding::Down]),
            Stratagem::new("Orbital Smoke Strike".to_string(), vec![Binding::Right,Binding::Right,Binding::Down,Binding::Up]),
            Stratagem::new("E/MG-101 HMG Emplacement".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Right,Binding::Right,Binding::Left]),
            Stratagem::new("FX-12 Shield Generator Relay".to_string(), vec![Binding::Down,Binding::Down,Binding::Left,Binding::Right,Binding::Left,Binding::Right]),
            Stratagem::new("A/ARC-3 Tesla Tower".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Up,Binding::Left,Binding::Right]),
            Stratagem::new("E/GL-21 Grenadier Battlement".to_string(), vec![Binding::Down,Binding::Right,Binding::Down,Binding::Left,Binding::Right]),
            Stratagem::new("MD-6 Anti-Personnel Minefield".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Right]),
            Stratagem::new("B-1 Supply Pack".to_string(), vec![Binding::Down,Binding::Left,Binding::Down,Binding::Up,Binding::Up,Binding::Down]),
            Stratagem::new("GL-21 Grenade Launcher".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Left,Binding::Down]),
            Stratagem::new("LAS-98 Laser Cannon".to_string(), vec![Binding::Down,Binding::Left,Binding::Down,Binding::Up,Binding::Left]),
            Stratagem::new("MD-I4 Incendiary Mines".to_string(), vec![Binding::Down,Binding::Left,Binding::Left,Binding::Down]),
            Stratagem::new("AX/LAS-5 \"Guard Dog\" Rover".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Up,Binding::Right,Binding::Right]),
            Stratagem::new("SH-20 Ballistic Shield Backpack".to_string(), vec![Binding::Down,Binding::Left,Binding::Down,Binding::Down,Binding::Up,Binding::Left]),
            Stratagem::new("ARC-3 Arc Thrower".to_string(), vec![Binding::Down,Binding::Right,Binding::Down,Binding::Up,Binding::Left,Binding::Left]),
            Stratagem::new("MD-17 Anti-Tank Mines".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Up]),
            Stratagem::new("LAS-99 Quasar Cannon".to_string(), vec![Binding::Down,Binding::Down,Binding::Up,Binding::Left,Binding::Right]),
            Stratagem::new("SH-32 Shield Generator Pack".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Right,Binding::Left,Binding::Right]),
            Stratagem::new("MD-8 Gas Mines".to_string(), vec![Binding::Down,Binding::Left,Binding::Left,Binding::Right]),
            Stratagem::new("A/MG-43 Machine Gun Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Right,Binding::Up]),
            Stratagem::new("A/G-16 Gatling Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Left]),
            Stratagem::new("A/M-12 Mortar Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Right,Binding::Down]),
            Stratagem::new("AX/AR-23 \"Guard Dog\"".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Up,Binding::Right,Binding::Down]),
            Stratagem::new("A/AC-8 Autocannon Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Up,Binding::Left,Binding::Up]),
            Stratagem::new("A/MLS-4X Rocket Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Right,Binding::Left]),
            Stratagem::new("A/M-23 EMS Mortar Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Down,Binding::Right]),
            Stratagem::new("EXO-45 Patriot Exosuit".to_string(), vec![Binding::Left,Binding::Down,Binding::Right,Binding::Up,Binding::Left,Binding::Down,Binding::Down]),
            Stratagem::new("EXO-49 Emancipator Exosuit".to_string(), vec![Binding::Left,Binding::Down,Binding::Right,Binding::Up,Binding::Left,Binding::Down,Binding::Up]),
            Stratagem::new("TX-41 Sterilizer".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Down,Binding::Left]),
            Stratagem::new("AX/TX-13 \"Guard Dog\" Dog Breath".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Up,Binding::Right,Binding::Up]),
            Stratagem::new("SH-51 Directional Shield".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Right,Binding::Up,Binding::Up]),
            Stratagem::new("E/AT-12 Anti-Tank Emplacement".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Right,Binding::Right,Binding::Right]),
            Stratagem::new("A/FLAM-40 Flame Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Down,Binding::Up,Binding::Up]),
            Stratagem::new("B-100 Portable Hellbomb".to_string(), vec![Binding::Down,Binding::Right,Binding::Up,Binding::Up,Binding::Up]),
            Stratagem::new("LIFT-860 Hover Pack".to_string(), vec![Binding::Down,Binding::Up,Binding::Up,Binding::Down,Binding::Left,Binding::Right]),
            Stratagem::new("CQC-1 One True Flag".to_string(), vec![Binding::Down,Binding::Left,Binding::Right,Binding::Right,Binding::Up]),
            Stratagem::new("GL-52 De-Escalator".to_string(), vec![Binding::Down,Binding::Right,Binding::Up,Binding::Left,Binding::Right]),
            Stratagem::new("AX/ARC-3 \"Guard Dog\" K-9".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Up,Binding::Right,Binding::Left]),
            Stratagem::new("PLAS-45 Epoch".to_string(), vec![Binding::Down,Binding::Left,Binding::Up,Binding::Left,Binding::Right]),
            Stratagem::new("A/LAS-98 Laser Sentry".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Down,Binding::Up,Binding::Right]),
            Stratagem::new("LIFT-182 Warp Pack".to_string(), vec![Binding::Down,Binding::Left,Binding::Right,Binding::Down,Binding::Left,Binding::Right]),
            Stratagem::new("S-11 Speargun".to_string(), vec![Binding::Down,Binding::Right,Binding::Down,Binding::Left,Binding::Up,Binding::Right]),
            Stratagem::new("EAT-700 Expendable Napalm".to_string(), vec![Binding::Down,Binding::Down,Binding::Left,Binding::Up,Binding::Left]),
            Stratagem::new("MS-11 Solo Silo".to_string(), vec![Binding::Down,Binding::Up,Binding::Right,Binding::Down,Binding::Down]),
            Stratagem::new("M-1000 Maxigun".to_string(), vec![Binding::Down,Binding::Left,Binding::Right,Binding::Down,Binding::Up,Binding::Up]),
            Stratagem::new("CQC-9 Defoliation Tool".to_string(), vec![Binding::Down,Binding::Left,Binding::Right,Binding::Right,Binding::Down]),
            Stratagem::new("AX/FLAM-75 \"Guard Dog\" Hot Dog".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Up,Binding::Left,Binding::Left]),
            Stratagem::new("Reinforce".to_string(), vec![Binding::Up,Binding::Down,Binding::Right,Binding::Left,Binding::Up]),
            Stratagem::new("SoS Beacon".to_string(), vec![Binding::Up,Binding::Down,Binding::Right,Binding::Up]),
            Stratagem::new("Resupply".to_string(), vec![Binding::Down,Binding::Down,Binding::Up,Binding::Right]),
            Stratagem::new("Eagle Rearm".to_string(), vec![Binding::Up,Binding::Up,Binding::Left,Binding::Up,Binding::Right]),
            Stratagem::new("SSSD Delivery".to_string(), vec![Binding::Down,Binding::Down,Binding::Down,Binding::Up,Binding::Up]),
            Stratagem::new("Prospecting Drill".to_string(), vec![Binding::Down,Binding::Down,Binding::Left,Binding::Right,Binding::Down,Binding::Down]),
            Stratagem::new("Super Earth Flag".to_string(), vec![Binding::Down,Binding::Up,Binding::Down,Binding::Up]),
            Stratagem::new("Hellbomb".to_string(), vec![Binding::Down,Binding::Up,Binding::Left,Binding::Down,Binding::Up,Binding::Right,Binding::Down,Binding::Up]),
            Stratagem::new("Upload Data".to_string(), vec![Binding::Left,Binding::Right,Binding::Up,Binding::Up,Binding::Up]),
            Stratagem::new("Seismic Probe".to_string(), vec![Binding::Up,Binding::Up,Binding::Left,Binding::Right,Binding::Down,Binding::Down]),
            Stratagem::new("Orbital Illumination Flare".to_string(), vec![Binding::Right,Binding::Right,Binding::Left,Binding::Left]),
            Stratagem::new("SEAF Artillery".to_string(), vec![Binding::Right,Binding::Up,Binding::Up,Binding::Down]),
            Stratagem::new("Dark Fluid Vessel".to_string(), vec![Binding::Up,Binding::Left,Binding::Right,Binding::Down,Binding::Up,Binding::Up]),
            Stratagem::new("Tectonic Drill".to_string(), vec![Binding::Up,Binding::Down,Binding::Up,Binding::Down,Binding::Up,Binding::Down]),
            Stratagem::new("Hive Breaker Drill".to_string(), vec![Binding::Left,Binding::Up,Binding::Down,Binding::Right,Binding::Down,Binding::Down]),
            Stratagem::new("Cargo Container".to_string(), vec![Binding::Up,Binding::Up,Binding::Down,Binding::Down,Binding::Right,Binding::Down])
        ]
    }
}
