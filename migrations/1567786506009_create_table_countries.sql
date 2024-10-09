-- # Put the your SQL below migration seperator.
-- !UP

CREATE TABLE IF NOT EXISTS countries (
  id SERIAL PRIMARY KEY,
  name TEXT,
  code VARCHAR(4) UNIQUE,
  idd_code VARCHAR(4),
  currency VARCHAR(4),
  status SMALLINT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

INSERT INTO countries
  (name, code, idd_code, currency, status, created_at, updated_at)
VALUES
  ('Australia', 'AU', '61', 'AUD', 1, NOW(), NOW()),
  ('Austria', 'AT', '43', 'EUR', 1, NOW(), NOW()),
  ('Belgium', 'BE', '32', 'EUR', 1, NOW(), NOW()),
  ('Brazil', 'BR', '55', 'BRL', 1, NOW(), NOW()),
  ('Canada', 'CA', '1', 'CAD', 1, NOW(), NOW()),
  ('Chile', 'CL', '56', 'CLP', 1, NOW(), NOW()),
  ('China', 'CN', '86', 'CNY', 1, NOW(), NOW()),
  ('Czech Republic', 'CZ', '420', 'CZK', 1, NOW(), NOW()),
  ('Denmark', 'DK', '45', 'DKK', 1, NOW(), NOW()),
  ('Estonia', 'EE', '372', 'EUR', 1, NOW(), NOW()),
  ('Finland', 'FI', '358', 'EUR', 1, NOW(), NOW()),
  ('France', 'FR', '33', 'EUR', 1, NOW(), NOW()),
  ('Germany', 'DE', '49', 'EUR', 1, NOW(), NOW()),
  ('Greece', 'GR', '30', 'EUR', 1, NOW(), NOW()),
  ('Hungary', 'HU', '36', 'HUF', 1, NOW(), NOW()),
  ('Iceland', 'IS', '354', 'ISK', 1, NOW(), NOW()),
  ('India', 'IN', '91', 'INR', 1, NOW(), NOW()),
  ('Ireland', 'IE', '353', 'EUR', 1, NOW(), NOW()),
  ('Israel', 'IL', '972', 'ILS', 1, NOW(), NOW()),
  ('Italy', 'IT', '39', 'EUR', 1, NOW(), NOW()),
  ('Japan', 'JP', '81', 'JPY', 1, NOW(), NOW()),
  ('Latvia', 'LV', '371', 'EUR', 1, NOW(), NOW()),
  ('Lithuania', 'LT', '370', 'EUR', 1, NOW(), NOW()),
  ('Luxembourg', 'LU', '352', 'EUR', 1, NOW(), NOW()),
  ('Malaysia', 'MY', '60', 'MYR', 1, NOW(), NOW()),
  ('Mexico', 'MX', '52', 'MXN', 1, NOW(), NOW()),
  ('Netherlands', 'NL', '31', 'EUR', 1, NOW(), NOW()),
  ('New Zealand', 'NZ', '64', 'NZD', 1, NOW(), NOW()),
  ('Norway', 'NO', '47', 'NOK', 1, NOW(), NOW()),
  ('Philippines', 'PH', '63', 'PHP', 1, NOW(), NOW()),
  ('Poland', 'PL', '48', 'PLN', 1, NOW(), NOW()),
  ('Portugal', 'PT', '351', 'EUR', 1, NOW(), NOW()),
  ('Russia', 'RU', '7', 'RUB', 1, NOW(), NOW()),
  ('Singapore', 'SG', '65', 'SGD', 1, NOW(), NOW()),
  ('Slovakia', 'SK', '421', 'EUR', 1, NOW(), NOW()),
  ('Slovenia', 'SI', '386', 'EUR', 1, NOW(), NOW()),
  ('South Africa', 'ZA', '27', 'ZAR', 1, NOW(), NOW()),
  ('South Korea', 'KR', '82', 'KRW', 1, NOW(), NOW()),
  ('Spain', 'ES', '34', 'EUR', 1, NOW(), NOW()),
  ('Sweden', 'SE', '46', 'SEK', 1, NOW(), NOW()),
  ('Switzerland', 'CH', '41', 'CHF', 1, NOW(), NOW()),
  ('Thailand', 'TH', '66', 'THB', 1, NOW(), NOW()),
  ('Turkey', 'TR', '90', 'TRY', 1, NOW(), NOW()),
  ('United Arab Emirates', 'AE', '971', 'AED', 1, NOW(), NOW()),
  ('United Kingdom', 'UK', '44', 'GBP', 1, NOW(), NOW()),
  ('United States', 'US', '1', 'USD', 1, NOW(), NOW()),
  ('Vietnam', 'VN', '84', 'VND', 1, NOW(), NOW());

-- !DOWN

DROP TABLE countries;
