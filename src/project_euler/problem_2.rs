//! 피보나치 수열에서 4백만 이하이면서 짝수인 항의 합
//! 
//! 피보나치(Fibonacci) 수열의 각 항은 바로 앞의 항 두 개를 더한 것입니다. 1과 2로 시작하는 경우 이 수열은 아래와 같습니다.
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// 4백만 이하의 짝수 값을 갖는 모든 피보나치 항을 더하면 얼마가 됩니까?

#[allow(dead_code)]
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
#[allow(dead_code)]
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
