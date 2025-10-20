/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

DROP TABLE inventory CASCADE;

create table inventory
(
    id                 uuid primary key     default uuid_generate_v4(),
    product_id         uuid        not null,
    warehouse_id       uuid        not null,
    quantity_on_hand   integer     not null default 0 check (quantity_on_hand >= 0),
    quantity_reserved  integer     not null default 0 check (quantity_reserved >= 0), -- for pending orders
    quantity_available integer generated always as (quantity_on_hand - quantity_reserved) stored,
    minimum_stock      integer              default 0 check (minimum_stock >= 0),
    maximum_stock      integer check (maximum_stock IS NULL OR maximum_stock > minimum_stock),
    currency_code      varchar(3)  not null,
    status             varchar(50) not null default 'active',
    created_by_id      uuid        not null,
    created_at         timestamptz not null default now(),
    updated_at         timestamptz not null default now(),
    deleted_at         timestamptz,
    foreign key (currency_code) references currencies (code),
    foreign key (product_id) references products (id),
    foreign key (warehouse_id) references warehouses (id),
    foreign key (created_by_id) references users (id),
    unique nulls not distinct (product_id, warehouse_id, deleted_at),
    constraint check_quantity_available check (quantity_available >= 0)
);

-- Index for foreign key relationships
CREATE INDEX idx_inventory_product_id ON inventory (product_id);
CREATE INDEX idx_inventory_warehouse_id ON inventory (warehouse_id);
CREATE INDEX idx_inventory_currency_code ON inventory (currency_code);
CREATE INDEX idx_inventory_created_by_id ON inventory (created_by_id);

-- Index for status queries, especially for active/inactive filtering
CREATE INDEX idx_inventory_status ON inventory (status);

-- Index for temporal queries
CREATE INDEX idx_inventory_created_at ON inventory (created_at);
CREATE INDEX idx_inventory_updated_at ON inventory (updated_at);
CREATE INDEX idx_inventory_deleted_at ON inventory (deleted_at);

-- Index for stock management queries
CREATE INDEX idx_inventory_quantity ON inventory (quantity_on_hand, quantity_reserved, quantity_available);

-- Partial index for active inventory items
CREATE INDEX idx_inventory_active ON inventory (product_id, warehouse_id)
    WHERE deleted_at IS NULL AND status = 'active';

-- Composite index for low stock alerts
CREATE INDEX idx_inventory_low_stock ON inventory (warehouse_id, minimum_stock, quantity_available)
    WHERE deleted_at IS NULL AND status = 'active' AND quantity_available <= minimum_stock;

CREATE TRIGGER update_updated_at_on_inventory_table
    BEFORE UPDATE
    ON inventory
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table inventory_movements
(
    id             uuid primary key     default uuid_generate_v4(),
    inventory_id   uuid        not null,
    movement_type  varchar(20) not null check (movement_type IN ('in', 'out', 'adjustment', 'transfer')),
    quantity       integer     not null,        -- positive for in, negative for out
    reference_type varchar(50),
    reference_id   uuid,
    unit_price     numeric(15, 2) check (unit_price IS NULL OR unit_price >= 0),
    total_price    numeric(15, 2) check (total_price IS NULL OR total_price >= 0),
    tax_id         uuid        not null,
    movement_date  timestamptz not null default now(),
    created_by_id  uuid        not null,
    created_at     timestamptz not null default now(),
    foreign key (inventory_id) references inventory (id),
    foreign key (created_by_id) references users (id),
    foreign key (tax_id) references taxes (id), -- Add missing foreign key
    constraint check_movement_quantity check (
        (movement_type = 'in' and quantity > 0) or
        (movement_type = 'out' and quantity < 0) or
        (movement_type = 'adjustment') or
        (movement_type = 'transfer')
        ),
    constraint check_reference_consistency check (
        (reference_type IS NULL AND reference_id IS NULL) OR
        (reference_type IS NOT NULL AND reference_id IS NOT NULL)
        ),
    constraint check_price_consistency check (
        (unit_price IS NULL AND total_price IS NULL) OR
        (unit_price IS NOT NULL AND total_price IS NOT NULL AND total_price = unit_price * ABS(quantity))
        )
);

CREATE INDEX idx_inventory_movements_inventory_id ON inventory_movements (inventory_id);
CREATE INDEX idx_inventory_movements_tax_id ON inventory_movements (tax_id);
CREATE INDEX idx_inventory_movements_movement_type ON inventory_movements (movement_type);
CREATE INDEX idx_inventory_movements_reference_type_id ON inventory_movements (reference_type, reference_id);
CREATE INDEX idx_inventory_movements_movement_date ON inventory_movements (movement_date);
CREATE INDEX idx_inventory_movements_created_by_id ON inventory_movements (created_by_id);
CREATE INDEX idx_inventory_movements_created_at ON inventory_movements (created_at);
-- Index for movement history queries
CREATE INDEX idx_inventory_movements_history ON inventory_movements (inventory_id, movement_date DESC, movement_type);

