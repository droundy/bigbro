def convertFile(binary_filename, header_filename, variable_name):
    with open(binary_filename, 'rb') as b:
        with open(header_filename, 'w') as h:
            h.write('''static unsigned char %s[] = { ''' % variable_name);
            h.write(',\n'.join('%d' % c for c in b.read()))
            h.write('\n};\n')
