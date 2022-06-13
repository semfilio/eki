#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Time(pub f64);

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Value(pub f64);

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum TransientEvent {
    None,
    InstantaneousChange(Value, Time),
    ValveClosure(Value, Time, Time),
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
        }
    }

    pub fn time(&self) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(_, time) => time.0,
            TransientEvent::ValveClosure(_, event_time, _) => event_time.0,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(value, _) => value.0,
            TransientEvent::ValveClosure(exponent, _, _) => exponent.0,
        }
    }

    pub fn closing_time(&self) -> f64 {
        match self {
            TransientEvent::None => 0.0,
            TransientEvent::InstantaneousChange(_, _) => 0.0,
            TransientEvent::ValveClosure(_, _, closing_time) => closing_time.0,
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
                } else if time < closing_time.0 {
                    let tau = 1.0 - (time - event_time.0) / (closing_time.0 - event_time.0);
                    steady_open * tau.powf( exponent.0 )
                } else {
                    0.0
                }
            }
        }
    }
}