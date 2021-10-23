struct Plugboard {
    wiring: [u32; 26],
    count: u32,
}

impl Plugboard {
    fn new() -> Plugboard {
        Plugboard { 
            wiring: [u32::MAX; 26],
            count: 0,
        }
    }

    fn add(&self, p1: u32, p2: u32) -> Result {
        if p1 == p2 | p1 > 25 | p2 > 25 { 
            return Err
        }
        if self.wiring[p1 as usize] != u32::MAX {
            return Err
        } 
        if self.wiring[p2 as usize] != u32::MAX {
            return Err
        }
        self.wiring[p1 as usize] = p2;
        self.wiring[p2 as usize] = p1;
    }

    
}