use rand::prelude::*;
fn random_number(start_inclusive:f32, end_inclusive:f32) -> f32
{ 
    rand::thread_rng()
        .gen::<f32>() * (end_inclusive - start_inclusive) + start_inclusive
}

