/*
 * main.c
 *
 *  Created on: 2019/02/05
 *      Author: sigur
 */
/* main.c */
#include <stdio.h>
#include "euclid.h"

main()
{
   int x, y, z;

   x = input_data();
   y = input_data();
   fprintf(stderr, "Inputed Data are %d and %d\n", x, y);

   z = euclid(x, y);
   output_data(z);
}
