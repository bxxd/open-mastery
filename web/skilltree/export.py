#!/usr/bin/env python3
"""Export a graph directory to skill-tree JSON: python3 export.py graph/language/english-spelling > out.json
Adds `members` for nodes whose context lists kana characters (so per-character
learner stats can satisfy row-level nodes)."""
import json, os, re, sys

root = sys.argv[1]
nodes = []
for dp, _, fs in os.walk(root):
    for f in sorted(fs):
        if not f.endswith('.yaml') or f == '_prompt.yaml':
            continue
        s = open(os.path.join(dp, f)).read()
        nid = re.search(r'^id: (.+)$', s, re.M).group(1)
        pre = re.findall(r'^  - (.+)$', s.split('bloom:')[0], re.M)
        ctx = re.search(r'^context: >\n((?:  .*\n?)+)', s, re.M)
        context = ' '.join(l.strip() for l in ctx.group(1).splitlines()) if ctx else ''
        region = os.path.relpath(dp, root).split(os.sep)[0]
        if region == '.': region = ''
        node = {'id': nid, 'name': nid.split('.')[-1].replace('_', ' '), 'region': region,
                'prereqs': pre, 'context': context}
        kana = re.findall(r'[ぁ-ゟ゠-ヿ]', context)
        run = [c for c in kana if c not in 'ーゃゅょャュョっッ']
        if len(run) >= 3 and ('.kana.' in nid or '.kata.' in nid):
            node['members'] = run[:5]
        nodes.append(node)
json.dump({'nodes': nodes}, sys.stdout, ensure_ascii=False, indent=1)
