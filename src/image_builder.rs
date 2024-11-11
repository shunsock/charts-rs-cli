use crate::chart_type::ChartType;
use charts_rs::svg_to_jpeg;
use std::error::Error;
use std::fs;

pub struct ImageBuilder {}

impl ImageBuilder {
    pub fn create_chart(chart_type: &str, json_strings: &str) -> Result<(), Box<dyn Error>> {
        let chart: ChartType = ChartType::from_json(chart_type, json_strings)?;

        let svg_chart: String = chart.svg()?;

        let jpg_chart: Vec<u8> =
            svg_to_jpeg(&svg_chart).map_err(|_| "(依存ライブラリエラー) JPEG変換に失敗しました")?;

        fs::write("chart.jpg", jpg_chart)
            .map_err(|_| "(依存ライブラリエラー) JPEGファイルの保存に失敗しました")?;

        Ok(())
    }
}
