example=$1

file=`basename $example`

cp target/debug/examples/$file target/debug/example-$file
