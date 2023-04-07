#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Time(pub f64);

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Value(pub f64);

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum TransientEvent {
    None,
    InstantaneousChange(Value, Time),
    ValveClosure(Value, Time, Time),
    ValveOpening(Value, Time, Time),
    PumpShutdown(Value, Time, Time),
    PumpStartup(Value, Value, Time, Time),
}

impl Default for TransientEvent {
    fn default() -> Self {
        TransientEvent::None
    }
}

impl TransientEvent {
    pub fn text(&self) -> String {
        match self {
            TransientEvent::None => "None".to_string(),
            TransientEvent::InstantaneousChange(_, _) => "Instantaneous change".to_string(),
            TransientEvent::ValveClosure(_, _, _) => "Valve closure".to_string(),
            TransientEvent::ValveOpening(_, _, _) => "Valve opening".to_string(),
            TransientEvent::PumpShutdown(_,_,_) => "Linear shutdown".to_string(),
            TransientEvent::PumpStartup(_,_,_,_) => "Linear startup".to_string(),
        }
    }

    pub fn time(&self) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(_, time) => time.0,
            TransientEvent::ValveClosure(_, event_time, _) => event_time.0,
            TransientEvent::ValveOpening(_, event_time, _) => event_time.0,
            TransientEvent::PumpShutdown(_,event_time, _) => event_time.0,
            TransientEvent::PumpStartup(_,_, event_time, _) => event_time.0,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(value, _) => value.0,
            TransientEvent::ValveClosure(exponent,_,_) => exponent.0,
            TransientEvent::ValveOpening(exponent,_,_) => exponent.0,
            TransientEvent::PumpShutdown(exponent,_,_) => exponent.0,
            TransientEvent::PumpStartup(value,_,_,_) => value.0,
        }
    }

    pub fn closing_time(&self) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(_, _) => 0.0,
            TransientEvent::ValveClosure(_, _, closing_time) => closing_time.0,
            TransientEvent::ValveOpening(_, _, closing_time) => closing_time.0,
            TransientEvent::PumpShutdown(_,_, shutdown_time) => shutdown_time.0,
            TransientEvent::PumpStartup(_,_,_,startup_time) => startup_time.0,
        }
    }

    pub fn open_percent(&self, time: f64, steady_open: f64 ) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(value, event_time) => {
                if time < event_time.0 {
                    steady_open
                } else {
                    value.0
                }
            },
            TransientEvent::ValveClosure( exponent, event_time, closing_time) => {
                if time < event_time.0 {
                    steady_open
                } else if time < event_time.0 + closing_time.0 {
                    let a = 1.0 - ((time - event_time.0) / (closing_time.0));
                    steady_open * a.powf( exponent.0 )
                } else {
                    0.0
                }
            },
            TransientEvent::ValveOpening( exponent, event_time, closing_time) => {
                if time < event_time.0 {
                    steady_open
                } else if time < event_time.0 + closing_time.0 {
                    let b = (time - event_time.0) / (closing_time.0);
                    steady_open + (1.0 - steady_open) * b.powf( exponent.0 )
                } else {
                    1.0
                }
            },
            TransientEvent::PumpShutdown(_,_,_) => 0.0,
            TransientEvent::PumpStartup(_,_,_,_) => 0.0,
        }
    }

    pub fn pump_speed(&self, time: f64, steady_speed: f64 ) -> f64 {
        match self {
            TransientEvent::None => steady_speed,
            TransientEvent::InstantaneousChange(_,_) => steady_speed, //TODO: implement instantaneous change
            TransientEvent::ValveClosure(_,_,_) => steady_speed,
            TransientEvent::ValveOpening(_,_,_) => steady_speed,
            TransientEvent::PumpShutdown(exponent, event_time, shutdown_time) => {
                if time < event_time.0 {
                    steady_speed
                } else if time < event_time.0 + shutdown_time.0 {
                    let tau = 1.0 - (time - event_time.0) / shutdown_time.0;
                    steady_speed * tau.powf( exponent.0 )
                } else {
                    0.0
                }
            },
            TransientEvent::PumpStartup( value, exponent, event_time, startup_time) => {
                if time < event_time.0 {
                    0.0
                } else if time < event_time.0 + startup_time.0 {
                    let tau = (time - event_time.0) / startup_time.0;
                    value.0 * tau.powf( exponent.0 )
                } else {
                    value.0
                }
            },
        }
    }
}