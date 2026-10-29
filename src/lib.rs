pub mod vpp;
// =============================================================================
// ORGANIZACIÓN: guadalupelopezjose96-org (ID: 320717657398 — Tech Arquitecture Pro-Elite)
// COMPONENTE: System-of-Systems Topology, Bulkhead Isolation & AAUFSA Ledger Engine
// MARCO LEGAL: Automated Asset Utilization and Fiscal Solvency Act of 2026 (AAUFSA)
//              Internal Revenue Code (IRC) Chapter 56 (Secs. 5000F, 5000G, 5000H) & Sec. 4
// AUTOR Y TITULAR: José Emanuel Guadalupe López (Prohibida la alteración estatutaria - 0% Drift)
// ESTÁNDARES: ISO/IEC/IEEE 21839, ISO/IEC/IEEE 42010, NIST SP 800-53 Rev. 5 (SC-2, SC-3, SC-7)
// =============================================================================

#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::float_arithmetic,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::wildcard_imports
)]
#![allow(clippy::struct_excessive_bools)]

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// Identificador inmutable de la Organización Raíz.
pub const ORG_ROOT_ID: &str = "320717657398";
pub const ORG_ROOT_HANDLE: &str = "guadalupelopezjose96-org";

/// Los 6 dominios corporativos autorizados bajo aislamiento Row-Level Security (RLS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CorporateDomain {
    AuraTetrax,
    Fourphonix,
    AuraIntelCore,
    AuraIntelGlobal,
    Standard1979,
    Prophonix,
}

impl CorporateDomain {
    #[must_use]
    pub const fn numeric_id(self) -> &'static str {
        match self {
            Self::AuraTetrax => "877771073297",
            Self::Fourphonix => "964521086569",
            Self::AuraIntelCore => "188680247367",
            Self::AuraIntelGlobal => "1087754464927",
            Self::Standard1979 => "282230018778",
            Self::Prophonix => "358233233",
        }
    }

    #[must_use]
    pub const fn canonical_fqdn(self) -> &'static str {
        match self {
            Self::AuraTetrax => "AuraTetrax-com",
            Self::Fourphonix => "Fourphonix-com",
            Self::AuraIntelCore => "AuraIntelCore-com",
            Self::AuraIntelGlobal => "AuraIntel-Global-com",
            Self::Standard1979 => "Standard1979-com",
            Self::Prophonix => "Prophonix-com",
        }
    }
}

/// Planos estructurales del Sistema de Sistemas (NIST SP 800-53 SC-2 / SC-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SystemPlane {
    ControlPlane = 0,
    DataPlaneVpp = 1,
    FiscalPlaneAaufsa = 2,
}

impl SystemPlane {
    #[must_use]
    pub const fn tier_rank(self) -> u8 {
        match self {
            Self::ControlPlane => 0,
            Self::DataPlaneVpp => 1,
            Self::FiscalPlaneAaufsa => 2,
        }
    }
}

/// Errores deterministas de verificación estructural, aislamiento y cálculo fiscal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralError {
    CyclicDependencyDetected {
        involved_nodes: Vec<String>,
    },
    IllegalCrossPlaneReverseCoupling {
        from_node: String,
        from_plane: SystemPlane,
        to_node: String,
        to_plane: SystemPlane,
    },
    UnknownComponentReference {
        component_id: String,
    },
    BulkheadCapacityExceeded {
        domain: CorporateDomain,
        plane: SystemPlane,
        requested_slots: u32,
        available_slots: u32,
    },
    ArithmeticOverflowOrPrecisionFault {
        context: String,
    },
    InvalidControlledGroupOwnership {
        entity_id: String,
        reported_percentage: Decimal,
    },
    UnbalancedDoubleEntryLedger {
        total_debits_usd: Decimal,
        total_credits_usd: Decimal,
    },
    EmptyControlledGroup,
}

