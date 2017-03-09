set -ev

cd tmp/root_symlink/tmp

sleep 60 &
ID=$!

kill $ID
