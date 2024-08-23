use super::*;
pub struct AntimatterCollector {
    buffer: f32,
}
impl Machine for AntimatterCollector {
    fn draw_gui(&mut self) -> Result<Rect> {
        todo!()
    }

    fn update(&mut self, map: &mut Map, dt: f32) -> Result<()> {
        self.buffer += 1.*dt;
        Ok(())
    }

    fn ty(&self) -> Tower {
        Tower::AntimatterCollector
    }
    
    fn serialize(&self) -> String {
        format!("")
    }
}

pub fn new() -> AntimatterCollector {
    AntimatterCollector { buffer: 0. }
}
pub fn deserialize(raw: &str) -> Result<AntimatterCollector> {
    Ok(new())
}