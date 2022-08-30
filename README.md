# Rust-REST-API

## Usage

### Running the server

```sh
cargo run
```

### Making requests

GET: http://localhost:8080/calculateDisselUsageForDistance

(Query parameters: distance: u32, yearOfProduction: u32, fuelUsagePer100KM: f32)

GET: http://localhost:8080/probabilityOfUnitInjectorFail

(Query parameters: VIN: String)
