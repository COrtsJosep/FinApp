import random
import pandas as pd
from datetime import date, timedelta

today = date(2024, 1, 1)

expenses_id = 0
party_id = 0
fund_mov_id = 0

### Income table
df_income = pd.DataFrame()
income_cols = 'income_id,value,currency,date,category,subcategory,description,entity_id,party_id'.split(',')
df_fund_mov = pd.DataFrame()
fund_mov_cols = 'fund_movement_id,fund_movement_type,value,currency,date,account_id,party_id'.split(',')
df_party = pd.DataFrame()
party_cols = 'party_id,creation_date'.split(',')
df_expenses = pd.DataFrame()
expense_cols = 'expense_id,value,currency,date,category,subcategory,description,entity_id,party_id'.split(',')
df_entity = pd.DataFrame()
entity_cols = 'entity_id,name,country,entity_type,entity_subtype,creation_date'.split(',')
df_account = pd.DataFrame()
account_cols = 'account_id,name,country,currency,account_type,initial_balance,creation_date'.split(',')

df_entity.loc[0, entity_cols] = [
    0, '', '', 'Firm', '', today,
]
df_account.loc[0, account_cols] = [
    0, '', '', 'EUR', 'Deposit', 0.0, today,
]

for i in range(12):
    currency = random.choice(['EUR', 'CHF'])
    transaction_date = today + timedelta(days = 25 + 30*i)
    
    df_income.loc[i, income_cols] = [
        i,
        34094.2,
        currency,
        transaction_date,
        'Salary',
        'Regular salary',
        '',
        0,
        party_id,
    ]
    
    df_fund_mov.loc[fund_mov_id, fund_mov_cols] = [
        fund_mov_id,
        'Credit',
        34094.2,
        currency,
        transaction_date,
        0,
        party_id,
    ]
    
    df_party.loc[party_id, party_cols] = [
        party_id,
        transaction_date,
    ]
    
    party_id += 1
    fund_mov_id += 1
    
    
for i in range(12):
    currency = random.choice(['EUR', 'CHF'])
    transaction_date = today + timedelta(days = 25 + 30*i)
    df_expenses.loc[expenses_id, expense_cols] = [
        expenses_id,
        1150,
        currency,
        transaction_date,
        'Rent',
        '',
        '',
        0,
        party_id,
    ]
    
    df_party.loc[party_id, party_cols] = [
        party_id,
        transaction_date,
    ]
    
    party_id += 1
    fund_mov_id += 1
    expenses_id += 1
    
for i in range(250):
    currency = random.choice(['EUR', 'CHF'])
    value = random.random() * 150
    transaction_date = today + timedelta(days = int(365*random.random()))
    category = random.choice(['Groceries', 'Utilities', 'Culture', 'Presents'])
    
    df_expenses.loc[expenses_id, expense_cols] = [
        expenses_id,
        value,
        currency,
        transaction_date,
        category,
        '',
        '',
        0,
        party_id,
    ]
    
    df_fund_mov.loc[fund_mov_id, fund_mov_cols] = [
        fund_mov_id,
        'Debit',
        -value,
        currency,
        transaction_date,
        0,
        party_id,
    ]
    
    df_party.loc[party_id, party_cols] = [
        party_id,
        transaction_date,
    ]
    
    party_id += 1
    expenses_id += 1
    

df_income.to_csv('income_table.csv', index = False)
df_expenses.to_csv('expense_table.csv', index = False)
df_party.to_csv('party_table.csv', index = False)
df_fund_mov.to_csv('fund_movement_table.csv', index = False)
df_entity.to_csv('entity_table.csv', index = False)
df_account.to_csv('account_table.csv', index = False)
