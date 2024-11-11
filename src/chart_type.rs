use charts_rs::{BarChart, ScatterChart};
use std::error::Error;

pub enum ChartType {
    Scatter(ScatterChart),
    Bar(BarChart),
}

impl ChartType {
    pub(crate) fn from_json(chart_name: &str, json: &str) -> Result<Self, Box<dyn Error>> {
        match chart_name {
            "scatter" => {
                let chart = ScatterChart::from_json(json)
                    .map_err(|e| -> String { format!("ScatterChartのJSONパースに失敗しました。入力したJSONを見直してください。: {}", e) })?;
                Ok(ChartType::Scatter(chart))
            }
            "bar" => {
                let chart = BarChart::from_json(json).map_err(|e| -> String {
                    format!(
                        "BarChartのJSONパースに失敗しました。入力したJSONを見直してください。: {}",
                        e
                    )
                })?;
                Ok(ChartType::Bar(chart))
            }
            _ => Err("不明なチャート名です".into()),
        }
    }

    pub(crate) fn svg(&self) -> Result<String, Box<dyn Error>> {
        match self {
            ChartType::Scatter(chart) => chart
                .svg()
                .map_err(|_| "(依存ライブラリエラー) SVG変換に失敗しました".into()),
            ChartType::Bar(chart) => chart
                .svg()
                .map_err(|_| "(依存ライブラリエラー) SVG変換に失敗しました".into()),
        }
    }
}
