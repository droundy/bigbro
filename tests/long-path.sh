set -ev

cd tmp/root_symlink/tmp

# we create a very nested structure with lots of files.
for n in `seq 80`; do
    NAME=$(printf "%-${n}s" "file").dat
    echo $n "$NAME"
    echo hello world > "$NAME"
    DIRNAME=$(printf "%-${n}s" "dir")foo
    echo dir $n "$DIRNAME"
    mkdir "$DIRNAME"
    cd "$DIRNAME"
done
