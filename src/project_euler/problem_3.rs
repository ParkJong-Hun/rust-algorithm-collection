//! https://euler.synap.co.kr/problem=3
//! 가장 큰 소인수 구하기
//! 
//! 어떤 수를 소수의 곱으로만 나타내는 것을 소인수분해라 하고, 이 소수들을 그 수의 소인수라고 합니다.
//! 예를 들면 13195의 소인수는 5, 7, 13, 29 입니다.
//! 600851475143의 소인수 중에서 가장 큰 수를 구하세요.

#[allow(dead_code)]
pub fn problem_3() -> u64 {
    let mut max: u64 = 600851475143;
    let mut largest_factor: u64 = 1;
    
    while max % 2 == 0 {
        largest_factor = 2;
        max /= 2;
    }
    
    let mut factor: u64 = 3;

    // calculate and then check
    while factor * factor <= max {
        while max % factor == 0 {
            largest_factor = factor;
            max /= factor;
        }
        factor += 2;
    }
    
    if max > 1 {
        largest_factor = max;
    }
    
    largest_factor
}
