// #include <stdio.h>
// #include "hardware/gpio.h"
// #include "hardware/structs/systick.h"
// #include "hardware/clocks.h"
// #include "hardware/pll.h"
// #include "pico/stdlib.h"

// const uint LED_PIN = PICO_DEFAULT_LED_PIN;
// const uint LED_PIN_T1 = 10;
// const uint LED_PIN_T2 = 11;
// const uint LED_PIN_T3 = 12;

// void task0();
// void task1();
// void task2();

// extern scheduler;

// // Task 0
// void task0(void)
// {
//     // printf("[ Début tâche 1 %d %d\n", schedule.current_process, NOW);
//     printf("D %d %d\n", scheduler.current_process, NOW);
//     gpio_put(LED_PIN_T1, 1);
//     while (task0_val < 10000000) // 10 000 000
//     {
//         task0_val += 1;
//         if (task0_val % 100000 == 0)
//             printf("Task 0\n");
//     }
//     // printf("[ Fin tâche 1 %d %d\n", scheduler.current_process, NOW);
//     gpio_put(LED_PIN_T1, 0);
//     task0_val = 0;
//     printf("F %d %d\n", scheduler.current_process, NOW);
// }

// // Task 1
// void task1(void)
// {
//     printf("D %d %d\n", scheduler.current_process, NOW);
//     gpio_put(LED_PIN_T2, 1);
//     while (task1_val < 2000000) // 2 000 000
//     {
//         task1_val += 1;
//         // if (task1_val % 100000 == 0) printf("Task 1\n");
//     }
//     // printf("[ Fin tâche 2 %d %d\n", scheduler.current_process, NOW);
//     gpio_put(LED_PIN_T2, 0);
//     task1_val = 0;
//     printf("F %d %d\n", scheduler.current_process, NOW);
// }

// int i = 0;
// // Task 2
// void task2(void)
// {
//     printf("D %d %d\n", scheduler.current_process, NOW);
//     gpio_put(LED_PIN_T3, 1);
//     while (i < 1000000) // 1 000 000
//     {
//         // if (i % 100000 == 0) printf("Task 2\n");
//         i++;
//     }
//     gpio_put(LED_PIN_T3, 0);
//     i = 0;
//     printf("D %d %d\n", scheduler.current_process, NOW);
//     return;
// }