-- Add down migration script here
-- Add down migration script here
-- Drop junction tables first due to foreign key dependencies
DROP TABLE IF EXISTS material_properties_additives;

DROP TABLE IF EXISTS material_properties_special_properties;

-- Drop dependent tables
DROP TABLE IF EXISTS material_properties;

DROP TABLE IF EXISTS materials;

-- Drop reference tables
DROP TABLE IF EXISTS shore_hardnesses;

DROP TABLE IF EXISTS finish_kinds;

DROP TABLE IF EXISTS special_properties;

DROP TABLE IF EXISTS material_families;

DROP TABLE IF EXISTS manufacturers;

DROP TABLE IF EXISTS printer_kinds;

DROP TYPE IF EXISTS shore_hardness_class;
