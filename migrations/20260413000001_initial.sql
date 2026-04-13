-- Initial migration for NeRF Studio
-- Creates tables for scenes, images, and rendering metadata

-- Scenes table: stores 3D scene metadata
CREATE TABLE IF NOT EXISTS scenes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    image_count INTEGER NOT NULL DEFAULT 0,
    training_progress REAL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Scene images table: stores uploaded photos for each scene
CREATE TABLE IF NOT EXISTS scene_images (
    id UUID PRIMARY KEY,
    scene_id UUID NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    original_name TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    camera_position_x REAL,
    camera_position_y REAL,
    camera_position_z REAL,
    camera_rotation_pitch REAL,
    camera_rotation_yaw REAL,
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_scenes_status ON scenes(status);
CREATE INDEX IF NOT EXISTS idx_scenes_created_at ON scenes(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_scene_images_scene_id ON scene_images(scene_id);
CREATE INDEX IF NOT EXISTS idx_scene_images_uploaded_at ON scene_images(uploaded_at DESC);

-- Trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_scenes_timestamp 
AFTER UPDATE ON scenes
BEGIN
    UPDATE scenes SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
