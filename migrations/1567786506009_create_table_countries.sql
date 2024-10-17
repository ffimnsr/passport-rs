-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS countries (
  id SERIAL PRIMARY KEY,
  name TEXT,
  code VARCHAR(4) UNIQUE,
  idd_code VARCHAR(4),
  currency VARCHAR(4),
  status SMALLINT DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO countries
  (name, code, idd_code, currency)
VALUES
  ('Australia', 'AU', '61', 'AUD'),
  ('Austria', 'AT', '43', 'EUR'),
  ('Belgium', 'BE', '32', 'EUR'),
  ('Brazil', 'BR', '55', 'BRL'),
  ('Canada', 'CA', '1', 'CAD'),
  ('Chile', 'CL', '56', 'CLP'),
  ('China', 'CN', '86', 'CNY'),
  ('Czech Republic', 'CZ', '420', 'CZK'),
  ('Denmark', 'DK', '45', 'DKK'),
  ('Estonia', 'EE', '372', 'EUR'),
  ('Finland', 'FI', '358', 'EUR'),
  ('France', 'FR', '33', 'EUR'),
  ('Germany', 'DE', '49', 'EUR'),
  ('Greece', 'GR', '30', 'EUR'),
  ('Hungary', 'HU', '36', 'HUF'),
  ('Iceland', 'IS', '354', 'ISK'),
  ('India', 'IN', '91', 'INR'),
  ('Ireland', 'IE', '353', 'EUR'),
  ('Israel', 'IL', '972', 'ILS'),
  ('Italy', 'IT', '39', 'EUR'),
  ('Japan', 'JP', '81', 'JPY'),
  ('Latvia', 'LV', '371', 'EUR'),
  ('Lithuania', 'LT', '370', 'EUR'),
  ('Luxembourg', 'LU', '352', 'EUR'),
  ('Malaysia', 'MY', '60', 'MYR'),
  ('Mexico', 'MX', '52', 'MXN'),
  ('Netherlands', 'NL', '31', 'EUR'),
  ('New Zealand', 'NZ', '64', 'NZD'),
  ('Norway', 'NO', '47', 'NOK'),
  ('Philippines', 'PH', '63', 'PHP'),
  ('Poland', 'PL', '48', 'PLN'),
  ('Portugal', 'PT', '351', 'EUR'),
  ('Russia', 'RU', '7', 'RUB'),
  ('Singapore', 'SG', '65', 'SGD'),
  ('Slovakia', 'SK', '421', 'EUR'),
  ('Slovenia', 'SI', '386', 'EUR'),
  ('South Africa', 'ZA', '27', 'ZAR'),
  ('South Korea', 'KR', '82', 'KRW'),
  ('Spain', 'ES', '34', 'EUR'),
  ('Sweden', 'SE', '46', 'SEK'),
  ('Switzerland', 'CH', '41', 'CHF'),
  ('Thailand', 'TH', '66', 'THB'),
  ('Turkey', 'TR', '90', 'TRY'),
  ('United Arab Emirates', 'AE', '971', 'AED'),
  ('United Kingdom', 'UK', '44', 'GBP'),
  ('United States', 'US', '1', 'USD'),
  ('Vietnam', 'VN', '84', 'VND');

-- !DOWN

DROP TABLE countries;
