#include <assert.h>
#include <stdio.h>
#include "crosu.h"

char* TEST_FILE  = "./tests/files/2785319.osu";
Score TEST_SCORE = {
      .combo  = 909,
      .n300   = 909,
      .n100   = 0,
      .n050   = 0,
      .misses = 0
};

void test_get_pp_from_map() {
    f64 pp = GetPPFromMap(TEST_FILE, true, 0, Osu, TEST_SCORE);

    assert(pp > 0);
    assert(pp > 200);
    printf("test_get_pp_from_map OK\n");
}

int main() {
    test_get_pp_from_map();
    printf("TESTS OK\n");
}
