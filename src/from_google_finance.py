import datetime
import polars as pl
from pathlib import Path

current_file_path = Path(__file__)
data_path = current_file_path.parent.parent / 'data'

expense_colnames = ['expense_id', 'value', 'currency', 'date', 'category', 'subcategory', 'description', 'entity_id', 'party_id']
income_colnames = [colname.replace('expense', 'income') for colname in expense_colnames]

### parsing of the expense data
expense_df = pl.read_csv(data_path / 'Monthly_Budget_Linda - Transactions.csv', 
    skip_rows = 3, 
    try_parse_dates = True,
    columns = [1, 2, 3, 4])

expense_id_series = pl.Series('expense_id', range(expense_df.shape[0]))
(
    expense_df
    .rename(lambda colname: colname.lower())
    .with_columns(
        pl.col('amount')
          .str.split_exact(' ', 1)
          .struct.rename_fields(['value', 'currency'])
          .alias('fields')
    )
    .unnest('fields')
    .with_columns(
        pl.col('value').str.replace_all(r'\.', '').str.replace(',', '.').cast(pl.Float64),
        pl.col('currency').map_elements(lambda x: {'€': 'EUR', 'SEK': 'SEK'}[x], return_dtype = pl.String),
        pl.lit('').alias('subcategory'),
        pl.lit(0).alias('entity_id'),
        pl.lit(0).alias('party_id')
        )
    .hstack([expense_id_series])
    .select(expense_colnames)
    .write_csv(data_path / 'expense_table.csv')
)

### parsing of the income data
income_df = pl.read_csv(data_path / 'Monthly_Budget_Linda - Transactions.csv', 
    skip_rows = 3, 
    try_parse_dates = True,
    columns = [6, 7, 8, 9],
    new_columns = ['Date', 'Amount', 'Description', 'Category'])
    
income_id_series = pl.Series('income_id', range(income_df.shape[0]))
(
    income_df
    .rename(lambda colname: colname.lower())
    .with_columns(
        pl.col('amount')
          .str.split_exact(' ', 1)
          .struct.rename_fields(['value', 'currency'])
          .alias('fields')
    )
    .unnest('fields')
    .with_columns(
        pl.col('value').str.replace_all(r'\.', '').str.replace(',', '.').cast(pl.Float64),
        pl.col('currency').map_elements(lambda x: {'€': 'EUR', 'SEK': 'SEK'}[x], return_dtype = pl.String),
        pl.lit('').alias('subcategory'),
        pl.lit(0).alias('entity_id'),
        pl.lit(0).alias('party_id')
        )
    .hstack([income_id_series])
    .select(income_colnames)
    .write_csv(data_path / 'income_table.csv')
)

### creation of party table
date = datetime.datetime.now().strftime('%Y-%m-%d')
with open(data_path / 'party_table.csv', 'w') as file:
    file.write(f'party_id,creation_date\n0,{date}')