create table inventory_reservations
(
    id             uuid primary key     default uuid_generate_v4(),
    inventory_id   uuid        not null,
    quantity       integer     not null check (quantity > 0),
    reference_type varchar(50) not null,
    reference_id   uuid        not null,
    reserved_until timestamptz,
    status         varchar(20) not null default 'active' check (status IN ('active', 'fulfilled', 'cancelled', 'expired')),
    created_by_id  uuid        not null,
    created_at     timestamptz not null default now(),
    updated_at     timestamptz not null default now(),
    foreign key (inventory_id) references inventory (id),
    foreign key (created_by_id) references users (id),
    constraint check_reservation_date check (reserved_until IS NULL OR reserved_until > created_at),
    unique (inventory_id, reference_type, reference_id, status) DEFERRABLE INITIALLY DEFERRED
);

CREATE INDEX idx_inventory_reservations_inventory_id ON inventory_reservations (inventory_id);
CREATE INDEX idx_inventory_reservations_reference_type_id ON inventory_reservations (reference_type, reference_id);
CREATE INDEX idx_inventory_reservations_status ON inventory_reservations (status);
CREATE INDEX idx_inventory_reservations_reserved_until ON inventory_reservations (reserved_until);
-- Index for expired reservations cleanup
CREATE INDEX idx_inventory_reservations_expired ON inventory_reservations (reserved_until, status)
    WHERE status = 'active' AND reserved_until IS NOT NULL;
-- Partial index for active reservations
CREATE INDEX idx_inventory_reservations_active ON inventory_reservations (inventory_id, quantity)
    WHERE status = 'active';

CREATE TRIGGER update_updated_at_on_inventory_reservations_table
    BEFORE UPDATE
    ON inventory_reservations
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

-- Function to update inventory quantities based on movements
CREATE OR REPLACE FUNCTION update_inventory_quantities()
    RETURNS TRIGGER AS
$$
DECLARE
    calculated_quantity integer;
    target_inventory_id uuid;
BEGIN
    -- Determine which inventory_id to use based on operation
    target_inventory_id := COALESCE(NEW.inventory_id, OLD.inventory_id);

    -- Calculate the new quantity
    SELECT COALESCE(SUM(quantity), 0)
    INTO calculated_quantity
    FROM inventory_movements
    WHERE inventory_id = target_inventory_id;

    -- Validate that the calculated quantity is not negative
    IF calculated_quantity < 0 THEN
        RAISE EXCEPTION 'Inventory quantity cannot be negative. Calculated quantity: %', calculated_quantity;
    END IF;

    -- Update the inventory quantities based on movements
    UPDATE inventory
    SET quantity_on_hand = calculated_quantity,
        updated_at       = now()
    WHERE id = target_inventory_id;

    -- Log a warning if quantity goes below minimum stock
    IF calculated_quantity <= (SELECT minimum_stock FROM inventory WHERE id = target_inventory_id) THEN
        RAISE NOTICE 'Inventory % is at or below minimum stock level', target_inventory_id;
    END IF;

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically update quantities
CREATE TRIGGER update_inventory_on_movement
    AFTER INSERT OR UPDATE OR DELETE
    ON inventory_movements
    FOR EACH ROW
EXECUTE FUNCTION update_inventory_quantities();

-- Function to update reserved quantities
CREATE OR REPLACE FUNCTION update_reserved_quantities()
    RETURNS TRIGGER AS
$$
BEGIN
    UPDATE inventory
    SET quantity_reserved = (SELECT COALESCE(SUM(quantity), 0)
                             FROM inventory_reservations
                             WHERE inventory_id = COALESCE(NEW.inventory_id, OLD.inventory_id)
                               AND status = 'active'),
        updated_at        = now()
    WHERE id = COALESCE(NEW.inventory_id, OLD.inventory_id);

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Trigger for reservations
CREATE TRIGGER update_inventory_on_reservation
    AFTER INSERT OR UPDATE OR DELETE
    ON inventory_reservations
    FOR EACH ROW
EXECUTE FUNCTION update_reserved_quantities();

CREATE OR REPLACE FUNCTION cleanup_expired_reservations()
    RETURNS integer AS
$$
DECLARE
    expired_count integer;
BEGIN
    UPDATE inventory_reservations
    SET status     = 'expired',
        updated_at = now()
    WHERE status = 'active'
      AND reserved_until IS NOT NULL
      AND reserved_until < now();

    GET DIAGNOSTICS expired_count = ROW_COUNT;

    RETURN expired_count;
END;
$$ LANGUAGE plpgsql;
