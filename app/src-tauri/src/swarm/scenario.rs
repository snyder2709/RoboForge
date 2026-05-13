pub struct ScenarioFrame {
    pub duration_ms: u64,
    pub servos: Vec<f64>,
}

pub fn scenario_frames(name: &str) -> Option<Vec<ScenarioFrame>> {
    fn n() -> Vec<f64> {
        vec![90.0; 20]
    }
    fn f(mut v: Vec<f64>, updates: &[(usize, f64)]) -> Vec<f64> {
        for &(i, val) in updates {
            v[i] = val;
        }
        v
    }
    Some(match name {
        "wave" => vec![
            ScenarioFrame { duration_ms: 400, servos: f(n(), &[(6, 45.), (7, 70.), (8, 90.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6, 45.), (7, 70.), (8, 130.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6, 45.), (7, 70.), (8, 55.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6, 45.), (7, 70.), (8, 130.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6, 45.), (7, 70.), (8, 55.)]) },
            ScenarioFrame { duration_ms: 400, servos: n() },
        ],
        "walk" => vec![
            ScenarioFrame {
                duration_ms: 350,
                servos: f(n(), &[(10, 65.), (12, 115.), (13, 110.), (15, 115.), (17, 65.), (18, 70.), (2, 115.), (6, 65.)]),
            },
            ScenarioFrame {
                duration_ms: 350,
                servos: f(n(), &[(10, 115.), (12, 65.), (13, 70.), (15, 65.), (17, 115.), (18, 110.), (2, 65.), (6, 115.)]),
            },
            ScenarioFrame {
                duration_ms: 350,
                servos: f(n(), &[(10, 65.), (12, 115.), (13, 110.), (15, 115.), (17, 65.), (18, 70.), (2, 115.), (6, 65.)]),
            },
            ScenarioFrame {
                duration_ms: 350,
                servos: f(n(), &[(10, 115.), (12, 65.), (13, 70.), (15, 65.), (17, 115.), (18, 110.), (2, 65.), (6, 115.)]),
            },
            ScenarioFrame { duration_ms: 350, servos: n() },
        ],
        "jump" => vec![
            ScenarioFrame {
                duration_ms: 300,
                servos: f(n(), &[(10, 120.), (12, 140.), (13, 65.), (15, 120.), (17, 140.), (18, 65.)]),
            },
            ScenarioFrame {
                duration_ms: 150,
                servos: f(n(), &[(10, 60.), (12, 60.), (13, 115.), (15, 60.), (17, 60.), (18, 115.)]),
            },
            ScenarioFrame { duration_ms: 250, servos: n() },
            ScenarioFrame {
                duration_ms: 200,
                servos: f(n(), &[(10, 110.), (12, 120.), (13, 75.), (15, 110.), (17, 120.), (18, 75.)]),
            },
            ScenarioFrame { duration_ms: 300, servos: n() },
        ],
        _ => return None,
    })
}
