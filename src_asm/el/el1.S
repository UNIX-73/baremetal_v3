.global switch_to_el1
switch_to_el1:
    /* Configurar la tabla de excepciones en VBAR_EL1 */
    bl irq_init_vectors

    /* Habilitar interrupciones IRQ en EL1 */
    bl irq_enable

    /* Configurar el puntero de pila para EL1 
    ldr x0, =stack_top   // Cargar dirección de la pila
    msr sp_el1, x0       // Configurar SP_EL1
    mov sp, x0           // Usar SP_EL1 como pila actual*/

    /* Opcional: Configuración de control de sistema */
    /*
       mov x0, #valor
       msr SCTLR_EL1, x0
    */

    /* Transferir el control a la función externa de Rust */
    bl _start_rust

.size switch_to_el1, . - switch_to_el1
