r"""Print the errors Axe.Windows recorded in .a11ytest files (zips holding el.snapshot, a JSON element tree with
ScanResults on each element). Reads only; no code from the files runs."""
import json
import os
import sys
import zipfile


def walk(node):
    yield node
    for child in node.get('Children') or []:
        yield from walk(child)


def describe(element):
    props = {p.get('Name'): p.get('TextValue') for p in (element.get('Properties') or {}).values()} if isinstance(element.get('Properties'), dict) else {}
    return props


for path in sys.argv[1:]:
    with zipfile.ZipFile(path) as z:
        data = json.loads(z.read('el.snapshot').decode('utf-8-sig'))
    errors = []
    for element in walk(data):
        results = (element.get('ScanResults') or {}).get('Items') or []
        for result in results:
            if result.get('Status') in (3, 'Fail', 'Error'):
                for item in result.get('Items') or [result]:
                    desc = item.get('Description') or result.get('Description')
                    errors.append(desc)
    name = os.path.basename(path)
    print(f'{name}: {len(errors)} error(s)')
    for e in errors:
        print(f'  error: {e}')
