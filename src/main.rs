use dicoco_safe_math::calc_basic;
use std::io::ErrorKind;

use tide::{Body, Request, Response, StatusCode};
use tide::utils::After;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Server starting...");
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<async_std::io::Error>() {
            if let ErrorKind::NotFound = err.kind() {
                let msg = format!("Error: {:?}", err);
                res.set_status(StatusCode::NotFound);

                // NOTE: You may want to avoid sending error messages in a production server.
                res.set_body(msg);
            }
        }
        Ok(res)
    }));

    app.at("/sum/:x/:y").get(sum);
    app.at("/sub/:x/:y").get(sub);

    app.at("/")
        .get(|_req: Request<_>| async { Ok(Body::from_file("./src/html/home.html").await?) });


    println!("Server started and running!");
    
    println!("Listening at http://serverip:4001");

    app.listen("0.0.0.0:4001").await?;

    Ok(())
}


async fn sum( req: Request<()>) -> tide::Result {
    // let Animal { name, legs } = req.body_json().await?;
    let x = req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y = req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let resp = calc_basic::sum_x_y(x, y);

    Ok(format!("{} + {} = {}", x, y, resp).into())
}

async fn sub( req: Request<()>) -> tide::Result {
    // let Animal { name, legs } = req.body_json().await?;
    let x = req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y = req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let resp = calc_basic::sub_x_y(x, y);

    Ok(format!("{} - {} = {}", x, y, resp).into())
}

