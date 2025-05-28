//! https://euler.synap.co.kr/problem=1
//! 1000보다 작은 자연수 중에서 3 또는 5의 배수를 모두 더하면?
//! 
//! 10보다 작은 자연수 중에서 3 또는 5의 배수는 3, 5, 6, 9 이고, 이것을 모두 더하면 23입니다.
//! 1000보다 작은 자연수 중에서 3 또는 5의 배수를 모두 더하면 얼마일까요? 

// pub = public
pub fn problem_1_1() -> u32 {
    let max: u16 = 1000;
    // variables must be initialized before
    let mut answer: u32 = 0;

    for i in 1..max {
        if i % 3 == 0 || i % 5 == 0 {
            // type conversion must be done manually
            answer += i as u32;
        }
    }
    // the return value must not be followed by a semicolon.
    answer
}

/// Using the principles of arithmetic progression and inclusion/exclusion.
pub fn problem_1_2() -> u32 {
    let max: u16 = 1000;
    // inclusion-exlusion principle.
    // |A∪B|=|A|+|B|−|A∩B|
    // |A∪B∪B|=|A|+|B|+|C|−|A∩B|−|A∩C|−|B∩C|+|A∩B∩C|
    let answer = sum_of_multiples(3, max as u32)
        + sum_of_multiples(5, max as u32)
        - sum_of_multiples(15, max as u32);
    answer
}

/// arithmetic series
/// Sn = n(a1+an)/2 = n[2a1+(n−1)d] / 2
fn sum_of_multiples(factor: u32, max: u32) -> u32 {
    let n = (max - 1) / factor;
    factor * n * (n + 1) / 2
}
