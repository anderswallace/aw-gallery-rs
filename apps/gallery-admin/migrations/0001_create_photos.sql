-- Migration number: 0001 	 2026-11-22T23:18:24.864Z
CREATE TABLE photos (
	id TEXT PRIMARY KEY,
	object_key TEXT NOT NULL UNIQUE,
	filename TEXT NOT NULL,
	content_type TEXT NOT NULL,
	camera TEXT NOT NULL,
	film TEXT,
	uploaded_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
