
// a solution to a codeforces problem (https://codeforces.com/problemset/problem/1031/B)
// can be tested via competest (https://pypi.org/project/competest/)
// for example:
// $ competest exe "target/release/test_fixture" -t "test_exes/msvc/cf1031B_tests.txt"

#include <stdlib.h>
#include <stdio.h>

#define bool char
#define true 1
#define false 0

int n;

bool* bits(int* v, int k) {
    int i;
    bool* r;

    r = calloc(n - 1, sizeof(bool));

    for (i = 0; i < n - 1; i++) {
        r[i] = ((v[i] >> k) & 1) != 0;
    }
    return r;
}

int tnext(bool a, bool b, bool t) {
    if (!a && !b && !t) return 0;
    if (a && !b && !t) return 1;
    if (a && !b && t) return 0;
    if (a && b && t) return 1;
    return -1;
}

bool* find_seqs(bool t0, bool* a, bool* b) {
    int i;
    bool* t;

    t = calloc(n, sizeof(bool));
    t[0] = t0;
    for (i = 0; i < n - 1; i++) {
        int v = tnext(a[i], b[i], t[i]);
        if (v == -1)
        {
            free(t);
            return NULL;
        }
        t[i + 1] = v != 0;
    }
    return t;
}

int main() {
    int i;
    int *a, *b;
    bool *a0, *a1, *b0, *b1, *p0, *p1;
    scanf("%d", &n);

    a = calloc(n - 1, sizeof(int));
    b = calloc(n - 1, sizeof(int));

    for (i = 0; i < n - 1; i++)
        scanf("%d", &a[i]);
    for (i = 0; i < n - 1; i++)
        scanf("%d", &b[i]);

    a0 = bits(a, 0);
    a1 = bits(a, 1);
    b0 = bits(b, 0);
    b1 = bits(b, 1);

    p0 = find_seqs(false, a0, b0);
    if (p0 == NULL)
        p0 = find_seqs(true, a0, b0);

    p1 = find_seqs(false, a1, b1);
    if (p1 == NULL)
        p1 = find_seqs(true, a1, b1);

    if (p0 == NULL || p1 == NULL) {
        printf("NO\n");
    } else {
        printf("YES\n");
        for (i = 0; i < n; i++) {
            int v = p0[i] | (p1[i] << 1);
            printf("%d ", v);
        }
    }
    printf("\n");

    free(a);
    free(b);
    free(a0);
    free(a1);
    free(b0);
    free(b1);
    free(p0);
    free(p1);
}
