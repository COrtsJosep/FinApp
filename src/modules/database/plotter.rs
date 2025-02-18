use chrono::NaiveDate;
use polars::prelude::*;
use plotters::prelude::*;
use crate::modules::currency_exchange::CurrencyExchange;
use crate::modules::database::DataBase;
use crate::modules::financial::Currency;

impl DataBase {
    pub(crate) fn funds_evolution(&self, currency_to: &Currency) -> () {
        let currency_exchange: CurrencyExchange = CurrencyExchange::init();
        
        let initial_balances: DataFrame = self
            .account_table
            .data_frame
            .clone()
            .lazy()
            .select([
                col("initial_balance").alias("value"),
                col("currency"),
                col("creation_date").alias("date")
                ])
            .collect().expect("Failed to select account table");
        
        let mut funds_table: DataFrame = self
            .funds_table
            .data_frame
            .clone()
            .select(["value", "currency", "date"]).expect("Failed to select funds table");
        
        // First step is getting all fund changes in history, and to those, adding the initial
        // balances of all accounts.
        funds_table = funds_table.vstack(&initial_balances).expect("Could not append new data");

        // Next step is converting values into the same currency
        funds_table = currency_exchange.exchange_currencies(currency_to, funds_table);
        
        // Final data manipulation step involves grouping fund changes per natural
        // day, expanding to all days without movements, and doing the cumsum!
        let result: DataFrame = funds_table
            .lazy()
            .sort(["date"], Default::default())
            .group_by_dynamic(
                col("date"),
                [],
                DynamicGroupOptions {
                    every: Duration::parse("1d"),
                    period: Duration::parse("1d"),
                    offset: Duration::parse("0"),
                    ..Default::default()
                },
            )
            .agg([col("value").sum()])
            .collect().expect("Failed to aggregate by day")
            .upsample::<[String; 0]>([], "date", Duration::parse("1d")).expect("Failed to expand date")
            .fill_null(FillNullStrategy::Forward(None)).expect("Failed to fill null values")
            .lazy()
            .select([col("date").alias("date"), col("value").cum_sum(false).alias("value")])
            .collect().expect("Failed to cumsum");

        println!("{}", &result);
        
        // Now comes the plotting part. First extract data as vectors.
        let dates: Vec<NaiveDate> = result
            .column("date").expect("Could not find date column")
            .date().expect("Could not convert date column to date (what the hell does that mean?)")
            .as_date_iter()
            .map(|opt_date| opt_date.expect("Found null value in date column"))
            .collect::<Vec<NaiveDate>>();

        let values: Vec<f64> = result
            .column("value").expect("Could not find value column")
            .f64().expect("Could not convert date column to f64 (what the hell does that mean?)")
            .into_no_null_iter().collect();

        // Then create the plot
        let root = BitMapBackend::new("figures/funds_evolution.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).expect("Failed to fill plotting root");

        let mut chart = ChartBuilder::on(&root)
            .caption("Evolution of Total Funds", ("sans-serif", 20).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(dates[0]..dates[dates.len() - 1], 0.0..values.iter().cloned().fold(0./0., f64::max))
            .expect("Failed to build chart");

        chart.configure_mesh().draw().expect("Failed to draw");

        chart.draw_series(LineSeries::new(
            dates.iter().zip(values.iter()).map(|(d, v)| (*d, *v)),
            &RED,
        )).expect("Failed to draw line");

        // Finally save the plot
        root.present().expect("Failed to present plot");
        println!("Plot saved to 'line_plot.png'");
    }

    pub(crate) fn monthly_expenses(&self, currency_to: &Currency) -> () {
        let currency_exchange: CurrencyExchange = CurrencyExchange::init();

        let mut data_frame: DataFrame = self
            .expenses_table
            .data_frame
            .clone()
            .select(["value", "currency", "date"]).expect("Failed to select funds table");

        data_frame = currency_exchange
            .exchange_currencies(currency_to, data_frame)
            .lazy()
            .sort(["date"], Default::default())
            .group_by_dynamic(
                col("date"),
                [],
                DynamicGroupOptions {
                    every: Duration::parse("1m"),
                    period: Duration::parse("1m"),
                    offset: Duration::parse("0"),
                    ..Default::default()
                },
            )
            .agg([col("value").sum()])
            .collect().expect("Failed to aggregate by month");
            
        print!("{}", &data_frame);

        let dates: Vec<String> = data_frame
            .column("date").unwrap()
            .date().unwrap()
            .as_date_iter()
            .map(|date| date.unwrap().to_string())
            .collect();

        let values: Vec<f64> = data_frame
            .column("value").unwrap()
            .f64().unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();

        // Create a drawing area
        let root = BitMapBackend::new("figures/monthly_expenses.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        // Set up the chart with floating-point y-axis
        let mut chart = ChartBuilder::on(&root)
            .caption("Monthly Values", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(0..(dates.len()-1), 0.0..values.iter().cloned().fold(0.0 / 0.0, f64::max))
            .unwrap();

        chart.configure_mesh()
            .x_labels(dates.len())
            .x_label_formatter(&|x| dates[*x].clone())
            .y_desc("Value")
            .x_desc("Month")
            .draw()
            .unwrap();

        // Draw bars with floating-point heights
        chart.draw_series(
            values.iter().enumerate().map(|(idx, &val)| {
                Rectangle::new(
                    [(idx, 0.0), (idx + 1, val)], // Use f64 for heights
                    BLUE.filled(),
                )
            }),
        ).unwrap();
    }
}
