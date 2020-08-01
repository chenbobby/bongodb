# Test Convenience Script
# CS 165
# Contact: Wilson Qin

# note this should be run from base project folder as `./infra_scripts/run_test 01`
# note this should be run inside the docker container

# If a container is already successfully running after `make startcontainer outputdir=<ABSOLUTE_PATH1> testdir=<ABSOLUTE_PATH2>`
# This endpoint takes a `TEST_ID` argument, from 01 up to 43,
#     runs the corresponding generated test DSLs
#    and checks the output against corresponding EXP file.

TEST_ID=$1
OUTPUT_DIR=${2:-"$HOME/bongodb/cs165_project_tests/test_output"}
DATA_DIR=${3:-"$HOME/bongodb/cs165_project_tests/generated_data"}

echo "Running test # $TEST_ID"

if [ -d $OUTPUT_DIR ]; then
    rm -r $OUTPUT_DIR
fi

mkdir $OUTPUT_DIR

# collect the client output for this test case by TEST_ID
bongodb-client < $DATA_DIR/test${TEST_ID}gen.dsl 2> ${OUTPUT_DIR}/test${TEST_ID}gen.out.err 1> ${OUTPUT_DIR}/test${TEST_ID}gen.out
# run the "comparison" script for comparing against expected output for TEST_ID
./verify_output_standalone.sh $TEST_ID ${OUTPUT_DIR}/test${TEST_ID}gen.out ${DATA_DIR}/test${TEST_ID}gen.exp ${OUTPUT_DIR}/test${TEST_ID}gen.cleaned.out ${OUTPUT_DIR}/test${TEST_ID}gen.cleaned.sorted.out
