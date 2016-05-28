set -ev

cd tmp/root_symlink/tmp

for i in `seq 10000`; do
    # echo creating file-$i.dat
    echo hello world > file-$i.dat
done

for i in `seq 10000 | sort -R`; do
    # echo removing file-$i.dat
    rm file-$i.dat
done
