// Copyright 2024 witchof0x20
// This file is part of discord_data_package.
//
// discord_data_package is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// discord_data_package is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with discord_data_package. If not, see <https://www.gnu.org/licenses/>.

use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use plotters::backend::{BitMapBackend, DrawingBackend};
use plotters::drawing::DrawingAreaErrorKind;
use plotters::prelude::*;

pub fn message_activity<'a>(
    times: &[DateTime<Utc>],
) -> Result<(), DrawingAreaErrorKind<<BitMapBackend<'a> as DrawingBackend>::ErrorType>> {
    // Initialize drawing backend
    let root = BitMapBackend::new("activity.png", (1280, 720)).into_drawing_area();
    // Fill with white
    root.fill(&WHITE)?;
    // Get date range
    let min_day = times
        .iter()
        .min()
        .unwrap_or(&DateTime::<Utc>::MIN_UTC)
        .date_naive();
    let max_day = times
        .iter()
        .max()
        .unwrap_or(&DateTime::<Utc>::MAX_UTC)
        .date_naive();
    // Initialize the chart
    let mut chart = ChartBuilder::on(&root)
        //.caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(min_day..max_day, 0u32..(24 * 60 * 60 + 1))?;
    // Draw a scatter plot of activity times
    chart.draw_series(times.iter().map(|time| {
        Circle::new(
            (time.date_naive(), time.num_seconds_from_midnight()),
            1,
            GREEN.filled(),
        )
    }))?;
    // Draw axis mesh after the scatter plot
    chart
        .configure_mesh()
        .disable_mesh()
        .x_label_formatter(&|date: &NaiveDate| format!("{}-{:02}", date.year(), date.month()))
        .y_label_formatter(&|x| format!("{:02}:{:02}", x / 3600, (x % 3600) / 60))
        .draw()?;
    // Draw the plot
    root.present()?;

    Ok(())
}
