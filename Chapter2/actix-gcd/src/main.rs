use actix_web::{web, App, HttpResponce, HttpServer};

fn main()
{
    // || is a closure expression. A value which can be called as if it were a function. Args can go between the ||.
    let server = HttpServer::new(|| {
        App::new().route("/", web.get().to(get_index))
    });

    println!("Serving on http://localhost:3000...");

    server.bind("127.0.0.1:3000").expect("error biunding server to address").run().expect("error running server");
}

fn get_index() -> HttpResponce
{
    HttpResponce::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#
    )
}