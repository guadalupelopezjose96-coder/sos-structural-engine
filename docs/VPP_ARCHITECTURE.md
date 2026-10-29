# Virtual Power Plant (VPP) Telemetry & Deterministic Dispatch Architecture

- **Organización:** `guadalupelopezjose96-org` (ID: `320717657398`)
- **Componente:** `sos-structural-engine::vpp`
- **Versión de Especificación:** `v1.2.0`
- **Estándares Normativos:** NIST SP 800-53 Rev. 5 (SC-2, SC-3, SC-7), IEEE 2030.5-2018, OpenADR 2.0b
- **Cumplimiento Fiscal:** AAUFSA / IRC Sec. 56

---

## 1. Alcance y Principios de Diseño

El subsistema VPP provee una interfaz determinista de asignación de despacho para Recursos Energéticos Distribuidos (DER). Opera bajo las siguientes garantías formales:

1. **Aritmética Determinista:** Ausencia total de operaciones en punto flotante (`f32`/`f64`). Todas las magnitudes energéticas y de potencia se representan en enteros deterministas de precisión fija (`u64` para milivatios, `u32` para milihercios).
2. **Memoria y Ejecución Segura:** Prohibición estricta de bloques `unsafe` (`#![forbid(unsafe_code)]`). Prevención de pánicos en producción mediante operaciones aritméticas con comprobación de desbordamiento (`checked_add`, `saturating_sub`).
3. **Aislamiento Multidominio (SC-7):** Las instrucciones de despacho están rígidamente vinculadas a uno de los dominios corporativos autorizados:
   - `320717657398` (Matriz Corporativa)
   - `188680247367` (AuraIntelCore - Fiscal / Ledger)
   - `877771073297` (AuraTetrax - VPP Telemetry & Dispatch)
   - `417859876210` (AlphaPulse)
   - `529810349812` (CyberShield)
   - `630921478531` (QuantumData)

---

## 2. Modelo de Latencia Crítica (NIST SP 800-53 / SC-2)

Conforme a los requisitos de despacho de frecuencia de respuesta rápida de la red, se impone un límite superior no negociable de latencia de telemetría:

$$\Delta t_{\text{max}} = 250\,000\,\mu\text{s}\quad (250\,\text{ms})$$

Cualquier activo DER que reporte una latencia de medición $\Delta t > \Delta t_{\text{max}}$ es rechazado de inmediato antes de procesar el cálculo de asignación, retornando un error estructural `ArithmeticOverflowOrPrecisionFault`.

---

## 3. Algoritmo Determinista de Asignación de Potencia

Dado un requerimiento de potencia objetivo $P_{\text{req}}$ (en milivatios) y un conjunto ordenado de activos DER $\{A_1, A_2, \dots, A_n\}$ pertenecientes al dominio destino con capacidad disponible $H_i$:

1. Inicializar $P_{\text{restante}} = P_{\text{req}}$ y $P_{\text{asignado}} = 0$.
2. Para cada activo $A_i$:
   - Si $\text{latencia}(A_i) > 250\,000\,\mu\text{s}$, abortar la transacción.
   - Calcular aporte: $\Delta P_i = \min(P_{\text{restante}}, H_i)$.
   - Actualizar acumuladores:
     $$P_{\text{asignado}} = P_{\text{asignado}} + \Delta P_i$$
     $$P_{\text{restante}} = P_{\text{restante}} - \Delta P_i$$
3. Retornar `DispatchAllocationResult` con el balance exacto asignado y remanente no cubierto.
