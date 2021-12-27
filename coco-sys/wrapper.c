#include "vendor/coco/code-experiments/src/coco.h"
#include "vendor/coco/code-experiments/src/coco_internal.h"

void coco_suite_forget_current_problem(coco_suite_t *suite)
{
    suite->current_problem = NULL;
}
