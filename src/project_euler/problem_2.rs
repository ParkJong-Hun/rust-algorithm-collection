pub fn problem_2_1() -> u32 {
    let mut even1: u32 = 1;
    let mut even2: u32 = 2;
    let mut sum: u32 = 0;
    let max: u32 = 4_000_000;
    
    loop {
        let next: u32 = even1 + even2;
        
        // rust's 'if' requires phrase.
        if next > max {
            break;
        }
        
        if next % 2 == 0 {
            sum += next;
        }
        
        even1 = even2;
        even2 = next;
    }
    
    sum
}

/// effcient solution.
pub fn problem_2_2() -> u32 {
    let mut even1: u32 = 2;
    let mut even2: u32 = 8;
    let mut sum: u32 = even1;
    let max: u32 = 4_000_000;
    
    // Manually calculate only even numbers.
    while even2 <= max {
        sum += even2;
        let next_even: u32 = 4 * even2 + even1;
        even1 = even2;
        even2 = next_even;
    }
    
    sum
}