impl fmt::Display for StructuralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CyclicDependencyDetected { involved_nodes } => {
                write!(f, "Violación DAG: Ciclo detectado en nodos {involved_nodes:?}")
            }
            Self::IllegalCrossPlaneReverseCoupling {
                from_node,
                from_plane,
                to_node,
                to_plane,
            } => write!(
                f,
                "Violación NIST SC-2/SC-3: Acoplamiento inverso ilegal desde {from_node} ({from_plane:?}) hacia {to_node} ({to_plane:?})"
            ),
            Self::UnknownComponentReference { component_id } => {
                write!(f, "Referencia estructural inexistente: {component_id}")
            }
            Self::BulkheadCapacityExceeded {
                domain,
                plane,
                requested_slots,
                available_slots,
            } => write!(
                f,
                "Violación Bulkhead en {domain:?}/{plane:?}: solicitado {requested_slots}, disponible {available_slots}"
            ),
            Self::ArithmeticOverflowOrPrecisionFault { context } => {
                write!(f, "Fallo aritmético determinista en contexto: {context}")
            }
            Self::InvalidControlledGroupOwnership {
                entity_id,
                reported_percentage,
            } => write!(
                f,
                "Porcentaje de propiedad inválido en entidad {entity_id}: {reported_percentage}"
            ),
            Self::UnbalancedDoubleEntryLedger {
                total_debits_usd,
                total_credits_usd,
            } => write!(
                f,
                "Violación ACID Partida Doble: Débitos ({total_debits_usd}) != Créditos ({total_credits_usd})"
            ),
            Self::EmptyControlledGroup => {
                write!(f, "El grupo controlado bajo IRC Sec. 414/1563 no contiene entidades")
            }
        }
    }
}

impl std::error::Error for StructuralError {}

