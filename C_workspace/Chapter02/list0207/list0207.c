#include <stdio.h>
int main(void)
{
    double vx, vy;

    puts("二つの実数を入力してください。");
    printf("整数vx:");   scanf("%lf", &vx);
    printf("整数vy:");   scanf("%lf", &vy);

    printf("vx + vy = %f\n", vx + vy);
    printf("vx - vy = %f\n", vx - vy);
    printf("vx * vy = %f\n", vx * vy);
    printf("vx / vy = %f\n", vx / vy);

    return 0;
}