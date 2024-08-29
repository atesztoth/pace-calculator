-- Your SQL goes here
CREATE TABLE `calculations` (
	`id` TEXT PRIMARY KEY NOT NULL,
	`time` INT4 NOT NULL, -- time in seconds
	`distance` INT4 NOT NULL, -- distance in meters
	`pace` INT4 NOT NULL, -- pace in seconds for 1 km
	`created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
