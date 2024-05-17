import re
import json


class Person:
    def __init__(self, name: str, color: str):
        self.name = name
        self.color = color

    def dict(self):
        return {'name': self.name, 'color': self.color}


class Replica:
    def __init__(self, person: int, msg: str):
        self.person = person
        self.msg = msg

    def dict(self):
        return {'person': self.person, 'text': self.msg}


class VoiceOver:
    def __init__(self, name: str, persons: [Person], replicas: [Replica]):
        self.name = name
        self.persons = persons
        self.replicas = replicas

    def dict(self):
        return {'name': self.name, 'persons': list(map(lambda x: x.dict(), self.persons)),
                'replicas': list(map(lambda x: x.dict(), self.replicas))}


PERSON_REGEX = re.compile(r'"([^"]+)" "([^"]+)"')
REPLICA_REGEX = re.compile(r'(\d+): (.+)')


def parse_person(s: str) -> Person | None:
    m = PERSON_REGEX.match(s)
    if m is not None:
        name, color = m.groups()
        return Person(name, color)
    return None


def parse_replica(s: str) -> Replica | None:
    m = REPLICA_REGEX.match(s)
    if m is not None:
        person, msg = m.groups()
        return Replica(int(person), msg)
    return None


def parse_voiceover(s):
    lines = s.splitlines()
    is_person = True
    persons = []
    replicas = []
    for line in lines[1:]:
        if is_person:
            p = parse_person(line)
            if p is None:
                is_person = False
            else:
                persons.append(p)
        if not is_person:
            r = parse_replica(line)
            if r is None:
                return None
            replicas.append(r)
    return VoiceOver(lines[0], persons, replicas)


def read_lines():
    lines = []
    while True:
        try:
            lines.append(input())
        except EOFError:
            break
    return lines

if __name__ == '__main__':
    lines = '\n'.join(read_lines())
    voiceover = parse_voiceover(lines)
    p = None
    if voiceover is None:
        exit(1)
    print(json.dumps(voiceover.dict()))
