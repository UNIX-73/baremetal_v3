/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Bootloader simplificado:
 * - Inicializa la BSS (pone a cero el segmento .bss).
 * - Configura el puntero de pila.
 * - Salta a la entrada de Rust (_start_rust).
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
	ADR_REL	x0, __bss_start      /* Dirección de inicio de .bss */
	ADR_REL	x1, __bss_end_exclusive /* Dirección final de .bss */

.L_bss_init_loop:
	cmp	x0, x1
	beq	.L_prepare_rust      /* Si se ha recorrido toda la BSS, saltar */
	stp	xzr, xzr, [x0], #16   /* Escribe 0 en dos registros de 64 bits y avanza 16 bytes */
	b	.L_bss_init_loop

.L_prepare_rust:
	/* Configuración del puntero de pila */
	ADR_REL	x0, __boot_core_stack_end_exclusive /* Carga la dirección final de la pila */
	mov	sp, x0

	/* Salta a la entrada en Rust */
	b	_start_rust

.size _start, . - _start
