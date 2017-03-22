import importlib

try:
    test = importlib.import_module('tests.noninterference.test')
except:
    test = importlib.import_module('test')

def run():
    return test.same_output(['git', '--version'])
