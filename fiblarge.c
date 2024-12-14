#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>

int main(int argc, char **argv) {
    int n = atoi(argv[1]);
    mpz_t a, b;
    mpz_init2(a, 3 * n);
    mpz_init2(b, 3 * n);
    mpz_set_ui(a, 1);
    mpz_set_ui(b, 1);
    for (int i = 0; i < n / 2; i++) {
        mpz_add(a, a, b);
        mpz_add(b, a, b);
    }
    gmp_printf("%Zd\n", b);
}
