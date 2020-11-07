/*
 * input.c
 *
 *  Created on: 2019/02/05
 *      Author: sigur
 */
#include <stdio.h>

int input_data()
{
   int i;

   fprintf(stderr, "Input Num = ");
   scanf("%d", &i);

   return i;
}

