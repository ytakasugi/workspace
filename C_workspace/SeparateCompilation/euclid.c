/*
 * euclid.c
 *
 *  Created on: 2019/02/05
 *      Author: sigur
 */
int  euclid(int i, int j)
{
   while (i!=j) {
       if (i<j) {
           int swp = i;
           i = j;
           j = swp;
       }
       i = i - j;
   }
   return i;
}
