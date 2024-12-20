INSERT INTO bidding.users(email, password, username, points)
VALUES ($1, $2, $3, 0)
RETURNING $table_fields;
