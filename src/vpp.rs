// =============================================================================
// ORGANIZACIÓN: guadalupelopezjose96-org (ID: 320717657398)
// DOMINIOS CORPORATIVOS VINCULADOS (NIST SP 800-53 / SC-2 / SC-7):
// - 320717657398: Matriz Corporativa / Control Central
// - 188680247367: AuraIntelCore-com (Fiscal / Ledger Engine)
// - 877771073297: AuraTetrax-com (VPP Telemetry & Dispatch)
// - 417859876210: AlphaPulse-com
// - 529810349812: CyberShield-com
// - 630921478531: QuantumData-com
// COMPONENTE: Virtual Power Plant (VPP) DER Telemetry & Dispatch Contract
// NORMATIVA: IEEE 2030.5 / OpenADR 2.0b / NIST SP 800-53 Rev. 5
// =============================================================================

use crate::{CorporateDomain, StructuralError};
use serde::{Deserialize, Serialize};

pub const MAX_PERMISSIBLE_TELEMETRY_LATENCY_US: u64 = 250_000; // 250 ms

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerAssetType {
    BatteryStorage,
    PhotovoltaicInverter,
    MicrogridTurbine,
    FlexibleLoad,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerTelemetryRecord {
    pub asset_id: String,
    pub domain: CorporateDomain,
    pub asset_type: DerAssetType,
    pub nameplate_capacity_milliwatts: u64,
    pub current_output_milliwatts: u64,
    pub available_headroom_milliwatts: u64,
    pub measured_latency_microseconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchInstruction {
    pub instruction_id: String,
    pub target_domain: CorporateDomain,
    pub total_requested_milliwatts: u64,
    pub target_frequency_millihertz: u32,
    pub ramp_rate_milliwatts_per_second: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchAllocationResult {
    pub instruction_id: String,
    pub domain: CorporateDomain,
    pub total_allocated_milliwatts: u64,
    pub remaining_unmet_milliwatts: u64,
    pub active_assets_count: usize,
}

/// Valida la latencia y calcula la asignación de despacho por activo DER.
///
/// # Errors
/// Retorna `StructuralError::ArithmeticOverflowOrPrecisionFault` si se viola la latencia o la capacidad.
pub fn compute_deterministic_vpp_dispatch(
    instruction: &DispatchInstruction,
    assets: &[DerTelemetryRecord],
) -> Result<DispatchAllocationResult, StructuralError> {
    let mut allocated_milliwatts: u64 = 0;
    let mut active_count: usize = 0;
    let mut remaining_needed = instruction.total_requested_milliwatts;

    for asset in assets {
        if asset.domain != instruction.target_domain {
            continue;
        }

        if asset.measured_latency_microseconds > MAX_PERMISSIBLE_TELEMETRY_LATENCY_US {
            let asset_id = &asset.asset_id;
            let measured = asset.measured_latency_microseconds;
            return Err(StructuralError::ArithmeticOverflowOrPrecisionFault {
                context: format!(
                    "Telemetria DER {asset_id} excedio latencia critica: {measured}us > {MAX_PERMISSIBLE_TELEMETRY_LATENCY_US}us"
                ),
            });
        }

        if remaining_needed > 0 && asset.available_headroom_milliwatts > 0 {
            let take = remaining_needed.min(asset.available_headroom_milliwatts);
            allocated_milliwatts = allocated_milliwatts
                .checked_add(take)
                .ok_or_else(|| StructuralError::ArithmeticOverflowOrPrecisionFault {
                    context: "Desbordamiento en suma de despacho DER".to_string(),
                })?;
            remaining_needed = remaining_needed.saturating_sub(take);
            active_count = active_count.saturating_add(1);
        }
    }

    Ok(DispatchAllocationResult {
        instruction_id: instruction.instruction_id.clone(),
        domain: instruction.target_domain,
        total_allocated_milliwatts: allocated_milliwatts,
        remaining_unmet_milliwatts: remaining_needed,
        active_assets_count: active_count,
    })
}
