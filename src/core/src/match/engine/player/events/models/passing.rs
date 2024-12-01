use nalgebra::Vector3;

#[derive(Debug)]
pub struct PassingEventModel {
    pub from_player_id: u32,
    pub pass_target: Vector3<f32>,
    pub pass_force: f64
}

impl PassingEventModel {
    pub fn build() -> PassingEventBuilder{
        PassingEventBuilder::new()
    }
}

pub struct PassingEventBuilder {
    from_player_id: Option<u32>,
    pass_target: Option<Vector3<f32>>,
    pass_force: Option<f64>
}

impl Default for PassingEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PassingEventBuilder {
    pub fn new() -> Self {
        PassingEventBuilder {
            from_player_id: None,
            pass_target: None,
            pass_force: None,
        }
    }
    
    pub fn from_player_id(mut self, from_player_id: u32) -> Self {
        self.from_player_id = Some(from_player_id);
        self
    }

    pub fn target(mut self, pass_target: Vector3<f32>) -> Self {
        self.pass_target = Some(pass_target);
        self
    }  
    
    pub fn force(mut self, pass_force: f64) -> Self {
        self.pass_force = Some(pass_force);
        self
    }    

    pub fn build(self) -> PassingEventModel {
        PassingEventModel {
            from_player_id: self.from_player_id.unwrap(),
            pass_target: self.pass_target.unwrap(),
            pass_force: self.pass_force.unwrap(),
        }
    }
}
