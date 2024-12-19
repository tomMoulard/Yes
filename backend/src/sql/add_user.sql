INSERT INTO bidding.users(email, password, username)
VALUES ($1, $2, $3)
RETURNING $table_fields;
