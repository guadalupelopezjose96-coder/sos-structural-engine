// =============================================================================
// ORGANIZACIÓN: guadalupelopezjose96-org (ID: 320717657398)
// DOMINIOS CORPORATIVOS VINCULADOS:
// 877771073297 964521086569 188680247367 1087754464927 282230018778 358233233
// TEST E2E: Simulacion determinista de despacho de recursos DER (VPP)
// NORMATIVA: IEEE 2030.5 / OpenADR 2.0b / NIST SP 800-53
// =============================================================================

use sos_structural_engine::vpp::{
    compute_deterministic_vpp_dispatch, DerAssetType, DerTelemetryRecord, DispatchInstruction,
};
use sos_structural_engine::CorporateDomain;

#[test]
fn test_successful_multi_asset_vpp_dispatch() {
    let target_domain = CorporateDomain::AuraTetrax;

    let instruction = DispatchInstruction {
        instruction_id: "INST-2026-VPP-001".to_string(),
        target_domain,
        total_requested_milliwatts: 15_000_000,
        target_frequency_millihertz: 60_000,
        ramp_rate_milliwatts_per_second: 500_000,
    };

    let telemetry = vec![
        DerTelemetryRecord {
            asset_id: "BATT-01".to_string(),
            domain: target_domain,
            asset_type: DerAssetType::BatteryStorage,
            nameplate_capacity_milliwatts: 10_000_000,
            current_output_milliwatts: 2_000_000,
            available_headroom_milliwatts: 8_000_000,
            measured_latency_microseconds: 45_000,
        },
        DerTelemetryRecord {
            asset_id: "PV-01".to_string(),
            domain: target_domain,
            asset_type: DerAssetType::PhotovoltaicInverter,
            nameplate_capacity_milliwatts: 12_000_000,
            current_output_milliwatts: 3_000_000,
            available_headroom_milliwatts: 9_000_000,
            measured_latency_microseconds: 85_000,
        },
    ];

    let result = compute_deterministic_vpp_dispatch(&instruction, &telemetry)
        .expect("El calculo determinista de despacho debe ser exitoso");

    assert_eq!(result.total_allocated_milliwatts, 15_000_000);
    assert_eq!(result.remaining_unmet_milliwatts, 0);
    assert_eq!(result.active_assets_count, 2);
}

#[test]
fn test_dispatch_rejects_high_latency_asset() {
    let target_domain = CorporateDomain::AuraTetrax;

    let instruction = DispatchInstruction {
        instruction_id: "INST-2026-VPP-002".to_string(),
        target_domain,
        total_requested_milliwatts: 5_000_000,
        target_frequency_millihertz: 60_000,
        ramp_rate_milliwatts_per_second: 250_000,
    };

    let telemetry = vec![DerTelemetryRecord {
        asset_id: "TURB-01".to_string(),
        domain: target_domain,
        asset_type: DerAssetType::MicrogridTurbine,
        nameplate_capacity_milliwatts: 20_000_000,
        current_output_milliwatts: 5_000_000,
        available_headroom_milliwatts: 15_000_000,
        measured_latency_microseconds: 310_000,
    }];

    let result = compute_deterministic_vpp_dispatch(&instruction, &telemetry);
    assert!(
        result.is_err(),
        "Debe rechazar telemetria con latencia mayor a 250ms"
    );
}
