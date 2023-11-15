use clap::Parser;
use dotenv;
use serde::Deserialize;

const LAT: f32 = -41.2;
const LON: f32 = 174.7;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    // number of days for the forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}
#[derive(Deserialize, Debug)]

struct Coord {
    lat: f32,
    lon: f32,
}
#[derive(Deserialize, Debug)]

struct Weather {
    id: u32,
    main: String,
    description: String, 
    icon: String,
}
#[derive(Deserialize, Debug)]

struct CurrentWeatherMain {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: CurrentWeatherMain,
}

fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().unwrap();
    
    let mut api_key = None;
    for (key, value) in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value)
    }
    if api_key.is_none() {
        panic!("NO API KEY")
    }
    let api_key: String = api_key.unwrap();

    let args: Args = Args::parse();
    let method = match args.days {
        0 => "weather",
        _ => "forecast"
    };
    let cnt = args.days * 8;

    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={cnt}");

    let body = reqwest::blocking::get(url)?.text();
    println!("{:?}", body);
    Ok(())
}
