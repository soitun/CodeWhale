/**
 * Regression guard for #3772: every Rust `ApiProvider` variant must be either
 * mapped to a website label or intentionally excluded. `unmappedProviderVariants`
 * reads crates/tui/src/config.rs relative to the repo root (__dirname-based), so
 * this test is independent of the vitest working directory.
 *
 * The CI hard-gate lives in `scripts/check-facts.mjs`; this test fails fast in
 * the unit suite if a new provider variant is added without a website mapping.
 */
import { describe, it, expect } from "vitest";
import { unmappedProviderVariants } from "../scripts/facts-lib.mjs";

describe("ApiProvider website mapping (#3772)", () => {
  it("has no unmapped, non-excluded provider variants", () => {
    expect(unmappedProviderVariants()).toEqual([]);
  });
});
