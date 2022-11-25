use crate::Position;
use bevy::prelude::*;
use std::fmt;

#[derive(Clone)]
pub(crate) enum Sensor {
    Age,
    BoundaryDist,
    BoundaryDistX,
    BoundaryDistY,
    LastMoveDirX,
    LastMoveDirY,
    LocX,
    LocY,
    LongProbePopFwd,
    LongProbeBarFwd,
    BarrierFwd,
    BarrierLr,
    Osc1,
    Population,
    PopulationFws,
    PopulationLr,
    Random,
    Signal0,
    Signal0Fwd,
    Signal0Lr,
    GeneticSimFwd,
}

#[derive(Clone)]
pub(crate) enum Action {
    MoveEast,
    MoveWest,
    MoveNorth,
    MoveSouth,
    MoveForward,
    MoveX,
    MoveY,
    SetResponsiveness,
    SetOscillatorPeriod,
    EmitSignal0,
    KillForward,
    MoveReverse,
    MoveLeft,
    MoveRight,
    MoveRl,
    MoveRandom,
    SetLongProbeDist,
}

#[derive(Component, Clone)]
pub(crate) struct Organism {
    pub(crate) id: i32,
    pub(crate) dna: Dna,
    pub(crate) brain: Brain,
    pub(crate) health: bool,
    pub(crate) loc: Position,
    pub(crate) birth_loc: Position,
    pub(crate) age: i32,
}

#[derive(Component, Clone)]
pub(crate) struct Brain {
    pub(crate) sensor_neurons: Vec<SensorNeuron>,
    pub(crate) action_neurons: Vec<ActionNeuron>,
}

impl Brain {
    pub(crate) fn new() -> Self {
        Self {
            sensor_neurons: vec![],
            action_neurons: vec![],
        }
    }
}

#[derive(Component, Clone)]
pub(crate) struct SensorNeuron {
    pub(crate) name: Sensor,
    pub(crate) weight: i32,
}

#[derive(Component, Clone)]
pub(crate) struct ActionNeuron {
    pub(crate) name: Action,
    pub(crate) weight: i32,
}

impl Organism {
    pub(crate) fn new(id: i32) -> Self {
        Self {
            id,
            dna: Dna::random(),
            brain: Brain::new(),
            health: false,
            loc: Position::new(),
            birth_loc: Position::new(),
            age: 0,
        }
    }
}

pub(crate) type RawDna = [u8; 10];

#[derive(Component, Clone)]
pub(crate) struct Dna {
    pub(crate) dna: RawDna,
}

impl Dna {
    pub(crate) fn new(dna: RawDna) -> Self {
        Self { dna }
    }

    pub(crate) fn random() -> Self {
        Self {
            dna: rand::random::<RawDna>(),
        }
    }

    fn get_bit_at(input: u8, n: u8) -> Option<bool> {
        if n < 8 {
            Some(input & (1 << n) != 0)
        } else {
            None
        }
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for int in self.dna {
            write!(f, "{},", int)?;
        }
        Ok(())
    }
}