// =============================================================================
// 1. VERIFICADOR DE TOPOLOGÍA DAG Y AISLAMIENTO DE PLANOS
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentNode {
    pub component_id: String,
    pub domain: CorporateDomain,
    pub plane: SystemPlane,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectedEdge {
    pub source_id: String,
    pub target_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct SoSTopologyVerifier {
    nodes: BTreeMap<String, ComponentNode>,
    adjacency: BTreeMap<String, BTreeSet<String>>,
}

impl SoSTopologyVerifier {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_node(&mut self, node: ComponentNode) {
        self.adjacency.entry(node.component_id.clone()).or_default();
        self.nodes.insert(node.component_id.clone(), node);
    }

    /// Registra una arista dirigida validando separación jerárquica de planos.
    ///
    /// # Errors
    /// Retorna `StructuralError` si un nodo no existe o viola el orden de planos.
    pub fn add_dependency(&mut self, edge: &DirectedEdge) -> Result<(), StructuralError> {
        let src = self.nodes.get(&edge.source_id).ok_or_else(|| {
            StructuralError::UnknownComponentReference {
                component_id: edge.source_id.clone(),
            }
        })?;
        let dst = self.nodes.get(&edge.target_id).ok_or_else(|| {
            StructuralError::UnknownComponentReference {
                component_id: edge.target_id.clone(),
            }
        })?;

        if src.plane.tier_rank() > dst.plane.tier_rank() {
            return Err(StructuralError::IllegalCrossPlaneReverseCoupling {
                from_node: src.component_id.clone(),
                from_plane: src.plane,
                to_node: dst.component_id.clone(),
                to_plane: dst.plane,
            });
        }

        if let Some(targets) = self.adjacency.get_mut(&edge.source_id) {
            targets.insert(edge.target_id.clone());
        }
        Ok(())
    }

    /// Algoritmo de Kahn para validar DAG acíclico.
    ///
    /// # Errors
    /// Retorna `StructuralError::CyclicDependencyDetected` si se detecta un ciclo.
    pub fn verify_dag_and_topological_sort(&self) -> Result<Vec<String>, StructuralError> {
        let mut in_degree: BTreeMap<String, usize> = BTreeMap::new();
        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        for targets in self.adjacency.values() {
            for target in targets {
                if let Some(deg) = in_degree.get_mut(target) {
                    *deg = deg.saturating_add(1);
                }
            }
        }

        let mut queue: VecDeque<String> = VecDeque::new();
        for (node_id, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(node_id.clone());
            }
        }

        let mut topological_order: Vec<String> = Vec::with_capacity(self.nodes.len());

        while let Some(current) = queue.pop_front() {
            topological_order.push(current.clone());
            if let Some(neighbors) = self.adjacency.get(&current) {
                for neighbor in neighbors {
                    if let Some(deg) = in_degree.get_mut(neighbor) {
                        *deg = deg.saturating_sub(1);
                        if *deg == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if topological_order.len() != self.nodes.len() {
            let cyclic_nodes: Vec<String> = in_degree
                .into_iter()
                .filter_map(|(k, v)| if v > 0 { Some(k) } else { None })
                .collect();
            return Err(StructuralError::CyclicDependencyDetected {
                involved_nodes: cyclic_nodes,
            });
        }

        Ok(topological_order)
    }
}

// =============================================================================
// 2. AISLAMIENTO BULKHEAD Y TELEMETRÍA DETERMINISTA
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkheadQuota {
    pub domain: CorporateDomain,
    pub plane: SystemPlane,
    pub max_concurrent_slots: u32,
    pub active_slots: u32,
    pub max_memory_bytes: u64,
}

impl BulkheadQuota {
    #[must_use]
    pub const fn new(
        domain: CorporateDomain,
        plane: SystemPlane,
        max_concurrent_slots: u32,
        max_memory_bytes: u64,
    ) -> Self {
        Self {
            domain,
            plane,
            max_concurrent_slots,
            active_slots: 0,
            max_memory_bytes,
        }
    }

    /// Intenta adquirir cuota de ejecución aislada.
    ///
    /// # Errors
    /// Retorna `StructuralError::BulkheadCapacityExceeded` si excede el límite.
    pub fn try_acquire(&mut self, requested_slots: u32) -> Result<(), StructuralError> {
        let available = self.max_concurrent_slots.saturating_sub(self.active_slots);
        if requested_slots > available {
            return Err(StructuralError::BulkheadCapacityExceeded {
                domain: self.domain,
                plane: self.plane,
                requested_slots,
                available_slots: available,
            });
        }
        self.active_slots = self.active_slots.saturating_add(requested_slots);
        Ok(())
    }

    pub fn release(&mut self, released_slots: u32) {
        self.active_slots = self.active_slots.saturating_sub(released_slots);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VppDerNodeTelemetry {
    pub node_id: String,
    pub domain: CorporateDomain,
    pub available_capacity_milliwatts: u64,
    pub dispatched_setpoint_milliwatts: u64,
    pub response_latency_microseconds: u64,
}

/// Agregación determinista de setpoints sin coma flotante.
///
/// # Errors
/// Retorna `StructuralError::ArithmeticOverflowOrPrecisionFault` en caso de sobrecapacidad o desbordamiento.
pub fn aggregate_vpp_dispatch_milliwatts(
    nodes: &[VppDerNodeTelemetry],
    target_domain: CorporateDomain,
) -> Result<u64, StructuralError> {
    let mut total_dispatched_mw: u64 = 0;
    for node in nodes {
        if node.domain == target_domain {
            if node.dispatched_setpoint_milliwatts > node.available_capacity_milliwatts {
                return Err(StructuralError::ArithmeticOverflowOrPrecisionFault {
                    context: format!(
                        "DER {} setpoint ({}) > capacidad disponible ({})",
                        node.node_id,
                        node.dispatched_setpoint_milliwatts,
                        node.available_capacity_milliwatts
                    ),
                });
            }
            total_dispatched_mw = total_dispatched_mw
                .checked_add(node.dispatched_setpoint_milliwatts)
                .ok_or_else(|| StructuralError::ArithmeticOverflowOrPrecisionFault {
                    context: "Desbordamiento en suma de mili-vatios VPP".to_string(),
                })?;
        }
    }
    Ok(total_dispatched_mw)
}

// =============================================================================
// 3. MOTOR FISCAL AAUFSA (IRC CAPÍTULO 56 & SEC. 4)
// =============================================================================

pub const AAUFSA_EXCISE_TAX_RATE: Decimal = dec!(0.1530);
pub const AAUFSA_CONTROLLED_GROUP_THRESHOLD_USD: Decimal = dec!(5000000.00);
pub const AAUFSA_SAFE_HARBOR_MIN_MONTHS: u32 = 36;
pub const AAUFSA_TRUST_SPLIT_OASDI_RATE: Decimal = dec!(0.7500);
pub const AAUFSA_TRUST_SPLIT_RESKILLING_RATE: Decimal = dec!(0.2500);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlledGroupMember {
    pub entity_id: String,
    pub domain: CorporateDomain,
    pub annual_gross_receipts_usd: Decimal,
    pub common_control_ownership_pct: Decimal,
    pub is_end_user_displacing_entity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomatedAssetTaxAssessmentInput {
    pub assessment_id: String,
    pub domain: CorporateDomain,
    pub imputed_human_displacement_cost_usd: Decimal,
    pub reassigned_at_equal_or_greater_wage: bool,
    pub verified_retention_months: u32,
    pub premature_termination_without_cause: bool,
    pub statutory_clawback_interest_rate: Decimal,
    /// Indica si la operación corresponde exclusivamente a provisión de infraestructura `SaaS` a un tercero (`Sec. 5000H(d)`).
    pub is_pure_cloud_saas_vendor_provision: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AaufsaConsolidatedTaxResult {
    pub consolidated_group_gross_receipts_usd: Decimal,
    pub is_exempt_small_business_sec_5000g_b: bool,
    pub is_exempt_saas_vendor_sec_5000h_d: bool,
    pub safe_harbor_36m_applied_sec_5000g_a: bool,
    pub clawback_triggered_sec_5000g_c: bool,
    pub base_excise_tax_15_3_pct_usd: Decimal,
    pub clawback_interest_usd: Decimal,
    pub total_chapter_56_liability_usd: Decimal,
    pub oasdi_trust_fund_75_pct_usd: Decimal,
    pub workforce_reskilling_trust_fund_25_pct_usd: Decimal,
}

/// Consolida ingresos brutos bajo IRC Secs. 414 y 1563.
///
/// # Errors
/// Retorna `StructuralError` si el grupo es inválido o se produce desbordamiento.
pub fn evaluate_irc_414_1563_controlled_group_receipts(
    members: &[ControlledGroupMember],
) -> Result<Decimal, StructuralError> {
    if members.is_empty() {
        return Err(StructuralError::EmptyControlledGroup);
    }

    let mut consolidated_receipts = dec!(0.00);
    for member in members {
        if member.common_control_ownership_pct < dec!(0.00)
            || member.common_control_ownership_pct > dec!(100.00)
        {
            return Err(StructuralError::InvalidControlledGroupOwnership {
                entity_id: member.entity_id.clone(),
                reported_percentage: member.common_control_ownership_pct,
            });
        }

        if member.common_control_ownership_pct >= dec!(50.00) {
            consolidated_receipts = consolidated_receipts
                .checked_add(member.annual_gross_receipts_usd)
                .ok_or_else(|| StructuralError::ArithmeticOverflowOrPrecisionFault {
                    context: "Desbordamiento consolidando ingresos IRC 414/1563".to_string(),
                })?;
        }
    }

    Ok(consolidated_receipts.round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven))
}

/// Calcula el impuesto especial del Capítulo 56 del IRC y partición fiduciaria.
///
/// # Errors
/// Retorna `StructuralError` ante desbordamientos o validaciones fallidas.
pub fn compute_aaufsa_chapter_56_obligation(
    group_members: &[ControlledGroupMember],
    input: &AutomatedAssetTaxAssessmentInput,
) -> Result<AaufsaConsolidatedTaxResult, StructuralError> {
    let consolidated_receipts = evaluate_irc_414_1563_controlled_group_receipts(group_members)?;

    if input.is_pure_cloud_saas_vendor_provision {
        return Ok(AaufsaConsolidatedTaxResult {
            consolidated_group_gross_receipts_usd: consolidated_receipts,
            is_exempt_small_business_sec_5000g_b: false,
            is_exempt_saas_vendor_sec_5000h_d: true,
            safe_harbor_36m_applied_sec_5000g_a: false,
            clawback_triggered_sec_5000g_c: false,
            base_excise_tax_15_3_pct_usd: dec!(0.00),
            clawback_interest_usd: dec!(0.00),
            total_chapter_56_liability_usd: dec!(0.00),
            oasdi_trust_fund_75_pct_usd: dec!(0.00),
            workforce_reskilling_trust_fund_25_pct_usd: dec!(0.00),
        });
    }

    if consolidated_receipts <= AAUFSA_CONTROLLED_GROUP_THRESHOLD_USD {
        return Ok(AaufsaConsolidatedTaxResult {
            consolidated_group_gross_receipts_usd: consolidated_receipts,
            is_exempt_small_business_sec_5000g_b: true,
            is_exempt_saas_vendor_sec_5000h_d: false,
            safe_harbor_36m_applied_sec_5000g_a: false,
            clawback_triggered_sec_5000g_c: false,
            base_excise_tax_15_3_pct_usd: dec!(0.00),
            clawback_interest_usd: dec!(0.00),
            total_chapter_56_liability_usd: dec!(0.00),
            oasdi_trust_fund_75_pct_usd: dec!(0.00),
            workforce_reskilling_trust_fund_25_pct_usd: dec!(0.00),
        });
    }

    let qualifies_safe_harbor = input.reassigned_at_equal_or_greater_wage
        && input.verified_retention_months >= AAUFSA_SAFE_HARBOR_MIN_MONTHS
        && !input.premature_termination_without_cause;

    if qualifies_safe_harbor {
        return Ok(AaufsaConsolidatedTaxResult {
            consolidated_group_gross_receipts_usd: consolidated_receipts,
            is_exempt_small_business_sec_5000g_b: false,
            is_exempt_saas_vendor_sec_5000h_d: false,
            safe_harbor_36m_applied_sec_5000g_a: true,
            clawback_triggered_sec_5000g_c: false,
            base_excise_tax_15_3_pct_usd: dec!(0.00),
            clawback_interest_usd: dec!(0.00),
            total_chapter_56_liability_usd: dec!(0.00),
            oasdi_trust_fund_75_pct_usd: dec!(0.00),
            workforce_reskilling_trust_fund_25_pct_usd: dec!(0.00),
        });
    }

    let raw_base_tax = input
        .imputed_human_displacement_cost_usd
        .checked_mul(AAUFSA_EXCISE_TAX_RATE)
        .ok_or_else(|| StructuralError::ArithmeticOverflowOrPrecisionFault {
            context: "Desbordamiento calculando 15.3% Sec. 5000F".to_string(),
        })?;
    let base_tax = raw_base_tax.round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven);

    let clawback_triggered = input.reassigned_at_equal_or_greater_wage
        && input.premature_termination_without_cause
        && input.verified_retention_months < AAUFSA_SAFE_HARBOR_MIN_MONTHS;

    let clawback_interest = if clawback_triggered {
        base_tax
            .checked_mul(input.statutory_clawback_interest_rate)
            .ok_or_else(|| StructuralError::ArithmeticOverflowOrPrecisionFault {
                context: "Desbordamiento calculando interés Clawback Sec. 5000G(c)".to_string(),
            })?
            .round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven)
    } else {
        dec!(0.00)
    };

    let total_liability = base_tax.checked_add(clawback_interest).ok_or_else(|| {
        StructuralError::ArithmeticOverflowOrPrecisionFault {
            context: "Desbordamiento sumando obligación total Capítulo 56".to_string(),
        }
    })?;

    let oasdi_share = total_liability
        .checked_mul(AAUFSA_TRUST_SPLIT_OASDI_RATE)
        .ok_or_else(|| StructuralError::ArithmeticOverflowOrPrecisionFault {
            context: "Desbordamiento calculando 75% OASDI Trust Fund".to_string(),
        })?
        .round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven);

    let reskilling_share = total_liability.checked_sub(oasdi_share).ok_or_else(|| {
        StructuralError::ArithmeticOverflowOrPrecisionFault {
            context: "Desbordamiento calculando 25% Workforce Reskilling Trust Fund".to_string(),
        }
    })?;

    Ok(AaufsaConsolidatedTaxResult {
        consolidated_group_gross_receipts_usd: consolidated_receipts,
        is_exempt_small_business_sec_5000g_b: false,
        is_exempt_saas_vendor_sec_5000h_d: false,
        safe_harbor_36m_applied_sec_5000g_a: false,
        clawback_triggered_sec_5000g_c: clawback_triggered,
        base_excise_tax_15_3_pct_usd: base_tax,
        clawback_interest_usd: clawback_interest,
        total_chapter_56_liability_usd: total_liability,
        oasdi_trust_fund_75_pct_usd: oasdi_share,
        workforce_reskilling_trust_fund_25_pct_usd: reskilling_share,
    })
}

// =============================================================================
// 4. PARTIDA DOBLE Y ENCADENAMIENTO MERKLE
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntrySide {
    Debit,
    Credit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerPostingLine {
    pub account_code: String,
    pub account_name: String,
    pub side: EntrySide,
    pub amount_usd: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializableJournalTransaction {
    pub idempotency_key: String,
    pub domain: CorporateDomain,
    pub lines: Vec<LedgerPostingLine>,
    pub previous_merkle_hash: String,
    pub current_merkle_hash: String,
}

/// Construye asiento contable balanceado y sella hash SHA-256.
///
/// # Errors
/// Retorna `StructuralError::UnbalancedDoubleEntryLedger` si débitos != créditos.
pub fn build_serializable_aaufsa_journal(
    idempotency_key: &str,
    domain: CorporateDomain,
    tax_result: &AaufsaConsolidatedTaxResult,
    previous_hash: &str,
) -> Result<SerializableJournalTransaction, StructuralError> {
    let lines = vec![
        LedgerPostingLine {
            account_code: "6156-AAUFSA-EXP".to_string(),
            account_name: "IRC Chapter 56 Automated Asset Excise Tax Expense".to_string(),
            side: EntrySide::Debit,
            amount_usd: tax_result.total_chapter_56_liability_usd,
        },
        LedgerPostingLine {
            account_code: "2156-OASDI-75".to_string(),
            account_name: "Payable - Federal Old-Age and Survivors Insurance Trust Fund (75%)"
                .to_string(),
            side: EntrySide::Credit,
            amount_usd: tax_result.oasdi_trust_fund_75_pct_usd,
        },
        LedgerPostingLine {
            account_code: "2157-RESKILL-25".to_string(),
            account_name: "Payable - Workforce Reskilling Trust Fund (25%)".to_string(),
            side: EntrySide::Credit,
            amount_usd: tax_result.workforce_reskilling_trust_fund_25_pct_usd,
        },
    ];

    let mut total_debits = dec!(0.00);
    let mut total_credits = dec!(0.00);

    for line in &lines {
        match line.side {
            EntrySide::Debit => {
                total_debits = total_debits.checked_add(line.amount_usd).ok_or_else(|| {
                    StructuralError::ArithmeticOverflowOrPrecisionFault {
                        context: "Suma de débitos en libro mayor".to_string(),
                    }
                })?;
            }
            EntrySide::Credit => {
                total_credits = total_credits.checked_add(line.amount_usd).ok_or_else(|| {
                    StructuralError::ArithmeticOverflowOrPrecisionFault {
                        context: "Suma de créditos en libro mayor".to_string(),
                    }
                })?;
            }
        }
    }

    if total_debits != total_credits {
        return Err(StructuralError::UnbalancedDoubleEntryLedger {
            total_debits_usd: total_debits,
            total_credits_usd: total_credits,
        });
    }

    let mut hasher = Sha256::new();
    hasher.update(previous_hash.as_bytes());
    hasher.update(idempotency_key.as_bytes());
    hasher.update(domain.numeric_id().as_bytes());
    hasher.update(total_debits.to_string().as_bytes());
    let digest = hasher.finalize();
    let current_merkle_hash = format!("{digest:x}");

    Ok(SerializableJournalTransaction {
        idempotency_key: idempotency_key.to_string(),
        domain,
        lines,
        previous_merkle_hash: previous_hash.to_string(),
        current_merkle_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_enforces_strict_plane_partitioning_and_acyclicity() {
        let mut verifier = SoSTopologyVerifier::new();
        verifier.register_node(ComponentNode {
            component_id: "ctrl-orchestrator".to_string(),
            domain: CorporateDomain::AuraIntelCore,
            plane: SystemPlane::ControlPlane,
        });
        verifier.register_node(ComponentNode {
            component_id: "vpp-dispatch-engine".to_string(),
            domain: CorporateDomain::AuraIntelGlobal,
            plane: SystemPlane::DataPlaneVpp,
        });
        verifier.register_node(ComponentNode {
            component_id: "aaufsa-tax-ledger".to_string(),
            domain: CorporateDomain::Standard1979,
            plane: SystemPlane::FiscalPlaneAaufsa,
        });

        assert!(verifier
            .add_dependency(&DirectedEdge {
                source_id: "ctrl-orchestrator".to_string(),
                target_id: "vpp-dispatch-engine".to_string(),
            })
            .is_ok());
        assert!(verifier
            .add_dependency(&DirectedEdge {
                source_id: "vpp-dispatch-engine".to_string(),
                target_id: "aaufsa-tax-ledger".to_string(),
            })
            .is_ok());

        let reverse_err = verifier.add_dependency(&DirectedEdge {
            source_id: "aaufsa-tax-ledger".to_string(),
            target_id: "vpp-dispatch-engine".to_string(),
        });
        assert!(matches!(
            reverse_err,
            Err(StructuralError::IllegalCrossPlaneReverseCoupling { .. })
        ));

        let order = verifier.verify_dag_and_topological_sort();
        assert!(order.is_ok());
    }

    #[test]
    fn test_irc_414_1563_controlled_group_anti_fragmentation_and_15_3_tax() {
        let group = vec![
            ControlledGroupMember {
                entity_id: "SUB-A".to_string(),
                domain: CorporateDomain::AuraTetrax,
                annual_gross_receipts_usd: dec!(3000000.00),
                common_control_ownership_pct: dec!(100.00),
                is_end_user_displacing_entity: true,
            },
            ControlledGroupMember {
                entity_id: "SUB-B".to_string(),
                domain: CorporateDomain::Fourphonix,
                annual_gross_receipts_usd: dec!(3000000.00),
                common_control_ownership_pct: dec!(80.00),
                is_end_user_displacing_entity: true,
            },
        ];

        let input = AutomatedAssetTaxAssessmentInput {
            assessment_id: "ASSESS-2026-001".to_string(),
            domain: CorporateDomain::AuraTetrax,
            imputed_human_displacement_cost_usd: dec!(1000000.00),
            reassigned_at_equal_or_greater_wage: false,
            verified_retention_months: 0,
            premature_termination_without_cause: false,
            statutory_clawback_interest_rate: dec!(0.00),
            is_pure_cloud_saas_vendor_provision: false,
        };

        let res = compute_aaufsa_chapter_56_obligation(&group, &input);
        assert!(res.is_ok());
        if let Ok(val) = res {
            assert_eq!(val.consolidated_group_gross_receipts_usd, dec!(6000000.00));
            assert!(!val.is_exempt_small_business_sec_5000g_b);
            assert_eq!(val.base_excise_tax_15_3_pct_usd, dec!(153000.00));
            assert_eq!(val.oasdi_trust_fund_75_pct_usd, dec!(114750.00));
            assert_eq!(
                val.workforce_reskilling_trust_fund_25_pct_usd,
                dec!(38250.00)
            );
        }
    }
}
