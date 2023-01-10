CREATE TABLE IF NOT EXISTS `posts` (
	`id` int(11) unsigned NOT NULL AUTO_INCREMENT,
	`title` varchar(255) NOT NULL,
	`body` text NOT NULL,
	`author_id` int(11) unsigned NOT NULL,
	`created_at` datetime NOT NULL,
	`updated_at` datetime NOT NULL,
	PRIMARY KEY (`id`),
	FOREIGN KEY (`author_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
