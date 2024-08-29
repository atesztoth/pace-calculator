-- Your SQL goes here

-- REAL. The value is a floating point value, stored as an 8-byte IEEE floating point number.
CREATE TABLE `calculations` (
	`id` TEXT PRIMARY KEY NOT NULL,
	`time` REAL NOT NULL, -- time in seconds
	`distance` REAL NOT NULL, -- distance in meters
	`pace` REAL NOT NULL, -- pace in seconds for 1 km
	`created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
