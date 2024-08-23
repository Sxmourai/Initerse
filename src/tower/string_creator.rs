// use super::*;

// pub struct StringCreator {
//     buffer: f32,
//     collect_speed: f32,
//     name: String,
// }
// impl StringCreator {
//     pub fn new() -> Self {
//         Self {
//             buffer: 0.,
//             collect_speed: 1.,
//             name: "String creator".to_string(),
//         }
//     }
// }
// impl Machine for StringCreator {
//     fn draw_gui(&mut self) -> Result<Rect> {
//         todo!()
//     }


//     fn update(&mut self, map: &mut Map, dt: f32) -> Result<()> {
//         self.buffer += self.collect_speed * dt;
//         Ok(())
//     }

//     fn ty(&self) -> Tower {
//         Tower::StringCreator
//     }
    
//     fn serialize(&self) -> String {
//         todo!()
//     }
// }