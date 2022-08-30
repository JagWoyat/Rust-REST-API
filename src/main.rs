use actix_web::{get, web, App, HttpResponse, HttpServer, Result,
    middleware::Logger
};
use serde::Deserialize;
use serde_json::json;
use env_logger::Env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamsCalculate {
   distance: u32,
   year_of_production: u32,
   #[serde(rename = "fuelUsagePer100KM")]
   fuel_usage_per100km: f64,
}

#[derive(Debug, Deserialize)]
pub struct ParamsFailure {
    #[serde(rename = "VIN")]
    vin: String,
}

#[get("/calculateDisselUsageForDistance")]
async fn calculate(req: web::Query::<ParamsCalculate>) -> HttpResponse {
    let usage = ((req.distance as f64) * req.fuel_usage_per100km)/100.0;
    let res = json!({
        "fuelUsage": usage,
        "req": {
            "distance": req.distance,
            "yearOfProduction": req.year_of_production,
            "fuelUsagePer100KM": req.fuel_usage_per100km,
        }
    });
    HttpResponse::Ok().json(res)
    //.body(format!("Distance: {}KM \nYear of production: {} \nFuel usage per 100KM: {} liters \nFuel used: {:.2} liters", 
    //    req.distance, req.yearOfProduction, req.fuelUsagePer100KM, usage))
}

#[get("/probabilityOfUnitInjectorFail")]
async fn failure_rate(req: web::Query::<ParamsFailure>) -> HttpResponse {
    let fail_probability: f64 = ((SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as f64) % 101.0) / 100.0;
    let res = json!({
        "failProbability": fail_probability,
        "req": {
            "VIN": req.vin,
        }
    });
    HttpResponse::Ok().json(res)
    //.body(format!("VIN: {} \nChance of an Unit Injector failure is: {}%", req.VIN, fail_probability))
}

async fn default_handler() -> Result<HttpResponse> {
    Ok(
        HttpResponse::NotFound().body(format!("It's not a correct request. \nTry GET \nhttp://localhost:8080/calculateDisselUsageForDistance 
(Query parameters: distance: u32, yearOfProduction: u32, fuelUsagePer100KM: f32) or \nhttp://localhost:8080/probabilityOfUnitInjectorFail 
(Query parameters: VIN: String) \nEx. http://localhost:8080/calculateDisselUsageForDistance?distance=200&yearOfProduction=2011&fuelUsagePer100KM=7.3"))
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(calculate)
            .service(failure_rate)
            .default_service(web::to(default_handler))
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}
