use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// 프로그램이 컴파일 될 때 serde 크레이트가 해당 타입을 살펴보고 HTML 폼이 POST 요청에 사용하는 형식으로 된 데이터를 그 타입으로 된 값으로 파싱하는 코드를 자동으로 만들어 준다.
// serde 크레이트는 이와 반대로 러스트 값을 가져다 이를 구조화된 형식으로 기록하는 코드를 만들어 주는 Serialize 어트리뷰트를 제공한다.
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {App::new().route("/", web::get().to(get_index)).route("/gcd", web::post().to(post_gcd))});

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the GCD with zero is boring.");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b> \n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
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