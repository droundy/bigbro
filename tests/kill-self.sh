set -ev

cd tmp/root_symlink/tmp
echo foo > foo

echo I am $$

kill $$
