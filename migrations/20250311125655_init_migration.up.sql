-- Create the enum type for shore_hardness_class
CREATE TYPE shore_hardness_class AS ENUM ('a', 'd', 'c');


-- Automatically set updated at
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgSQL;



--- Manufacturer ---
Create Table Manufacturer (
    id UUID Primary Key,
    name TEXT NOT NULL,
    created timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
);

-- implement update on manufacturer --
CREATE TRIGGER update_updated_at_trigger
BEFORE UPDATE ON Manufacturer
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

--- Printer Kind ie. FDM, SLS, SLA ---

Create Table PrinterKind (
    id UUID Primary Key,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP(0)
);


--- Material Properties ---

--- Material ---

--- Material Unit Data ie. 1kg(1000g) spool of 1.75mm filament that cost x$ ---
-- a material can be sold in multiple units for
Create Table MaterialUnitData(
    id UUID Primary Key,
    material_id UUID NOT NULL REFERENCES Materials (id),
    unit_size_in_grams INT8 NOT NULL,
    cost_per_unit FLOAT8 NOT NULL,
    filament_diameter FLOAT8 DEFAULT NULL,


)
