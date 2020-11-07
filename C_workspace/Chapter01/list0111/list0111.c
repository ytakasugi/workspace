#include <stdio.h>
int main(void)
{
    int no;

    printf("数値を入力してください:");
    scanf("%d", &no);

    printf("あなたは%dと入力しましたね。 \n", no);

    return 0;
}