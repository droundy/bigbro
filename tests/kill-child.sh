set -ev

cd tmp/root_symlink/tmp

sleep 60 &

# I hope the following is safe.  Who creates important processes
# called "sleep"?
killall sleep
