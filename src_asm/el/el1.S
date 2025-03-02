/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Archivo: el1.S
 * Punto de entrada en EL1.
 *
 * Este código es invocado desde boot.S una vez realizada la transición de EL2 a EL1.
 * Se configura el entorno de EL1 y luego se transfiere el control al código Rust.
 */

.global switch_to_el1
switch_to_el1:
    /* Configurar el vector de excepciones para EL1 */
    adrp    x0, vector_table       /* Carga la base de la tabla de vectores */
    add     x0, x0, :lo12:vector_table
    msr     VBAR_EL1, x0           /* Establece VBAR_EL1 para EL1 */

    /* Opcional: Inicialización adicional del entorno de EL1 */
    /* Por ejemplo, configurar SCTLR_EL1, MMU, cachés, etc. */
    /*
       mov     x0, #<valor_deseado>
       msr     SCTLR_EL1, x0
    */

    /* Transferir el control a la función externa de Rust (_start_rust) */
    bl _start_rust

.size switch_to_el1, . - switch_to_el1

/* Definición de una tabla de vectores de excepción mínima */
.section .text.vector_table, "ax"
.align 7
vector_table:
    /* Rutina por defecto: en caso de excepción, se entra en un bucle infinito */
    b .
