import { describe, expect, it } from "vitest";
import { customerFormSchema } from "./customers/lib/schemas";
import { inventoryFormSchema } from "./inventory/lib/schemas";
import { inventoryMovementFormSchema } from "./inventory_movements/lib/schemas";
import { inventoryReservationFormSchema } from "./inventory_reservations/lib/schemas";
import { serviceFormSchema } from "./services/lib/schemas";
import { taskFormSchema } from "./tasks/lib/schemas";
import { taxFormSchema } from "./taxes/lib/schemas";

describe("derived form schemas", () => {
    it("accepts customer address defaults and optional fields", () => {
        expect(
            customerFormSchema.safeParse({
                name: "",
                contact_name: "",
                email: "",
                phone_number: "",
                customer_type: "natural",
                status: "active",
                billing_address: null,
                mailing_address: undefined,
            }).success,
        ).toBe(true);
    });

    it("keeps numeric and derived fields outside their form schemas", () => {
        expect(inventoryFormSchema.keyof().options).not.toContain("minimum_stock");
        expect(inventoryFormSchema.keyof().options).not.toContain("maximum_stock");
        expect(inventoryMovementFormSchema.keyof().options).toEqual([
            "inventory_id",
            "movement_type",
            "tax_id",
        ]);
        expect(inventoryReservationFormSchema.keyof().options).not.toContain(
            "quantity",
        );
        expect(taskFormSchema.keyof().options).not.toContain("price");
        expect(serviceFormSchema.keyof().options).not.toContain("default_price");
    });

    it("uses text for nullable service fields in the form", () => {
        const form = {
            name: "",
            description: "",
            default_tax_id: "",
            currency_code: "HUF",
            status: "active",
            unit_of_measure_id: "",
            new_unit_of_measure: "",
        };
        expect(serviceFormSchema.safeParse(form).success).toBe(true);
        expect(
            serviceFormSchema.safeParse({ ...form, description: null }).success,
        ).toBe(false);
    });

    it("uses a string for the tax applicability select", () => {
        const form = {
            rate: "",
            description: "",
            country_code: "HU",
            tax_category: "",
            is_rate_applicable: "",
            legal_text: "",
            reporting_code: "",
            is_default: false,
            status: "active",
        };
        expect(taxFormSchema.safeParse(form).success).toBe(true);
        expect(
            taxFormSchema.safeParse({ ...form, is_rate_applicable: false })
                .success,
        ).toBe(false);
    });
});