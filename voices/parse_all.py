import os
import json
import parse_voiceover

os.makedirs('./jsons/', exist_ok=True)
print(os.get_exec_path())

for filename in os.listdir('./sources'):
    with open(f'./sources/{filename}', 'r', encoding='UTF-8') as f:
        with open(f'./jsons/{filename + '.json'}', 'w') as f1:
            lines = ''.join(f.readlines())
            voiceover = parse_voiceover.parse_voiceover(lines)
            if voiceover is None:
                print(f'Error while parsing {filename} ')
            else:
                f1.write(json.dumps(voiceover.dict()))
