/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Bootloader simplificado:
 * - Inicializa la BSS (pone a cero el segmento .bss).
 * - Configura el puntero de pila.
 * - Salta a la entrada de Rust (_start_rust) en EL1.
 */

/* Macro para cargar una dirección de un símbolo de forma PC-relativa */
.macro ADR_REL reg, sym
	adrp	\reg, \sym
	add	\reg, \reg, #:lo12:\sym
.endm

.section .text._start
.global _start
_start:
	/* Inicialización de la BSS */
	ADR_REL	x0, __bss_start           /* Dirección de inicio de .bss */
	ADR_REL	x1, __bss_end_exclusive     /* Dirección final de .bss */

.L_bss_init_loop:
	cmp	x0, x1
	beq	.L_prepare_rust           /* Si se ha recorrido toda la BSS, saltar */
	stp	xzr, xzr, [x0], #16        /* Escribe 0 en dos registros de 64 bits y avanza 16 bytes */
	b	.L_bss_init_loop

.L_prepare_rust:
	/* Configurar el puntero de pila para EL1 */
	ADR_REL	x0, __boot_core_stack_end_exclusive   /* Carga la dirección final de la pila */
	mov	sp, x0                                   /* Usar la pila actual para la fase de boot */
	msr	SP_EL1, x0                              /* Establecer SP_EL1 para EL1 */

	/* Configurar SPSR_EL2 para regresar a EL1 en modo AArch64 con SP_EL1 (EL1h)
	   y con interrupciones enmascaradas (por ejemplo, con valor 0x3c5) */
	mov	x0, #0x3c5
	msr	SPSR_EL2, x0

	/* Configurar HCR_EL2 para que EL1 se ejecute en AArch64 */
    mov     x0, #(1 << 31)    /* Establece RW=1 */
    msr     HCR_EL2, x0

	/* Configurar ELR_EL2 con la dirección de entrada en EL1 (switch_to_el1) */
	ADR_REL	x0, switch_to_el1
	msr	ELR_EL2, x0

	/* Realiza la transición a EL1: eret usará SPSR_EL2 y ELR_EL2 para cambiar a EL1 */
	eret

.size _start, . - _start
