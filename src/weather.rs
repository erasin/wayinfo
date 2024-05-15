use std::fs;
use std::str::FromStr;

use reqwest;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{args::WeatherArgs, errors::Error, waybar::WaybarData};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Weather {
    date: String,
    week: String,
    icon: String,
    weather: String,
    temp: String,
    temp_float: f64,
    wind: String,
    power: String,
}

impl Into<WaybarData> for Weather {
    fn into(self) -> WaybarData {
        // class 根据不同的条件提供不同的 class
        let class = "wayinfo-weather-sun";

        WaybarData {
            text: format!("{} {} {}󰔄", self.icon, self.weather, self.temp),
            alt: Some(format!("{} {}", self.wind, self.power)),
            tooltip: None,
            class: class.to_owned(),
            percentage: None,
        }
    }
}

impl From<Forecast> for Weather {
    fn from(value: Forecast) -> Self {
        // Todo: Night
        let day_icon = weather_icon(&value.day_weather).to_owned();
        Weather {
            date: value.date,
            week: value.week,
            icon: day_icon,
            weather: value.day_weather,
            temp: value.day_temp,
            temp_float: value.day_temp_float,
            wind: value.day_wind,
            power: value.day_power,
        }
    }
}

/// 高德
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Forecast {
    date: String,
    week: String,

    #[serde(skip_deserializing)]
    day_icon: String,
    #[serde(alias = "dayweather")]
    day_weather: String,
    #[serde(alias = "daytemp")]
    day_temp: String,
    #[serde(alias = "daywind")]
    day_wind: String,
    #[serde(alias = "daypower")]
    day_power: String,
    #[serde(alias = "daytemp_float", deserialize_with = "parse_float")]
    day_temp_float: f64,

    #[serde(skip_deserializing)]
    night_icon: String,
    #[serde(alias = "nightweather")]
    night_weather: String,
    #[serde(alias = "nighttemp")]
    night_temp: String,
    #[serde(alias = "nightwind")]
    night_wind: String,
    #[serde(alias = "nightpower")]
    night_power: String,
    #[serde(alias = "nighttemp_float", deserialize_with = "parse_float")]
    night_temp_float: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Forecasts {
    city: String,
    adcode: String,
    province: String,
    reporttime: String,
    casts: Vec<Forecast>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    status: String,
    count: String,
    info: String,
    infocode: String,
    forecasts: Vec<Forecasts>,
}

fn weather_icon(s: &str) -> &str {
    match s {
        "晴" => "󰖙",
        "少云" => "󰖐",
        "晴间多云" => "󰖕",
        "多云" => "󰼯",
        "阴" => "󰼰",
        "有风" | "平静" | "微风" | "和风" | "清风" => "",
        "强风/劲风" | "疾风" | "大风" => "󰖝",
        "烈风" | "风暴" | "狂爆风" => "󰼸",
        "飓风" | "热带风暴" | "龙卷风" => "󰢘",
        "霾" | "中度霾" | "重度霾" | "严重霾" => "󰖑",
        "阵雨" => "󰖓",
        "雷阵雨" | "雷阵雨并伴有冰雹" => "󰙾",
        "毛毛雨/细雨" | "雨" | "小雨" => "󰖒",
        "中雨" | "大雨" | "小雨-中雨" | "中雨-大雨" | "大雨-暴雨" => "󰖗",
        "暴雨"
        | "大暴雨"
        | "特大暴雨"
        | "强阵雨"
        | "强雷阵雨"
        | "极端降雨"
        | "暴雨-大暴雨"
        | "大暴雨-特大暴雨" => "󰖖",
        "雨雪天气" | "雨夹雪" | "阵雨夹雪" | "冻雨" => "󰙿",
        "阵雪" => "󰼴",
        "雪" | "小雪" | "中雪" | "小雪-中雪" => "󰖘",
        "大雪" | "暴雪" | "中雪-大雪" | "大雪-暴雪" => "󰼶",
        "浮尘" | "扬沙" | "沙尘暴" | "强沙尘暴" => "",
        "雾" | "浓雾" | "强浓雾" | "轻雾" | "大雾" | "特强浓雾" => "󰖑",

        "热" => "󰖙 ",
        "冷" => "",

        // "未知"=>"未知",
        _ => s,
    }
}

fn parse_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    f64::from_str(&s).map_err(serde::de::Error::custom)
}

fn get_weather(api_key: &str, city: &str) -> Result<WeatherData, Error> {
    // 构建API请求URL
    let url = format!(
        "https://restapi.amap.com/v3/weather/weatherInfo?key={}&city={}&extensions=all&output=json",
        api_key, city
    );

    // 发送请求
    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(Error::WeatherResponseError {
            code: response.status(),
        });
    }

    let data: WeatherData = response.json()?;
    Ok(data)
}

pub fn parse(args: &WeatherArgs) -> Result<(), Error> {
    // let city = matches.value_of("city").unwrap();
    let city = args.city.clone();

    if args.key.is_none() && args.key_file.is_none() {
        return Err(Error::WeatherKeyError);
    }

    let api_key = match args.key.clone() {
        Some(key) => key,
        None => match args.key_file.clone() {
            Some(key_file) => match fs::read_to_string(key_file) {
                Ok(s) => s,
                Err(_err) => return Err(Error::WeatherKeyError),
            },
            None => String::new(),
        },
    };

    if api_key.is_empty() {
        return Err(Error::WeatherKeyError);
    }

    let data = get_weather(&api_key, &city)?;

    // let json_output = serde_json::to_string(&weather_data).unwrap();

    if data.status != "1" || data.forecasts.is_empty() || data.forecasts[0].casts.is_empty() {
        return Err(Error::WeatherFailError);
    }

    let casts = &data.forecasts[0].casts;
    let data_day = match args.day {
        i if casts.len() >= i - 1 => casts.get(i - 1).expect("forecast list is empty"),
        _ => casts.last().unwrap(),
    };

    let data: Weather = data_day.clone().into();

    if args.waybar {
        // loop_stdout(data.into(), Duration::from_secs(5));
        let data: WaybarData = data.into();
        let re = serde_json::to_string(&data).unwrap();
        println!("{}", re);
    } else {
        let re = serde_json::to_string(&data).unwrap();
        println!("{}", re);
    }

    Ok(())
}
