-- Migration number: 0002 	 2026-11-24T10:15:05.140Z
ALTER TABLE photos
ADD COLUMN status TEXT NOT NULL DEFAULT 'completed'
CHECK (status IN ('pending', 'completed', 'failed'));
