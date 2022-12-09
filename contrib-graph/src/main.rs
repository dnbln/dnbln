use plotters::prelude::*;
use plotters::style::full_palette::GREY_200;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const OUT_FILE_NAME: &'static str = "3d-contrib-plot.svg";

const GITHUB_GRAPHQL: &str = "https://api.github.com/graphql";
const GITHUB_TOKEN: &str = env!("GITHUB_TOKEN");
const GITHUB_USERNAME: &str = env!("GITHUB_USERNAME");
const FAIL_IF_ZERO: bool = match option_env!("FAIL_IF_ZERO") {
    Some("") | Some("true") | Some("1") => true,
    _ => false,
};

#[derive(Serialize, Deserialize)]
struct ContributionCalendar {
    weeks: Vec<ContributionWeek>,
}

#[derive(Serialize, Deserialize)]
struct ContributionWeek {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<ContributionDay>,
}

#[derive(Serialize, Deserialize)]
struct ContributionDay {
    weekday: u8,
    #[serde(rename = "contributionCount")]
    contribution_count: u32,
}

async fn get_contribs(
    login: &str,
    force: bool,
) -> Result<ContributionCalendar, Box<dyn std::error::Error>> {
    #[derive(Serialize, Deserialize)]
    struct ContributionResults {
        data: ContributionData,
    }

    #[derive(Serialize, Deserialize)]
    struct ContributionData {
        user: UserContributionData,
    }

    #[derive(Serialize, Deserialize)]
    struct UserContributionData {
        #[serde(rename = "contributionsCollection")]
        contributions_collection: ContributionCollection,
    }

    #[derive(Serialize, Deserialize)]
    struct ContributionCollection {
        #[serde(rename = "contributionCalendar")]
        contribution_calendar: ContributionCalendar,
    }

    let mut path = std::env::current_dir()?;
    path.push("contributions.json");

    if !force && path.exists() {
        println!("Loaded contributions from file");
        let s = tokio::fs::read_to_string(path).await?;

        let result = serde_json::from_str::<ContributionResults>(&s)?;

        return Ok(result
            .data
            .user
            .contributions_collection
            .contribution_calendar);
    }

    const Q: &str = r#"
    query ($login: String!) {
      user(login: $login) {
        contributionsCollection {
          contributionCalendar {
            weeks {
              contributionDays {
                weekday
                contributionCount
              }
            }
          }
        }
      }
    }
"#;

    let graphql_query = serde_json::json! {{
        "query": Q,
        "variables": {
            "login": login
        }
    }};

    let client = reqwest::Client::new();

    let result: ContributionResults = client
        .post(GITHUB_GRAPHQL)
        .json(&graphql_query)
        .bearer_auth(GITHUB_TOKEN)
        .header("User-Agent", "reqwest")
        .send()
        .await?
        .json()
        .await?;

    tokio::fs::write(path, serde_json::to_string_pretty(&result)?).await?;

    Ok(result
        .data
        .user
        .contributions_collection
        .contribution_calendar)
}

fn compile_calendar(calendar: &ContributionCalendar) -> (BTreeMap<i32, i32>, i32) {
    let mut result = BTreeMap::new();
    let mut max = 0;

    for (week_index, week) in calendar.weeks.iter().enumerate() {
        for day in &week.contribution_days {
            if day.contribution_count != 0 {
                let key = (week_index as i32) * 7 + day.weekday as i32;
                let value = day.contribution_count as i32;
                result.insert(key, value);

                max = max.max(value);
            }
        }
    }

    if FAIL_IF_ZERO {
        if let Some(week) = calendar.weeks.last() {
            if let Some(day) = week.contribution_days.last() {
                if day.contribution_count == 0 {
                    panic!("No contributions in the last day");
                }
            }
        }
    }

    (result, max)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let area = SVGBackend::new(OUT_FILE_NAME, (1400, 720)).into_drawing_area();

    area.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&area).build_cartesian_3d(0.0..53.0, 0.0..10.0, 0.0..7.0)?;

    chart.set_3d_pixel_range((3000, 800, 1800));

    chart.with_projection(|mut p| {
        p.pitch = 0.3;
        p.yaw = 0.6;
        p.scale = 0.4;
        p.into_matrix() // build the projection matrix
    });

    fn contrib_height(
        contribs: &BTreeMap<i32, i32>,
        contrib_max: i32,
        x: f64,
        z: f64,
    ) -> Option<f64> {
        if x - x.floor() < 0.3 || z - z.floor() < 0.3 {
            return None;
        }

        let x = x as i32;
        let z = z as i32;

        let contrib = contribs.get(&(x * 7 + z)).unwrap_or(&0);
        let contrib = 5.0 * *contrib as f64 / contrib_max as f64;
        Some(contrib as f64)
    }

    let (contribs, contrib_max) = compile_calendar(&get_contribs(GITHUB_USERNAME, false).await?);

    fn style_for_height(h: f64) -> ShapeStyle {
        if h == -0.001 {
            TRANSPARENT.filled()
        } else if h == 0.0 {
            GREY_200.filled()
        } else {
            RGBAColor(0, 0xA3, 0, 1.0).filled()
        }
    }

    chart.draw_series(
        SurfaceSeries::xoz(
            (0..=530).map(|x| x as f64 / 10.0),
            (0..=70).map(|x| x as f64 / 10.0),
            |x: f64, z: f64| contrib_height(&contribs, contrib_max, x, z).unwrap_or(-0.001),
        )
            .style_func(&|v: &f64| style_for_height(*v)),
    )?;

    area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
