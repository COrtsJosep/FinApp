use std::str::FromStr;
use chrono::NaiveDate;
use polars::prelude::*;
use plotters::prelude::*;
use crate::modules::currency_exchange::CurrencyExchange;
use crate::modules::database::DataBase;
use crate::modules::financial::Currency;
use std::iter::zip;

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
            .collect()
            .expect("Failed to select account table");
        
        let mut funds_table: DataFrame = self
            .funds_table
            .data_frame
            .clone()
            .select(["value", "currency", "date"])
            .expect("Failed to select funds table");
        
        // First step is getting all fund changes in history, and to those, adding the initial
        // balances of all accounts.
        funds_table = funds_table.vstack(&initial_balances).expect("Could not append new data");

        // Next step is creating a new column with the exchange rates for all necessary dates and 
        // currencies
        let mut exchange_rates: Vec<f64> = vec![];
        let date_iter = funds_table.column("date").unwrap().date().unwrap().as_date_iter();
        let currency_iter = funds_table.column("currency").unwrap().str().unwrap().into_iter();

        for (date, currency) in zip(date_iter, currency_iter){
            let date = date.unwrap();
            let currency_from = Currency::from_str(currency.unwrap()).expect("Could not find currency shortname");
            let exchange_rate = currency_exchange.exchange(&currency_from, &currency_to, date);

            exchange_rates.push(exchange_rate);
        }
        
        // Final data manipulation step involves adding the exchange rate column to the data_frame,
        // converting all currencies into the target currency, grouping fund changes per natural
        // day, expanding to all days without movements, and doing the cumsum!
        let result: DataFrame = funds_table
            .with_column(Series::new("exchange_rate".into(), exchange_rates))
            .expect("Failed to add exchange_rate column")
            .clone()
            .lazy()
            .select([col("date"), (col("exchange_rate") * col("value")).alias("value")])
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
}
