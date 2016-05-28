set -ev

cd tmp/root_symlink/tmp

echo foo > foo
truncate --size 0 foo

truncate --size 0 foobar
