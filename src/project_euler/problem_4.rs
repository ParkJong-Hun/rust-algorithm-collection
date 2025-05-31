//! https://euler.synap.co.kr/problem=4
//! 세자리 수를 곱해 만들 수 있는 가장 큰 대칭수
//!
//! 앞에서부터 읽을 때나 뒤에서부터 읽을 때나 모양이 같은 수를 대칭수(palindrome)라고 부릅니다.
//! 두 자리 수를 곱해 만들 수 있는 대칭수 중 가장 큰 수는 9009 (= 91 × 99) 입니다.
//! 세 자리 수를 곱해 만들 수 있는 가장 큰 대칭수는 얼마입니까?

#[allow(dead_code)]
pub fn problem_4() -> u32 {
    let mut largest_palindrome: u32 = 0;
    
    // rev: reversing like 999, 998, 997...
    for i in (100..=999).rev() {
        // prevent double counting
        for j in (i..=999).rev() {
            let product: u32 = i * j;
            
            if product <= largest_palindrome {
                break;
            }
            
            if is_palindrome(product) {
                largest_palindrome = product;
            }
        }
    }
    
    largest_palindrome
}

fn is_palindrome(n: u32) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();
    let length = chars.len();
    
    for i in 0..length/2 {
        if chars[i] != chars[length - 1 - i] {
            return false;
        }
    }
    
    true
}
