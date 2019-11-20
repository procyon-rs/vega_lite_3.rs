example=$1

echo "copying $example"

file=`basename $example`

cp target/debug/examples/$file target/debug/example-$file
