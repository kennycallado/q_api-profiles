CREATE TABLE IF NOT EXISTS profiles ();

ALTER TABLE profiles
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id SERIAL NOT NULL UNIQUE,
  ADD COLUMN profile_token VARCHAR(60) NOT NULL,
  ADD COLUMN name VARCHAR(30) NOT NULL,
  ADD COLUMN surname VARCHAR(60) NOT NULL,
  ADD COLUMN email VARCHAR(100) NOT NULL,
  ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
  ;

SELECT diesel_manage_updated_at('profiles');
INSERT INTO profiles (user_id, profile_token, name, surname, email) VALUES
  ( 1, 'admin', 'admin', 'admin', 'admin@admin.es')
  ;
