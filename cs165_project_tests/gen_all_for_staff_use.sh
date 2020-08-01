#!/bin/bash
# Example Usage:
#   ./gen_all_for_bobs_use.sh ~/repo/cs165-docker-test-runner/test_data

BASE_DIR="${1:-$(pwd)}"
OUTPUT_TEST_DIR="$BASE_DIR/generated_data"

_DOCKER_TEST_DIR="${2}" # Deprecated

TBL_SIZE="${3:-100}"
RAND_SEED="${4:-165}"
JOIN_DIM1_SIZE="${5:-100}"
JOIN_DIM2_SIZE="${6:-100}"
ZIPFIAN_PARAM="${7:-1.0}"
NUM_UNIQUE_ZIPF="${8:-100}"

echo "Generated files will be stored in: $OUTPUT_TEST_DIR"
echo "DSL scripts will use data size: $TBL_SIZE"
echo "DSL scripts will use random seed: $RAND_SEED"

if [ -d $OUTPUT_TEST_DIR ]; then
    echo "Deleting existing generated files..."
    rm -r $OUTPUT_TEST_DIR
fi


echo "Beginning data generation..."
mkdir $OUTPUT_TEST_DIR
source $BASE_DIR/venv/bin/activate

python $BASE_DIR/data_generation_scripts/milestone1.py $TBL_SIZE $RAND_SEED ${OUTPUT_TEST_DIR} ${OUTPUT_TEST_DIR}
echo "Generated data for milestone 1."

python $BASE_DIR/data_generation_scripts/milestone2.py $TBL_SIZE $RAND_SEED ${OUTPUT_TEST_DIR} ${OUTPUT_TEST_DIR}
echo "Generated data for milestone 2."

python $BASE_DIR/data_generation_scripts/milestone3.py $TBL_SIZE $RAND_SEED ${OUTPUT_TEST_DIR} ${OUTPUT_TEST_DIR}
echo "Generated data for milestone 3."

python $BASE_DIR/data_generation_scripts/milestone4.py $TBL_SIZE $JOIN_DIM1_SIZE $JOIN_DIM2_SIZE $RAND_SEED $ZIPFIAN_PARAM $NUM_UNIQUE_ZIPF ${OUTPUT_TEST_DIR} ${OUTPUT_TEST_DIR}
echo "Generated data for milestone 4."

python $BASE_DIR/data_generation_scripts/milestone5.py $TBL_SIZE $RAND_SEED ${OUTPUT_TEST_DIR} ${OUTPUT_TEST_DIR}
echo "Generated data for milestone 5."

deactivate

echo "Data generation completed."
