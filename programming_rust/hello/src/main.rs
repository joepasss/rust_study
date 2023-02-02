use std::str::FromStr; // trait FromStr 가져옴

// 트레이트는 타입이 구현할수 있는 메서드의 모임
// FromStr 트레이트를 구현하는 모든 타입은 문자열을 해당 타입의 값으로 파싱하는 from_str 메서드를 갖는다
// 이 프로그램은 u64 타입이 FromStr을 구현하고 있기 때문에 u64::from_str을 호출해 명령줄 인수를 파싱한다.

use std::env; // env 모듈을 가져옴
              // 실행 환경과 상호 작용하는데 필요한 여러 가지 함수와 타입을 제공
              // args 함수를 사용하면 프로그램의 명령줄 인수에 접근 가능하다

fn main() {
    let mut numbers = Vec::new(); // 지역변수 numbers를 선언한 뒤 이를 빈 벡터로 초기화
                                  // Vec은 러스트의 벡터 타입으로 크기가 동적으로 늘고 줄게 되어 있지만, mut를 붙이지 않는다면 아무것도 넣을 수 없다.

    // numbers의 타입은 u64값의 벡터인 Vec<u64>이지만, 적을 필요는 없다.

    for arg in env::args().skip(1) {
        // std::env 모듈의 args 함수는 iterator를 반환한다
        // 이 이터레이터는 요구가 있을 때 마다 인수를 하나씩 반환하고, 더 이상 반환할 게 없을 경우에는 없음을 나타내는 값을 반환한다.

        // 이터레이터는 for 루프와 함께 사용하는 방법 외에도 직접 호출해 쓸 수 있는 여러 종류의 매서드를 가진다.

        numbers.push(u64::from_str(&arg).expect("error parsing argument"))
        // u64::from_str를 호출해 명령줄 인수 arg를 부호 없는 64비트 정수로 파싱한다.
        // from_str 함수는 u64를 직접 반환하지 않고, 파싱의 성공 여부를 나타내는 Result 값을 반환한다.
        // Result 값은 다음의 두 variant 중 하나다.
        // Ok(v) 라고 쓰는 값. 파싱이 성공했음을 의미하고, v는 그 결괏값이다
        // Err(e) 라고 쓰는 값. 파싱이 실패했음을 의미하고, e는 그 이유를 설명하는 오륫값이다.

        // 입출력을 수행하거나 운영체제와 상호작용 하는것 등의 실패할 수도 있는 일을 하는 함수들은 Result 타입을 반환 할 수 있다. 모든 오류는 Result 혹은 패닉으로 처리한다.

        // 여기서는 Result의 expect 메서드로 파싱의 성공 여부를 확인한다. expect는 결과가 Err(e)일 경우 e에 대한 설명이 포함된 메시지를 출력한 뒤 프로그램을 곧바로 종료하지만, 결과가 Ok(v)일 경우 단순히 v자체를 반환한다.
    }

    // 공집합의 최대 공약수는 존재하지 않기 때문에 위 numbers 벡터가 하나 이상의 요소를 가졌는지 확인한다.
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ..."); // 표준 오류 출력 스트림에 오류를 출력하는 매크로
        std::process::exit(1); // 프로그램 종료
    }

    let mut d = numbers[0]; // 실행값
    for m in &numbers[1..] {
        // &연산자
        d = gcd(d, *m); // *연산자

        // 매번 반복때 마다 그때까지 처리한 모든 수의 공약수를 유지

        // &연산자와 *연산자의 역할
        // 벡터에 있는 값을 반복 처리하려 하는데 벡터의 크기는 고정되어 있지 않아 이 개별 값의 수명을 명시해줘 더 이상 필요치 않은 메모리가 지체없이 해제될 수 있게 만들어야 한다
        // 따라서 반복 처리를 진행할 때 러스트에게 벡터의 소유권이 계속해서 numbers에 남아 있음을 알리고 루프를 위해 벡터의 요소를 빌려오는 작업이 필요하다.
        // & 연산자는 벡터의 두 번째 이후 요소들에 대한 레퍼런스를 빌려 온다. 이 for 루프는 매번 반복할 때 마다 이 참조된 요소드를 차례로 하나씩 m에 빌려 온다.
        // * 연산자는 m을 역참조해서 그 것이 가리키는 값을 넘겨주는데 이것이 바로 gcd에 전달할 다음 u64값이 된다.

        // 끝으로 벡터는 numbers가 소유하기 때문에 numbers가 main의 범위 끝을 넘어가면 자동으로 해제된다
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d); // 결과를 표준 출력 스트림에 출력

    // 러스트는 main이 아무것도 반환하지 않으면 프로그램이 정상 종료된 것으로 간주한다.
    // expect나 std::process::exit 같은 함수를 명시적으로 호출할 때에만 프로그램이 오류 상태 코드를 가지고 종료하게 만들 수 있다
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}

// cargo test 명령으로 실행 가능하다
#[test]
fn text_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
