import os

with open('tmp/hello-tmp', 'w') as f:
    f.write('test')

os.rename('tmp/hello-tmp',  'tmp/hello-final')
