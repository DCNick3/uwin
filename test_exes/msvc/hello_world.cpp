
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>

//#include <algorithm>

// float random_float() {
// 	return static_cast <float> (rand()) / static_cast <float> (RAND_MAX);
// }
// 
// double random_double() {
// 	return static_cast <double> (rand()) / static_cast <double> (RAND_MAX);
// }
// 
// void showfloat(float a) {
// 	char numbuf[100];
// 	sprintf(numbuf, "%f", a);
// 	MessageBoxA(0, numbuf, "Float:", 0);
// }
// 
// void showdouble(double a) {
// 	char numbuf[100];
// 	sprintf(numbuf, "%lf", a);
// 	MessageBoxA(0, numbuf, "Double:", 0);
// }

int PASCAL WinMain( HINSTANCE hInstance, HINSTANCE hPrevInstance,
                        LPSTR lpCmdLine, int nCmdShow)
{
	MessageBoxA(0, "Hello world!", "World, hello!", MB_OK);

// 	showfloat(1.337f);
// 	showdouble(1.338);

    return 0;

} /* WinMain */
