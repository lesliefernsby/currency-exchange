-- Your SQL goes here
CREATE TABLE exchange_rates (
  id SERIAL PRIMARY KEY,
  base_currency_id INTEGER ,
  target_currency_id INTEGER,
  rate DECIMAL(6) NOT NULL,
  CONSTRAINT base_currency_fk
      FOREIGN KEY(base_currency_id) 
	  REFERENCES currencies(id),
  CONSTRAINT target_currency_fk
      FOREIGN KEY(target_currency_id) 
	  REFERENCES currencies(id)
)
