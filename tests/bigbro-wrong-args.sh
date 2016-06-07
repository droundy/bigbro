set -ev

if ./bigbro; then
    echo this should have failed due to lacking an argument
    exit 1
fi
