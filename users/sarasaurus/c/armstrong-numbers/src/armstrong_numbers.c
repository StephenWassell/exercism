#include "armstrong_numbers.h"
#include <stdio.h> 
#include <math.h>

#define FALSE (1==0)
#define TRUE  (1==1)


int is_armstrong_number(int candidate) 
{
	// candidate = 0;
	//int len_cad = 0;
	// printf("candidate: %d\n", candidate);
	// set up vars to hold sum, array of number broken into individual parts, and the divisor to get each number
	int sum = 0;
	int arr[100] = {0};
	double divisor = 10.0;
	
	// i up to 100 to account for very long numbers
	// divides by larger and larger squares of 10, this breaks the number into it's indivudual digits
	// and allows to count how many digits per num 
	// once digit it extracted, put in array so can use for the is armstrong calcs
	for (int i=0;i<=100;i++) {
		double num = candidate / divisor;
		// printf("[Beginning] Value: %d Divisor: %f Count: %d\n", arr[i], divisor, i);
		if (num >= 1) {
			num = ((num - floor(num)) * 10 + .000001); // add very small number because was having issue with 2.000 becoming 1 when putting in array (int)
			// printf("i: %d num: %f\n", i, num);
			arr[i] = num;
			// printf("%d %d %f\n", i, arr[i], num);
		} 
		// once we get to last digit in number, add it to array 
		// then determine if armstrong num in for loop
		else if (num < 1) {
			// printf("else loop %d\n", i);
			num = num * 10;
			arr[i] = num;
			for (int j=0;j<=i;j++) {
				sum = sum + pow(arr[j], (i+1)) ;
				// printf("num: %d sum: %d base: %d\n", arr[j], sum, (i+1));
				
			}
			// printf("%d\n", sum);
			// printf("candidate: %d sum: %d\n", candidate, sum);

			// tell test if armstrong num or not
			if (candidate == sum) {
				// printf("armstrong num\n");
				return TRUE;
			} else {
				// printf("NOT armstrong num\n");
				return FALSE;				
			}
			break;
		}
		// printf("Value: %d Divisor: %f Count: %d num: %f\n", arr[i], divisor, i, num);

		divisor = divisor * 10;
		
	}
	


	// if (modulus < 1) {
	// 	//len_cad = 1;
	// 	//arr[0] = candidate;
	// } else if (modulus < 10) {
	// 	//len_cad = 2;

	// }

	return 0;
}
